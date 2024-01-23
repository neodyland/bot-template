use crate::utils;
use poise::serenity_prelude as serenity;

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn ping(
    ctx: Context<'_>,
    #[description = "The text to echo back"] text: String,
) -> Result<(), Error> {
    ctx.say(format!("Pong! {}", text)).await?;
    Ok(())
}
