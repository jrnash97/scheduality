use poise::serenity_prelude as serenity;
use sqlx::{query, query_as, FromRow, PgPool};

#[derive(FromRow)]
struct InsertId {
    id: i32,
}

pub(crate) async fn create_guild(
    guild_id: &serenity::GuildId,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    let guild_db_id: InsertId =
        query_as("INSERT INTO guilds (Snowflake) VALUES ($1) RETURNING id;")
            .bind(guild_id.get() as i64)
            .fetch_one(pool)
            .await?;
    initialise_guild_config(guild_db_id.id, pool).await?;
    Ok(())
}

pub(crate) async fn remove_guild(
    guild_id: &serenity::GuildId,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    query("DELETE FROM Guilds WHERE Snowflake=$1")
        .bind(guild_id.get() as i64)
        .execute(pool)
        .await?;
    Ok(())
}

async fn initialise_guild_config(guild_db_id: i32, pool: &PgPool) -> Result<(), sqlx::Error> {
    query("INSERT INTO GuildUpdatePolicies (GuildId) VALUES ($1)")
        .bind(guild_db_id)
        .execute(pool)
        .await?;
    Ok(())
}
