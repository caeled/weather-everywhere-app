# Weather Everywhere — Desktop App

A small desktop shell for [Weather Everywhere](https://github.com/caeled/weather-everywhere),
Steven Powell's portable world-weather workshop — the version with the six
November 14, 2026 additions inside: the Sun & tilt room, Look back, Opposite
weather, the Sun slider, the plain-words weather glossary, and Beaufort wind
labels. The shell is built with [Tauri v2](https://v2.tauri.app/) and hosts the
workshop in the operating system's own webview — WebView2 on Windows,
WKWebView on macOS — so there is no bundled browser and the download stays
small.

## The one rule

**The workshop stays in the app. Only outside sources leave.**

- The workshop is one page whose rooms — map room, compare places, Ripple lab,
  Cloud maker, missions, notebook — are sections of that single page. Moving
  between them never leaves the window, and everything works there exactly as
  it does on the web, including the live Open-Meteo lookups (Look live, Look
  back, Opposite weather), which are data requests, not page changes.
- The links that *do* leave are the "Go deeper" sources and the project's own
  outside links (NOAA, NASA Worldview, the British Antarctic Survey,
  Open-Meteo, the source repository). Any navigation or new-window request to
  an external http/https address is intercepted by the shell, cancelled inside
  the app, and handed to the operating system's **default browser** instead.
  No second in-app window ever opens, and the workshop stays exactly where it
  was.
- Downloads are allowed through with the platform's default behavior, because
  the notebook's export saves its JSON as a download. That is the workshop's
  own business, so the shell does not block it.

The interception lives in `src-tauri/src/main.rs`: `on_navigation` allows only
the app's own URLs (`tauri://`, `http://tauri.localhost`, `about:`, `data:`,
`blob:`) and routes any other http/https navigation to the default browser via
the opener plugin; `on_new_window` denies every new window and forwards
http/https URLs the same way; `on_download` returns `true` so downloads
proceed with default behavior.

## Storage note

The app webview keeps its **own** local storage, separate from your web
browser. The Weather Everywhere notebook and settings (units, Quiet skies)
inside the app live in the app; the ones in your browser live in your browser.
**Notebook entries do not carry between the two** — use the notebook's JSON
export in one and its import in the other if you want to move your notes.

## Updating the embedded workshop

The app embeds the workshop's front end: `dist/index.html` plus the whole
`dist/assets/` folder, copied unedited from the weather-everywhere build.

1. Copy the newest `index.html` over `dist/index.html` and the newest
   `assets/` folder over `dist/assets/`.
2. Rebuild (locally with `npm run tauri build`, or push a `v*` tag / run the
   GitHub Actions workflow manually).

Nothing else changes; the shell does not care what the workshop contains.

## Building locally

Prerequisites: Node 20+, a stable Rust toolchain, and the platform webview
pieces (WebView2 is built into current Windows; macOS needs nothing extra).
Linux prerequisites are listed in the Tauri docs if you want to try it there.

```sh
npm install
npm run tauri build
```

Bundles land in `src-tauri/target/release/bundle/` (Windows: NSIS `.exe`
installer and `.msi`; macOS: `.dmg` and `.app`).

## Releases from GitHub Actions

`.github/workflows/build.yml` builds both platforms:

- Trigger it manually from the Actions tab ("Build desktop bundles" → Run
  workflow), or
- push a tag like `v1.0.0`.

Each run uploads the produced bundles as artifacts named
`weather-everywhere-windows-latest` and `weather-everywhere-macos-latest`
(the macOS build is a universal binary for Apple silicon and Intel).

### The macOS build is unsigned

There is no Apple code-signing certificate in this project, so macOS will
warn on first launch. To open it the first time: **right-click the app →
Open → Open**. After that it launches normally. This is expected for an
unsigned build, not a defect.

## License

MIT — see [LICENSE](LICENSE). The embedded workshop's own educational
content carries its own notices in the source project,
[caeled/weather-everywhere](https://github.com/caeled/weather-everywhere).
