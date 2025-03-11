use commands::*;
use poise::serenity_prelude as serenity;
use sqlx::PgPool;
use utils::*;

mod commands;
mod db;
mod events;
mod modals;
mod utils;

#[tokio::main]
async fn main() {
    let (token, pass, db_name, host) = connection_vars();

    let intents = serenity::GatewayIntents::non_privileged();
    let pool: PgPool =
        PgPool::connect(format!("postgresql://scheduardo:{pass}@{host}:5432/{db_name}").as_str())
            .await
            .expect("Could not connect to database");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), ping(), scheduality()],
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

fn connection_vars() -> (String, String, String, String) {
    let token = std::env::var("DISCORD_TOKEN").expect("Could not find valid DISCORD_TOKEN");
    let pass =
        std::env::var("SCHEDUARDO_DB_PASS").expect("Could not find valid database credentials");
    let host = std::env::var("SCHEDUALITY_HOST").unwrap_or("127.0.0.1".to_string());
    let db_name = if cfg!(debug_assertions) {
        "scheduality_dev"
    } else {
        "scheduality"
    }
    .to_string();

    (token, pass, db_name, host)
}
