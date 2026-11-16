use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use scrapekit::{extract, fetch};
use url::Url;

#[derive(Copy, Clone, ValueEnum)]
enum Format {
    Text,
    Json,
}

/// Kleiner Web-Scraper: lädt eine Seite und extrahiert Elemente per CSS-Selektor.
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Die zu scrapende URL
    url: String,

    /// CSS-Selektor, z. B. "h2.title" oder "a"
    #[arg(short, long, default_value = "a")]
    selector: String,

    /// Attribut statt Text ausgeben, z. B. "href"
    #[arg(short, long)]
    attr: Option<String>,

    /// Ausgabeformat
    #[arg(short, long, value_enum, default_value_t = Format::Text)]
    format: Format,

    /// Timeout in Sekunden
    #[arg(short, long, default_value_t = 10)]
    timeout: u64,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let base = Url::parse(&cli.url).context("Ungültige URL")?;
    let html = fetch(base.as_str(), cli.timeout)
        .with_context(|| format!("Konnte {} nicht laden", cli.url))?;
    let results = extract(&html, &cli.selector, cli.attr.as_deref(), Some(&base))?;

    match cli.format {
        Format::Text => results.iter().for_each(|r| println!("{r}")),
        Format::Json => println!("{}", serde_json::to_string_pretty(&results)?),
    }
    eprintln!("{} Treffer", results.len());
    Ok(())
}
