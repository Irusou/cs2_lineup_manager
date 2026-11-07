# CS2 Lineup Manager: Project Spec

A Rust learning project: functional requirements, definitions of done, and delivery plan.

## 1. Core idea

A desktop app to store, organize, and quickly find CS2 utility lineups (smokes, flashes, molotovs, HEs). Each lineup records where you stand, where you aim, what the result looks like, and how to throw it. The goal is to replace scattered screenshots, bookmarks, and YouTube timestamps with one searchable, offline, shareable library.

**Primary user:** you, a CS2 player who wants to look up "A-site smoke on Mirage" in two clicks while practicing.

**Out of scope:** anything that touches the game process or its memory, overlays, and automatic lineup detection. The app is fully external.

## 2. Non-functional constraints

These shape the design, so decide them up front.

- **NFR-1 Offline-first:** no network needed for any feature.
- **NFR-2 Cross-platform:** runs on Windows and Linux at minimum.
- **NFR-3 Data safety:** a crash must never corrupt or lose the library.
- **NFR-4 Responsiveness:** the UI stays smooth with 1,000 lineups and 3,000 images.
- **NFR-5 Maintainability:** domain logic (data, filtering, storage) is separated from UI code and unit-testable without a window.

## 3. Global Definition of Done

Applies to every requirement below, in addition to its specific DoD.

- Compiles with no warnings; `cargo clippy` is clean and `cargo fmt` applied.
- Domain logic has unit tests; no `unwrap()` or `expect()` on user-influenced paths.
- Errors are propagated with `Result` and surfaced to the user, never silently swallowed.
- Manually tested against the happy path and at least two failure cases.
- Committed with a meaningful message, and the README is updated if behavior changed.

## 4. Functional requirements

### Library management

#### FR-01: Create a lineup

The user can create a lineup with: title, map, utility type, throw type, side (T/CT), free-text notes, and tags.

*DoD:*

- A form validates that title, map, and utility type are required; other fields are optional.
- Invalid input shows an inline message and blocks saving.
- A new lineup gets a unique, stable ID that is never reused.
- The lineup appears in the list immediately after saving.
- Domain-level validation has unit tests.

#### FR-02: View lineup list

The user sees all lineups in a scrollable list showing title, map, utility type, and a thumbnail if one exists.

*DoD:*

- The list is sorted by most recently updated by default.
- An empty library shows a helpful empty state instead of a blank panel.
- The list stays responsive with 1,000 entries (NFR-4).

#### FR-03: View lineup details

Selecting a lineup shows all its fields and images in a detail panel.

*DoD:*

- All fields are displayed, with images in a consistent order (position, aim, result).
- Missing images show a placeholder instead of crashing.
- Selection persists while filtering, unless the item is filtered out.

#### FR-04: Edit a lineup

The user can modify any field of an existing lineup.

*DoD:*

- Editing reuses the same validation as creation.
- The user can cancel, and cancelling discards changes without side effects.
- The "last updated" timestamp changes on save.
- The ID never changes.

#### FR-05: Delete a lineup

The user can delete a lineup, including its image files.

*DoD:*

- A confirmation step is required before deletion.
- Associated image files are removed from disk if no other lineup references them.
- Deleting a nonexistent lineup is handled gracefully.
- The deletion is reflected in storage immediately.

### Persistence

#### FR-06: Persistent storage

The library is loaded on startup and saved after every change.

*DoD:*

- Data is stored in an OS-appropriate app data directory, not the working directory.
- Saves are atomic (write to a temp file, then rename) so a crash cannot corrupt data (NFR-3).
- The stored format includes a schema version number.
- A missing file on first launch creates an empty library.
- A corrupted file is not overwritten; it is backed up and the user is told.
- Round-trip test: save, load, and compare equal.

#### FR-07: Automatic backups

The app keeps rolling backups of the library file.

*DoD:*

- The last N (suggest 5) backups are retained, created on startup.
- Older backups are pruned automatically.
- The user can find the backup folder from within the app.

### Images

#### FR-08: Attach images from files

The user can attach image files to a lineup, labeled by role (position, aim, result, other).

*DoD:*

- Supported formats: PNG, JPG, WebP; others are rejected with a clear message.
- The image is copied into the app's data folder; the original is never moved or modified.
- Stored filenames cannot collide.
- The user can remove or reorder images on a lineup.

#### FR-09: Paste images from clipboard

