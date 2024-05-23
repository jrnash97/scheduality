use sqlx::PgPool;

// User data, which is stored and accessible in all command invocations
#[derive(Debug)]
pub(crate) struct Data {
    pub pool: PgPool,
}

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type Context<'a> = poise::Context<'a, Data, Error>;
