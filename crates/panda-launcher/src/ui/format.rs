//! Short number and date forms for cards.

/// 999 → "999", 12_345 → "12.3k", 4_560_000 → "4.6M".
pub fn compact(n: u64) -> String {
  match n {
    0..1_000 => n.to_string(),
    1_000..1_000_000 => trim(n as f64 / 1_000.0, "k"),
    _ => trim(n as f64 / 1_000_000.0, "M"),
  }
}

fn trim(value: f64, unit: &str) -> String {
  let text = format!("{value:.1}");
  format!("{}{unit}", text.strip_suffix(".0").unwrap_or(&text))
}

/// "2025-10-03T14:22:01Z" → "2025-10-03".
pub fn date(iso: &str) -> &str {
  iso.get(..10).unwrap_or(iso)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn numbers_and_dates() {
    assert_eq!(compact(999), "999");
    assert_eq!(compact(1_000), "1k");
    assert_eq!(compact(12_345), "12.3k");
    assert_eq!(compact(4_560_000), "4.6M");
    assert_eq!(date("2025-10-03T14:22:01.123Z"), "2025-10-03");
    assert_eq!(date("bad"), "bad");
  }
}
