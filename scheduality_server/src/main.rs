use actix_web::{web, Responder};
use chrono;
use clap;
use serde::{Deserialize, Serialize};
use sqlx::postgres;
use std::env;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ReleaseInfo {
    user_id: String,
    artist: String,
    release_name: String,
    release_date: String,
    label: Option<String>,
}

struct AppData {
    db_connection_pool: postgres::PgPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cmd = clap::Command::new("scheduality")
        .arg(
            clap::Arg::new("drop_db")
                .long("drop-database")
                .required(false)
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let con_var = if cfg!(debug_assertions) {
        "SCHEDUALITY_TEST_DB_URL"
    } else {
        "DB_URI"
    };
    let db_connection_pool = postgres::PgPool::connect(&env::var(con_var).unwrap())
        .await
        .unwrap();
    if *cmd.get_one::<bool>("drop_db").unwrap() {
        drop_db_tables(&db_connection_pool).await.unwrap();
    }
    db_setup(&db_connection_pool).await.unwrap();
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(web::Data::new(AppData {
                db_connection_pool: db_connection_pool.clone(),
            }))
            .service(echo)
            .service(web::scope("/api").service(add_release))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn drop_db_tables(db_connection_pool: &postgres::PgPool) -> Result<(), sqlx::Error> {
    use std::fs;

    let drop_query = fs::read_to_string("./schema/drop.sql").unwrap();
    sqlx::raw_sql(&drop_query)
        .execute(db_connection_pool)
        .await?;

    Ok(())
}

async fn db_setup(db_connection_pool: &postgres::PgPool) -> Result<(), sqlx::Error> {
    use std::fs;

    let tables_schema = fs::read_to_string("./schema/tables.sql")?;
    let tables_future = sqlx::raw_sql(&tables_schema).execute(db_connection_pool);
    let functions_schema = fs::read_to_string("./schema/functions.sql")?;
    let views_schema = fs::read_to_string("./schema/views.sql")?;
    tables_future.await?;

    let functions_future = sqlx::raw_sql(&functions_schema).execute(db_connection_pool);
    let views_future = sqlx::raw_sql(&views_schema).execute(db_connection_pool);

    functions_future.await?;
    views_future.await?;

    Ok(())
}

#[actix_web::get("/echo")]
async fn echo() -> &'static str {
    "Hello!"
}

#[actix_web::post("/add-release")]
async fn add_release(info: web::Json<ReleaseInfo>, data: web::Data<AppData>) -> impl Responder {
    let db_connection_pool = &data.db_connection_pool;
    let input = format!("{info:#?}");

    println!("{}", input);

    let release_date = match chrono::NaiveDate::parse_from_str(&info.release_date, "%Y%m%d") {
        Ok(date) => date,
        Err(_) => return "Invalid Date Format (use yyyymmdd)".to_string(),
    };

    println!("{release_date:#?}");
    input.to_string()
}
