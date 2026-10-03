//! "Worth a look": the biggest things in a scan that could plausibly go.
//!
//! Three kinds of finding, each one a directory a person can judge in a
//! second: reclaimable space ([`crate::classify::Reclaim`]), agent worktrees,
//! and experiments nobody has written to in a month. Findings never nest, so
//! their total is space that really exists once.

use crate::classify::{Category, Reclaim};
use crate::tree::Node;

/// Seconds in a day.
const DAY: i64 = 86_400;

/// An experiment untouched this long is worth a look.
pub const STALE_DAYS: i64 = 30;

/// Smaller than this is not worth a line.
const MIN_BYTES: u64 = 64 * 1024 * 1024;

/// Why a directory is on the list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Finding {
    /// Its space can be had back, for this reason.
    Reclaimable(Reclaim),
    /// A directory of agent worktrees: how many, and the oldest one's age.
    Worktrees { count: usize, oldest_days: i64 },
    /// Experiments untouched for [`STALE_DAYS`]: only those are counted.
    StaleExperiments { count: usize },
}

/// One line of the list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Candidate {
    /// Where it is, from the scanned root.
    pub crumbs: Vec<usize>,
    /// What clearing it frees. For stale experiments, only the stale ones.
    pub bytes: u64,
    pub finding: Finding,
}

/// The `limit` largest findings beneath `root`, largest first. `now` is Unix
/// seconds, passed in so the answer is testable.
pub fn worth_a_look(root: &Node, now: i64, limit: usize) -> Vec<Candidate> {
    let mut found = Vec::new();
    let mut crumbs = Vec::new();
    for (index, child) in root.children.iter().enumerate() {
        crumbs.push(index);
        visit(child, &mut crumbs, now, &mut found);
        crumbs.pop();
    }
    found.retain(|candidate| candidate.bytes >= MIN_BYTES);
    found.sort_by_key(|candidate| std::cmp::Reverse(candidate.bytes));
    found.truncate(limit);
    found
}

fn visit(
    node: &Node,
    crumbs: &mut Vec<usize>,
    now: i64,
    found: &mut Vec<Candidate>,
) {
    // Nothing beneath a small directory can reach `MIN_BYTES` either, so a
    // scan of millions of files looks at a few thousand directories.
    if !node.is_dir() || node.bytes < MIN_BYTES {
        return;
    }
    // Topmost only: everything beneath a reclaimable directory goes with it.
    if let Some(reason) = node.reclaim {
        found.push(Candidate {
            crumbs: crumbs.clone(),
            bytes: node.bytes,
            finding: Finding::Reclaimable(reason),
        });
        return;
    }
    let name = &*node.name;
    let scratch = node.category == Category::AgentScratch;
    if scratch && name.eq_ignore_ascii_case("worktrees") {
        let trees: Vec<&Node> = node
            .children
            .iter()
            .filter(|child| child.is_dir())
            .collect();
        if !trees.is_empty() {
            let oldest = trees
                .iter()
                .map(|tree| tree.modified)
                .filter(|&time| time > 0)
                .min()
                .unwrap_or(now);
            found.push(Candidate {
                crumbs: crumbs.clone(),
                bytes: node.bytes,
                finding: Finding::Worktrees {
                    count: trees.len(),
                    oldest_days: (now - oldest).max(0) / DAY,
                },
            });
            return;
        }
    }
    let experiments = scratch
        && (name.eq_ignore_ascii_case("tries")
            || name.eq_ignore_ascii_case("experiments"));
    let mut stale = (0_usize, 0_u64);
    for (index, child) in node.children.iter().enumerate() {
        let is_stale = experiments
            && child.is_dir()
            && child.modified > 0
            && now - child.modified > STALE_DAYS * DAY;
        if is_stale {
            stale.0 += 1;
            stale.1 += child.bytes;
            // A stale experiment is judged whole; its caches go with it.
            continue;
        }
        crumbs.push(index);
        visit(child, crumbs, now, found);
        crumbs.pop();
    }
    if stale.0 > 0 {
        found.push(Candidate {
            crumbs: crumbs.clone(),
            bytes: stale.1,
            finding: Finding::StaleExperiments { count: stale.0 },
        });
    }
}

