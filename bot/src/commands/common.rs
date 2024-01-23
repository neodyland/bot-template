use crate::utils;

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn ping(
    ctx: utils::Context<'_>,
    #[description = "The text to echo back"] text: String,
) -> utils::CommandResult {
    ctx.say(format!("Pong! {}", text)).await?;
    Ok(())
}
