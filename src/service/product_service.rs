use std::sync::Arc;
use std::sync::Mutex;

// use crate::models::product_model::Product as ProductModel;
use crate::proto::product::{
    product_service_server::ProductService, CreateProductRequest, CreateProductResponse,
    GetProductRequest, GetProductResponse, Product, UpdateProductRequest, UpdateProductResponse,
};
use crate::repository::product_repos::ProductRepository;
// #[derive(Default)]
pub struct ProductServiceImpl {
    // repository: Box<dyn ProductRepository>,
    repository: Arc<Mutex<Box<dyn ProductRepository + Send + Sync>>>,
}

// 实现的是后面是ProductService 还是 ProductServiceImpl？？
impl ProductServiceImpl {
//   pub fn new(product_repository: Box<dyn ProductRepository>) -> Self {
//     ProductServiceImpl {
//           product_repository,
//       }
//   }
pub async fn new(repository: impl ProductRepository + 'static + Send + Sync) -> Self {
    ProductServiceImpl {
        repository: Arc::new(Mutex::new(Box::new(repository))),
    }
}

  // pub fn create_product(&self, product: &Product) -> Result<Product, anyhow::Error> {
  //     self.product_repository.create_product(product)
  // }

  // 实现其他服务方法...
}

#[tonic::async_trait]
impl ProductService for ProductServiceImpl {
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
        _: tonic::Request<CreateProductRequest>,
    ) -> Result<tonic::Response<CreateProductResponse>, tonic::Status> {
        let product = Product {
            id: "1".to_string(),
            name: "Example Product".to_string(),
            description: "This is an example product.".to_string(),
            price: 9.99,
        };
        Ok(tonic::Response::new(CreateProductResponse {
            product: Some(product),
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
