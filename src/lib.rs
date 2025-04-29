use scraper::Html;

/// Get the string representation of a HTML document fetched from the given string.
pub fn get_frontpage(url: &str) -> Result<Html, ureq::Error> {
    let response_body: String = ureq::get(url).call()?.body_mut().read_to_string()?;
    Ok(Html::parse_document(&response_body))
}