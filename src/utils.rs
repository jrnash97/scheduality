use sqlx::PgPool;

pub(crate) struct Data {
    pub pool: PgPool,
} // User data, which is stored and accessible in all command invocations
pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type Context<'a> = poise::Context<'a, Data, Error>;
