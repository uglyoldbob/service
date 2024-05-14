fn smain() {
    let mut i = 0;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(250));
        i += 1;
        service::log::debug!("I am groot {}", i);
    }
}

#[cfg(windows)]
extern "system" fn smain_start(argc: winapi::shared::minwindef::DWORD, argv: *mut winapi::um::winnt::LPWSTR) {
    let args = service::convert_args(argc, argv);
    service::log::debug!("The service arguments are {:?}", args);
    smain();
}

#[cfg(windows)]
service::ServiceMacro!(service_starter, smain_start);

#[cfg(windows)]
fn main() {
    let service = service::Service::new("example-service".into());
    service.new_log(service::LogLevel::Debug);
    service::log::debug!("Windows service dispatching now");
    service.dispatch(service_starter);
}

#[cfg(not(windows))]
fn main() {
    let service = service::Service::new("example-service".into());
    service.new_log(service::LogLevel::Debug);
    smain();
}
