use anyhow::Result;

pub fn copy(text: String) -> Result<()> {
  let mut clipboard = arboard::Clipboard::new()?;
  clipboard.set_text(text)?;
  Ok(())
}
