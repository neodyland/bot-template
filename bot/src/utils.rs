use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct Data {
    pool: Arc<PgPool>,
}

impl Data {
    pub async fn new() -> anyhow::Result<Self> {
        let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
        sqlx::migrate!("../migrations").run(&pool).await?;
        Ok(Self {
            pool: Arc::new(pool),
        })
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type CommandResult = Result<(), Error>;
