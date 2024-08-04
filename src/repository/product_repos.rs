use crate::models::product_model::Product;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;


#[tonic::async_trait]
pub trait ProductRepository{
    async fn get_product(&self, id: u64) -> Result<Product, anyhow::Error>; 
    async fn create_product(&self, product: &Product) ->Result<Product, anyhow::Error>;
    async fn update_product(&self, product: &Product) ->Result<Product, anyhow::Error>;
    async fn delete_product(&self, id:u64) ->Result<(), anyhow::Error>;
}
#[derive(Clone)]
pub struct InMemoryProductRepository{
    products: Arc<RwLock<HashMap<u64, Product>>>,
}

impl InMemoryProductRepository {
    pub fn new() -> Self {
        InMemoryProductRepository {
            products: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[tonic::async_trait]
impl ProductRepository for InMemoryProductRepository{
    async fn get_product(&self, _id: u64) -> Result<Product, anyhow::Error>{
        todo!()
        // let products = self.products.read().await;
        // products.get(&id).cloned().ok_or_else(|| anyhow::anyhow!("Product not found"))
    }
    
    async fn create_product(&self, _product: &Product) ->Result<Product, anyhow::Error> {
        todo!()
    }
    
    async fn update_product(&self, _product: &Product) ->Result<Product, anyhow::Error> {
        todo!()
    }
    
    async fn delete_product(&self, _id:u64) ->Result<(), anyhow::Error> {
        todo!()
    }
}


// pub struct PostgresProductRepository {
//     // 具体的 PostgreSQL 实现
// }

// impl ProductRepository for PostgresProductRepository {
//     async fn get_product(&self, _id: u64) -> Result<Product, anyhow::Error> {
//         todo!()
//     }
    
//     async fn create_product(&self, _product: &Product) ->Result<Product, anyhow::Error> {
//         todo!()
//     }
    
//     async fn update_product(&self, _product: &Product) ->Result<Product, anyhow::Error> {
//         todo!()
//     }
    
//     async fn delete_product(&self, _id:u64) ->Result<(), anyhow::Error> {
//         todo!()
//     }
//     // 实现 ProductRepository trait 的方法
// }