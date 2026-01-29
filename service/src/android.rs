//! Android specific code

use std::path::PathBuf;

/// Dummy function for uniformity to windows
pub type DispatchFn = fn();

#[derive(Debug)]
/// A placeholder, not currently used
pub struct Session(String);

/// The macro generates the service function required
#[macro_export]
macro_rules! ServiceMacro {
    ($entry:ident, $function:ident, $t:ident) => {
        fn $entry() {
            $function(None, None);
        }
    };
}

/// This macro is for the async dispatch on a linux service
#[macro_export]
macro_rules! DispatchAsync {
    ($self:ident, $function:ident) => {{
        $function().await;
        let r: Result<(), u32> = Ok(());
        r
    }};
}

#[cfg(feature = "async")]
/// The macro generates the service function required
#[macro_export]
macro_rules! ServiceAsyncMacro {
    ($entry:ident, $function:ident, $t:ident) => {
        async fn $entry() {
            $function().await;
        }
    };
}

/// The configuration for constructing a Service.
pub struct ServiceConfig {
    /// The arguments for the service
    arguments: Vec<String>,
    /// The description of the service as presented to the user
    description: String,
    /// The path to the service binary
    binary: PathBuf,
    /// The username that the service should run as
    username: Option<String>,
    /// The path to the configuration data for the service
    pub config_path: PathBuf,
}

impl ServiceConfig {
    /// Build a new service config with reasonable defaults.
    /// # Arguments
    /// * display - The display name of the service
    /// * arguments - The list of arguments to provide to the service
    /// * description - The description of the service
    /// * binary - The path to the binary that runs the service
    /// * config_path - The configuration path for the service
    /// * username - The username the service runs as
    pub fn new(
        arguments: Vec<String>,
        description: String,
        binary: PathBuf,
        username: Option<String>,
    ) -> Self {
        Self {
            arguments,
            description,
            binary,
            config_path: PathBuf::new(),
            username,
        }
    }
}

#[derive(Debug)]
/// Errors that can occur when creating a service
pub enum CreateError {
    /// Systemctl does not exist or is not callable for some reason
    NoSystemCtl,
    /// The systemctl command returned an error
    SystemCtlFailed,
    /// Systemctl reload command failed for some reason
    SystemCtlReloadFailed,
    /// Unable to create or write to the systemctl service file
    FileIoError(std::io::Error),
}

#[derive(Debug)]
/// Errors that can occur when interfacing with systemctl
pub enum StartStopError {
    /// Systemctl does not exist or is not callable for some reason
    NoSystemCtl,
    /// The systemctl command returned an error
    SystemCtlFailed,
}

impl From<StartStopError> for CreateError {
    fn from(value: StartStopError) -> Self {
        match value {
            StartStopError::NoSystemCtl => Self::NoSystemCtl,
            StartStopError::SystemCtlFailed => Self::SystemCtlFailed,
        }
    }
}

/// Represents a service on the system
pub struct Service {
    /// The name of the service, as known by the operating system
    name: String,
}

impl Service {
    /// Construct a new self
    pub fn new(name: String) -> Self {
        Self { name }
    }

    /// Does the service already exist?
    pub fn exists(&self) -> bool {
        false
    }

    /// Initialize a new log instance
    pub fn new_log(&self, level: super::LogLevel) {
        simple_logger::SimpleLogger::new().init().unwrap();
        log::set_max_level(level.level_filter());
    }

    /// Create the service
    pub fn create(&mut self, config: ServiceConfig) -> Result<(), CreateError> {
        Err(CreateError::NoSystemCtl)
    }

    /// Start the service
    pub fn start(&mut self) -> Result<(), StartStopError> {
        Err(StartStopError::NoSystemCtl)
    }

    /// Stop the service
    pub fn stop(&mut self) -> Result<(), StartStopError> {
        Err(StartStopError::NoSystemCtl)
    }

    /// Delete the service
    pub fn delete(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }

    /// Run the required dispatch code
    pub fn dispatch(&self, service_main: DispatchFn) -> Result<(), u32> {
        service_main();
        Ok(())
    }
}