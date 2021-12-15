fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(foo());
}

async fn foo() {
    tokio::spawn(bar()).await;
}

async fn bar() {
    panic!();
}