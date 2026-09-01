use std::env;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct NewsPubDELETERes {
    slug: String,
    locale: String,
}

pub fn delete(slug: String, locale: String, token: &String) -> Result<(), Box<dyn std::error::Error>> {
    let use_test = env::var("USE_TEST")
        .ok()
        .and_then(|value| value.trim().parse::<bool>().ok())
        .unwrap_or(false);

    let endpoint = if !use_test { "https://hexa.blue/news/pub" } else { "http://localhost:5173/news/pub" };

    let client = reqwest::blocking::Client::new();
    let res = client
        .delete(endpoint)
        .bearer_auth(token)
        .query(&[("slug", slug), ("locale", locale)])
        .send()?;

    if !res.status().is_success() {
        return Err(format!("HTTP Error {}: {}", res.status(), res.text().unwrap_or_default()).into());
    }

    let rdata: NewsPubDELETERes = res.json()?;
    println!("Deleted article `{}` for locale `{}`", rdata.slug, rdata.locale);

    Ok(())
}