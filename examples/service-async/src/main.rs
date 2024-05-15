async fn smain() {
    let mut i = 0;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        i += 1;
        service::log::debug!("I am groot async {}", i);
    }
}

#[cfg(windows)]
async fn smain_start(
    mut rx: tokio::sync::mpsc::Receiver<service::ServiceEvent<u64>>,
    _tx: tokio::sync::mpsc::Sender<service::ServiceEvent<u64>>,
    args: Vec<String>,
    _standalone_mode: bool,
) -> u32 {
    service::log::debug!("The service arguments are {:?}", args);
    service::log::debug!("The service env args are {:?}", std::env::args());
    let main = tokio::task::spawn(smain());

    loop {
        tokio::select! {
            Some(m) = rx.recv() => {
                service::log::debug!("Received message {:?}", m);
                if let service::ServiceEvent::Stop = m {
                    service::log::debug!("Attempting to stop");
                    break;
                }
            }
        }
    }
    main.abort();
    0
}

#[cfg(windows)]
service::ServiceAsyncMacro!(service_starter, smain_start, u64);

#[cfg(windows)]
#[tokio::main]
async fn main() {
    let service = service::Service::new("example-service".into());
    service.new_log(service::LogLevel::Debug);
    service::log::debug!("Windows service dispatching now {:?}", std::env::args());
    if let Err(e) = service.dispatch(service_starter) {
        service::log::error!("Failed to dispatch service: {:?}", e);
    }
    service::log::debug!("Service stopping");
}

#[cfg(not(windows))]
#[tokio::main]
async fn main() {
    let service = service::Service::new("example-service".into());
    service.new_log(service::LogLevel::Debug);
    smain().await;
}
