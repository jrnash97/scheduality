use crate::db;
use crate::modals::ReleaseSubmission;
use crate::utils::*;
use poise::serenity_prelude as serenity;
use sqlx::Executor;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot } => {
            data.pool
                .execute(include_str!("../schema/tables.sql"))
                .await
                .expect("Could not initialise database tables");
            data.pool
                .execute(include_str!("../schema/views.sql"))
                .await
                .expect("Could not initialise database views");
            data.pool
                .execute(include_str!("../schema/functions.sql"))
                .await
                .expect("Could not initialise database functions");

            println!("{} is online.", data_about_bot.user.name);
        }
        serenity::FullEvent::GuildCreate { guild, .. } => {
            db::create_guild(guild, &data.pool)
                .await
                .expect("Couldn't add guild'");
        }
        serenity::FullEvent::GuildDelete { incomplete, .. } => {
            println!("{incomplete:#?}");
            db::remove_guild(&incomplete.id, &data.pool)
                .await
                .expect("Couldn't remove guild'")
        }
        serenity::FullEvent::InteractionCreate { interaction } => {
            if let serenity::InteractionType::Modal = interaction.kind() {
                let interaction = interaction
                    .to_owned()
                    .modal_submit()
                    .ok_or(Error::from("Something went wrong on modal submittion"))?;
                println!("{interaction:#?}");
                let modal_response = &interaction.data;
                let guild_id = interaction.guild_id.unwrap();
                let user = &interaction.user;

                let release_submission = ReleaseSubmission::from_modal_response(modal_response)?;
                println!("{release_submission:#?}");

                match db::add_release_to_guild(
                    &guild_id,
                    user,
                    ctx,
                    &release_submission,
                    &data.pool,
                )
                .await
                {
                    Ok(_) => {}
                    Err(err) => println!("{}", err),
                }
                let builder =
                    create_add_success_message(user, ctx, &guild_id, &release_submission).await?;
                interaction.create_followup(ctx, builder).await?;
            }
        }
        _ => (),
    }
    Ok(())
}

async fn create_add_success_message(
    user: &serenity::User,
    ctx: &serenity::Context,
    guild_id: &serenity::GuildId,
    release_submission: &ReleaseSubmission,
) -> Result<serenity::CreateInteractionResponseFollowup, Error> {
    let user_nick = user
        .nick_in(ctx, guild_id)
        .await
        .unwrap_or(user.global_name.clone().unwrap_or(user.name.clone()));
    Ok(serenity::CreateInteractionResponseFollowup::new().embed(
        serenity::CreateEmbed::new()
            .author(serenity::CreateEmbedAuthor::new(user_nick).icon_url(user.face()))
            .color(serenity::Color::MAGENTA)
            .title("")
            .field(
                "",
                format!(
                    "{} - {} ({})",
                    &release_submission.artist,
                    &release_submission.name,
                    release_submission
                        .label
                        .clone()
                        .unwrap_or("self".to_string()),
                ),
                false,
            )
            .field(
                "Release Date",
                release_submission.release_date.to_string(),
                false,
            )
            .footer(serenity::CreateEmbedFooter::new("via Scheduardo")),
    ))
}
