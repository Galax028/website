use chrono::{DateTime, Datelike as _, Utc};

/// Formats a day from a `DateTime` struct into a day `String` with an ordinal.
fn format_day(date: &DateTime<Utc>) -> String {
    match date.day() {
        day @ (1 | 21 | 31) => format!("{day:02}st"),
        day @ (2 | 22) => format!("{day}nd"),
        day @ (3 | 23) => format!("{day}rd"),
        day @ (1..=31) => format!("{day}th"),
        _ => unreachable!(),
    }
}

/// Formats a `DateTime` struct into a short date `String`.
///
/// Example: `3rd Jan. 2024`
pub(crate) fn format_short_date(date: DateTime<Utc>) -> String {
    format!("{} {}", format_day(&date), date.format("%b. %Y"))
}

/// Formats a `DateTime` struct into a long date `String`.
///
/// Example: `3rd January 2024`
pub(crate) fn format_long_date(date: DateTime<Utc>) -> String {
    format!("{} {}", format_day(&date), date.format("%B %Y"))
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
/// Gets an estimation in minutes of when a person finishes reading `content`.
pub(crate) fn get_read_minutes(content: &str) -> u64 {
    (content.split_whitespace().count() as f64 / 200f64).ceil() as u64
}
