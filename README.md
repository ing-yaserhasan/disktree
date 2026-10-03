# disktree

<div align="center">

[![Release](https://img.shields.io/github/v/release/ing-yaserhasan/disktree?style=for-the-badge&color=3b82f6)](https://github.com/ing-yaserhasan/disktree/releases/latest)
[![Build & CI](https://img.shields.io/github/actions/workflow/status/ing-yaserhasan/disktree/ci.yml?branch=main&style=for-the-badge&label=Build%20%26%20CI)](https://github.com/ing-yaserhasan/disktree/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-10b981.svg?style=for-the-badge)](LICENSE)
[![Platforms](https://img.shields.io/badge/Platforms-Windows%20%7C%20macOS%20%7C%20Linux-f59e0b?style=for-the-badge)](https://github.com/ing-yaserhasan/disktree/releases/latest)
[![Rust](https://img.shields.io/badge/Rust-1.97+-black?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![GUI](https://img.shields.io/badge/GUI-GPUI-8b5cf6?style=for-the-badge)](https://gpui-kit.com/)

<br/>

**[English](#english)** • **[Deutsch](#deutsch)** • **[العربية](#العربية)** • **[日本語](#日本語)** • **[简体中文](#简体中文)** • **[Français](#français)** • **[Türkçe](#türkçe)** • **[Español](#español)** • **[Русский](#русский)** • **[Українська](#українська)**

</div>

---

![disktree screenshot](assets/screenshot.png)

---

## ⚡ Direct Downloads / Sofort-Downloads (v0.10.4)

| Operating System / Betriebssystem | Architecture / Architektur | Format | Direct Download / Direktdownload |
| :--- | :--- | :--- | :--- |
| **🪟 Windows** | **x86_64 (Intel / AMD 64-bit)** | Portable `.zip` (Ready to run) | [📥 **Download Windows x64**](https://github.com/ing-yaserhasan/disktree/releases/download/v0.10.4/disktree-0.10.4-x86_64-windows.zip) |
| **🪟 Windows** | **ARM64** | Portable `.zip` | [📥 **Download Windows ARM64**](https://github.com/ing-yaserhasan/disktree/releases/download/v0.10.4/disktree-0.10.4-aarch64-windows.zip) |
| **🍏 macOS** | **Apple Silicon (M1 / M2 / M3 / M4)** | `.zip` (`disktree.app`) | [📥 **Download macOS Apple Silicon**](https://github.com/ing-yaserhasan/disktree/releases/download/v0.10.4/disktree-0.10.4-aarch64-macos.zip) |
| **🍏 macOS** | **Intel (x86_64)** | `.zip` (`disktree.app`) | [📥 **Download macOS Intel**](https://github.com/ing-yaserhasan/disktree/releases/download/v0.10.4/disktree-0.10.4-x86_64-macos.zip) |
| **🐧 Linux** | **x86_64** | `.tar.gz` (Binary + Installer) | [📥 **Download Linux x86_64**](https://github.com/ing-yaserhasan/disktree/releases/download/v0.10.4/disktree-0.10.4-x86_64-linux.tar.gz) |
| **🐧 Linux** | **ARM64 (aarch64)** | `.tar.gz` (Binary + Installer) | [📥 **Download Linux ARM64**](https://github.com/ing-yaserhasan/disktree/releases/download/v0.10.4/disktree-0.10.4-aarch64-linux.tar.gz) |

> 💡 All releases and verification checksums are available on the [GitHub Releases Page](https://github.com/ing-yaserhasan/disktree/releases/latest).

---

# English

**disktree** is a blazing-fast, modern disk usage visualizer and cleaner built with **Rust** and **GPUI**. It represents your disk space as an interactive, squarified treemap mosaic colored by data type, allowing you to instantly locate giant folders, mark unneeded files, and reclaim storage safely with real-time free space projections.

### 🌟 Key Highlights
- 🔍 **Permanent Search Bar:** Always accessible at the top. Click to filter items by name with instant visual feedback and a quick clear (`✕`) button.
- ⬆ **One-Click Parent Navigation:** Dedicated `▲` button to jump up through folder levels effortlessly.
- 🗑 **Direct & Safe Deletion:** Delete items directly via sidebar button, right-click context menu, or the `Del` key, backed by smart confirmation dialogs (Recycle Bin vs. Permanent).
- ⚡ **Extreme Performance:** Scans millions of files in seconds. On Windows, reads directly from the NTFS Master File Table (MFT) when elevated.
- 🎨 **Rich Visual Classification:** Color-coded by file kind (code, media, toolchains, caches, git artifacts, and documents).
- 🛡️ **Guaranteed Safety:** Never deletes system directories, profile roots, or mount points. Deletions are strictly contained within scanned boundaries.

### Installation & Quick Start

#### Windows
Download `disktree-*-x86_64-windows.zip`, extract it anywhere, and double-click `disktree.exe`. No installation required!
- Statically linked C runtime (`+crt-static`) guarantees seamless execution on any Windows 10/11 machine without extra runtime dependencies.
- Run as Administrator for instant MFT-accelerated disk scanning.

#### macOS
Download the appropriate `.zip` for your architecture (Apple Silicon or Intel), unzip, and drag `disktree.app` into your Applications folder.
- If Gatekeeper flags the application on first launch, run:
  ```sh
  xattr -dr com.apple.quarantine /Applications/disktree.app
  ```
- Grant **Full Disk Access** in *System Settings › Privacy & Security* to allow scanning user caches and protected app data.

#### Linux
Download `disktree-*-linux.tar.gz`, extract it, and execute:
```sh
./install.sh
```
Or install directly from the Arch Linux AUR:
```sh
yay -S disktree-bin
```

### Keyboard Shortcuts

| Shortcut | Description |
| :--- | :--- |
| `Space` / `X` | Mark or unmark the focused item for review |
| `Del` | Delete selected item with confirmation |
| `Enter` | Enter directory / zoom into folder |
| `Backspace` / `Esc` | Navigate up one directory level |
| `Alt ←` / `Alt →` | History back / forward |
| `Tab` | Select next largest sibling |
| `/` | Focus search bar |
| `Scroll Wheel` | Fluid zoom into folders |
| `C` | Open Review & Commit removal screen |
| `R` / `F5` | Rescan directory |
| `V` | Switch between mounted drives / volumes |
| `Q` | Quit application |

---

# Deutsch

**disktree** ist ein extrem schneller, moderner Speicherplatz-Visualisierer und Cleaner, entwickelt in **Rust** und **GPUI**. Es stellt Ihren Festplattenspeicher als interaktives Treemap-Mosaik dar, farblich nach Dateitypen kategorisiert. So finden Sie riesige Ordner im Handumdrehen, markieren überflüssige Daten und gewinnen Speicherplatz sicher und mit Live-Vorschau zurück.

### 🌟 Wichtigste Funktionen
- 🔍 **Permanente Suchleiste:** Immer oben sichtbar. Sofortiges Filtern von Dateien und Ordnern bei der Eingabe mit Schnell-Löschbutton (`✕`).
- ⬆ **Einfache Ordner-Navigation:** Eigener `▲`-Button für den schnellen Wechsel in den übergeordneten Ordner.
- 🗑 **Direktes & sicheres Löschen:** Löschen direkt über die Seitenleiste, das Kontextmenü oder die `Entf`-Taste mit differenziertem Bestätigungsdialog (Papierkorb vs. unwiderrufliches Löschen).
- ⚡ **Herausragende Geschwindigkeit:** Scannt Millionen von Dateien in Sekunden. Unter Windows nutzt es mit Administratorrechten das NTFS Master File Table (MFT) für maximale Performance.
- 🎨 **Intelligente Farbkodierung:** Automatische Erkennung und Farbgebung nach Datentypen (Quellcode, Medien, Caches, Toolchains, Git-Repositories, Dokumente).
- 🛡️ **Integrierter Schutz:** Systemordner, Benutzerprofile und Einhängepunkte werden zuverlässig vor versehentlichem Löschen geschützt.

### Installation & Schnellstart

#### Windows
Laden Sie `disktree-*-x86_64-windows.zip` herunter, entpacken Sie das Archiv an einem beliebigen Ort und starten Sie `disktree.exe`. Keine Installation erforderlich (Portable Version)!
- Statisch gelinkte C-Runtime (`+crt-static`) stellt sicher, dass die Anwendung auf jedem Windows 10/11-System ohne zusätzliche DLLs sofort läuft.
- Starten Sie als Administrator für MFT-beschleunigte Scans kompletter Laufwerke.

#### macOS
Laden Sie die passende `.zip`-Datei für Ihren Mac herunter (Apple Silicon oder Intel), entpacken Sie diese und ziehen Sie `disktree.app` in den Ordner *Programme*.
- Sollte macOS beim ersten Öffnen eine Warnmeldung anzeigen, führen Sie im Terminal aus:
  ```sh
  xattr -dr com.apple.quarantine /Applications/disktree.app
  ```
- Erteilen Sie **Festplattenvollzugriff** in den *Systemeinstellungen › Datenschutz & Sicherheit*, um alle Systembereiche und Caches vollständig analysieren zu können.

#### Linux
Laden Sie `disktree-*-linux.tar.gz` herunter, entpacken Sie das Archiv und führen Sie folgendes Skript aus:
```sh
./install.sh
```
Auf Arch Linux kann disktree direkt aus dem AUR installiert werden:
```sh
yay -S disktree-bin
```

### Tastenkombinationen (Shortcuts)

| Taste | Funktion |
| :--- | :--- |
| `Leertaste` / `X` | Element zur Überprüfung markieren / Markierung aufheben |
| `Entf` (`Del`) | Ausgewähltes Element direkt löschen (mit Bestätigung) |
| `Eingabe` (`Enter`) | Ordner öffnen / hineinzoomen |
| `Rücktaste` (`Backspace`) / `Esc` | Eine Verzeichnisebene nach oben |
| `Alt ←` / `Alt →` | Verlauf zurück / vorwärts |
| `Tab` | Nächstgrößeres Element auswählen |
| `/` | Suchleiste fokussieren |
| `Mausrad` | Stufenloses Hinein- und Herauszoomen |
| `C` | Überprüfungs- und Löschbildschirm öffnen |
| `R` / `F5` | Erneut scannen (Aktualisieren) |
| `V` | Zwischen gemounteten Laufwerken / Volumes wechseln |
| `Q` | Anwendung beenden |

---

# العربية

**disktree** هو متصفح ومنظف مساحة تخزين فائق السرعة وحديث مبني بلغة **Rust** وواجهة **GPUI**. يعرض مساحة القرص كمخطط شجري تفاعلي (Treemap) ملون بذكاء حسب نوع البيانات، مما يساعدك على كشف المجلدات الضخمة فوراً، وتحديد الملفات غير الضرورية، واستعادة مساحة التخزين بأمان تام مع عرض فوري لحجم المساحة المسترجعة.

### 🌟 المميزات الرئيسية
- 🔍 **شريط بحث دائم:** مدمج دائماً في أعلى النافذة للتصفية والبحث الفوري أثناء الكتابة مع زر مسح سريع (`✕`).
- ⬆ **زر الصعود للأعلى:** زر تنقل مخصص (`▲`) للصعود المباشر إلى المجلد الأب بسهولة.
- 🗑 **حذف مباشر وآمن:** حذف فوري عبر زر اللوحة الجانبية، القائمة المنبثقة، أو زر `Del` مع تأكيد ذكي (سلة المحذوفات أو الحذف النهائي).
- ⚡ **أداء فائق واستثنائي:** مسح ملايين الملفات خلال ثوانٍ معدودة. وعلى نظام Windows، يقرأ مباشرة من جدول ملفات النظام NTFS (MFT) عند التشغيل كمسؤول.
- 🎨 **تصنيف بصري ملون:** تصنيف تلقائي حسب نوع الملفات (برمجيات، وسائط، حزم تطوير، كاش، سجلات Git، ومستندات).
- 🛡️ **حماية قصوى:** حظر صارم للمساس بملفات النظام الأساسية أو جذور حسابات المستخدمين أو نقاط التثبيت الخارجية.

### التثبيت والبدء السريع

#### نظام Windows
قم بتحميل ملف `disktree-*-x86_64-windows.zip`، وفك الضغط في أي مكان، وشغّل `disktree.exe` مباشرة. لا يتطلب أي تثبيت (نسخة محمولة بالكامل)!
- تم ربط مكتبات C بربط ثابت (`+crt-static`) ليعمل على أي جهاز Windows 10/11 دون الحاجة لحزم تشغيل إضافية.
- شغّله كمسؤول (Run as Administrator) لتفعيل قراءة MFT فائقة السرعة للأقراص الكاملة.

#### نظام macOS
قم بتحميل ملف `.zip` المناسب لمعمارية جهازك (Apple Silicon أو Intel)، وفك الضغط واسحب `disktree.app` إلى مجلد التطبيقات (Applications).
- إذا ظهرت رسالة أمان عند التشغيل الأول، نفّذ الأمر التالي في Terminal:
  ```sh
  xattr -dr com.apple.quarantine /Applications/disktree.app
  ```
- امنح التطبيق **صلاحية الوصول الكامل للقرص (Full Disk Access)** من *System Settings › Privacy & Security* لتمكينه من فحص مجلدات الكاش ومساحات التطبيقات المحمية.

#### نظام Linux
قم بتحميل ملف `disktree-*-linux.tar.gz`، وفك الضغط ونفّذ السكريبت التالي:
```sh
./install.sh
```
أو ثبته مباشرة على توزيعة Arch Linux من خلال AUR:
```sh
yay -S disktree-bin
```

### اختصارات لوحة المفاتيح

| الاختصار | الوظيفة |
| :--- | :--- |
| `المسافة` / `X` | تحديد أو إلغاء تحديد العنصر للمراجعة |
| `Del` | حذف العنصر المحدد مباشرة (مع نافذة تأكيد) |
| `Enter` | فتح المجلد / الدخول والتقريب في المجلد |
| `Backspace` / `Esc` | الصعود مستوى واحد للأعلى |
| `Alt ←` / `Alt →` | الانتقال في سجل التصفح للخلف / للأمام |
| `Tab` | تحديد العنصر الأكبر التالي |
| `/` | التركيز في شريط البحث |
| `عجلة الفأرة` | التكبير والتصغير السلس داخل المجلدات |
| `C` | فتح نافذة المراجعة وتأكيد الحذف |
| `R` / `F5` | إعادة فحص ومسح المجلد |
| `V` | التبديل بين الأقراص ووحدات التخزين المتصلة |
| `Q` | إغلاق التطبيق |

---

# 日本語

**disktree** は、**Rust** と **GPUI** で構築された超高速でモダンなディスク容量ビジュアライザー兼クリーナーです。ストレージをデータ型ごとに色分けされたインタラクティブなツリーマップとして視覚化し、大容量フォルダーを瞬時に特定して不要なデータを安全に削除し、空き容量を効率的に確保できます。

### 🌟 主な機能
- 🔍 **常時表示の検索バー:** ウィンドウ上部に常に固定表示。入力と同時にリアルタイムでファイルやフォルダーを絞り込み、クリアボタン（`✕`）で瞬時にリセット。
- ⬆ **親フォルダーへの移動ボタン:** 専用の `▲` ボタンにより、上位ディレクトリへの移動がワンクリックで可能。
- 🗑 **安全な直接削除:** サイドバー、コンテキストメニュー、または `Del` キーから直接削除可能。ごみ箱への移動と完全削除を区別する安全な確認ダイアログ付き。
- ⚡ **圧倒的な高速スキャン:** 数百万のファイルを数秒でスキャン。Windowsでは管理者権限で実行することでNTFSマスターファイルテーブル（MFT）を直接高速読み取り。
- 🎨 **インテリジェントな色分け:** ファイルの種類（ソースコード、メディア、開発ツール、キャッシュ、Gitリポジトリ、ドキュメント）を自動判別して視覚化。
- 🛡️ **堅牢な安全性保護:** システムフォルダー、ユーザープロファイルのルート、マウントポイントの誤削除を完全に防止。

### インストールとクイックスタート

#### Windows
`disktree-*-x86_64-windows.zip` をダウンロードし、任意の場所に解凍して `disktree.exe` を実行するだけです。インストーラー不要のポータブル版です！
- Cランタイムが静的リンク（`+crt-static`）されているため、Visual C++ 再頒布可能パッケージが未インストールのWindows 10/11でも即座に動作します。
- ドライブ全体をスキャンする場合は、管理者として実行することでMFT超高速読み込みが有効になります。

#### macOS
お使いの環境（Apple Silicon または Intel）に合った `.zip` をダウンロードし、解凍した `disktree.app` を「アプリケーション」フォルダーにドラッグ＆ドロップします。
- 初回起動時にGatekeeper警告が表示された場合は、ターミナルで以下を実行してください：
  ```sh
  xattr -dr com.apple.quarantine /Applications/disktree.app
  ```
- システム領域やキャッシュを完全にスキャンするには、*システム設定 › プライバシーとセキュリティ* で **フルディスクアクセス** を付与してください。

#### Linux
`disktree-*-linux.tar.gz` をダウンロードして解凍し、次のコマンドを実行します：
```sh
./install.sh
```
Arch Linux では AUR から直接インストール可能です：
```sh
yay -S disktree-bin
```

### キーボードショートカット

| キー | 動作 |
| :--- | :--- |
| `Space` / `X` | 選択中の要素にレビュー用マークを付ける / 解除 |
| `Del` | 選択中の要素を直接削除（確認画面あり） |
| `Enter` | フォルダーを開く / 内部へズームイン |
| `Backspace` / `Esc` | 1つ上のフォルダー階層へ戻る |
| `Alt ←` / `Alt →` | 履歴の戻る / 進む |
| `Tab` | 次に大きい要素を選択 |
| `/` | 検索バーにフォーカス |
| `マウスホイール` | フォルダーのシームレスな拡大・縮小 |
| `C` | マーク一覧の確認と削除実行画面を開く |
| `R` / `F5` | 現在のディレクトリを再スキャン |
| `V` | 接続されているドライブやボリュームの切り替え |
| `Q` | アプリケーションを終了 |

---

# 简体中文

**disktree** 是一款采用 **Rust** 与 **GPUI** 构建的高性能现代化磁盘空间可视化分析与清理工具。它将磁盘空间以按数据类型智能着色的交互式树状图（Treemap）呈现，让您能够一目了然地定位超大文件夹，标记无用文件，并通过实时可用空间预估安全地释放存储空间。

### 🌟 核心特性
- 🔍 **顶部常驻搜索栏:** 始终置于窗口顶端，输入即时高亮与过滤对应文件及文件夹，支持一键快速清除（`✕`）。
- ⬆ **上一级目录导航:** 专属 `▲` 按钮，层级向上跳转更轻松直观。
- 🗑 **直接且安全的删除:** 可直接通过侧边栏按钮、右键菜单或 `Del` 键发起删除，支持移至回收站或彻底删除的智能确认对话框。
- ⚡ **极致扫描性能:** 数秒内遍历数百万个文件。在 Windows 上以管理员身份运行即可直接读取 NTFS 主文件表（MFT），享受飞一般的扫描体验。
- 🎨 **智能色彩分类:** 自动识别并区分数据类别（代码、多媒体、工具链、系统缓存、Git 仓库、文档）。
- 🛡️ **多重安全防护:** 严禁删除关键系统目录、用户主目录根路径及外部挂载点，避免任何误操作风险。

### 安装与快速入门

#### Windows
下载 `disktree-*-x86_64-windows.zip`，解压到任意文件夹后双击 `disktree.exe` 即可使用。绿色便携，无需安装！
- 采用静态 C 运行时链接（`+crt-static`），在任何未安装 Visual C++ 运行库的 Windows 10/11 系统上均可即开即用。
- 右键选择“以管理员身份运行”可启用极速 MFT 磁盘全盘扫描。

#### macOS
根据您的 Mac 芯片架构（Apple Silicon 或 Intel）下载对应的 `.zip` 压缩包，解压后将 `disktree.app` 拖入“应用程序”文件夹。
- 若首次打开时系统提示安全隔离警告，请在终端中执行：
  ```sh
  xattr -dr com.apple.quarantine /Applications/disktree.app
  ```
- 请在*系统设置 › 隐私与安全性*中授予 disktree **完全磁盘访问权限**，以便完整扫描受保护的应用缓存与用户数据。

#### Linux
下载 `disktree-*-linux.tar.gz` 解压后运行安装脚本：
```sh
./install.sh
```
Arch Linux 用户可直接通过 AUR 安装：
```sh
yay -S disktree-bin
```

### 快捷键指南

| 快捷键 | 功能说明 |
| :--- | :--- |
| `空格` / `X` | 标记或取消标记当前选中项以供审核 |
| `Del` | 直接删除选中项（弹出确认窗口） |
| `回车` (`Enter`) | 进入该目录 / 缩放深入下级目录 |
| `退格` (`Backspace`) / `Esc` | 返回上一级目录 |
| `Alt ←` / `Alt →` | 历史记录后退 / 前进 |
| `Tab` | 选择同级目录中下一个最大项目 |
| `/` | 聚焦至搜索栏 |
| `鼠标滚轮` | 平滑缩放进出文件夹 |
| `C` | 打开标记列表审核与确认删除界面 |
| `R` / `F5` | 重新扫描当前目录 |
| `V` | 切换已挂载的硬盘或分区 |
| `Q` | 退出程序 |

---

# Français

**disktree** est un visualiseur et nettoyeur d'espace disque ultra-rapide et moderne conçu avec **Rust** et **GPUI**. Il modélise votre stockage sous la forme d'un treemap interactif coloré par type de données, vous permettant d'identifier immédiatement les répertoires volumineux, de marquer les fichiers inutiles et de récupérer de l'espace en toute sécurité avec une prévisualisation en direct.

### 🌟 Fonctionnalités clés
- 🔍 **Barre de recherche permanente :** Toujours accessible en haut de la fenêtre. Filtrez instantanément vos répertoires et fichiers avec bouton d'effacement rapide (`✕`).
- ⬆ **Bouton dossier parent :** Bouton `▲` dédié pour remonter facilement dans l'arborescence des dossiers.
- 🗑 **Suppression directe et sécurisée :** Action de suppression directe depuis le volet latéral, le menu contextuel ou la touche `Suppr`, accompagnée d'une boîte de dialogue intelligente (Corbeille ou définitif).
- ⚡ **Performances fulgurantes :** Scanne des millions de fichiers en quelques secondes. Sous Windows, lit directement la table des fichiers NTFS (MFT) en mode administrateur.
- 🎨 **Classification visuelle intelligente :** Couleurs distinctes selon le type de fichier (code source, médias, environnements, caches, Git, documents).
- 🛡️ **Sécurité maximale :** Refus absolu de supprimer les répertoires système critiques, les dossiers racines utilisateurs ou les points de montage externes.

### Installation et démarrage rapide

#### Windows
Téléchargez `disktree-*-x86_64-windows.zip`, décompressez l'archive dans le dossier de votre choix et lancez `disktree.exe`. Version portable, aucune installation requise !
- Liaison statique de la bibliothèque C (`+crt-static`) garantissant un fonctionnement parfait sur Windows 10/11 sans dépendance DLL.
- Exécutez en tant qu'administrateur pour profiter du scan accéléré par MFT sur vos disques complets.

#### macOS
Téléchargez l'archive `.zip` adaptée à votre machine (Apple Silicon ou Intel), décompressez-la et glissez `disktree.app` dans le dossier *Applications*.
- Si Gatekeeper bloque le premier lancement, saisissez dans le Terminal :
  ```sh
  xattr -dr com.apple.quarantine /Applications/disktree.app
  ```
- Accordez l'**Accès complet au disque** dans *Réglages Système › Confidentialité et sécurité* pour analyser les caches système et données d'applications.

#### Linux
Téléchargez `disktree-*-linux.tar.gz`, décompressez l'archive et exécutez :
```sh
./install.sh
```
Sous Arch Linux, disktree est disponible sur AUR :
```sh
yay -S disktree-bin
```

### Raccourcis clavier

| Raccourci | Action |
| :--- | :--- |
| `Espace` / `X` | Marquer ou démarquer l'élément pour examen |
| `Suppr` (`Del`) | Supprimer directement l'élément sélectionné avec confirmation |
| `Entrée` | Ouvrir le dossier / zoomer à l'intérieur |
| `Retour arrière` / `Échap` | Remonter d'un niveau de répertoire |
| `Alt ←` / `Alt →` | Historique précédent / suivant |
| `Tab` | Sélectionner le frère suivant le plus volumineux |
| `/` | Activer la barre de recherche |
| `Molette souris` | Zoom fluide dans les dossiers |
| `C` | Ouvrir la fenêtre d'examen et de suppression |
| `R` / `F5` | Réanalyser le dossier courant |
| `V` | Changer de disque ou de volume monté |
| `Q` | Quitter l'application |

---

# Türkçe

**disktree**, **Rust** ve **GPUI** kullanılarak geliştirilmiş, ışık hızında çalışan modern bir disk alanı görselleştirici ve temizleme aracıdır. Disk kullanımınızı veri türüne göre renklendirilmiş etkileşimli bir ağaç haritası (treemap) olarak sunar; dev klasörleri anında tespit etmenizi, gereksiz dosyaları işaretlemenizi ve gerçek zamanlı alan projeksiyonuyla güvenle depolama alanı kazanmanızı sağlar.

### 🌟 Önemli Özellikler
- 🔍 **Kalıcı Arama Çubuğu:** Pencerenin üst kısmında her zaman hazırdır. Yazdığınız anda dosyaları anlık olarak filtreler ve hızlı temizleme düğmesi (`✕`) içerir.
- ⬆ **Üst Dizin Gezintisi:** Klasör katmanlarında kolayca yukarı çıkmak için özel `▲` gezinti butonu.
- 🗑 **Doğrudan ve Güvenli Silme:** Yan panel, sağ tık menüsü veya `Del` tuşu ile doğrudan silme; Geri Dönüşüm Kutusu veya kalıcı silme seçenekli akıllı onay penceresi.
- ⚡ **Üstün Performans:** Milyonlarca dosyayı saniyeler içinde tarar. Windows'ta yönetici olarak çalıştırıldığında NTFS Ana Dosya Tablosunu (MFT) doğrudan okur.
- 🎨 **Akıllı Renklendirme:** Dosya türlerine göre otomatik renk ayrımı (kod, medya, araç zincirleri, önbellekler, Git depoları ve belgeler).
- 🛡️ **Güvenlik Koruması:** Sistem dizinlerini, kullanıcı profili ana dizinlerini ve bağlama noktalarını silmeyi kesin olarak reddeder.

### Kurulum ve Hızlı Başlangıç

#### Windows
`disktree-*-x86_64-windows.zip` dosyasını indirin, dilediğiniz bir klasöre çıkartın ve `disktree.exe` dosyasını çalıştırın. Kurulum gerektirmeyen taşınabilir (portable) sürümdür!
- Statik C çalışma zamanı bağlantısı (`+crt-static`), herhangi bir Windows 10/11 cihazında harici DLL gerektirmeden sorunsuz çalışmasını sağlar.
- Tüm sürücüyü taramak için yönetici olarak çalıştırarak MFT hızlandırmasını etkinleştirin.

#### macOS
Mimarinize uygun `.zip` dosyasını (Apple Silicon veya Intel) indirin, arşivi açın ve `disktree.app` dosyasını Uygulamalar (Applications) klasörüne sürükleyin.
- Gatekeeper ilk açılışta uyarı verirse Terminal'de şu komutu çalıştırın:
  ```sh
  xattr -dr com.apple.quarantine /Applications/disktree.app
  ```
- Önbellekleri ve korumalı alanları eksiksiz tarayabilmek için *Sistem Ayarları › Gizlilik ve Güvenlik* bölümünden **Tam Disk Erişimi** izni verin.

#### Linux
`disktree-*-linux.tar.gz` dosyasını indirin, çıkartın ve şu betiği çalıştırın:
```sh
./install.sh
```
Arch Linux kullanıcıları AUR üzerinden doğrudan yükleyebilir:
```sh
yay -S disktree-bin
```

### Klavye Kısayolları

| Kısayol | İşlev |
| :--- | :--- |
| `Boşluk` / `X` | Seçili öğeyi inceleme için işaretle veya işareti kaldır |
| `Del` | Seçili öğeyi doğrudan onayla sil |
| `Enter` | Klasöre gir / klasörün içine yakınlaş |
| `Geri tuşu` / `Esc` | Bir üst dizin seviyesine çık |
| `Alt ←` / `Alt →` | Gezinti geçmişinde geri / ileri git |
| `Tab` | Bir sonraki en büyük öğeyi seç |
| `/` | Arama çubuğuna odaklan |
| `Fare Tekerleği` | Klasörlerde akıcı yakınlaştırma / uzaklaştırma |
| `C` | İşaretli öğeleri inceleme ve silme ekranını aç |
| `R` / `F5` | Mevcut dizini yeniden tara |
| `V` | Bağlı sürücüler ve birimler arasında geçiş yap |
| `Q` | Uygulamadan çık |

---

# Español

**disktree** es una herramienta moderna y ultrarrápida de visualización y limpieza de espacio en disco desarrollada en **Rust** y **GPUI**. Modela el almacenamiento como un mosaico de mapa de árbol (treemap) interactivo clasificado por tipo de datos, lo que le permite ubicar al instante carpetas pesadas, marcar elementos innecesarios y recuperar espacio en disco con proyecciones de ahorro en tiempo real.

### 🌟 Características destacadas
- 🔍 **Barra de búsqueda permanente:** Siempre visible en la parte superior. Filtre archivos y carpetas al escribir con botón de reinicio rápido (`✕`).
- ⬆ **Navegación al directorio superior:** Botón `▲` exclusivo para subir de nivel de carpeta de forma ágil y cómoda.
- 🗑 **Eliminación directa y segura:** Borrado rápido mediante el panel lateral, menú contextual o tecla `Supr`, con ventana de confirmación inteligente (Papelera de reciclaje o permanente).
- ⚡ **Rendimiento extraordinario:** Escanea millones de archivos en segundos. En Windows lee directamente la Master File Table (MFT) de NTFS al ejecutarse como administrador.
- 🎨 **Clasificación visual inteligente:** Código de colores por categorías (código fuente, multimedia, entornos, cachés, repositorios Git, documentos).
- 🛡️ **Seguridad garantizada:** Protección estricta contra la eliminación de directorios del sistema, perfiles de usuario principales o unidades externas montadas.

### Instalación y puesta en marcha

#### Windows
Descargue `disktree-*-x86_64-windows.zip`, descomprima el archivo en cualquier ubicación y ejecute `disktree.exe`. ¡Completamente portátil, no requiere instalación!
- Enlace estático al runtime de C (`+crt-static`) para funcionar en cualquier Windows 10/11 sin necesidad de instalar librerías Visual C++.
- Ejecútelo como Administrador para activar el escaneo MFT ultrarrápido de discos completos.

#### macOS
Descargue el archivo `.zip` correspondiente a su arquitectura (Apple Silicon o Intel), descomprímalo y arrastre `disktree.app` a su carpeta Aplicaciones.
- Si Gatekeeper muestra una advertencia de seguridad al abrir por primera vez, ejecute en la Terminal:
  ```sh
  xattr -dr com.apple.quarantine /Applications/disktree.app
  ```
- Conceda **Acceso total al disco** en *Ajustes del Sistema › Privacidad y seguridad* para permitir el análisis de cachés y datos protegidos.

#### Linux
Descargue `disktree-*-linux.tar.gz`, descomprímalo y ejecute:
```sh
./install.sh
```
En Arch Linux puede instalarse directamente desde AUR:
```sh
yay -S disktree-bin
```

### Atajos de teclado

| Atajo | Acción |
| :--- | :--- |
| `Espacio` / `X` | Marcar o desmarcar el elemento para revisión |
| `Supr` (`Del`) | Eliminar el elemento seleccionado (con confirmación) |
| `Enter` | Entrar al directorio / hacer zoom hacia adentro |
| `Retroceso` / `Esc` | Subir un nivel de directorio |
| `Alt ←` / `Alt →` | Historial atrás / adelante |
| `Tab` | Seleccionar el siguiente elemento más grande |
| `/` | Enfocar la barra de búsqueda |
| `Rueda del ratón` | Zoom continuo en el mosaico |
| `C` | Abrir pantalla de revisión y confirmación de eliminación |
| `R` / `F5` | Volver a escanear el directorio |
| `V` | Cambiar entre discos o volúmenes montados |
| `Q` | Salir de la aplicación |

---

# Русский

**disktree** — это сверхбыстрый и современный визуализатор и инструмент очистки дискового пространства, созданный на **Rust** и **GPUI**. Приложение отображает файловую систему в виде интерактивной древовидной карты (treemap), раскрашенной по типам данных, что позволяет мгновенно находить тяжелые папки, помечать ненужные файлы и безопасно освобождать память с отображением точного прогноза освобождаемого объема.

### 🌟 Ключевые возможности
- 🔍 **Постоянная строка поиска:** Всегда закреплена в верхней части окна. Мгновенная фильтрация по имени в реальном времени с кнопкой быстрой очистки (`✕`).
- ⬆ **Кнопка перехода наверх:** Специальная кнопка `▲` для быстрого и удобного перехода в родительский каталог.
- 🗑 **Прямое и безопасное удаление:** Удаление через боковую панель, контекстное меню или клавишу `Del` с продуманным диалогом подтверждения (Корзина или безвозвратное удаление).
- ⚡ **Высочайшая скорость работы:** Сканирует миллионы файлов за считанные секунды. В Windows при запуске от имени администратора читает напрямую главную таблицу файлов NTFS (MFT).
- 🎨 **Интеллектуальная цветовая разметка:** Автоматическое разделение по категориям (исходный код, медиафайлы, сборочные пакеты, кэши, репозитории Git, документы).
- 🛡️ **Гарантированная защита:** Категорический отказ от удаления системных папок, корней пользовательских профилей и точек монтирования.

### Установка и быстрый старт

#### Windows
Скачайте архив `disktree-*-x86_64-windows.zip`, распакуйте в любую удобную папку и запустите `disktree.exe`. Установка не требуется (полноценная портативная версия)!
- Статическая линковка среды выполнения C (`+crt-static`) гарантирует запуск на любой Windows 10/11 без необходимости установки сторонних библиотек DLL.
- Запустите от имени администратора для активации быстрого сканирования всего диска через MFT.

#### macOS
Скачайте `.zip` архив для вашей архитектуры (Apple Silicon или Intel), распакуйте и переместите `disktree.app` в папку «Программы».
- Если при первом запуске Gatekeeper выдает предупреждение, выполните в Терминале:
  ```sh
  xattr -dr com.apple.quarantine /Applications/disktree.app
  ```
- Предоставьте **Полный доступ к диску** в *Системных настройках › Конфиденциальность и безопасность*, чтобы разрешить сканирование кэша и защищенных данных.

#### Linux
Скачайте `disktree-*-linux.tar.gz`, распакуйте архив и запустите установочный скрипт:
```sh
./install.sh
```
В Arch Linux доступна установка напрямую из AUR:
```sh
yay -S disktree-bin
```

### Горячие клавиши

| Клавиша | Описание действия |
| :--- | :--- |
| `Пробел` / `X` | Отметить или снять отметку с элемента для проверки |
| `Del` | Удалить выбранный элемент напрямую (с подтверждением) |
| `Enter` | Войти в папку / приблизить просмотр |
| `Backspace` / `Esc` | Подняться на один уровень папки вверх |
| `Alt ←` / `Alt →` | Перемещение по истории назад / вперед |
| `Tab` | Выбрать следующий наибольший по размеру объект |
| `/` | Активировать строку поиска |
| `Колесо мыши` | Плавное масштабирование каталогов |
| `C` | Открыть экран проверки и подтверждения удаления |
| `R` / `F5` | Пересканировать текущую папку |
| `V` | Переключить подключенный диск или раздел |
| `Q` | Закрыть программу |

---

# Українська

**disktree** — це надзвичайно швидкий та сучасний інструмент для візуалізації й очищення дискового простору, створений на базі **Rust** та **GPUI**. Програма представляє файлову систему у вигляді інтерактивної деревоподібної карти (treemap) з колірним кодуванням за типами даних. Вона дозволяє миттєво виявляти найбільші папки, позначати зайві файли та безпечно звільняти місце з наочним прогнозом повернутого обсягу пам'яті.

### 🌟 Головні переваги
- 🔍 **Постійний рядок пошуку:** Завжди доступний угорі для миттєвої фільтрації файлів та тек у реальному часі з кнопкою швидкого скасування (`✕`).
- ⬆ **Перехід на рівень вгору:** Окрема кнопка `▲` для зручного та швидкого підйому до батьківського каталогу.
- 🗑 **Пряме та безпечне видалення:** Видалення з бічної панелі, контекстного меню або клавішею `Del` із розумним вікном підтвердження (Кошик або остаточне видалення).
- ⚡ **Колосальна швидкодія:** Сканування мільйонів файлів за лічені секунди. У Windows із правами адміністратора сканує напряму через Master File Table (MFT) файлової системи NTFS.
- 🎨 **Зрозуміле колірне кодування:** Автоматичне розпізнавання за категоріями (вихідний код, медіа, пакети розробки, системний кеш, репозиторії Git, документи).
- 🛡️ **Надійний захист:** Сувора заборона видалення критичних системних каталогів, кореневих профілів користувачів та зовнішніх точок монтування.

### Встановлення та швидкий старт

#### Windows
Завантажте архів `disktree-*-x86_64-windows.zip`, розпакуйте його в будь-яку теку та запустіть `disktree.exe`. Повна портативна версія, встановлення не потрібне!
- Статичне лінкування бібліотек C (`+crt-static`) забезпечує бездоганну роботу на будь-якій Windows 10/11 без додаткових пакетів розповсюдження.
- Запуск від імені адміністратора активує надшвидке сканування повного диска через таблицю MFT.

#### macOS
Завантажте архів `.zip` для вашої платформи (Apple Silicon або Intel), розархівуйте та перетягніть `disktree.app` до теки «Програми».
- Якщо під час першого запуску Gatekeeper блокує додаток, виконайте в Терміналі:
  ```sh
  xattr -dr com.apple.quarantine /Applications/disktree.app
  ```
- Надайте **Повний доступ до диска** в меню *Системні параметри › Приватність і безпека* для сканування системних кешів та прихованих файлів.

#### Linux
Завантажте `disktree-*-linux.tar.gz`, розархівуйте та запустіть інсталяційний скрипт:
```sh
./install.sh
```
В Arch Linux доступне пряме встановлення з AUR:
```sh
yay -S disktree-bin
```

### Сполучення клавіш

| Клавіша | Дія |
| :--- | :--- |
| `Пробіл` / `X` | Позначити або зняти позначку з елемента для перевірки |
| `Del` | Безпосередньо видалити вибраний елемент (з підтвердженням) |
| `Enter` | Відкрити каталог / наблизити перегляд теки |
| `Backspace` / `Esc` | Піднятися на один рівень теки вгору |
| `Alt ←` / `Alt →` | Навігація історією назад / вперед |
| `Tab` | Вибрати наступний найбільший за розміром елемент |
| `/` | Сфокусуватися на рядку пошуку |
| `Коліщатко миші` | Плавне масштабування вмісту тек |
| `C` | Відкрити екран перевірки та підтвердження видалення |
| `R` / `F5` | Повторно просканувати поточний каталог |
| `V` | Перемкнути змонтований диск або розділ |
| `Q` | Вийти з програми |

---

## 🛠️ Building from Source / Selbst kompilieren

Prerequisites: **Rust 1.97+**

```sh
git clone https://github.com/ing-yaserhasan/disktree.git
cd disktree-1

# Release Build
cargo build --release -p disktree-app

# Linux / macOS with Makefile
make build
make test
```

---

## 📄 License / Lizenz

Distributed under the **MIT License**. See [`LICENSE`](LICENSE) for more information.
