use std::{fs, path};

pub fn init(path: &path::Path) -> Result<(), std::io::Error> {
    fs::create_dir_all(path)?;
    fs::create_dir_all(path.join("images"))?;

    fs::write(
        path.join("body.html"),
        "Example <b>news body</b> <i>content</i>.",
    )?;
    fs::write(path.join("short.html"), "Example <b>preview</b>.")?;
    fs::write(
        path.join("meta.toml"),
        r#"slug = ""
locale = ""
title = ""
author = ""
tags = []
use_short = true

# Optional polls. Remove the `#` prefixes to enable one.
# [[polls]]
# key = "favorite-project"
# question = "Which project do you like most?"
# options = ["dibsy", "Server Eggs"]
# ends_at = "2026-12-31T23:59:59Z""#,
    )?;

    Ok(())
}
