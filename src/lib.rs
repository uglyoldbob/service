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

impl LogLevel {
    pub fn level_filter(&self) -> log::LevelFilter {
        match self {
            crate::LogLevel::Debug => log::LevelFilter::Debug,
            crate::LogLevel::Info => log::LevelFilter::Info,
            crate::LogLevel::Warning => log::LevelFilter::Warn,
            crate::LogLevel::Error => log::LevelFilter::Error,
            crate::LogLevel::Trace => log::LevelFilter::Trace,
        }
    }

    pub fn level(&self) -> log::Level {
        match self {
            crate::LogLevel::Debug => log::Level::Debug,
            crate::LogLevel::Info => log::Level::Info,
            crate::LogLevel::Warning => log::Level::Warn,
            crate::LogLevel::Error => log::Level::Error,
            crate::LogLevel::Trace => log::Level::Trace,
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(windows)] {
        mod windows;
        pub use winapi;
        pub use self::windows::ServiceConfig as ServiceConfig;
        pub use self::windows::Service as Service;
        pub use self::windows::ServiceFn as ServiceFn;
        pub use self::windows::convert_args as convert_args;
        pub use self::windows::run_service as run_service;
    } else if #[cfg(target_os = "macos")] {
        todo!();
    } else if #[cfg(target_os = "linux")] {
        mod linux;
        pub use self::linux::ServiceConfig as ServiceConfig;
        pub use self::linux::Service as Service;
    } else {
        todo!();
    }
}
