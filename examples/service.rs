fn main() {
    let mut i = 0;
    service::new_log(service::LogLevel::Debug);
    loop {
        std::thread::sleep(std::time::Duration::from_millis(250));
        i += 1;
        log::debug!("I am groot {}", i);
    }
}