/// One entry in the largest files list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LargestFile {
    /// Where it is, from the scanned root.
    pub crumbs: Vec<usize>,
    /// Its size in bytes.
    pub bytes: u64,
}

/// A group of duplicate files sharing the same byte size.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DuplicateGroup {
    /// Size of each duplicate file in bytes.
    pub bytes: u64,
    /// Total potential space wasted: `bytes * (files.len() - 1)`.
    pub wasted_bytes: u64,
    /// The duplicate files in this group.
    pub files: Vec<DuplicateFile>,
}

/// A duplicate file candidate within a group.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DuplicateFile {
    /// File name.
    pub name: String,
    /// Path crumbs from root.
    pub crumbs: Vec<usize>,
    /// File size in bytes.
    pub bytes: u64,
}

/// The `limit` largest files beneath `root`, largest first.
pub fn largest_files(root: &Node, limit: usize) -> Vec<LargestFile> {
    if limit == 0 {
        return Vec::new();
    }
    let mut heap = std::collections::BinaryHeap::with_capacity(limit);
    let mut crumbs = Vec::new();
    for (index, child) in root.children.iter().enumerate() {
        crumbs.push(index);
        visit_largest_files(child, &mut crumbs, limit, &mut heap);
        crumbs.pop();
    }
    let mut files: Vec<LargestFile> = heap
        .into_iter()
        .map(|std::cmp::Reverse((bytes, crumbs))| LargestFile { crumbs, bytes })
        .collect();
    files.sort_by_key(|candidate| std::cmp::Reverse(candidate.bytes));
    files
}

/// Detect groups of duplicate files beneath `root`, sorted by wasted bytes.
///
/// Files are considered duplicate candidates if they have non-zero identical
/// sizes and either:
/// 1. Share the exact same name, or
/// 2. Are >= 64 KiB with identical byte counts.
pub fn duplicate_files(root: &Node, limit: usize) -> Vec<DuplicateGroup> {
    if limit == 0 {
        return Vec::new();
    }
    let mut files_by_size: rustc_hash::FxHashMap<u64, Vec<DuplicateFile>> =
        rustc_hash::FxHashMap::default();
    let mut crumbs = Vec::new();
    for (index, child) in root.children.iter().enumerate() {
        crumbs.push(index);
        collect_duplicate_candidates(child, &mut crumbs, &mut files_by_size);
        crumbs.pop();
    }

    let mut groups = Vec::new();
    for (bytes, files) in files_by_size {
        if files.len() < 2 || bytes == 0 {
            continue;
        }
        // Sub-partition: files >= 64 KiB with byte-exact match are candidates.
        // Files < 64 KiB must also share the same name to avoid false matches.
        if bytes >= 64 * 1024 {
            let wasted = bytes.saturating_mul(files.len() as u64 - 1);
            groups.push(DuplicateGroup {
                bytes,
                wasted_bytes: wasted,
                files,
            });
        } else {
            let mut by_name: rustc_hash::FxHashMap<String, Vec<DuplicateFile>> =
                rustc_hash::FxHashMap::default();
            for file in files {
                by_name.entry(file.name.clone()).or_default().push(file);
            }
            for (_, name_files) in by_name {
                if name_files.len() >= 2 {
                    let wasted =
                        bytes.saturating_mul(name_files.len() as u64 - 1);
                    groups.push(DuplicateGroup {
                        bytes,
                        wasted_bytes: wasted,
                        files: name_files,
                    });
                }
            }
        }
    }

    groups.sort_by_key(|group| std::cmp::Reverse(group.wasted_bytes));
    if groups.len() > limit {
        groups.truncate(limit);
    }
    groups
}

