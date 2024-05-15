fn main() {
    let mut service = service::Service::new("example-service".into());
    if !service.exists() {
        panic!("Service does not exist");
    }
    let e = service.stop();
    println!("Stop service result is {:?}", e);
    service.delete().unwrap();
}
