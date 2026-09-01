use std::{ env, fs, path };
use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
struct Meta {
    slug: String,
    locale: String,
    title: String,
    author: String,
    use_short: Option<bool>
}

#[derive(Debug, Serialize)]
struct NewsPubPUT<'a> {
    slug: &'a str,
    locale: &'a str,
    title: &'a str,
    author: &'a str,
    body: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    short: Option<&'a str>,
}

#[derive(Debug, Deserialize)]
struct NewsPubPUTRes {
    created: bool,
    slug: String,
    locale: String
}

pub fn publish(dir: &path::Path, token: &String) -> Result<(), Box<dyn std::error::Error>> {
    let fmeta = fs::read_to_string(dir.join("meta.toml"))?;
    let meta: Meta = toml::from_str(&fmeta)?;

    let body = fs::read_to_string(dir.join("body.html"))?;
    
    let fshort = if meta.use_short != Some(false) { fs::read_to_string(dir.join("short.html")).ok() } else { None };
    let short = fshort.as_deref().map(str::trim).filter(|s| !s.is_empty());

    let rbody = NewsPubPUT {
        slug: &meta.slug,
        locale: &meta.locale,
        title: &meta.title,
        author: &meta.author,
        body: body.trim(),
        short
    };

    let use_test = env::var("USE_TEST")
        .ok()
        .and_then(|value| value.trim().parse::<bool>().ok())
        .unwrap_or(false);

    let endpoint = if !use_test { "https://hexa.blue/news/pub" } else { "http://localhost:5173/news/pub" };

    let client = reqwest::blocking::Client::new();
    let res = client
        .put(endpoint)
        .bearer_auth(token)
        .json(&rbody)
        .send()?;

    if !res.status().is_success() {
        return Err(format!("HTTP Error {}: {}", res.status(), res.text().unwrap_or_default()).into());
    }

    let rdata: NewsPubPUTRes = res.json()?;
    let action = if rdata.created { "Created" } else { "Updated" };
    println!("{action} article `{}` for locale {}", rdata.slug, rdata.locale);

    fs::remove_dir_all(dir)?;

    Ok(())
}