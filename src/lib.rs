//! Kernlogik von scrapekit: HTML laden und per CSS-Selektor Daten extrahieren.

use anyhow::{anyhow, Result};
use scraper::{Html, Selector};
use std::time::Duration;
use url::Url;

/// Lädt eine Seite per HTTP GET und gibt den Body als String zurück.
pub fn fetch(url: &str, timeout_secs: u64) -> Result<String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent(concat!("scrapekit/", env!("CARGO_PKG_VERSION")))
        .timeout(Duration::from_secs(timeout_secs))
        .build()?;
    let body = client.get(url).send()?.error_for_status()?.text()?;
    Ok(body)
}

/// Extrahiert Werte aus HTML.
///
/// - `attr = None`: Textinhalt der Elemente (Whitespace normalisiert)
/// - `attr = Some("href")`: Wert des Attributs; `href`/`src` werden mit `base` zu absoluten URLs aufgelöst
pub fn extract(
    html: &str,
    selector: &str,
    attr: Option<&str>,
    base: Option<&Url>,
) -> Result<Vec<String>> {
    let document = Html::parse_document(html);
    let sel = Selector::parse(selector)
        .map_err(|e| anyhow!("Ungültiger CSS-Selektor '{selector}': {e}"))?;

    let mut results = Vec::new();
    for element in document.select(&sel) {
        let value = match attr {
            Some(name) => match element.value().attr(name) {
                Some(v) => v.trim().to_string(),
                None => continue,
            },
            None => element
                .text()
                .collect::<Vec<_>>()
                .join(" ")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" "),
        };
        if value.is_empty() {
            continue;
        }
        let value = match (attr, base) {
            (Some("href" | "src"), Some(b)) => {
                b.join(&value).map(|u| u.to_string()).unwrap_or(value)
            }
            _ => value,
        };
        results.push(value);
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    const HTML: &str = r#"
        <html><body>
          <h1>  Hallo   Welt </h1>
          <a href="/docs">Docs</a>
          <a href="https://example.org/x">Extern</a>
          <a>Ohne href</a>
        </body></html>"#;

    #[test]
    fn extrahiert_text_und_normalisiert_whitespace() {
        let r = extract(HTML, "h1", None, None).unwrap();
        assert_eq!(r, vec!["Hallo Welt"]);
    }

    #[test]
    fn loest_relative_links_auf() {
        let base = Url::parse("https://example.com/start").unwrap();
        let r = extract(HTML, "a", Some("href"), Some(&base)).unwrap();
        assert_eq!(r, vec!["https://example.com/docs", "https://example.org/x"]);
    }

    #[test]
    fn ungueltiger_selektor_ist_fehler() {
        assert!(extract(HTML, "<<<", None, None).is_err());
    }
}
