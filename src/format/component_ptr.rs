use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// A pointer to an arbitrary format component.
///
/// The idea is this can either hold a component in the same file, or a path
/// to a component in a different file. Thus, making files smaller and more human-readable.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormatComponentPointer<T> {
    /// A pointer to a component in a different file.
    File(PathBuf),

    /// An inline declaration of a component.
    Inline(T),
}
