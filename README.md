# scrapekit

Ein schneller Web-Scraper in Rust mit CLI und lokaler Web-UI: URL und CSS-Selektor rein, Text, Links oder JSON raus.

![CI](https://github.com/DEIN-USERNAME/scrapekit/actions/workflows/ci.yml/badge.svg)

## Features

- CSS-Selektoren zur Extraktion (`h2.title`, `a[href^="http"]`, …)
- Text oder beliebige Attribute (`--attr href`)
- Relative Links werden zu absoluten URLs aufgelöst
- Ausgabe als Klartext oder JSON
- Lokale Web-UI (`axum`) mit Presets und JSON-Kopieren
- Einheitliche Fehlermeldungen mit `anyhow`, Tests und CI

## Installation

```bash
git clone https://github.com/khalil135711/scrapekit
cd scrapekit
cargo install --path .
```

## Benutzung

```bash
# Alle Überschriften einer Seite
scrapekit https://www.rust-lang.org -s "h1, h2"

# Alle Links als absolute URLs
scrapekit https://www.rust-lang.org -s a -a href

# Als JSON
scrapekit https://www.rust-lang.org -s a -a href -f json
```

| Option | Bedeutung | Standard |
|---|---|---|
| `-s, --selector` | CSS-Selektor | `a` |
| `-a, --attr` | Attribut statt Text | – |
| `-f, --format` | `text` oder `json` | `text` |
| `-t, --timeout` | Timeout in Sekunden | `10` |

## Web-UI

```bash
cargo run --bin scrapekit-ui
# dann http://127.0.0.1:3000 im Browser öffnen
```

Die UI bindet bewusst nur an `127.0.0.1`, weil sie beliebige URLs lädt. Nicht ohne Absicherung ins Netz stellen.

## Entwicklung

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt
```

## Roadmap

- [ ] Mehrere URLs parallel (`rayon` oder `tokio`)
- [ ] Crawling mit Tiefenlimit und `robots.txt`-Respekt
- [ ] Export als CSV
- [ ] Verlauf und gespeicherte Selektoren in der UI

## Hinweis

Bitte respektiere die Nutzungsbedingungen und `robots.txt` der Seiten, die du scrapst.

## Lizenz

MIT
