use crate::models::product_model::Product;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;

use sqlx::PgPool;

use tonic::Status;
use anyhow::{Result, Context};
#[tonic::async_trait]
pub trait ProductRepository{
    async fn find(&self, id: u64) -> Result<Product, anyhow::Error>; 
    async fn create(&self, product: &Product) ->Result<Product, anyhow::Error>;
    async fn update(&self, product: &Product) ->Result<Product, anyhow::Error>;
    async fn delete(&self, id:u64) ->Result<(), anyhow::Error>;
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
    async fn find(&self, _id: u64) -> Result<Product, anyhow::Error>{
        todo!()
    }
    async fn create(&self, _product: &Product) ->Result<Product, anyhow::Error> {
        todo!()
    }
    async fn update(&self, _product: &Product) ->Result<Product, anyhow::Error> {
        todo!()
    }
    async fn delete(&self, _id:u64) ->Result<(), anyhow::Error> {
        todo!()
    }
}


#[derive(Clone)]
pub struct PostgresProductRepository {
    pool: PgPool,
}

impl PostgresProductRepository {
    pub async fn new(pool: sqlx::PgPool) -> Box<dyn ProductRepository + Send + Sync + 'static> {
        Box::new(PostgresProductRepository {
            pool
        })
    }
}


#[tonic::async_trait]
impl ProductRepository for PostgresProductRepository {
    async fn find(&self, _: u64) -> Result<Product, anyhow::Error> {
        // let product: Product = sqlx::query_as("SELECT * FROM products WHERE id = $1")
        //     .bind(id)
        //     .fetch_one(&self.pool)
        //     .await?;
        // Ok(product)
        todo!()
    }

    async fn create(&self, product: &Product) -> Result<Product, anyhow::Error> {

        let query = "
        INSERT INTO products (name, description, price)
        VALUES ($1, $2, $3)
        RETURNING id, name, description, price
    ";

    let created_product = sqlx::query_as::<_, Product>(query)
            .bind(&product.name)
            .bind(&product.description)
            .bind(&product.price)
            .fetch_one(&self.pool)
            .await?;
        Ok(created_product)
    }
    async fn update(&self, _product: &Product) ->Result<Product, anyhow::Error> {
        todo!()
    }
    
    async fn delete(&self, _id:u64) ->Result<(), anyhow::Error> {
        todo!()
    }
}