use sqlx::PgPool;

// User data, which is stored and accessible in all command invocations
#[derive(Debug)]
pub(crate) struct Data {
    pub pool: PgPool,
}

// TODO: Custom Error Type which encapsulates all other error types to handle both server-side
// logging and client facing error messages
pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type Context<'a> = poise::Context<'a, Data, Error>;
