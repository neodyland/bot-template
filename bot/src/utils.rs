use std::sync::Arc;
use sqlx::PgPool;

#[derive(Clone)]
pub struct Data {
    pool: Arc<PgPool>,
}

impl Data {
    pub async fn new() -> Result<Self, Error> {
        let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self {
            pool: Arc::new(pool),
        })
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type Result = Result<(), Error>;