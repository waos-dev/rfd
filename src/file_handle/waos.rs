use std::path::{Path, PathBuf};

/// FileHandle is a way of abstracting over a file returned by a dialog
#[derive(Clone)]
pub struct FileHandle(PathBuf);

impl FileHandle {
    pub(crate) fn wrap(path_buf: PathBuf) -> Self {
        Self(path_buf)
    }

    /// Get name of a file
    pub fn file_name(&self) -> String {
        self.0
            .file_name()
            .and_then(|f| f.to_str())
            .map(|f| f.to_string())
            .unwrap_or_default()
    }

    /// Gets path to a file.
    pub fn path(&self) -> &Path {
        &self.0
    }

    /// Reads a file directly from the mounted path.
    pub async fn read(&self) -> Vec<u8> {
        std::fs::read(&self.0).unwrap()
    }

    /// Writes a file directly to the mounted path.
    pub async fn write(&self, data: &[u8]) -> std::io::Result<()> {
        std::fs::write(&self.0, data)
    }

    /// Unwraps a `FileHandle` and returns inner type.
    ///
    /// #### Behind a `file-handle-inner` feature flag
    #[cfg(feature = "file-handle-inner")]
    pub fn inner(&self) -> &Path {
        &self.0
    }
}

impl std::fmt::Debug for FileHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.path())
    }
}

impl From<PathBuf> for FileHandle {
    fn from(path: PathBuf) -> Self {
        Self::wrap(path)
    }
}

impl From<FileHandle> for PathBuf {
    fn from(file_handle: FileHandle) -> Self {
        file_handle.0
    }
}

impl From<&FileHandle> for PathBuf {
    fn from(file_handle: &FileHandle) -> Self {
        PathBuf::from(file_handle.path())
    }
}
