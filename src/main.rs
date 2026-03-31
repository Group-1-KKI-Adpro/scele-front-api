use std::{sync::Mutex, time::Duration};

use actix_web::{error::ErrorInternalServerError, get, web, Responder, Result};
use chrono::{DateTime, Utc};
use rand::Rng;
use scele_frontapi::get_frontpage;
use scraper::{Html, Selector};
use serde::Serialize;

#[derive(Serialize, Clone)]
struct AnnouncementResponse {
    pub id: String,
    pub title: String,
    pub author: String,
    pub date_time: DateTime<Utc>,
}
struct ServerState {
    pub request_count: Mutex<u64>,
}



fn parse_frontpage(page: Html) -> Vec<AnnouncementResponse> {
    let selector = Selector::parse("article").unwrap();
    let elements_iterator = page.select(&selector);
    let mut announcements = Vec::<AnnouncementResponse>::new();

    for element in elements_iterator {
        let id = String::from(element.attr("id").unwrap());
        let title = element
            .select(&Selector::parse("h3").unwrap())
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap();
        let author = element
            .select(&Selector::parse("a").unwrap())
            .next()
            .map(|e| e.text().collect::<String>())
            .unwrap();
        
        let announcement = AnnouncementResponse {
            id,
            title,
            author,
            date_time: Utc::now(), // TODO: Parse the time value from the HTML
        };

        announcements.push(announcement);
    }

    return announcements;
}

// Starts on an Actix Worker thread
#[get("/announcements")]
async fn get_all_announcements(data: web::Data<ServerState>) -> Result<impl Responder> {
    let delay_ms = rand::thread_rng().gen_range(0..1000_u64);
    actix_web::rt::time::sleep(Duration::from_millis(delay_ms)).await;
    // Request goes to wait
    // There will be Many Waits , the earlier one will lock it self

    // get_frontpage and HTML parsing are synchronous
    let announcements = web::block(|| {
        let page = get_frontpage("https://scele.cs.ui.ac.id")?;
        //Retrieve it as Vec<AnnouncementResponse>
        //if it went well go to Ok(Vec<AnnouncementResponse>)
        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(parse_frontpage(page))
    })
    .await
     // Either the Blocking failed or the get_frontpage fails
    .map_err(ErrorInternalServerError)
    .and_then(|res| res.map_err(ErrorInternalServerError))?;


    // We Lock it here , then safely increment it
    let request_count = {
        let mut count = data.request_count.lock().unwrap();
        *count += 1;
        *count
    };

    println!("Request count: {}", request_count);

    Ok(web::Json(announcements))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(ServerState {
        request_count: Mutex::new(0),
    });

    use actix_web::{App, HttpServer};

    HttpServer::new(move || App::new()
        .app_data(state.clone())
        .service(get_all_announcements))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
