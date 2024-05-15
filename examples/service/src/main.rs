fn smain(
    rx: Option<std::sync::mpsc::Receiver<service::ServiceEvent<u64>>>,
    _tx: Option<std::sync::mpsc::Sender<service::ServiceEvent<u64>>>,
) {
    let mut i = 0;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        i += 1;
        service::log::debug!("I am groot {}", i);
        if let Some(rx) = &rx {
            if let Ok(m) = rx.try_recv() {
                match m {
                    service::ServiceEvent::Stop => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }
}

#[cfg(windows)]
fn smain_start(
    rx: std::sync::mpsc::Receiver<service::ServiceEvent<u64>>,
    tx: std::sync::mpsc::Sender<service::ServiceEvent<u64>>,
    args: Vec<String>,
    _standalone_mode: bool,
) -> u32 {
    service::log::debug!("The service arguments are {:?}", args);
    service::log::debug!("The service env args are {:?}", std::env::args());
    smain(Some(rx), Some(tx));
    0
}

#[cfg(windows)]
service::ServiceMacro!(service_starter, smain_start);

#[cfg(windows)]
fn main() {
    let service = service::Service::new("example-service".into());
    service.new_log(service::LogLevel::Debug);
    service::log::debug!("Windows service dispatching now {:?}", std::env::args());
    if let Err(e) = service.dispatch(service_starter) {
        service::log::error!("Failed to dispatch service: {:?}", e);
    }
}

#[cfg(not(windows))]
fn main() {
    let service = service::Service::new("example-service".into());
    service.new_log(service::LogLevel::Debug);
    smain(None, None);
}
