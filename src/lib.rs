//! Contains code for establishing a service

pub use log;

/// The various levels of log, increasing in severity
pub enum LogLevel {
    /// Trace
    Trace,
    /// Debug
    Debug,
    /// Informational
    Info,
    /// Warning
    Warning,
    /// Error
    Error,
}

cfg_if::cfg_if! {
    if #[cfg(windows)] {
        mod windows;
        pub use self::windows::ServiceConfig as ServiceConfig;
        pub use self::windows::Service as Service;
        pub use self::windows::new_log as new_log;
    } else if #[cfg(target_os = "macos")] {
        todo!();
    } else if #[cfg(target_os = "linux")] {
        mod linux;
        pub use self::linux::ServiceConfig as ServiceConfig;
        pub use self::linux::Service as Service;
        pub use self::linux::new_log as new_log;
    } else {
        todo!();
    }
}
