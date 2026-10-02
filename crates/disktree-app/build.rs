fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("../../assets/disktree.ico");
        res.set("CompanyName", "Yaser");
        res.set("FileDescription", "disktree (Yaser Edition)");
        res.set("LegalCopyright", "Copyright (C) 2026 Yaser");
        res.set("ProductName", "disktree");
        res.set("ProductVersion", env!("CARGO_PKG_VERSION"));
        res.set("FileVersion", env!("CARGO_PKG_VERSION"));
        let _ = res.compile();
    }
}
