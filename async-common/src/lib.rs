pub async fn foo() {
    bar().await
}

async fn bar() {
    baz().await
}

async fn baz() {
    panic!()
}
