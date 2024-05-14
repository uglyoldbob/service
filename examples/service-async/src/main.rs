async fn smain() {
    let mut i = 0;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        i += 1;
        service::log::debug!("I am groot {}", i);
    }
}

#[cfg(windows)]
async fn smain_start(
    rx: std::sync::mpsc::Receiver<service::ServiceEvent<u64>>,
    tx: std::sync::mpsc::Sender<service::ServiceEvent<u64>>,
    args: Vec<String>,
    standalone_mode: bool,
) -> u32 {
    service::log::debug!("The service arguments are {:?}", args);
    service::log::debug!("The service env args are {:?}", std::env::args());
    smain().await;
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
    service.dispatch(service_starter);
}

#[cfg(not(windows))]
#[tokio::main]
async fn main() {
    let service = service::Service::new("example-service".into());
    service.new_log(service::LogLevel::Debug);
    smain().await;
}
