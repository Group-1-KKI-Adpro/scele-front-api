use actix_web::{get, web, Responder, Result};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
struct AnnouncementResponse {
    pub id: String,
    pub title: String,
    pub author: String,
    pub date_time: DateTime<Utc>,
}

#[get("/announcements")]
async fn get_all_announcements() -> Result<impl Responder> {
    let announcements = vec![
        AnnouncementResponse {
            id: "1".to_string(),
            title: "Hello World".to_string(),
            author: "John Doe".to_string(),
            date_time: Utc::now(),
        },
        AnnouncementResponse {
            id: "2".to_string(),
            title: "Another announcement".to_string(),
            author: "Jane Smith".to_string(),
            date_time: Utc::now(),
        },
    ];

    Ok(web::Json(announcements))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(get_all_announcements))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
