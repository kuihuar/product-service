
use std::sync::Arc;
use std::sync::Mutex;
use crate::repository::product_repos::PostgresProductRepository;
    // let product_repository = Arc::new(InMemoryProductRepository::new());
use tonic::transport::Server;
use crate::proto::product::product_service_server::ProductServiceServer;
use crate::service::product_service;
use sqlx::postgres::PgPoolOptions;
#[warn(unused_must_use)]
pub async fn start_server() ->Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    // let service_impl  = product_service::ProductServiceImpl::default();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@127.0.0.1/postgres")
        .await?;


        let product_repo = Arc::new(Mutex::new(PostgresProductRepository::new(pool).await));
    let service_impl = product_service::ProductServiceImpl::new(product_repo);
    println!("GreeterService listening on {}", addr);

    Server::builder()
        .add_service(ProductServiceServer::new(service_impl))
        .serve(addr)
        .await?;

    Ok(())
}


