#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct Point {
  pub x: u8,
  pub y: u8,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct Sprite<'a>(&'a [u8]);

impl<'a> Sprite<'a> {
  pub fn new(value: &'a [u8]) -> Self {
    Self(value)
  }

  pub fn iter_pixels(&self) -> impl Iterator<Item = Point> + '_ {
    self.0.iter().enumerate().flat_map(|(i, row)| {
      (0..8).filter_map(move |x| {
        let y = i as u8;
        let val = row & (0x80 >> x);
        if val == 0 {
          return None;
        }
        Some(Point { x, y })
      })
    })
  }
}
