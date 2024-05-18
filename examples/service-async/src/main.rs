async fn smain() {
    let mut i = 0;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        i += 1;
        service::log::debug!("I am groot async {}", i);
    }
}

service::ServiceAsyncMacro!(service_starter, smain, u64);

#[tokio::main]
async fn main() {
    #[cfg(target_family = "windows")]
    {
        std::panic::set_hook(Box::new(|p| {
            service::log::debug!("Panic {:?}", p);
            service::log::error!("Backtrace3: {:?}", std::backtrace::Backtrace::capture());
        }))
    }

    let service = service::Service::new("example-service".into());
    service.new_log(service::LogLevel::Debug);
    service::log::debug!("Service dispatching now {:?}", std::env::args());
    if let Err(e) = service::DispatchAsync!(service, service_starter) {
        service::log::error!("Failed to dispatch service: {:?}", e);
    }
    service::log::debug!("Service stopping");
}
