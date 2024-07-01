use std::io::{Error, ErrorKind, Write};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::time::{Duration, SystemTime};

pub struct OverlayControl {
    overlay: Option<OverlayProcess>,
    pub overlay_status: Result<(), String>,
    pub use_overlay: bool,
    pub text_size: u8,
}

impl OverlayControl {
    pub fn new() -> Self {
        Self {
            overlay: None,
            overlay_status: Ok(()),
            use_overlay: false,
            text_size: 5,
        }
    }

    /// Launch the overlay program. Set the failed flag if the binary is not found.
    pub fn launch_overlay(&mut self) {
        match OverlayProcess::new() {
            Ok(process) => {
                self.overlay = Some(process);
                self.overlay_status = Ok(());
            }
            Err(_) => {
                self.use_overlay = false;
                self.overlay_status = Err(String::from("Overlay program not found"));
            }
        }
    }

    /// Update the mission timer and SA status of the overlay. Update 10 times per second
    /// to limit the amount of data sent to the overlay as it is unessecary.
    pub fn set_overlay_timer(&mut self, time: u32, sa_status: bool) {
        if let Some(ref mut overlay) = self.overlay {
            if overlay
                .last_update
                .elapsed()
                .unwrap_or(Duration::from_secs(1))
                .as_millis()
                > 200
            {
                let cmd = format!(
                    "data {:0>2} {:0>2} {}\n",
                    time / 3600,
                    (time / 60) % 60,
                    sa_status
                );

                overlay.stdin.write_all(cmd.as_bytes()).unwrap_or_default();
                overlay.last_update = SystemTime::now();
            }
        }
    }

    /// Update the size of the overlay
    pub fn set_overlay_size(&mut self, size: u8) {
        if let Some(ref mut overlay) = self.overlay {
            let cmd = format!("size {}\n", size);
            overlay.stdin.write_all(cmd.as_bytes()).unwrap_or_default();
        }
    }

    /// Update the color map used by the overlay
    pub fn set_overlay_colormap(&mut self, cmap: &str) {
        if let Some(ref mut overlay) = self.overlay {
            let cmd = format!("cmap {}\n", cmap);
            overlay.stdin.write_all(cmd.as_bytes()).unwrap_or_default();
        }
    }

    pub fn close_overlay(&mut self) {
        self.overlay = None;
    }
}

/// Struct to simplify managing the overlay process
struct OverlayProcess {
    child: Child,
    stdin: ChildStdin,
    last_update: SystemTime,
}

impl OverlayProcess {
    // Launch the overlay process and take control of its stdin
    pub fn new() -> Result<Self, Error> {
        let mut child = Command::new("./target/release/Overlay")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        let stdin = child.stdin.take().ok_or(Error::new(ErrorKind::Other, ""))?;

        Ok(Self {
            child: child,
            stdin: stdin,
            last_update: SystemTime::now(),
        })
    }
}

// Close the overlay process when the struct is dropped
impl Drop for OverlayProcess {
    fn drop(&mut self) {
        self.child.kill().unwrap_or_default();
    }
}
