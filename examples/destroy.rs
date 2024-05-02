fn main() {
    let mut service = service::Service::new("example-service".into());
    if !service.exists() {
        panic!("Service does not exist");
    }
    service.stop().unwrap();
    service.delete().unwrap();
}
