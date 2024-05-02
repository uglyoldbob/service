fn main() {
    let mut service = service::Service::new("example-service".into());
    if service.exists() {
        panic!("Service already exists");
    }

    let mut exe = std::env::current_exe().unwrap();
    exe.pop();
    let exe = exe.join("service");

    let service_config = service::ServiceConfig::new(
        #[cfg(target_family = "windows")]
        "Example service".into(),
        vec!["example-arg1".to_string(), "arg2".to_string()],
        "The Example service".into(),
        exe,
        std::path::PathBuf::from("./"),
        None,
        #[cfg(target_family = "windows")]
        None,
    );
    service.create(service_config);
    service.start().unwrap();
}
