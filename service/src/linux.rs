//! Linux specific code for managing a service

use std::path::PathBuf;

/// Dummy function for uniformity to windows
pub type DispatchFn = fn();

#[derive(Debug)]
/// Errors that can occur when interfacing with systemctl
pub enum StartStopError {
    /// Systemctl does not exist or is not callable for some reason
    NoSystemCtl,
    /// The systemctl command returned an error
    SystemCtlFailed,
}

/// The macro generates the service function required for windows
#[macro_export]
macro_rules! ServiceMacro {
    ($entry:ident, $function:ident, $t:ident) => {
        fn $entry() {
            $function(None, None);
        }
    };
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

impl From<StartStopError> for CreateError {
    fn from(value: StartStopError) -> Self {
        match value {
            StartStopError::NoSystemCtl => Self::NoSystemCtl,
            StartStopError::SystemCtlFailed => Self::SystemCtlFailed,
        }
    }
}

#[derive(Debug)]
/// A placeholder, not currently used
pub struct Session(String);

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

    /// Initialize a new log instance
    pub fn new_log(&self, level: super::LogLevel) {
        simple_logger::SimpleLogger::new().init().unwrap();
        log::set_max_level(level.level_filter());
    }

    /// The systemd path for linux
    pub fn systemd_path(&self) -> PathBuf {
        PathBuf::from("/etc/systemd/system")
    }

    /// Does the service already exist?
    pub fn exists(&self) -> bool {
        let systemd_path = self.systemd_path();
        let pb = systemd_path.join(format!("{}.service", self.name));
        pb.exists()
    }

    /// Stop the service
    pub fn stop(&mut self) -> Result<(), StartStopError> {
        let o = std::process::Command::new("systemctl")
            .arg("stop")
            .arg(&self.name)
            .output()
            .map_err(|_| StartStopError::NoSystemCtl)?;
        if !o.status.success() {
            Err(StartStopError::SystemCtlFailed)
        } else {
            Ok(())
        }
    }

    /// Start the service
    pub fn start(&mut self) -> Result<(), StartStopError> {
        let o = std::process::Command::new("systemctl")
            .arg("start")
            .arg(&self.name)
            .output()
            .map_err(|_| StartStopError::NoSystemCtl)?;
        if !o.status.success() {
            Err(StartStopError::SystemCtlFailed)
        } else {
            Ok(())
        }
    }

    /// Delete the service
    pub fn delete(&mut self) -> Result<(), std::io::Error> {
        let pb = self.systemd_path().join(format!("{}.service", self.name));
        println!("Deleting {}", pb.display());
        std::fs::remove_file(pb)
    }

    #[cfg(feature = "async")]
    /// Delete the service
    pub async fn delete_async(&mut self) -> Result<(), std::io::Error> {
        let pb = self.systemd_path().join(format!("{}.service", self.name));
        println!("Deleting {}", pb.display());
        tokio::fs::remove_file(pb).await
    }

    /// Reload system services if required
    fn reload(&mut self) -> Result<(), StartStopError> {
        let o = std::process::Command::new("systemctl")
            .arg("daemon-reload")
            .output()
            .map_err(|_| StartStopError::NoSystemCtl)?;
        if !o.status.success() {
            Err(StartStopError::SystemCtlFailed)
        } else {
            Ok(())
        }
    }

    /// Construct the systemd file with the specified config
    fn build_systemd_file(&self, config: ServiceConfig) -> String {
        let mut con = String::new();
        con.push_str("[Unit]\n");
        con.push_str(&format!("Description={}\n", config.description));
        con.push_str("[Service]\n");
        if let Some(user) = config.username {
            con.push_str(&format!("User={}\n", user));
        }
        con.push_str(&format!(
            "WorkingDirectory={}\n",
            config.config_path.display()
        ));
        con.push_str(&format!(
            "ExecStart={} {}\n",
            config.binary.display(),
            config.arguments.join(" ")
        ));
        con.push_str("\n[Install]\nWantedBy=multi-user.target\n");
        con
    }

    /// Create the service
    pub fn create(&mut self, config: ServiceConfig) -> Result<(), CreateError> {
        use std::io::Write;
        let con = self.build_systemd_file(config);
        let pb = self.systemd_path().join(format!("{}.service", self.name));
        println!("Saving service file as {}", pb.display());
        let mut fpw = std::fs::File::create(pb).map_err(CreateError::FileIoError)?;
        fpw.write_all(con.as_bytes())
            .map_err(CreateError::FileIoError)?;
        Ok(self.reload()?)
    }

    #[cfg(feature = "async")]
    /// Create the service
    pub async fn create_async(&mut self, config: ServiceConfig) -> Result<(), CreateError> {
        use tokio::io::AsyncWriteExt;

        let con = self.build_systemd_file(config);
        let pb = self.systemd_path().join(format!("{}.service", self.name));
        println!("Saving service file as {}", pb.display());
        let mut fpw = tokio::fs::File::create(pb)
            .await
            .map_err(CreateError::FileIoError)?;
        fpw.write_all(con.as_bytes())
            .await
            .map_err(CreateError::FileIoError)?;
        Ok(self.reload()?)
    }

    /// Run the required dispatch code for windows
    pub fn dispatch(&self, service_main: DispatchFn) -> Result<(), u32> {
        service_main();
        Ok(())
    }
}
