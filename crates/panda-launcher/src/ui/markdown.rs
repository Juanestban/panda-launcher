//! Just enough Markdown for Modrinth descriptions: headings, paragraphs, list items and code blocks.
//!
//! shortcut: images, tables and inline styles are dropped (links keep their text); render them once zgui has rich text.

#[derive(Clone, Debug, PartialEq)]
pub enum Block {
  Heading(u8, String),
  Paragraph(String),
  Item(String),
  Code(String),
}

impl Block {
  pub fn class(&self) -> &'static str {
    match self {
      Block::Heading(1, _) => "md-h1",
      Block::Heading(2, _) => "md-h2",
      Block::Heading(..) => "md-h3",
      Block::Paragraph(_) => "md-p",
      Block::Item(_) => "md-li",
      Block::Code(_) => "md-code",
    }
  }

  pub fn text(&self) -> String {
    match self {
      Block::Item(t) => format!("•  {t}"),
      Block::Heading(_, t) | Block::Paragraph(t) | Block::Code(t) => t.clone(),
    }
  }
}

pub fn parse(source: &str) -> Vec<Block> {
  let mut blocks = Vec::new();
  let mut paragraph = Vec::<String>::new();
  let mut code: Option<Vec<&str>> = None;
  let flush = |paragraph: &mut Vec<String>, blocks: &mut Vec<Block>| {
    if !paragraph.is_empty() {
      blocks.push(Block::Paragraph(paragraph.join(" ")));
      paragraph.clear();
    }
  };

  for raw in source.lines() {
    let line = raw.trim();
    if line.starts_with("```") {
      match code.take() {
        Some(lines) => blocks.push(Block::Code(lines.join("\n"))),
        None => {
          flush(&mut paragraph, &mut blocks);
          code = Some(Vec::new());
        }
      }
      continue;
    }
    if let Some(lines) = &mut code {
      lines.push(raw);
      continue;
    }
    let text = inline(line);
    if text.is_empty() || line.starts_with("---") || line.starts_with('|') {
      flush(&mut paragraph, &mut blocks);
      continue;
    }
    let hashes = line.chars().take_while(|c| *c == '#').count();
    if (1..=6).contains(&hashes) && line[hashes..].starts_with(' ') {
      flush(&mut paragraph, &mut blocks);
      blocks.push(Block::Heading(hashes.min(3) as u8, inline(&line[hashes..])));
    } else if let Some(item) = ["- ", "* ", "+ "].iter().find_map(|m| line.strip_prefix(m)).or_else(|| numbered(line)) {
      flush(&mut paragraph, &mut blocks);
      blocks.push(Block::Item(inline(item)));
    } else {
      paragraph.push(text);
    }
  }
  flush(&mut paragraph, &mut blocks);
  if let Some(lines) = code {
    blocks.push(Block::Code(lines.join("\n")));
  }
  blocks
}

/// "3. text" → "text".
fn numbered(line: &str) -> Option<&str> {
  let digits = line.chars().take_while(char::is_ascii_digit).count();
  (digits > 0).then(|| line[digits..].strip_prefix(". ")).flatten()
}

/// Plain text of one line: images and HTML tags removed, links reduced to their text, emphasis marks dropped.
fn inline(line: &str) -> String {
  // Images first, so a linked badge `[![alt](img)](url)` becomes `[](url)` and then nothing.
  let mut without_images = String::new();
  let mut rest = line;
  while let Some(start) = rest.find("![") {
    without_images.push_str(&rest[..start]);
    rest = skip_link(&rest[start + 1..]).unwrap_or(&rest[start + 2..]);
  }
  without_images.push_str(rest);
  let mut out = String::new();
  let mut rest = without_images.as_str();
  while let Some(c) = rest.chars().next() {
    if c == '[' {
      match link_text(rest) {
        Some((text, after)) => {
          out.push_str(&inline(text));
          rest = after;
        }
        None => {
          out.push(c);
          rest = &rest[1..];
        }
      }
    } else if c == '<' && rest.contains('>') {
      rest = &rest[rest.find('>').expect("checked") + 1..];
    } else if matches!(c, '*' | '_' | '`' | '>') && (c != '_' || !out.ends_with(|p: char| p.is_alphanumeric())) {
      rest = &rest[1..];
    } else {
      // Geist has no emoji; they would show as empty boxes.
      if !is_emoji(c) {
        out.push(c);
      }
      rest = &rest[c.len_utf8()..];
    }
  }
  out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn is_emoji(c: char) -> bool {
  matches!(c as u32, 0x1F000..=0x1FAFF | 0x2600..=0x27BF | 0xFE0F | 0x200D)
}

/// `[text](url)rest` → (`text`, `rest`).
fn link_text(s: &str) -> Option<(&str, &str)> {
  let close = s.find("](")?;
  let after = skip_link(s)?;
  Some((&s[1..close], after))
}

/// Skips `[...](...)` and returns what follows.
fn skip_link(s: &str) -> Option<&str> {
  let close = s.find("](")?;
  let end = s[close..].find(')')? + close;
  Some(&s[end + 1..])
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn modrinth_style_description() {
    let md = "![banner](https://x/b.png)\n# Mod Menu\n\nAdds a **mod menu** to view the list\nof [installed mods](https://x).\n\n<p align=\"center\">Hi</p>\n\n## Features\n- Search mods\n* Config `screens`\n1. First\n\n```\nlet x = 1;\n```\nsnake_case stays";
    assert_eq!(
      parse(md),
      vec![
        Block::Heading(1, "Mod Menu".into()),
        Block::Paragraph("Adds a mod menu to view the list of installed mods.".into()),
        Block::Paragraph("Hi".into()),
        Block::Heading(2, "Features".into()),
        Block::Item("Search mods".into()),
        Block::Item("Config screens".into()),
        Block::Item("First".into()),
        Block::Code("let x = 1;".into()),
        Block::Paragraph("snake_case stays".into()),
      ]
    );
    assert_eq!(parse("## 📖 Installation Guide ✨"), vec![Block::Heading(2, "Installation Guide".into())]);
    assert_eq!(parse("[![Coffee](https://i/c.png)](https://d) Join [Discord](https://x)"), vec![Block::Paragraph("Join Discord".into())]);
  }

  #[test]
  fn broken_markup_does_not_panic() {
    for s in ["[", "![", "[a](", "<", "#", "1.", "```", "ä*ö", "<a href"] {
      parse(s);
    }
  }
}
