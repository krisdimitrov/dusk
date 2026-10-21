use std::path::Path;

use owo_colors::{AnsiColors, OwoColorize, Stream};

use crate::scan::Entry;

const BAR_WIDTH: usize = 30;

pub fn print(root: &Path, entries: &[Entry], top: usize) {
    let total: u64 = entries.iter().map(|e| e.size).sum();
    let largest = entries.first().map_or(0, |e| e.size);

    println!("{}\n", root.display().if_supports_color(Stream::Stdout, |p| p.bold()));

    for entry in entries.iter().take(top) {
        let share = ratio(entry.size, total);
        // Bars are scaled to the largest entry so the top one always fills the bar.
        let filled = (ratio(entry.size, largest) * BAR_WIDTH as f64).round() as usize;
        let bar = format!("{}{}", "█".repeat(filled), "░".repeat(BAR_WIDTH - filled));
        let color = share_color(share);

        let name = if entry.is_dir { format!("{}/", entry.name) } else { entry.name.clone() };
        let name = name.if_supports_color(Stream::Stdout, |n| {
            if entry.is_dir { n.blue().bold().to_string() } else { n.to_string() }
        });

        println!(
            "{:>9}  {:>5.1}%  {}  {}",
            human_size(entry.size),
            share * 100.0,
            bar.if_supports_color(Stream::Stdout, |b| b.color(color)),
            name,
        );
    }

    if entries.len() > top {
        println!("{:>9}  … and {} more", "", entries.len() - top);
    }
    println!("\n{:>9}  total", human_size(total).if_supports_color(Stream::Stdout, |t| t.bold()));
}

fn ratio(part: u64, whole: u64) -> f64 {
    if whole == 0 { 0.0 } else { part as f64 / whole as f64 }
}

/// Red for the big offenders, yellow for notable ones, green for the rest.
fn share_color(share: f64) -> AnsiColors {
    if share >= 0.25 {
        AnsiColors::Red
    } else if share >= 0.10 {
        AnsiColors::Yellow
    } else {
        AnsiColors::Green
    }
}

fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    if unit == 0 { format!("{bytes} B") } else { format!("{size:.1} {}", UNITS[unit]) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_sizes() {
        assert_eq!(human_size(0), "0 B");
        assert_eq!(human_size(1023), "1023 B");
        assert_eq!(human_size(1024), "1.0 KB");
        assert_eq!(human_size(1536), "1.5 KB");
        assert_eq!(human_size(5 * 1024 * 1024 * 1024), "5.0 GB");
    }
}
