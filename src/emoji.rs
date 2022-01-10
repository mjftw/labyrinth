use unicode_width::UnicodeWidthStr;

pub trait Emoji {
  /// Get emoji representation
  fn emoji(&self) -> &str;

  // Get how many printing characters the emoji takes up
  fn emoji_width(&self) -> usize {
    UnicodeWidthStr::width(self.emoji())
  }
}
