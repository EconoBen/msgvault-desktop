//! Icon constants using Unicode symbols
//!
//! Consistent text-based icons until an icon font is bundled.

// === Navigation ===
pub const DASHBOARD: &str = "◉";
pub const SEARCH: &str = "⌕";
pub const SYNC: &str = "↻";
pub const SETTINGS: &str = "⚙";
pub const HELP: &str = "?";
pub const ACCOUNTS: &str = "●";

// === Actions ===
pub const COMPOSE: &str = "+";
pub const REPLY: &str = "↩";
pub const REPLY_ALL: &str = "↩↩";
pub const FORWARD: &str = "↪";
pub const DELETE: &str = "✕";
pub const DOWNLOAD: &str = "↓";
pub const OPEN: &str = "↗";

// === State ===
pub const CHECK: &str = "✓";
pub const CROSS: &str = "✗";
pub const DOTS: &str = "···";
pub const ATTACH: &str = "⊕";

// === Navigation Arrows ===
pub const ARROW_LEFT: &str = "←";
pub const ARROW_RIGHT: &str = "→";
pub const ARROW_UP: &str = "↑";
pub const ARROW_DOWN: &str = "↓";

// === Expand / Collapse ===
pub const EXPAND: &str = "▸";
pub const COLLAPSE: &str = "▾";
pub const EXPAND_ALL: &str = "▸▸";

// === Indicators ===
pub const DOT_FILLED: &str = "●";
pub const DOT_EMPTY: &str = "○";
pub const DIAMOND: &str = "◆";
pub const DIAMOND_SM: &str = "◇";

// === File Types ===
pub const FILE_PDF: &str = "PDF";
pub const FILE_DOC: &str = "DOC";
pub const FILE_XLS: &str = "XLS";
pub const FILE_IMG: &str = "IMG";
pub const FILE_ZIP: &str = "ZIP";
pub const FILE_AUDIO: &str = "AUD";
pub const FILE_VIDEO: &str = "VID";
pub const FILE_GENERIC: &str = "FILE";

/// Get file type icon from filename extension
pub fn file_icon(filename: &str) -> &'static str {
    let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "pdf" => FILE_PDF,
        "doc" | "docx" | "odt" | "rtf" => FILE_DOC,
        "xls" | "xlsx" | "csv" | "ods" => FILE_XLS,
        "png" | "jpg" | "jpeg" | "gif" | "webp" | "svg" | "bmp" => FILE_IMG,
        "zip" | "tar" | "gz" | "rar" | "7z" => FILE_ZIP,
        "mp3" | "wav" | "m4a" | "flac" | "ogg" => FILE_AUDIO,
        "mp4" | "mov" | "avi" | "mkv" | "webm" => FILE_VIDEO,
        _ => FILE_GENERIC,
    }
}
