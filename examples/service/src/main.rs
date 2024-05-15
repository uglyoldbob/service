fn smain(
    rx: Option<std::sync::mpsc::Receiver<service::ServiceEvent<u64>>>,
    _tx: Option<std::sync::mpsc::Sender<service::ServiceEvent<u64>>>,
) {
    service::log::debug!("Service args are now {:?}", std::env::args());
    let mut i = 0;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        i += 1;
        service::log::debug!("I am groot {}", i);
        if let Some(rx) = &rx {
            if let Ok(service::ServiceEvent::Stop) = rx.try_recv() {
                break;
            }
        }
    }
}

service::ServiceMacro!(service_starter, smain, u64);

fn main() {
    let service = service::Service::new("example-service".into());
    service.new_log(service::LogLevel::Debug);
    if let Err(e) = service.dispatch(service_starter) {
        service::log::error!("Failed to dispatch service: {:?}", e);
    }
}
