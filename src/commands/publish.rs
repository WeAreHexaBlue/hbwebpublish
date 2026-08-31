use std::{fs, path};
use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
struct Meta {
    slug: String,
    locale: String,
    title: String,
    author: String,
}

pub fn publish(dir: &path::Path) -> Result<(), Box<dyn std::error::Error>> {
    let fmeta = fs::read_to_string(dir.join("meta.toml"))?;
    let meta: Meta = toml::from_str(&fmeta)?;

    let mut short: Option<String> = None;
    if fs::exists(dir.join("short.html"))? {
        short = Some(fs::read_to_string(dir.join("short.html"))?);
        short = short.filter(|s| !s.is_empty());
    }

    let body = fs::read_to_string(dir.join("body.html"))?;

    let client = reqwest::Client::new();

    Ok(())
}