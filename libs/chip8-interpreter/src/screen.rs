pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct ScreenFrame(pub [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT]);

impl ScreenFrame {
  pub fn new() -> Self {
    Self([[false; SCREEN_WIDTH]; SCREEN_HEIGHT])
  }

  pub fn iter_rows(&self) -> impl Iterator<Item = &[bool; SCREEN_WIDTH]> {
    self.0.iter()
  }

  pub fn iter_rows_mut(&mut self) -> impl Iterator<Item = &mut [bool; SCREEN_WIDTH]> {
    self.0.iter_mut()
  }
}

impl Default for ScreenFrame {
  fn default() -> Self {
    Self::new()
  }
}
