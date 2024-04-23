fn main() {
    let mut i = 0;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(250));
        i += 1;
        println!("I am groot {}", i);
    }
}
