use sqlx::PgPool;

pub async fn set_introduction(
    pool: &PgPool,
    user_id: i64,
    description: String,
) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
        INSERT INTO introduction (user_id, description) VALUES ($1, $2)
        ON CONFLICT (user_id) DO UPDATE SET description = $2
        "#,
        user_id,
        description
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_introduction(pool: &PgPool, user_id: i64) -> anyhow::Result<Option<String>> {
    let row = sqlx::query!(
        r#"
        SELECT description FROM introduction WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|row| row.description))
}
