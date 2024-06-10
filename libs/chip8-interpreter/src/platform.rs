use crate::{screen::ScreenFrame, Nibble};

use super::sprite::{Point, Sprite};

pub trait Platform {
  fn get_random_byte(&mut self) -> u8;
  fn draw_sprite(&mut self, pos: Point, sprite: Sprite) -> bool;
  fn clear_screen(&mut self);
  fn get_delay_timer(&self) -> u8;
  fn set_delay_timer(&mut self, value: u8);
  fn set_sound_timer(&mut self, value: u8);
  fn is_key_down(&self, key: Nibble) -> bool;
  fn get_last_pressed_key(&self) -> Option<Nibble>;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct BasePlatform {
  pub screen_frame: ScreenFrame,
}

impl BasePlatform {
  pub fn new() -> Self {
    Self {
      screen_frame: ScreenFrame::new(),
    }
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

  fn get_delay_timer(&self) -> u8 {
    todo!()
  }

  fn set_delay_timer(&mut self, value: u8) {
    todo!()
  }

  fn set_sound_timer(&mut self, value: u8) {
    todo!()
  }

  fn is_key_down(&self, key: Nibble) -> bool {
    todo!()
  }
  fn get_last_pressed_key(&self) -> Option<Nibble> {
    todo!()
  }
}
