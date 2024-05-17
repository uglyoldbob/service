#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
#![warn(unused_extern_crates)]

//! Contains code for establishing a service

pub use log;

#[cfg(feature = "egui-prompt")]
use prompt::egui;

/// The various levels of log, increasing in severity
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "prompt", derive(prompt::Prompting))]
#[cfg_attr(feature = "egui-prompt", derive(prompt::EguiPrompting))]
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

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

#[derive(Debug)]
/// The events that can be sent to the service handler
pub enum ServiceEvent<T> {
    /// Continue a previously paused service
    Continue,
    /// Pause the service
    Pause,
    /// Stop the service
    Stop,
    /// Windows specific session message
    SessionConnect(Session),
    /// Windows specific session message
    SessionDisconnect(Session),
    /// Windows specific session message
    SessionRemoteConnect(Session),
    /// Windows specific session message
    SessionRemoteDisconnect(Session),
    /// Windows specific session message
    SessionLogon(Session),
    /// Windows specific session message
    SessionLogoff(Session),
    /// Windows specific session message
    SessionLock(Session),
    /// Windows specific session message
    SessionUnlock(Session),
    /// A custom message for the service
    Custom(T),
}

impl LogLevel {
    /// Convert log level to a level filter
    pub fn level_filter(&self) -> log::LevelFilter {
        match self {
            crate::LogLevel::Debug => log::LevelFilter::Debug,
            crate::LogLevel::Info => log::LevelFilter::Info,
            crate::LogLevel::Warning => log::LevelFilter::Warn,
            crate::LogLevel::Error => log::LevelFilter::Error,
            crate::LogLevel::Trace => log::LevelFilter::Trace,
        }
    }

    /// Convert self to a log::Level
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
