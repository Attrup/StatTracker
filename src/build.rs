extern crate winres;

/// Build script to embed the icon into the executable on Windows.
fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/icon.ico")
            .set("FileDescription", "Hitman StatTracker, a statistics app for HM 2 SA and HM Contracts.");
        res.compile().unwrap();
    }
}
