# Travel Kit — desktop app

Four travel rooms from Steven Powell's **Everyday Cool** collection in one installable
app for Windows and macOS:

- **Tonight's Sky** — a drag-and-zoom night sky, planets, the Moon's phase, satellite
  pass predictions, and radio astronomy explainers. Works offline.
- **Find It** — set a place, tap one big category button, and get the nearest matches
  on a live OpenStreetMap map, each with directions. Needs the internet.
- **Weather Everywhere** — a portable world-weather workshop: maps, comparisons, ocean
  and cloud labs, field missions, and live forecasts. Core workshop works offline.
- **The Radio Room** — a retro wooden radio that really plays (six SomaFM stations on
  a make-believe FM dial), a shelf of everywhere radio lives now, and a scanner tuner
  that finds your county's feeds on Broadcastify. Needs the internet.

The app's home page links to all four rooms, and a built-in **Help** page documents
what each tool does, how to use it, and where its data comes from.

- The collection: https://caeled.github.io/everyday-cool/
- Tonight's Sky: https://caeled.github.io/everyday-cool/projects/tonights-sky/
- Find It: https://caeled.github.io/everyday-cool/projects/find-it/
- Weather Everywhere: https://caeled.github.io/everyday-cool/projects/weather-everywhere/
- The Radio Room: https://caeled.github.io/everyday-cool/projects/radio-room/

## Downloads

Grab the installer for your system from the
[latest release](https://github.com/caeled/travel-kit-app/releases/latest):

- **Windows:** `Travel.Kit_<version>_x64-setup.exe`
- **macOS (Intel + Apple Silicon):** `Travel.Kit_<version>_universal.dmg`

The installers are unsigned hobby builds: Windows SmartScreen and macOS Gatekeeper
will warn on first run; choose "More info → Run anyway" (Windows) or right-click →
Open (macOS).

## Build

Tauri v2 shell around the rooms' own web files in `dist/`. Tagging `v*` builds and
publishes the release via GitHub Actions.

Idea by Steven Powell. Built with the Council: Grok Bot built the rooms, Muse made
the banner and packaged the app.
