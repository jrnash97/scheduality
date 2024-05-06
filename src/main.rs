use commands::*;
use poise::serenity_prelude as serenity;
use sqlx::PgPool;
use utils::*;

mod commands;
mod db;
mod events;
mod utils;

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Could not find valid DISCORD_TOKEN");
    let db_pass = std::env::var("DB_PASS").expect("Could not find DATABASE_PASS");
    let intents = serenity::GatewayIntents::non_privileged();
    let pool: PgPool = PgPool::connect(
        format!(
            "postgresql://postgres:{}@138.68.176.79:5432/postgres",
            db_pass
        )
        .as_str(),
    )
    .await
    .expect("Could not connect to database");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), hello()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(events::event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { pool })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
