// MIT License Copyright (c) 2024-present Frank Zhang
// Windows process helper to prevent console window flashing

/// Trait to suppress console window creation on Windows
///
/// This trait provides a `no_window()` method for process commands
/// that prevents console windows from appearing when spawning child processes
/// on Windows systems.
pub trait ProcessCommandExt {
    fn no_window(&mut self) -> &mut Self;
}

#[cfg(target_os = "windows")]
impl ProcessCommandExt for std::process::Command {
    fn no_window(&mut self) -> &mut Self {
        use std::os::windows::process::CommandExt as _;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        self.creation_flags(CREATE_NO_WINDOW)
    }
}

#[cfg(not(target_os = "windows"))]
impl ProcessCommandExt for std::process::Command {
    fn no_window(&mut self) -> &mut Self {
        self
    }
}

#[cfg(target_os = "windows")]
impl ProcessCommandExt for tokio::process::Command {
    fn no_window(&mut self) -> &mut Self {
        // tokio::process::Command uses std::process::Command internally
        // We need to use the OS extension trait for Windows
        use std::os::windows::process::CommandExt as _;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        self.creation_flags(CREATE_NO_WINDOW)
    }
}

#[cfg(not(target_os = "windows"))]
impl ProcessCommandExt for tokio::process::Command {
    fn no_window(&mut self) -> &mut Self {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_command_ext_std() {
        let mut cmd = std::process::Command::new("echo");
        cmd.no_window();
        // If this compiles, the trait is working correctly
    }

    #[test]
    fn test_process_command_ext_tokio() {
        let mut cmd = tokio::process::Command::new("echo");
        cmd.no_window();
        // If this compiles, the trait is working correctly
    }
}
