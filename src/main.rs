use std::{sync::Mutex, time::Duration};

use actix_web::{error::ErrorInternalServerError, get, web, Responder, Result};
use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
use rand::Rng;
use scele_frontapi::get_frontpage;
use scraper::{ElementRef, Html, Selector};
use serde::Serialize;

#[derive(Serialize, Clone)]
struct AnnouncementResponse {
    pub id: String,
    pub title: String,
    pub author: String,
    pub date_time: DateTime<Utc>,
}

#[derive(Clone)]
struct CacheData {
    pub announcements: Vec<AnnouncementResponse>,
    pub fetched_at: DateTime<Utc>,
}

struct ServerState {
    pub request_count: Mutex<u64>,
    pub cache: Mutex<Option<CacheData>>,
}

fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn try_parse_scele_datetime(raw: &str) -> Option<DateTime<Utc>> {
    let cleaned = raw.trim().trim_start_matches('-').trim();

    let formats = [
        "%A, %-d %B %Y, %-I:%M %p",
        "%A, %d %B %Y, %-I:%M %p",
        "%A, %-d %B %Y, %I:%M %p",
        "%A, %d %B %Y, %I:%M %p",
    ];

    for fmt in formats {
        if let Ok(naive) = NaiveDateTime::parse_from_str(cleaned, fmt) {
            // SCELE timestamps are shown in WIB (UTC+7)
            let wib = FixedOffset::east_opt(7 * 3600).unwrap();
            if let Some(local_dt) = wib.from_local_datetime(&naive).single() {
                return Some(local_dt.with_timezone(&Utc));
            }
        }
    }

    None
}

fn parse_announcement_datetime(element: &ElementRef) -> DateTime<Utc> {
    for text_node in element.text() {
        let candidate = normalize_whitespace(text_node);
        if candidate.is_empty() {
            continue;
        }

        if let Some(parsed) = try_parse_scele_datetime(&candidate) {
            return parsed;
        }
    }

    // Fallback if the page format changes
    Utc::now()
}

fn parse_frontpage(page: Html) -> Vec<AnnouncementResponse> {
    let selector = Selector::parse("article").unwrap();
    let title_selector = Selector::parse("h3").unwrap();
    let author_selector = Selector::parse("a").unwrap();

    let elements_iterator = page.select(&selector);
    let mut announcements = Vec::<AnnouncementResponse>::new();

    for element in elements_iterator {
        let id = String::from(element.attr("id").unwrap_or("unknown"));

        let title = element
            .select(&title_selector)
            .next()
            .map(|e| normalize_whitespace(&e.text().collect::<String>()))
            .unwrap_or_else(|| "Untitled".to_string());

        let author = element
            .select(&author_selector)
            .next()
            .map(|e| normalize_whitespace(&e.text().collect::<String>()))
            .unwrap_or_else(|| "Unknown".to_string());

        let announcement = AnnouncementResponse {
            id,
            title,
            author,
            date_time: parse_announcement_datetime(&element),
        };

        announcements.push(announcement);
    }

    announcements
}

// Starts on an Actix Worker thread
#[get("/announcements")]
async fn get_all_announcements(data: web::Data<ServerState>) -> Result<impl Responder> {
    let delay_ms = rand::thread_rng().gen_range(0..1000_u64);
    actix_web::rt::time::sleep(Duration::from_millis(delay_ms)).await;

    let cache_ttl = chrono::Duration::seconds(60);

    // Check cache first
    {
        let cache_guard = data.cache.lock().unwrap();

        if let Some(cache) = &*cache_guard {
            let age = Utc::now() - cache.fetched_at;

            if age < cache_ttl {
                let request_count = {
                    let mut count = data.request_count.lock().unwrap();
                    *count += 1;
                    *count
                };

                println!("Request count: {} (served from cache)", request_count);

                return Ok(web::Json(cache.announcements.clone()));
            }
        }
    }

    // Cache miss or cache expired:
    // fetch + parse outside the mutex so we do not block other requests unnecessarily
    let announcements = web::block(|| {
        let page = get_frontpage("https://scele.cs.ui.ac.id")?;
        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(parse_frontpage(page))
    })
    .await
    .map_err(ErrorInternalServerError)
    .and_then(|res| res.map_err(ErrorInternalServerError))?;

    // Update cache
    {
        let mut cache_guard = data.cache.lock().unwrap();
        *cache_guard = Some(CacheData {
            announcements: announcements.clone(),
            fetched_at: Utc::now(),
        });
    }

    // Safely increment request count
    let request_count = {
        let mut count = data.request_count.lock().unwrap();
        *count += 1;
        *count
    };

    println!("Request count: {} (served from fresh fetch)", request_count);

    Ok(web::Json(announcements))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(ServerState {
        request_count: Mutex::new(0),
        cache: Mutex::new(None),
    });

    use actix_web::{App, HttpServer};

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(get_all_announcements)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}