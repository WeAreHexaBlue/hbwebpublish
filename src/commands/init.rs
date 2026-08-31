use std::{fs, path};

pub fn init(path: &path::Path) -> Result<(), std::io::Error> {
    fs::create_dir_all(path)?;

    fs::write(path.join("body.html"), "Example <b>news body</b> <i>content</i>.")?;
    fs::write(path.join("short.html"), "Example <b>preview</b>.")?;
    fs::write(path.join("meta.toml"), r#"[meta]
slug = ""
locale = ""
title = ""
use_short = true
    "#)?;

    Ok(())
}