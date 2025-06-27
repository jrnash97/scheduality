use actix_web::{web, Responder};
use scheduality::scheduality_db::{SchedualityDb, SchedualityDbExt};
use serde::{Deserialize, Serialize};
use sqlx::{types::uuid::Uuid, Postgres};
use std::env;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AddReleaseInfo {
    entity_id: String,
    tag: Option<String>,
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
            .service(
                web::scope("/api")
                    .service(web::scope("/releases").service(add_release))
                    .service(
                        web::scope("/entities")
                            .service(delete_entity)
                            .service(create_entity),
                    ),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[actix_web::get("/echo")]
async fn echo() -> &'static str {
    "Hello, Scheduality!"
}

#[actix_web::post("/add")]
async fn add_release(info: web::Json<AddReleaseInfo>, data: web::Data<AppData>) -> impl Responder {
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

#[actix_web::post("/create")]
async fn create_entity(data: web::Data<AppData>) -> String {
    let db = &data.db;
    let q = "INSERT INTO Entity DEFAULT VALUES RETURNING uid;";
    let client_id: Uuid = sqlx::query_scalar(q).fetch_one(db).await.unwrap();
    client_id.simple().to_string()
}

#[actix_web::delete("/delete")]
async fn delete_entity(info: web::Query<String>, data: web::Data<AppData>) -> String {
    if let Ok(uuid) = Uuid::parse_str(&info) {
        let db = &data.db;
        let q = format!("DELETE FROM Entity WHERE uid = {}", uuid);
        if let Ok(_) = sqlx::query(&q).execute(db).await {
            format!("Entity {} deleted successfully", uuid.simple())
        } else {
            format!(
                "Something went wrong: Could not delete Entitiy {}",
                uuid.simple()
            )
        }
    } else {
        "Something went wrong: could not parse Guild uuid".to_string()
    }
}
