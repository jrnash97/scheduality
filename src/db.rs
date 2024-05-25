use crate::modals::ReleaseSubmission;
use crate::utils::Error;
use poise::serenity_prelude as serenity;
use sqlx::{query, query_scalar, PgPool};

// Use these type aliases to interface with the database to avoid type conversion issues
type Int = i32;
type BigInt = i64;

pub(super) mod response {
    use chrono::NaiveDate;
    use sqlx::FromRow;

    #[derive(FromRow)]
    pub struct ReleaseData {
        pub artist: String,
        pub name: String,
        pub label: Option<String>,
        pub date: NaiveDate,
    }
}

pub(crate) async fn create_guild(
    guild: &serenity::Guild,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    // Add owner to guild users
    let user_db_id: Int = query_scalar("SELECT fetch_or_insert_guild_user($1)")
        .bind(guild.owner_id.get() as BigInt)
        .fetch_one(pool)
        .await?;

    // Add Guild to database
    let guild_db_id: Int =
        query_scalar("INSERT INTO Guild (Snowflake, Owner) VALUES ($1, $2) RETURNING id;")
            .bind(guild.id.get() as BigInt)
            .bind(user_db_id as Int)
            .fetch_one(pool)
            .await?;

    initialise_guild_config(guild_db_id, pool).await?;

    Ok(())
}

pub(crate) async fn remove_guild(
    guild_id: &serenity::GuildId,
    pool: &PgPool,
) -> Result<(), sqlx::Error> {
    query("DELETE FROM Guild WHERE Snowflake=$1;")
        .bind(guild_id.get() as BigInt)
        .execute(pool)
        .await?;
    Ok(())
}

pub(crate) async fn add_release_to_guild(
    guild_id: &serenity::GuildId,
    user: &serenity::User,
    ctx: &serenity::Context,
    release_submission: &ReleaseSubmission,
    pool: &PgPool,
) -> Result<(), Error> {
    // Check that GuildUser is allowed to add releases from the current guild (Is either Guild owner, or
    // in the requested guild's privileged roles)
    let owner = query_scalar::<_, BigInt>("SELECT GuildUser.Snowflake FROM Guild INNER JOIN GuildUser ON Guild.Owner=GuildUser.id WHERE Guild.Snowflake=$1")
        .bind(guild_id.get() as BigInt)
        .fetch_one(pool)
        .await
        .expect("Couldn't find guild owner"); // TODO: May be able to handle this case by retrieving full Guild struct, although it shouldn't happen...

    let is_privileged = is_user_privileged(user, guild_id, ctx, pool).await?;

    // If user is not allowed to add releases return appropriate response (Likely an Err)
    if !(owner == user.id.get() as BigInt || is_privileged) {
        return Ok(());
    }

    // Find release if it already exists, otherwise add it to the database
    let release_id = if let Some(id) = fetch_release_id(release_submission, pool).await? {
        id
    } else {
        add_release(release_submission, pool).await?
    };

    // Link the release ID to the current guild
    let user_db_id: Int = query_scalar("SELECT fetch_or_insert_guild_user($1)")
        .bind(user.id.get() as BigInt)
        .fetch_one(pool)
        .await
        .expect("Couldn't find or insert user id'");

    let guild_db_id: Int = query_scalar("SELECT id FROM Guild WHERE Snowflake=$1")
        .bind(guild_id.get() as BigInt)
        .fetch_one(pool)
        .await?;

    query("INSERT INTO GuildRelease (GuildId, ReleaseId, GuildUserId) VALUES ($1, $2, $3)")
        .bind(guild_db_id)
        .bind(release_id)
        .bind(user_db_id)
        .execute(pool)
        .await
        .expect("Couldn't link guild to release'");

    // TODO: Return the client response message
    Ok(())
}

async fn is_user_privileged(
    user: &serenity::User,
    guild_id: &serenity::GuildId,
    ctx: &serenity::Context,
    pool: &PgPool,
) -> Result<bool, Error> {
    let mut is_privileged = false;
    let privileged_roles: Vec<BigInt> = query_scalar("SELECT GuildPrivilegedRole.Snowflake FROM GuildPrivilegedRole INNER JOIN Guild ON GuildPrivilegedRole.GuildId=Guild.id WHERE Guild.Snowflake=$1;")
        .bind(guild_id.get() as BigInt)
        .fetch_all(pool).await?;
    for role in privileged_roles.iter() {
        is_privileged = is_privileged
            || user
                .has_role(ctx, guild_id, serenity::RoleId::new(*role as u64))
                .await?
    }
    Ok(is_privileged)
}

async fn fetch_release_id(
    release_submission: &ReleaseSubmission,
    pool: &PgPool,
) -> Result<Option<Int>, sqlx::Error> {
    if let Some(label) = &release_submission.label {
        query_scalar(
            "SELECT id FROM ReleaseData WHERE name=$1 AND artist=$2 AND label=$3 AND releasedate=$4 LIMIT 1;",
        )
        .bind(&release_submission.name)
        .bind(&release_submission.artist)
        .bind(label)
        .bind(release_submission.release_date)
        .fetch_optional(pool)
        .await
    } else {
        query_scalar(
            "SELECT id FROM ReleaseData WHERE name=$1 AND artist=$2 AND label IS NULL AND releasedate=$3 LIMIT 1;",
        )
        .bind(&release_submission.name)
        .bind(&release_submission.artist)
        .bind(release_submission.release_date)
        .fetch_optional(pool)
        .await
    }
}

// TODO: Proper Error Handling
pub(crate) async fn add_release(
    release_submission: &ReleaseSubmission,
    pool: &PgPool,
) -> Result<Int, sqlx::Error> {
    let artist_id: Int = query_scalar("SELECT fetch_or_insert_artist($1);")
        .bind(&release_submission.artist)
        .fetch_one(pool)
        .await?;

    let label_id: Option<Int> = if let Some(label) = &release_submission.label {
        query_scalar("SELECT fetch_or_insert_label($1);")
            .bind(label)
            .fetch_optional(pool)
            .await
            .expect("Couldn't add label")
    } else {
        None
    };

    let release_id = query_scalar("INSERT INTO RELEASE (Artist, Label, Name, ReleaseDate) VALUES ($1, $2, $3, $4) RETURNING id;")
        .bind(artist_id)
        .bind(label_id)
        .bind(&release_submission.name)
        .bind(release_submission.release_date)
        .fetch_one(pool)
        .await
        .expect("Couldn't add release");

    println!(
        "Added Release: {} - {} [{}] {}",
        release_submission.artist,
        release_submission.name,
        release_submission
            .label
            .clone()
            .unwrap_or("self".to_string()),
        release_submission.release_date.format("%Y-%m-%d")
    );

    Ok(release_id)
}

async fn initialise_guild_config(guild_db_id: Int, pool: &PgPool) -> Result<(), sqlx::Error> {
    query("INSERT INTO GuildUpdatePolicy (GuildId) VALUES ($1)")
        .bind(guild_db_id)
        .execute(pool)
        .await?;
    Ok(())
}
