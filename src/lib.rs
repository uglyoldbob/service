//! Contains code for establishing a service

cfg_if::cfg_if! {
    if #[cfg(windows)] {
        mod windows;
        pub use self::windows::ServiceConfig as ServiceConfig;
    } else if #[cfg(target_os = "macos")] {
        todo!();
    } else if #[cfg(target_os = "linux")] {
        mod linux;
        pub use self::linux::ServiceConfig as ServiceConfig;
    } else {
        todo!();
    }
}
