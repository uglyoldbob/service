fn main() {
    let mut service = service::Service::new("example-service".into());
    if !service.exists() {
        panic!("Service does not exist");
    }
    let _e = service.stop();
    service.delete().unwrap();
}
