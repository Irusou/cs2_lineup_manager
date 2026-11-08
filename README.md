# CS2 Lineup Manager

A desktop app, written in Rust, to store, organize, and quickly find Counter-Strike 2 utility lineups (smokes, flashes, molotovs, HEs).

Each lineup records where you stand, where you aim, what the result looks like, and how to throw it, so you can stop juggling scattered screenshots, bookmarks, and video timestamps and keep everything in one searchable, offline library.

> **Project status: early development.** This is a learning project. I'm learning Rust by building something I'd actually use as a CS2 player, so expect rough edges and frequent changes.

## Current state

- [x] Domain types: maps, utility kinds, sides, throw types
- [x] `Lineup` model with unique IDs (`Uuid`)
- [x] Desktop window with a header, a lineup list sidebar, and a main panel
- [x] Selecting a lineup from the list and showing it in the main panel
- [ ] Persistence (the app currently starts with hardcoded sample data)
- [ ] Create, edit, and delete lineups
- [ ] Validation
- [ ] Screenshots attached to lineups
- [ ] Filtering and search
- [ ] Favorites and import/export of lineup packs

## Planned features

- **Library management:** create, view, edit, and delete lineups with validation
- **Reliable storage:** versioned JSON saved in the OS app data folder, with atomic writes and rolling backups
- **Images:** attach screenshots from files or paste them from the clipboard, with a zoomable viewer
- **Discovery:** filter by map, utility type, side, and tags, plus text search
- **Practice and sharing:** favorites list, and export/import of lineup packs to share with friends

The full requirements, with a definition of done for each, are tracked in the project spec (see `docs/spec.md`).

## What this app does not do

It is fully external. It never reads or writes the game's memory, injects overlays, or interacts with the CS2 process in any way, so it carries no VAC risk.

## Tech stack

- [Rust](https://www.rust-lang.org/)
- [egui](https://github.com/emilk/egui) / [eframe](https://crates.io/crates/eframe) for the GUI
- [serde](https://serde.rs/) for (de)serialization
- [uuid](https://crates.io/crates/uuid) for lineup IDs

## Project structure

The logic lives in a library crate, and the front end is a thin binary on top of it. The domain code knows nothing about files or the UI, so it can be unit-tested without opening a window.

```
src/
├── lib.rs        # library root, declares the modules
├── main.rs       # GUI entry point
├── domain/       # lineup types, validation, filtering (no I/O, no UI)
└── storage/      # loading, saving, backups (planned)
```

## Getting started

### Requirements

- A recent stable [Rust toolchain](https://rustup.rs/)
- Windows or Linux (macOS is untested)

### Run it

```bash
git clone https://github.com/Irusou/cs2-lineup-manager.git
cd cs2-lineup-manager
cargo run
```

### Check the code

```bash
cargo test
cargo clippy
cargo fmt --check
```

## Roadmap

1. Real create/edit/delete with validation
2. JSON persistence with atomic writes and backups
3. Images: attach, paste, and view
4. Filtering, search, and tag management
5. Favorites and lineup pack import/export
6. Polish, settings, and release builds

Possible stretch goals: SQLite storage, lineups as markers on a map radar image, and a spaced-repetition practice mode.

## Contributing

This is a personal learning project, but suggestions and feedback are welcome. Feel free to open an issue.

## License

To be decided. Until a license file is added, all rights are reserved by the author.