fn collect_duplicate_candidates(
    node: &Node,
    crumbs: &mut Vec<usize>,
    acc: &mut rustc_hash::FxHashMap<u64, Vec<DuplicateFile>>,
) {
    if node.is_dir() {
        for (index, child) in node.children.iter().enumerate() {
            crumbs.push(index);
            collect_duplicate_candidates(child, crumbs, acc);
            crumbs.pop();
        }
    } else if node.kind == crate::tree::NodeKind::File && node.bytes > 0 {
        acc.entry(node.bytes).or_default().push(DuplicateFile {
            name: node.name.to_string(),
            crumbs: crumbs.clone(),
            bytes: node.bytes,
        });
    }
}

fn visit_largest_files(
    node: &Node,
    crumbs: &mut Vec<usize>,
    limit: usize,
    heap: &mut std::collections::BinaryHeap<
        std::cmp::Reverse<(u64, Vec<usize>)>,
    >,
) {
    if node.is_dir() {
        // Pruning: if we already have `limit` files and this directory's total
        // bytes is no larger than the smallest file in our heap, no file
        // within this subtree can displace anything in the top `limit`.
        if heap.len() == limit
            && let Some(&std::cmp::Reverse((min_bytes, _))) = heap.peek()
            && node.bytes <= min_bytes
        {
            return;
        }
        for (index, child) in node.children.iter().enumerate() {
            crumbs.push(index);
            visit_largest_files(child, crumbs, limit, heap);
            crumbs.pop();
        }
    } else if node.kind == crate::tree::NodeKind::File {
        if heap.len() < limit {
            heap.push(std::cmp::Reverse((node.bytes, crumbs.clone())));
        } else if let Some(mut top) = heap.peek_mut()
            && node.bytes > top.0.0
        {
            *top = std::cmp::Reverse((node.bytes, crumbs.clone()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classify::classify;
    use crate::tree::{Metric, NodeKind, aggregate};

    const GIB: u64 = 1024 * 1024 * 1024;
    const NOW: i64 = 1_800_000_000;

    fn file(name: &str, bytes: u64, days_old: i64) -> Node {
        let mut node = Node::entry(name, NodeKind::File, bytes);
        node.modified = NOW - days_old * DAY;
        node
    }

    fn dir(name: &str, children: Vec<Node>) -> Node {
        let mut node = Node::directory(name);
        node.children = children;
        node
    }

    fn scan(mut root: Node) -> Node {
        aggregate(&mut root, Metric::Bytes);
        classify(&mut root);
        root
    }

    fn home() -> Node {
        scan(dir(
            "tobi",
            vec![
                dir(
                    ".cache",
                    vec![dir("kache", vec![file("blob", 5 * GIB, 1)])],
                ),
                dir(
                    ".codex",
                    vec![dir(
                        "worktrees",
                        vec![
                            dir("a1", vec![file("x", 3 * GIB, 41)]),
                            dir("b2", vec![file("y", 2 * GIB, 3)]),
                        ],
                    )],
                ),
                dir(
                    "src",
                    vec![dir(
                        "tries",
                        vec![
                            dir("old", vec![file("z", 4 * GIB, 90)]),
                            dir(
                                "fresh",
                                vec![
                                    file("Cargo.toml", 1, 1),
                                    dir("target", vec![file("o", GIB, 1)]),
                                ],
                            ),
                        ],
                    )],
                ),
                dir("Documents", vec![file("tax.pdf", 9 * GIB, 400)]),
                dir("tiny", vec![dir(".cache", vec![file("t", 1024, 1)])]),
            ],
        ))
    }

    #[test]
    fn ranks_findings_largest_first_and_skips_the_tiny() {
        let found = worth_a_look(&home(), NOW, 10);
        let kinds: Vec<&Finding> = found.iter().map(|c| &c.finding).collect();
        assert_eq!(
            kinds,
            vec![
                &Finding::Reclaimable(Reclaim::Regenerable),
                &Finding::Worktrees {
                    count: 2,
                    oldest_days: 41
                },
                &Finding::StaleExperiments { count: 1 },
                &Finding::Reclaimable(Reclaim::BuildOutput),
            ]
        );
        assert_eq!(found[0].bytes, 5 * GIB);
        assert_eq!(found[2].bytes, 4 * GIB, "only the stale experiment counts");
    }

    #[test]
    fn documents_are_never_suggested() {
        let found = worth_a_look(&home(), NOW, 10);
        assert!(found.iter().all(|c| c.crumbs != vec![3]));
    }

    #[test]
    fn crumbs_address_the_finding_from_the_root() {
        let root = home();
        for candidate in worth_a_look(&root, NOW, 10) {
            let node = root.resolve(&candidate.crumbs).expect("resolves");
            assert!(node.is_dir());
        }
    }

    #[test]
    fn the_limit_keeps_the_largest() {
        let found = worth_a_look(&home(), NOW, 2);
        assert_eq!(found.len(), 2);
        assert_eq!(
            found[1].finding,
            Finding::Worktrees {
                count: 2,
                oldest_days: 41
            }
        );
    }

    #[test]
    fn largest_files_ranks_by_size_descending() {
        let root = home();
        let top = largest_files(&root, 3);
        assert_eq!(top.len(), 3);
        assert_eq!(top[0].bytes, 9 * GIB);
        assert_eq!(top[1].bytes, 5 * GIB);
        assert_eq!(top[2].bytes, 4 * GIB);

        let node0 = root.resolve(&top[0].crumbs).expect("resolves");
        assert_eq!(&*node0.name, "tax.pdf");
        assert_eq!(node0.kind, NodeKind::File);

        let node1 = root.resolve(&top[1].crumbs).expect("resolves");
        assert_eq!(&*node1.name, "blob");
        assert_eq!(node1.kind, NodeKind::File);

        let node2 = root.resolve(&top[2].crumbs).expect("resolves");
        assert_eq!(&*node2.name, "z");
        assert_eq!(node2.kind, NodeKind::File);
    }

    #[test]
    fn largest_files_skips_directories() {
        let root = home();
        let top = largest_files(&root, 10);
        for item in &top {
            let node = root.resolve(&item.crumbs).expect("resolves");
            assert_eq!(node.kind, NodeKind::File);
        }
    }

    #[test]
    fn largest_files_handles_zero_limit_and_empty_root() {
        let root = home();
        assert!(largest_files(&root, 0).is_empty());
        let empty = dir("empty", Vec::new());
        assert!(largest_files(&empty, 5).is_empty());
    }

    #[test]
    fn duplicate_files_finds_and_ranks_duplicates() {
        let root = scan(dir(
            "root",
            vec![
                dir(
                    "d1",
                    vec![
                        file("video.mp4", 100 * 1024 * 1024, 0),
                        file("shared.txt", 500, 0),
                    ],
                ),
                dir(
                    "d2",
                    vec![
                        file("video.mp4", 100 * 1024 * 1024, 0),
                        file("shared.txt", 500, 0),
                        file("unique.txt", 500, 0),
                    ],
                ),
            ],
        ));
        let dupes = duplicate_files(&root, 10);
        assert_eq!(dupes.len(), 2);
        assert_eq!(dupes[0].bytes, 100 * 1024 * 1024);
        assert_eq!(dupes[0].wasted_bytes, 100 * 1024 * 1024);
        assert_eq!(dupes[0].files.len(), 2);

        assert_eq!(dupes[1].bytes, 500);
        assert_eq!(dupes[1].wasted_bytes, 500);
        assert_eq!(dupes[1].files.len(), 2);
    }
}
