#[no_mangle]
pub extern "C" fn hello() {
    tokio::spawn(async { println!("Hello world!") });
}
