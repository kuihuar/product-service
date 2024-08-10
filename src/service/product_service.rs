use std::sync::Arc;
use std::sync::Mutex;
use tonic::Status;
use crate::models::product_model::Product as ProductModel;
use crate::proto::product::{
    product_service_server::ProductService, CreateProductRequest, CreateProductResponse,
    GetProductRequest, GetProductResponse, Product, UpdateProductRequest, UpdateProductResponse,
};
use crate::repository::product_repos::ProductRepository;
// #[derive(Default)]
// pub struct ProductServiceImpl {
//     // repository: Box<dyn ProductRepository>,
//     repository: Arc<Mutex<Box<dyn ProductRepository + Send + Sync>>>,
// }

// // 实现的是后面是ProductService 还是 ProductServiceImpl？？
// impl ProductServiceImpl {
//     pub async fn new(repository: impl ProductRepository + 'static + Send + Sync) -> Self {
//         ProductServiceImpl {
//             repository: Arc::new(Mutex::new(Box::new(repository))),
//         }
//     }
// }
pub struct ProductServiceImpl {
    repository: Arc<Mutex<Box<dyn ProductRepository + Send + Sync + 'static>>>,
    
}
impl ProductServiceImpl {
    pub fn new(repository: Arc<Mutex<Box<dyn ProductRepository + Send + Sync + 'static>>>) -> Self {

        
        ProductServiceImpl { repository }
    }
}


#[tonic::async_trait]
impl ProductService for ProductServiceImpl  {
    async fn get_product(
        &self,
        _: tonic::Request<GetProductRequest>,
    ) -> Result<tonic::Response<GetProductResponse>, tonic::Status> {
        // println!(":?", request);
        let product = Product {
            id: "1".to_string(),
            name: "Example Product".to_string(),
            description: "This is an example product.".to_string(),
            price: 9.99,
        };
        Ok(tonic::Response::new(GetProductResponse {
            product: Some(product),
        }))
    }

    async fn create_product(
        &self,
        request: tonic::Request<CreateProductRequest>,
    ) -> Result<tonic::Response<CreateProductResponse>, tonic::Status> {
        // let create_product_request = request.into_inner();

        let repos = self.repository.try_lock().unwrap();

        let new_product = ProductModel {
            name: request.into_inner().name,
            description: request.into_inner().description,
            price: request.into_inner().price as f64,
            id: "1".to_string(),
        };

        let created_product: ProductModel = repos.create(&new_product).await?;
        // let created_product = repos.create(&new_product).await.map_err(ServiceError::from)?;
        Ok(tonic::Response::new(CreateProductResponse {
            product: Some(Product::from(created_product)),
        }))
    }

    async fn update_product(
        &self,
        _: tonic::Request<UpdateProductRequest>,
    ) -> Result<tonic::Response<UpdateProductResponse>, tonic::Status> {
        let product = Product {
            id: "1".to_string(),
            name: "Example Product".to_string(),
            description: "This is an example product.".to_string(),
            price: 9.99,
        };
        // let productUpdate = ProductModel {
        //     id: 1,
        //     name: "Example Product".to_string(),
        //     description: "This is an example product.".to_string(),
        //     price: 9.99,
        // };
        // let _ = self.product_repository.update_product(&productUpdate);
        Ok(tonic::Response::new(UpdateProductResponse {
            product: Some(product),
        }))
    }
}


