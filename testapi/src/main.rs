use librus_api::hello;
use librus_api::test_connect;

#[tokio::main]
async fn main() {
    hello();
    test_connect().await;
    
}