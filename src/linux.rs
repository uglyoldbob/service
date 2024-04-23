use std::path::PathBuf;

/// The configuration for constructing a Service.
pub struct ServiceConfig {
    /// The display name of the service for the user.
    display: String,
    /// The short name of the service
    shortname: String,
    /// The description of the service as presented to the user
    description: String,
    /// The path to the service binary
    binary: PathBuf,
    /// The path to the configuration data for the service
    config_path: PathBuf,
    /// The username that the service should run as
    username: String,
}

impl ServiceConfig {
    /// Build a new service config with reasonable defaults.
    /// # Arguments
    /// * display - The display name of the service
    /// * description - The description of the service
    /// * binary - The path to the binary that runs the service
    /// * config_path - The configuration path for the service
    /// * username - The username the service runs as
    pub fn new(
        display: String,
        shortname: String,
        description: String,
        binary: PathBuf,
        config_path: PathBuf,
        username: String,
    ) -> Self {
        Self {
            display,
            shortname,
            description,
            binary,
            config_path,
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
    pub fn stop(&mut self) -> Result<(), ()> {
        let o = std::process::Command::new("systemctl")
            .arg("stop")
            .arg(&self.name)
            .output()
            .unwrap();
        if !o.status.success() {
            Err(())
        } else {
            Ok(())
        }
    }

    /// Start the service
    pub fn start(&mut self) -> Result<(), ()> {
        let o = std::process::Command::new("systemctl")
            .arg("start")
            .arg(&self.name)
            .output()
            .unwrap();
        if !o.status.success() {
            Err(())
        } else {
            Ok(())
        }
    }

    /// Delete the service
    pub async fn delete(&mut self) {
        let pb = self.systemd_path().join(format!("{}.service", self.name));
        println!("Deleting {}", pb.display());
        std::fs::remove_file(pb).unwrap();
    }

    /// Reload system services if required
    pub fn reload(&mut self) {
        let o = std::process::Command::new("systemctl")
            .arg("daemon-reload")
            .output()
            .unwrap();
        if !o.status.success() {
            panic!("Failed to reload systemctl");
        }
    }

    /// Create the service
    pub async fn create(&mut self, config: ServiceConfig) {
        use tokio::io::AsyncWriteExt;

        let mut con = String::new();
        con.push_str(&format!(
            "[Unit]
Description={4}

[Service]
User={2}
WorkingDirectory={0}
ExecStart={3} --name={1}

[Install]
WantedBy=multi-user.target
",
            config.config_path.display(),
            config.shortname,
            config.username,
            config.binary.display(),
            config.description,
        ));
        let pb = self.systemd_path().join(format!("{}.service", self.name));
        println!("Saving service file as {}", pb.display());
        let mut fpw = tokio::fs::File::create(pb).await.unwrap();
        fpw.write_all(con.as_bytes())
            .await
            .expect("Failed to write service file");
        self.reload();
    }
}
