use std::fmt::Display;

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

#[derive(Debug)]
enum ErrorKind {
    Sqlx,
    Poise,
    App,
}

#[derive(Debug)]
struct CustomError {
    kind: ErrorKind,
    error: Error,
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.kind, self.error)
    }
}

impl std::error::Error for CustomError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.error.source()
    }
}
