use std::env;

use crate::i18n::Lang;

fn normalize_site_url(raw: String) -> String {
    raw.trim().trim_end_matches('/').to_string()
}

pub fn site_url() -> String {
    let default_url = "https://www.smartdawn.eu".to_string();
    let raw = env::var("SITE_URL").unwrap_or(default_url);
    let normalized = normalize_site_url(raw);
    if normalized.is_empty() {
        "https://www.smartdawn.eu".to_string()
    } else {
        normalized
    }
}

pub fn render_document(
    body_html: &str,
    title: &str,
    description: &str,
    lang: Lang,
    path: &str,
    noindex: bool,
) -> String {
    let base = site_url();
    let canonical = format!("{}{}", base, path);
    let html_lang = lang.code();
    let robots = if noindex {
        "noindex,nofollow"
    } else {
        "index,follow"
    };
    let ld_json = format!(
        r#"{{"@context":"https://schema.org","@type":"Organization","name":"Smart Dawn","url":"{}","description":"{}"}}"#,
        base, description
    );

    // hreflang alternates for the homepage in each language.
    let alternates = format!(
        r#"<link rel="alternate" hreflang="sk" href="{b}/">
  <link rel="alternate" hreflang="en" href="{b}/en">
  <link rel="alternate" hreflang="cs" href="{b}/cz">
  <link rel="alternate" hreflang="de" href="{b}/de">
  <link rel="alternate" hreflang="x-default" href="{b}/">"#,
        b = base
    );

    format!(
        r##"<!doctype html>
<html lang="{html_lang}">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width,initial-scale=1">
  <script>document.documentElement.classList.add('js')</script>
  <title>{title}</title>
  <meta name="description" content="{description}">
  <meta name="robots" content="{robots}">
  <link rel="canonical" href="{canonical}">
  {alternates}
  <meta name="theme-color" content="#0a0b10">
  <meta name="color-scheme" content="dark">
  <link rel="icon" href="/assets/favicon.svg" type="image/svg+xml">
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Space+Grotesk:wght@400;500;600;700&family=Outfit:wght@400;500;600;700;800&family=JetBrains+Mono:wght@400;500;700&display=swap">
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/devicons/devicon@v2.16.0/devicon.min.css">
  <link rel="stylesheet" href="/assets/style.css">
  <script src="/assets/hero.js" defer></script>
  <script src="/assets/delivery.js" defer></script>
  <meta property="og:type" content="website">
  <meta property="og:locale" content="{html_lang}">
  <meta property="og:site_name" content="Smart Dawn">
  <meta property="og:title" content="{title}">
  <meta property="og:description" content="{description}">
  <meta property="og:url" content="{canonical}">
  <meta name="twitter:card" content="summary">
  <meta name="twitter:title" content="{title}">
  <meta name="twitter:description" content="{description}">
  <script type="application/ld+json">{ld_json}</script>
</head>
<body>{body_html}</body>
</html>"##
    )
}
