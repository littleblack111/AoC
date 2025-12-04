use std::{env, str::pattern::Pattern, sync::Arc};

use reqwest::{Url, blocking::Client, cookie::Jar};

const YEAR: usize = 2025;

pub fn fetch(lv: usize, p: impl Pattern) -> Vec<String> {
    let jar = Arc::new(Jar::default());
    jar.add_cookie_str(
        &format!(
            "session={}",
            env::var("AOC_SESSION").unwrap()
        ),
        &Url::parse("https:///adventofcode.com").unwrap(),
    );

    Client::builder()
        .cookie_provider(jar)
        .build()
        .unwrap()
        .get(format!("https://adventofcode.com/{YEAR}/day/{lv}/input"))
        .send()
        .unwrap()
        .text()
        .unwrap()
        .split(p)
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect()
}
