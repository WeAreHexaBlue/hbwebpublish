use std::{ env, fs, path };
use base64::{ engine::general_purpose::STANDARD, Engine as _ };
use serde::{ Serialize, Deserialize };

const MAX_IMAGE_BYTES: u64 = 8 * 1024 * 1024;
const IMAGE_EXTENSIONS: [&str; 6] = ["png", "jpg", "jpeg", "gif", "webp", "avif"];

#[derive(Debug, Serialize, Deserialize)]
struct Meta {
    slug: String,
    locale: String,
    title: String,
    author: String,
    #[serde(default)]
    tags: Option<Vec<String>>,
    use_short: Option<bool>
}

#[derive(Debug, Serialize)]
struct NewsImage {
    name: String,
    data: String
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
    #[serde(skip_serializing_if = "Option::is_none")]
    tags: Option<&'a [String]>,
    images: &'a [NewsImage]
}

#[derive(Debug, Deserialize)]
struct NewsPubPUTRes {
    created: bool,
    slug: String,
    locale: String
}

fn collect_images(dir: &path::Path) -> Result<Vec<NewsImage>, Box<dyn std::error::Error>> {
    let images_dir = dir.join("images");

    if !images_dir.exists() { return Ok(vec![]); }
    if !images_dir.is_dir() { return Err("`images` must be a directory.".into()); }

    let mut entries = fs::read_dir(&images_dir)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.file_name());

    let mut images = Vec::new();

    for entry in entries {
        let image_path = entry.path();
        if !image_path.is_file() { continue; }

        let name = entry.file_name().to_string_lossy().into_owned();
        let extension = image_path
            .extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();

        if !IMAGE_EXTENSIONS.contains(&extension.as_str()) {
            eprintln!("Skipping unsupported image `{name}`.");
            continue;
        }

        let size = fs::metadata(&image_path)?.len();
        if size > MAX_IMAGE_BYTES {
            return Err(format!("Image `{name}` is larger than 8 MB.").into());
        }

        let bytes = fs::read(&image_path)?;
        images.push(NewsImage { name, data: STANDARD.encode(bytes) });
    }

    Ok(images)
}

pub fn publish(dir: &path::Path, token: &String, preserve: bool) -> Result<(), Box<dyn std::error::Error>> {
    let fmeta = fs::read_to_string(dir.join("meta.toml"))?;
    let meta: Meta = toml::from_str(&fmeta)?;

    let body = fs::read_to_string(dir.join("body.html"))?;
    
    let fshort = if meta.use_short != Some(false) { fs::read_to_string(dir.join("short.html")).ok() } else { None };
    let short = fshort.as_deref().map(str::trim).filter(|s| !s.is_empty());

    let images = collect_images(dir)?;

    let rbody = NewsPubPUT {
        slug: &meta.slug,
        locale: &meta.locale,
        title: &meta.title,
        author: &meta.author,
        body: body.trim(),
        short,
        tags: meta.tags.as_deref(),
        images: &images
    };

    let use_test = env::var("USE_TEST")
        .ok()
        .and_then(|value| value.trim().parse::<bool>().ok())
        .unwrap_or(false);

    let default_endpoint = if !use_test { "https://hexa.blue/news/pub" } else { "http://localhost:5173/news/pub" };
    let endpoint = env::var("PUBLISH_ENDPOINT")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| default_endpoint.to_string());

    let client = reqwest::blocking::Client::new();
    let res = client
        .put(&endpoint)
        .bearer_auth(token)
        .json(&rbody)
        .send()?;

    if !res.status().is_success() {
        return Err(format!("HTTP Error {}: {}", res.status(), res.text().unwrap_or_default()).into());
    }

    let rdata: NewsPubPUTRes = res.json()?;
    let action = if rdata.created { "Created" } else { "Updated" };
    println!("{action} article `{}` for locale {}", rdata.slug, rdata.locale);

    if !images.is_empty() {
        println!("Uploaded {} image(s).", images.len());
    }

    if !preserve { fs::remove_dir_all(dir)? };

    Ok(())
}