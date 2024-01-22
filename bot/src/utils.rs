use std::sync::Arc;
use sqlx::PgPool;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Clone)]
pub struct Data {
    pool: Arc<PgPool>,
}
