use std::io::{Error, ErrorKind, Write};
use std::process::{Child, ChildStdin, Command, Stdio};

pub struct OverlayProcess {
    child: Child,
    stdin: ChildStdin,
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
        })
    }

    // Update the text size of the overlay
    pub fn set_text_size(&mut self, size: u8) {
        let cmd = format!("size {}\n", size);
        self.stdin.write_all(cmd.as_bytes()).unwrap_or_default();
    }

    // Update the mission timer and SA status of the overlay
    pub fn set_timer(&mut self, time: u32, sa_status: bool) {
        let cmd = format!(
            "data {:0>2} {:0>2} {}\n",
            time / 3600,
            (time / 60) % 60,
            sa_status
        );
        self.stdin.write_all(cmd.as_bytes()).unwrap_or_default();
    }

    // Update the color map used by the overlay
    pub fn set_colormap(&mut self, cmap: &str) {
        let cmd = format!("cmap {}\n", cmap);
        self.stdin.write_all(cmd.as_bytes()).unwrap_or_default();
    }
}

// Close the overlay process when the struct is dropped
impl Drop for OverlayProcess {
    fn drop(&mut self) {
        self.child.kill().unwrap_or_default();
    }
}
