//! FileHandle is a way of abstracting over a file returned by a dialog
//!
//! On native targets it just wraps a path of a file.
//! In web browsers it wraps `File` js object
//!
//! It should allow a user to treat web browser files same way as native files

#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub use native::FileHandle;

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
mod web;
#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
pub use web::FileHandle;
#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
pub(crate) use web::WasmFileHandleKind;

#[cfg(target_os = "wasi")]
mod waos;
#[cfg(target_os = "wasi")]
pub use waos::FileHandle;

#[cfg(test)]
mod tests {
    use super::FileHandle;

    #[test]
    fn fn_def_check() {
        let _ = FileHandle::wrap;
        let _ = FileHandle::read;
        #[cfg(feature = "file-handle-inner")]
        let _ = FileHandle::inner;
        #[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))]
        let _ = FileHandle::path;
    }
}
