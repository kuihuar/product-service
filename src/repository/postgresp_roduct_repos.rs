// use sqlx::PgPool;

// pub struct PostgresProductRepository {
//     pool: PgPool,
// }

// impl PostgresProductRepository {
//     pub async fn new(database_url: &str) -> Result<Self, anyhow::Error> {
//         let pool = PgPool::connect(database_url).await?;
//         Ok(PostgresProductRepository { pool })
//     }
// }

