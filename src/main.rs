mod es;
mod interceptor;
mod service;
mod utils;

use dotenv::dotenv;
use es::init;
use kafka::kafka::consume_messages;
use service::products::product_server::ProductServer;
use service::ProductService;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init().await;
    consume_messages("USER_CREATED", "localhost:29092").await;
    dotenv().ok();
    let addr = "127.0.0.1:5003".parse()?;
    let product_service = ProductService::default();

    let svc = ProductServer::with_interceptor(product_service, interceptor::check_auth);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
