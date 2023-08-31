use std::time::Duration;

use libloading::{library_filename, Library, Symbol};

#[tokio::main]
async fn main() {
    let plugin = unsafe { Library::new(library_filename("plugin")) }.unwrap();
    let func: Symbol<extern "C" fn()> = unsafe { plugin.get(b"hello\0") }.unwrap();
    func();
    // Wait output.
    tokio::time::sleep(Duration::from_secs(1)).await;
}
