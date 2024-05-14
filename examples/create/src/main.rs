fn main() {
    let mut service = service::Service::new("example-service".into());
    if service.exists() {
        panic!("Service already exists");
    }

    let mut exe = std::env::current_exe().unwrap();
    exe.pop();
    let exe = exe.join("example-service-async");

    let mut service_config = service::ServiceConfig::new(
        vec!["example-arg1".to_string(), "arg2".to_string()],
        "The Example service".into(),
        exe,
        None,
    );
    #[cfg(target_os = "linux")]
    {
        service_config.config_path = std::path::PathBuf::from("./");
    }
    #[cfg(target_family = "windows")]
    {
        service_config.display = "Example service".into();
        service_config.user_password = None;
    }
    service.create(service_config).unwrap();
    service.start().unwrap();
}
