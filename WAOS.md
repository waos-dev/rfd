# rfd for WaOS

This fork adds WaOS to rfd 0.17.2: on a WaOS app (a WASI component, `target_os = "wasi"`), `FileDialog` and `AsyncFileDialog` open the platform's own file dialogs through `wasdk::file_dialogs` instead of treating `wasm32` as a browser. The change is `src/backend/waos.rs`, `src/file_handle/waos.rs`, the target switches in `Cargo.toml`, `src/lib.rs`, `src/backend.rs`, `src/file_dialog.rs` and `src/file_handle/mod.rs`, and the `wasdk` dependency for `target_os = "wasi"`; everything else is upstream's. It is kept for whoever ports an app to WaOS, and is not offered upstream.

## Using it

```toml
[patch.crates-io]
rfd = { git = "https://github.com/waos-dev/rfd", branch = "waos" }
```

The fork asks for the published `wasdk` (0.8.0) with its `raw-window-handle` feature.

## What it does

On WaOS, rfd exposes the same path-based API as native targets: both the synchronous `FileDialog` and the asynchronous `AsyncFileDialog`, and a `FileHandle` backed by a `PathBuf`. `pick_file`, `pick_files`, `pick_folder` and `save_file` call `wasdk::file_dialogs` and hand back paths mounted inside the app that `std::fs` can open directly; `FileHandle::read` and `FileHandle::write` call `std::fs` directly without spawning threads. Filters map to `add_type` (skipping a `*` catch-all filter), `set_directory` maps to `initial_folder`, and `set_file_name` maps to `suggested_name` on save dialogs. `set_parent` resolves the window's `raw-window-handle` to a WaOS view through `wasdk::view_of_window_handle` so the dialog attaches to that window. Synchronous methods wait on the app's thread with `wasdk::block_on`.

## What it leaves out

Dialog titles (`set_title`) and creating folders (`set_can_create_directories`) have no counterpart in `wasdk::file_dialogs` and are ignored. WaOS picks one folder at a time, so `pick_folders` opens the single-folder picker and returns its result as a one-element list. Picked files and folders are always mounted read-only (asking to write a picked file is not exposed through rfd; a saved file is mounted writable by WaOS's rule). WaOS has no system message dialog, so `MessageDialog` and `AsyncMessageDialog` log a warning with the dialog's title and return `MessageDialogResult::Cancel`.
