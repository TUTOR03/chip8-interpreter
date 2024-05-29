pub trait Platform {
  fn get_random_byte(&mut self) -> u8;
  fn draw_sprite(&mut self, pos: Point, sprite: Sprite) -> bool;
  fn clear_screen(&mut self);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
  pub x: u8,
  pub y: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sprite<'a> {
  data: &'a [u8],
}

impl<'a> Sprite<'a> {
  pub fn new(data: &'a [u8]) -> Self {
    Self { data }
  }

  pub fn iter_pixels(&self) -> impl Iterator<Item = Point> + '_ {
    self.data.iter().enumerate().flat_map(|(i, row)| {
      (0..8).filter_map(move |x| {
        let y = i as u8;
        let val = row & (1 << x);
        if val == 0 {
          return None;
        }
        Some(Point { x, y })
      })
    })
  }
}

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

pub struct ScreenFrame([[bool; SCREEN_WIDTH]; SCREEN_HEIGHT]);
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

pub struct BasePlatform {
  screen_frame: ScreenFrame,
}

impl BasePlatform {
  pub fn new() -> Self {
    Self {
      screen_frame: ScreenFrame::new(),
    }
  }
}

impl Default for BasePlatform {
  fn default() -> Self {
    Self::new()
  }
}

impl Platform for BasePlatform {
  fn get_random_byte(&mut self) -> u8 {
    todo!()
  }

  fn clear_screen(&mut self) {
    self.screen_frame.iter_rows_mut().for_each(|row| {
      row.fill(false);
    });
  }

  fn draw_sprite(&mut self, pos: Point, sprite: Sprite) -> bool {
    let mut was_collision = false;
    sprite.iter_pixels().for_each(|point| {
      let y_index = (point.y + pos.y) as usize;
      let x_index = (point.x + pos.x) as usize;
      let cur_state = &mut self.screen_frame.0[y_index][x_index];
      *cur_state ^= true;
      was_collision |= *cur_state;
    });

    was_collision
  }
}
