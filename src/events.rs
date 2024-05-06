use crate::db;
use crate::utils::*;
use poise::serenity_prelude as serenity;
use sqlx::Executor;

pub async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot } => {
            println!("{} is online.", data_about_bot.user.name);
            data.pool
                .execute(include_str!("../schema.sql"))
                .await
                .expect("Could not initialise database");
        }
        serenity::FullEvent::GuildCreate { guild, .. } => {
            db::create_guild(&guild.id, &data.pool).await?
        }
        serenity::FullEvent::GuildDelete { incomplete, .. } => {
            db::remove_guild(&incomplete.id, &data.pool).await?
        }
        _ => (),
    }
    Ok(())
}
