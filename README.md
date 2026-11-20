# Travel Kit — desktop app

Five travel rooms from Steven Powell's **Everyday Cool** collection in one installable
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
- **The Survival Guide** — putting it all together outdoors: fire, water, soap,
  direction, a remedies shelf, and first aid, every card with a safety line and an
  honesty line. Works offline; only its links out need the internet.

The app's home page links to all five rooms, and a built-in **Help** page documents
what each tool does, how to use it, and where its data comes from.

## What's new in v1.3.1

A documentation release — the rooms and the splash are unchanged; the manual
caught up with the app:

- **Help → GitHub Repository** joins the Help menu, opening this app's source
  repository in your browser, beside Documentation and the Product Index.
- **The Documentation and Help pages are rebuilt** to a professional manual
  standard: a contents row, numbered sections, and two new sections —
  **Troubleshooting** (a room won't load, a room needs the internet, audio
  won't play) and **Source & support** (this repository, the Everyday Cool
  collection repository, where to report an issue, and Steven Powell's
  LinkedIn — "Steve's Profile").
- **Every room's Help now shows its source:** Find It, The Radio Room, and
  The Survival Guide gained a Source on GitHub link in their Help menus, and
  Tonight's Sky and Weather Everywhere gained Documentation & source panels.

## What's new in v1.3.0

- **The Survival Guide joins as the fifth room** — Steven's capstone room from the
  collection: "Putting it all together: A Survival Guide." Fire from little, a
  layered water filter (with its honesty line in the room's biggest type:
  filtering does not make water safe — boiling does), soap from rendered fat and
  wood-ash lye, pantry batteries, direction graded A/B/D with moss as the weakest
  clue, a remedies shelf (slug slime, willow bark, plantain, oak bark), and First
  Aid as the anchor section, checked against Red Cross / Scouting America /
  Smokey Bear / CDC sources. Every card carries a safety line and an honesty
  line. It works fully offline; only its Links Out shelf needs the internet.
- Inside the kit, the room's links to Tonight's Sky stay in-app; its links to
  Everyday Chemistry, Everyday Electronics, and Everyday Body point at those
  rooms' live pages in the full collection online, following the same convention
  the Radio Room's out-of-kit links use.
- The splash screen, menu bar, and the four existing rooms are unchanged.

## What's new in v1.2.0

The same professional treatment the Everyday Cool app got:

- **Splash screen** — new robot-traveler artwork (map, telescope, suitcase, a
  smiling sun behind a cloud) with "Travel Kit" set in real type over it and the
  version in the corner. It stays up for a **10-second minimum** and dismisses at
  the later of the main window loading or those 10 seconds, with an unconditional
  15-second hard cap so it can never trap you. (The Everyday Cool splash closed
  the moment the app loaded and flashed past too fast to see; this one is built
  to be looked at.)
- **A native menu bar** — **File → Exit** (on macOS, the Travel Kit menu → Quit),
  and **Help → Documentation** (an app-level docs page: the rooms, getting
  around, online vs. offline), **Help → Product Index** (Steven Powell's product
  index on GitHub Pages, in your browser), and **Help → About Travel Kit** (name
  and version).
- **The Radio Room** was already aboard — it joined in v1.1.0 as the fourth room —
  so this release leaves the four rooms exactly as they were and changes only the
  shell around them.

- The collection: https://caeled.github.io/everyday-cool/
- Tonight's Sky: https://caeled.github.io/everyday-cool/projects/tonights-sky/
- Find It: https://caeled.github.io/everyday-cool/projects/find-it/
- Weather Everywhere: https://caeled.github.io/everyday-cool/projects/weather-everywhere/
- The Radio Room: https://caeled.github.io/everyday-cool/projects/radio-room/
- The Survival Guide: https://caeled.github.io/everyday-cool/projects/survival-guide/

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
