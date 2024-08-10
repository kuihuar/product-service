use sqlx::{postgres::PgRow, Error, FromRow, Row};

#[derive(Debug, Clone)]


pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: f64,
}

use crate::proto::product::Product as ProtoProduct;



impl From <ProtoProduct>  for Product{
    fn from(product: ProtoProduct) -> Self {
        Product { id: product.id, name: product.name, description: product.description, price: product.price as f64 }
    }
}


impl From <Product>  for ProtoProduct{
    fn from(product: Product) -> Self {
        ProtoProduct { id: product.id, name: product.name, description: product.description, price: product.price as f32 }
    }
}




impl FromRow<'_, PgRow> for Product {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Product {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            price: row.get("price"),
        })
    }
}