The user can paste a screenshot directly from the clipboard.

*DoD:*

- A shortcut and a button both trigger paste.
- An empty or non-image clipboard shows a clear message, not a crash.
- The pasted image is saved in the same manner and constraints as FR-08.

#### FR-10: Image viewer

Clicking an image opens a large view.

*DoD:*

- Supports zoom and closing with Esc.
- Left/right arrows navigate between a lineup's images.
- Large images load without freezing the UI (thumbnails or lazy loading).

### Discovery

#### FR-11: Filter

The user can filter by map, utility type, side, and tags, combinable.

*DoD:*

- Filters combine with AND logic across categories.
- A visible "clear filters" action exists, and the result count is shown.
- The filtering function is pure and unit-tested with edge cases (no results, all filters set, empty library).

#### FR-12: Text search

The user can search across title, notes, and tags.

*DoD:*

- Case-insensitive, partial matching.
- Works together with filters from FR-11.
- Results update as the user types without noticeable lag.

#### FR-13: Tag management

Tags are reusable across lineups, with autocomplete when entering them.

*DoD:*

- Tags are normalized (trimmed, lowercased) to avoid duplicates like "A-Site" vs "a-site".
- The user can rename or delete a tag globally, and all lineups update.
- Unused tags disappear from suggestions.

### Practice and sharing

#### FR-14: Favorites / practice list

The user can mark lineups as favorites and view them as a separate list.

*DoD:*

- Toggling is one click from both the list and the detail view.
- The favorites view is a filter, not a duplicate of the data.
- Favorite state persists.

#### FR-15: Export a lineup pack

The user can export a selection of lineups (e.g., all of one map) to a single shareable file.

*DoD:*

- The pack contains the lineup data and images in one self-contained file (e.g., a zip).
- Exporting an empty selection is prevented with a message.
- The file includes a format version.

#### FR-16: Import a lineup pack

The user can import a pack received from someone else.

*DoD:*

- Imported lineups receive new IDs to prevent collisions.
- Malformed or unknown-version packs are rejected with a clear message and no partial import.
- Potential duplicates are detected, and the user chooses to skip or import anyway.
- The import never executes or trusts file paths from inside the pack (guard against path traversal).

### Application quality

#### FR-17: Error feedback

All failures are visible to the user in a consistent way.

*DoD:*

- A single mechanism (toast or status bar) shows errors and confirmations.
- Messages are human-readable; technical details go to a log file.
- Logging is set up with levels, and the log location is discoverable.

#### FR-18: Settings

The user can see and change basic preferences.

*DoD:*

- Shows the data folder location, with an "open folder" action.
- Preferences (e.g., default map filter, theme) persist across launches.

## 5. Delivery plan

Each phase ends in a working, usable app.

| Phase | Goal | Requirements | Key Rust learning |
| --- | --- | --- | --- |
| 1 | Window and in-memory list | FR-02, FR-03, FR-01 (basic) | Project layout, structs/enums, GUI loop |
| 2 | Real CRUD with validation | FR-01, FR-04, FR-05 | Ownership and borrowing in UI state, `Result` |
| 3 | Persistence | FR-06, FR-07, FR-17 | serde, file I/O, error types, atomic writes |
| 4 | Images | FR-08, FR-09, FR-10 | Resource handling, paths, lazy loading |
| 5 | Discovery | FR-11, FR-12, FR-13 | Iterators, closures, pure functions, testing |
| 6 | Practice and sharing | FR-14, FR-15, FR-16 | Zip handling, versioned formats, input trust |
| 7 | Polish | FR-18, README, release build | Packaging, docs |

## 6. Suggested module layout (conceptual)

Keep these as separate concerns, matching NFR-5:

- **Domain:** lineup types, validation, filtering logic. No I/O, no UI.
- **Storage:** loading, saving, backups, image files, import/export.
- **App state:** what is selected, current filters, pending form edits.
- **UI:** only draws state and emits user actions.

If domain and storage never import UI code, you can test most of the app without ever opening a window.

## 7. Stretch goals (after the spec is done)

- Migrate storage from JSON to SQLite.
- Show lineups as markers on a map radar image.
- Spaced-repetition mode for practice.
- Video or GIF attachments.

Treat each requirement as a ticket: implement it, check every DoD item, and only then move on. If a requirement turns out to be ambiguous while you build it, write down your decision. That habit matters as much as the code.
