use actix_web::{web, Responder};
use scheduality::scheduality_db::{ExtSchedualityDb, SchedualityDb};
use serde::{Deserialize, Serialize};
use sqlx::Postgres;
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
    db: SchedualityDb<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cmd = clap::Command::new("")
        .arg(
            clap::Arg::new("drop_db")
                .long("drop-database")
                .required(false)
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let con_var = env::var(if cfg!(debug_assertions) {
        "SCHEDUALITY_TEST_DB_URL"
    } else {
        "DB_URI"
    })
    .unwrap();

    let db = SchedualityDb::connect(&con_var).await.unwrap();
    if *cmd.get_one::<bool>("drop_db").unwrap_or(&false) {
        db.drop_tables().await.unwrap();
    }
    db.setup().await.unwrap();

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(web::Data::new(AppData { db: db.clone() }))
            .service(echo)
            .service(web::scope("/api").service(add_release))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[actix_web::get("/echo")]
async fn echo() -> &'static str {
    "Hello, Scheduality!"
}

#[actix_web::post("/add-release")]
async fn add_release(info: web::Json<ReleaseInfo>, data: web::Data<AppData>) -> impl Responder {
    let _db_connection_pool = &data.db;
    let input = format!("{info:#?}");

    println!("{}", input);

    let release_date = match chrono::NaiveDate::parse_from_str(&info.release_date, "%Y%m%d") {
        Ok(date) => date,
        Err(_) => return "Invalid Date Format (use yyyymmdd)".to_string(),
    };

    println!("{release_date:#?}");
    input.to_string()
}
