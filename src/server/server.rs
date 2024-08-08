
// use std::sync::Arc;
// use crate::repository::product_repos::InMemoryProductRepository;
    // let product_repository = Arc::new(InMemoryProductRepository::new());
use tonic::transport::Server;
use crate::proto::product::product_service_server::ProductServiceServer;
// use crate::service::product_service;
use crate::repository::product_repos::InMemoryProductRepository;
use crate::service::product_service::ProductServiceImpl;
#[warn(unused_must_use)]
pub async fn start_server() ->Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    // let service_impl  = product_service::ProductServiceImpl::default();
    let service_impl: ProductServiceImpl = ProductServiceImpl::new(InMemoryProductRepository::new()).await;
    println!("GreeterService listening on {}", addr);

    Server::builder()
        .add_service(ProductServiceServer::new(service_impl))
        .serve(addr)
        .await?;

    Ok(())
}