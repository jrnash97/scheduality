use crate::db;
use crate::modals::*;
use crate::utils::*;
use poise::serenity_prelude as serenity;
use poise::Modal;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, subcommands("add", "get"))]
pub async fn scheduality(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Say "World!"
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

/// Add release
#[poise::command(slash_command)]
pub async fn add(ctx: poise::ApplicationContext<'_, Data, Error>) -> Result<(), Error> {
    AddRelease::execute(ctx).await?;
    Ok(())
}

/// List releases
#[poise::command(slash_command)]
pub async fn get(ctx: Context<'_>) -> Result<(), Error> {
    let releases = db::fetch_releases(ctx).await;
    if let Ok(r) = releases {
        println!("{r:#?}");
    } else {
        ctx.say("Something went wrong").await?;
    }
    Ok(())
}
