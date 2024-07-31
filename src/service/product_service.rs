use std::sync::Arc;

use crate::proto::product::{
  Product,
  GetProductRequest,GetProductResponse,
  UpdateProductRequest,UpdateProductResponse,
  CreateProductRequest,CreateProductResponse,
  product_service_server::ProductService};
use crate::repository::product_repos;
use crate::models::product_model::Product as ProductModel; 
pub struct ProductServiceImpl {
  product_repository: Arc<dyn product_repos::ProductRepository>
}



impl ProductServiceImpl {
    pub fn new(product_repos: Arc<dyn product_repos::ProductRepository>) -> Self {
      ProductServiceImpl{ product_repository: product_repos }
    }
}
// async fn create_product(
//   &self,
//   request: tonic::Request<super::CreateProductRequest>,
// ) -> std::result::Result<
//   tonic::Response<super::CreateProductResponse>,
//   tonic::Status,
// >;
// async fn get_product(
//   &self,
//   request: tonic::Request<super::GetProductRequest>,
// ) -> std::result::Result<
//   tonic::Response<super::GetProductResponse>,
//   tonic::Status,
// >;
// async fn update_product(
//   &self,
//   request: tonic::Request<super::UpdateProductRequest>,
// ) -> std::result::Result<
//   tonic::Response<super::UpdateProductResponse>,
//   tonic::Status,
// >;


#[tonic::async_trait]
impl ProductService for ProductServiceImpl {
    async fn get_product(
        &self,
        _: tonic::Request<GetProductRequest>,
    ) -> Result<tonic::Response<GetProductResponse>, tonic::Status> {
        // 在这里实现获取产品的逻辑
        // let product = Product {
        //     id: "1".to_string(),
        //     name: "Example Product".to_string(),
        //     description: "This is an example product.".to_string(),
        //     price: 9.99,
        // };
        // let id = request.into_inner().id;

    let product = self.product_repository.get_product(1).await?;

    Ok(tonic::Response::new(GetProductResponse {
        product: Some(product),
    }))


        // let _ = self.product_repository.get_product(1);
        // Ok(tonic::Response::new(product))
    }

    async fn create_product(
        &self,
        request: tonic::Request<CreateProductRequest>,
      ) -> Result<tonic::Response<CreateProductResponse>, tonic::Status> {
        let product = Product {
          id: "1".to_string(),
          name: "Example Product".to_string(),
          description: "This is an example product.".to_string(),
          price: 9.99,
      };
      let productModel = ProductModel {
        id: 1,
        name: "Example Product".to_string(),
        description: "This is an example product.".to_string(),
        price: 9.99,
      };
      let _= self.product_repository.create_product(&productModel);
      
      Ok(tonic::Response::new(CreateProductResponse {
        product: Some(product),
    }))


      }
    
      async fn update_product(
        &self,
        request: tonic::Request<UpdateProductRequest>,
      ) -> Result<tonic::Response<UpdateProductResponse>, tonic::Status> {
        let product = Product {
          id: "1".to_string(),
          name: "Example Product".to_string(),
          description: "This is an example product.".to_string(),
          price: 9.99,
      };
      let productUpdate = ProductModel {
        id: 1,
        name: "Example Product".to_string(),
        description: "This is an example product.".to_string(),
        price: 9.99,
      };
      let _ = self.product_repository.update_product(&productUpdate);
      Ok(tonic::Response::new(UpdateProductResponse {
        product: Some(product),
    }))


      }



    // 实现其他 ProductServer trait 方法, 如 create_product、update_product 等
}