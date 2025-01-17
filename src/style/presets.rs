/// The default style for tables.
///
/// ```text
/// +-------+-------+
/// | Hello | there |
/// +===============+
/// | a     | b     |
/// |-------+-------|
/// | c     | d     |
/// +-------+-------+
/// ```
pub const ASCII_FULL: &str = "||--+==+|-+||++++++";

/// Default style without any borders.
///
/// ```text
///  Hello | there
/// ===============
///  a     | b
/// -------+-------
///  c     | d
/// ```
pub const ASCII_NO_BORDERS: &str = "     == |-+        ";

/// Just like ASCII_FULL, but without vertical/horizontal middle lines.
///
/// ```text
/// +---------------+
/// | Hello   there |
/// +===============+
/// | a       b     |
/// |               |
/// | c       d     |
/// +---------------+
/// ```
pub const ASCII_BORDERS_ONLY: &str = "||--+==+   ||--++++";

/// Just like ASCII_BORDERS_ONLY, but without spacing between rows.
///
/// ```text
/// +---------------+
/// | Hello   there |
/// +===============+
/// | a       b     |
/// | c       d     |
/// +---------------+
/// ```
pub const ASCII_BORDERS_ONLY_CONDENSED: &str = "||--+==+     --++++";

/// Just like ASCII_FULL, but without vertical/horizontal middle lines and no side borders.
///
/// ```text
/// ---------------
///  Hello   there
/// ===============
///  a       b
/// ---------------
///  c       d
/// ---------------
/// ```
pub const ASCII_HORIZONTAL_ONLY: &str = "  -- ==  --  --    ";

/// Markdown like table styles.
///
/// ```text
/// | Hello | there |
/// |-------|-------|
/// | a     | b     |
/// | c     | d     |
/// ```
pub const ASCII_MARKDOWN: &str = "||  |-|||           ";

/// The UTF8 enabled version of the default style for tables.\
/// Quite beautiful isn't it? It's drawn with UTF8's box drawing characters.
///
/// ```text
/// ┌───────┬───────┐
/// │ Hello ┆ there │
/// ╞═══════╪═══════╡
/// │ a     ┆ b     │
/// ├╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┤
/// │ c     ┆ d     │
/// └───────┴───────┘
/// ```
pub const UTF8_FULL: &str = "││──╞═╪╡┆╌┼├┤┬┴┌┐└┘";

/// Default UTF8 style, but without any borders.
///
/// ```text
///  Hello ┆ there
/// ═══════╪═══════
///  a     ┆ b
/// ╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌
///  c     ┆ d
/// ```
pub const UTF8_NO_BORDERS: &str = "     ═╪ ┆╌┼        ";

/// Just like the UTF8 FULL version, but without vertical/horizontal middle lines.
///
/// ```text
/// ┌───────────────┐
/// │ Hello   there │
/// ╞═══════════════╡
/// │ a       b     │
/// │ c       d     │
/// └───────────────┘
/// ```
pub const UTF8_BORDERS_ONLY: &str = "││──╞══╡     ──┌┐└┘";

/// Only display vertical lines.
///
/// ```text
/// ───────────────
///  Hello   there
/// ═══════════════
///  a       b
/// ───────────────
///  c       d
/// ───────────────
/// ```
pub const UTF8_HORIZONTAL_ONLY: &str = "  ── ══  ──  ──    ";

/// Don't draw any borders or other lines.
/// Useful, if you want to simply organize some data without any cosmetics.
///
/// ```text
///  Hello  there
///  a      b
///  c      d
/// ```
pub const NOTHING: &str = "                   ";

/// This is here for compatibiliity reasons.
/// It'll be removed in the next major version update.
/// TODO: Remove in v5.0
pub const ASCII_HORIZONTAL_BORDERS_ONLY: &str = "  -- ==  --  --    ";

/// This is here for compatibiliity reasons.
/// It'll be removed in the next major version update.
/// TODO: Remove in v5.0
pub const UTF8_HORIZONTAL_BORDERS_ONLY: &str = "  ── ══  ──  ──    ";
