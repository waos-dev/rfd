//! WaOS backend for file dialogs, backed by `wasdk::file_dialogs`.
//!
//! Each picked file or folder is mounted into the program at its own path and is opened
//! read-only; a save target is mounted writable. The parent window is resolved to a WaOS
//! view from its `raw-window-handle` when `set_parent` is called. WaOS has no system
//! message dialog, so message dialogs log a warning and return as dismissed.

use std::path::{Path, PathBuf};

use wasdk::file_dialogs::{
    self, DialogError, FileDialogs, FolderOptions, OpenOptions, SaveOptions,
};

use crate::backend::{
    AsyncFilePickerDialogImpl, AsyncFileSaveDialogImpl, AsyncFolderPickerDialogImpl,
    AsyncMessageDialogImpl, DialogFutureType, FilePickerDialogImpl, FileSaveDialogImpl,
    FolderPickerDialogImpl, MessageDialogImpl,
};
use crate::file_dialog::Filter;
use crate::message_dialog::{MessageDialog, MessageDialogResult};
use crate::{FileDialog, FileHandle};

fn is_any_file_filter(filter: &Filter) -> bool {
    filter.extensions.is_empty() || filter.extensions.iter().all(|ext| ext == "*")
}

fn build_open_options(dialog: &FileDialog) -> OpenOptions {
    let options = OpenOptions::new();
    for filter in &dialog.filters {
        if !is_any_file_filter(filter) {
            options.add_type(&filter.name, &filter.extensions, &[]);
        }
    }
    if let Some(dir) = dialog.starting_directory.as_deref().and_then(Path::to_str) {
        options.initial_folder(dir);
    }
    options
}

fn build_folder_options(dialog: &FileDialog) -> FolderOptions {
    let options = FolderOptions::new();
    if let Some(dir) = dialog.starting_directory.as_deref().and_then(Path::to_str) {
        options.initial_folder(dir);
    }
    options
}

fn build_save_options(dialog: &FileDialog) -> SaveOptions {
    let options = SaveOptions::new();
    for filter in &dialog.filters {
        if !is_any_file_filter(filter) {
            options.add_type(&filter.name, &filter.extensions, &[]);
        }
    }
    if let Some(dir) = dialog.starting_directory.as_deref().and_then(Path::to_str) {
        options.initial_folder(dir);
    }
    if let Some(name) = &dialog.file_name {
        options.suggested_name(name);
    }
    options
}

fn file_dialogs() -> Option<FileDialogs> {
    let dialogs = file_dialogs::get_file_dialogs();
    if dialogs.is_none() {
        log::warn!("WaOS file dialogs are not available");
    }
    dialogs
}

fn unwrap_single(kind: &str, result: Result<Option<String>, DialogError>) -> Option<FileHandle> {
    match result {
        Ok(Some(path)) => Some(FileHandle::wrap(PathBuf::from(path))),
        Ok(None) => None,
        Err(DialogError::Failed(reason)) => {
            log::warn!("WaOS {kind} dialog failed: {reason}");
            None
        }
    }
}

fn unwrap_list(kind: &str, result: Result<Vec<String>, DialogError>) -> Option<Vec<FileHandle>> {
    match result {
        Ok(paths) if paths.is_empty() => None,
        Ok(paths) => Some(
            paths
                .into_iter()
                .map(|path| FileHandle::wrap(PathBuf::from(path)))
                .collect(),
        ),
        Err(DialogError::Failed(reason)) => {
            log::warn!("WaOS {kind} dialog failed: {reason}");
            None
        }
    }
}

//
// File Picker
//

impl AsyncFilePickerDialogImpl for FileDialog {
    fn pick_file_async(self) -> DialogFutureType<Option<FileHandle>> {
        Box::pin(async move {
            let dialogs = file_dialogs()?;
            let options = build_open_options(&self);
            let result = dialogs
                .open_file(self.parent_view.as_deref(), options)
                .await;
            unwrap_single("open file", result)
        })
    }

    fn pick_files_async(self) -> DialogFutureType<Option<Vec<FileHandle>>> {
        Box::pin(async move {
            let dialogs = file_dialogs()?;
            let options = build_open_options(&self);
            let result = dialogs
                .open_files(self.parent_view.as_deref(), options)
                .await;
            unwrap_list("open files", result)
        })
    }
}

impl FilePickerDialogImpl for FileDialog {
    fn pick_file(self) -> Option<PathBuf> {
        wasdk::block_on(self.pick_file_async()).map(PathBuf::from)
    }

    fn pick_files(self) -> Option<Vec<PathBuf>> {
        wasdk::block_on(self.pick_files_async())
            .map(|handles| handles.into_iter().map(PathBuf::from).collect())
    }
}

//
// Folder Picker
//

impl AsyncFolderPickerDialogImpl for FileDialog {
    fn pick_folder_async(self) -> DialogFutureType<Option<FileHandle>> {
        Box::pin(async move {
            let dialogs = file_dialogs()?;
            let options = build_folder_options(&self);
            let result = dialogs
                .open_folder(self.parent_view.as_deref(), options)
                .await;
            unwrap_single("open folder", result)
        })
    }

    fn pick_folders_async(self) -> DialogFutureType<Option<Vec<FileHandle>>> {
        Box::pin(async move { self.pick_folder_async().await.map(|handle| vec![handle]) })
    }
}

impl FolderPickerDialogImpl for FileDialog {
    fn pick_folder(self) -> Option<PathBuf> {
        wasdk::block_on(self.pick_folder_async()).map(PathBuf::from)
    }

    fn pick_folders(self) -> Option<Vec<PathBuf>> {
        wasdk::block_on(self.pick_folders_async())
            .map(|handles| handles.into_iter().map(PathBuf::from).collect())
    }
}

//
// File Save
//

impl AsyncFileSaveDialogImpl for FileDialog {
    fn save_file_async(self) -> DialogFutureType<Option<FileHandle>> {
        Box::pin(async move {
            let dialogs = file_dialogs()?;
            let options = build_save_options(&self);
            let result = dialogs
                .save_file(self.parent_view.as_deref(), options)
                .await;
            unwrap_single("save file", result)
        })
    }
}

impl FileSaveDialogImpl for FileDialog {
    fn save_file(self) -> Option<PathBuf> {
        wasdk::block_on(self.save_file_async()).map(PathBuf::from)
    }
}

//
// Message Dialog
//

impl AsyncMessageDialogImpl for MessageDialog {
    fn show_async(self) -> DialogFutureType<MessageDialogResult> {
        Box::pin(async move {
            log::warn!(
                "WaOS has no system message dialog; dismissing {:?}",
                self.title
            );
            MessageDialogResult::Cancel
        })
    }
}

impl MessageDialogImpl for MessageDialog {
    fn show(self) -> MessageDialogResult {
        wasdk::block_on(self.show_async())
    }
}
