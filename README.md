# Debate Dioxus

A desktop application for managing and reviewing **British Parliamentary (BP)** debate rounds. Built with [Dioxus](https://dioxuslabs.com/) (Rust UI framework) and SQLite.

---

## Features

- **Create & manage debates** — store motions and all 8 speaker roles with their speeches
- **Speech entry** — record arguments, rebuttals, and POIs per speaker, with a built-in countdown timer
- **Timer** — configurable speech timer with audio bell cues (at 1 min elapsed, 1 min remaining, and time-up)
- **Three review views:**
  - **Card View** — per-group speaker cards in a fixed 2×2 grid (OG/OO top, CG/CO bottom)
  - **Table View** — side-by-side comparison of all 8 speakers
  - **Placement View** — rank the four groups (1st–4th), assign points, and add notes
- **Rich text formatting** — `*text*` highlights in red, `$text$` highlights in blue
- **Keyboard shortcuts** in speech entry:
  - `Ctrl+B` — wrap selected text in `*` (red highlight)
  - `Ctrl+D` — wrap selected text in `$` (blue highlight)
  - `Ctrl+↑ / Ctrl+↓` — navigate between text fields
  - `Tab` — indent list item (numbers → letters); inserts `\t` on non-list lines
  - `Shift+Tab` — dedent list item (letters → numbers)
  - `Enter` — continue ordered list; `Shift+Enter` for plain newline
- **Auto ordered lists** — type `1. text` and press Enter to auto-continue; supports alternating depth levels (numbers → letters → numbers…)
- **Internationalization** — English and Hebrew (with full RTL layout support)
- **Theming** — light / dark themes, configurable font size
- **Persistent settings** — language, theme, font size, timer duration, sound toggle, rebuttal/POI visibility

---

## BP Speaker Roles

| Role | Abbr | Team |
|---|---|---|
| Prime Minister | PM | Opening Government (OG) |
| Deputy Prime Minister | DPM | Opening Government (OG) |
| Leader of Opposition | LO | Opening Opposition (OO) |
| Deputy Leader of Opposition | DLO | Opening Opposition (OO) |
| Member of Government | MG | Closing Government (CG) |
| Government Whip | GW | Closing Government (CG) |
| Member of Opposition | MO | Closing Opposition (CO) |
| Opposition Whip | OW | Closing Opposition (CO) |

---

## Project Structure

```
src/
├── main.rs                   # App entry point, routing, theme/dir setup
├── types.rs                  # Debate & Speech structs, SPEAKER_ORDER
├── db.rs                     # SQLite CRUD (debates, speeches, speakers)
├── settings.rs               # Settings struct, load/save to settings.toml
├── i18n.rs                   # t() translation function, Lang context
└── components/
    ├── icons.rs              # SVG icon components (flat, stroke-based)
    ├── home.rs               # Debate list + Card / Table / Placement views
    ├── create_new.rs         # New debate form
    ├── order_of_speakers.rs  # Speaker order screen
    ├── speech.rs             # Speech entry screen + keyboard shortcuts
    ├── timer.rs              # Countdown timer with bell
    ├── navbar.rs             # Navigation bar
    └── settings_modal.rs     # Settings UI modal
```

---

## Routes

| Path | Component | Description |
|---|---|---|
| `/` | `Home` | Debate list |
| `/home/:id` | `HomeWithId` | Debate detail (pre-selected) |
| `/create` | `CreateNew` | Create a new debate |
| `/speakers/:id` | `OrderOfSpeakers` | Set speaker names and order |
| `/speech/:speaker/:id` | `Speech` | Enter speech for a specific role |

---

## Settings

Settings are saved to `settings.toml` in `%LOCALAPPDATA%\debate_dioxus\` (same directory as the database).

| Setting | Default | Description |
|---|---|---|
| `language` | `en` | UI language (`en` / `he`) |
| `theme` | `light` | Color theme (`light` / `dark`) |
| `font_size` | `medium` | Font size (`small` / `medium` / `large` / `extra-large`) |
| `speech_timer_default` | `7` | Timer duration in minutes |
| `enable_sound` | `true` | Bell sound on timer events |
| `include_rebuttal` | `true` | Show rebuttal section in views |
| `include_poi` | `true` | Show POI section in views |
| `always_on_top` | `false` | Keep window above other windows |

---

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable, 2021 edition)
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started/) — `cargo install dioxus-cli`

---

## Build & Run

```bash
# Development
dx serve --platform desktop

# Release build
dx build --platform desktop --release

# Installer (Windows — produces .msi and .exe setup in dist/)
dx bundle --platform desktop --release
```

---

## Dependencies

| Crate | Purpose |
|---|---|
| `dioxus` | UI framework (desktop + router) |
| `rusqlite` | SQLite database (bundled) |
| `serde` / `toml` | Settings serialization |
| `serde_json` | JS↔Rust eval communication |
| `once_cell` | Lazy-initialized translation maps |
| `dirs` | Platform-specific directory paths |
| `async-std` | Async timer coroutine |
| `ico` | App icon loading |

---

## Text Formatting

Inside speech fields, two inline markup tokens are supported:

- `*text*` — renders bold in **red** (highlight key arguments)
- `$text$` — renders bold in **blue** (highlight counter-arguments)

---

## Ordered Lists

Type a list item manually and the editor continues it automatically:

```
1. First point        ← type this, press Enter
2.                    ← auto-inserted
```

Press **Tab** on a numbered line to indent to a sub-list (letters), **Shift+Tab** to dedent back. Levels alternate: numbers → letters → numbers…

```
1. Top level
    a. Sub item       ← Tab
        1. Sub-sub    ← Tab again
    b.                ← Enter continues letters
2.                    ← Shift+Tab back to numbers
```

Press **Enter** on a blank list line to exit the list (or dedent one level if nested).
