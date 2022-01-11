
#[tokio::main]
async fn main() {
    better_backtrace::install_panic_handler().unwrap();
    common::foo().await
}
