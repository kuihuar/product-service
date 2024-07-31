
use crate::service::product_service;
use crate::proto::product;
use crate::repository::product_repos::InMemoryProductRepository;
pub async fn start_server() ->Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let product_repository = Box::new(InMemoryProductRepository::new());
    let service_impl  = product_service::ProductServiceImpl::new(product_repository);

    tonic::transport::Server::builder()
        .add_service(product::product_service_server::ProductServiceServer::new(service_impl))
        .serve(addr)
        .await?;

    Ok(())
}