use crate::utils;
use db::introduce as db;
use poise::serenity_prelude as serenity;

/// Introduction group command
#[poise::command(slash_command, prefix_command, subcommands("set", "get"))]
pub async fn introduction(
    ctx: utils::Context<'_>,
    #[description = "The text to echo back"] text: String,
) -> utils::CommandResult {
    ctx.say(format!("Pong! {}", text)).await?;
    Ok(())
}

/// Set your introduction
#[poise::command(slash_command, prefix_command)]
pub async fn set(
    ctx: utils::Context<'_>,
    #[description = "description"] text: String,
) -> utils::CommandResult {
    db::set_introduction(&ctx.data().pool, ctx.author().id.get() as i64, text).await?;
    ctx.say("success").await?;
    Ok(())
}

/// Get your introduction
#[poise::command(slash_command, prefix_command)]
pub async fn get(ctx: utils::Context<'_>, user: Option<serenity::User>) -> utils::CommandResult {
    let user = user.unwrap_or(ctx.author().clone());
    let introduction = db::get_introduction(&ctx.data().pool, user.id.get() as i64).await?;
    ctx.say(format!(
        "{}",
        user.name,
        introduction.unwrap_or("".to_string())
    ))
    .await?;
    Ok(())
}
