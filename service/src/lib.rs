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

#[derive(Debug)]
pub enum ServiceEvent<T> {
    Continue,
    Pause,
    Stop,
    SessionConnect(Session),
    SessionDisconnect(Session),
    SessionRemoteConnect(Session),
    SessionRemoteDisconnect(Session),
    SessionLogon(Session),
    SessionLogoff(Session),
    SessionLock(Session),
    SessionUnlock(Session),
    Custom(T),
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
        pub use self::windows::*;
    } else if #[cfg(target_os = "macos")] {
        todo!();
    } else if #[cfg(target_os = "linux")] {
        mod linux;
        pub use self::linux::*;
    } else {
        todo!();
    }
}
