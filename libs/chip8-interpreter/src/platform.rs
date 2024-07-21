use crate::{keyboard::Keyboard, screen::ScreenFrame, Nibble, SCREEN_HEIGHT, SCREEN_WIDTH};

use super::sprite::{Point, Sprite};

pub trait Platform {
  fn get_random_byte(&mut self) -> u8;
  fn draw_sprite(&mut self, pos: Point, sprite: Sprite) -> bool;
  fn clear_screen(&mut self);
  fn get_delay_timer(&self) -> u8;
  fn get_sound_timer(&self) -> u8;
  fn set_delay_timer(&mut self, value: u8);
  fn set_sound_timer(&mut self, value: u8);
  fn is_key_down(&self, key: Nibble) -> bool;
  fn get_last_pressed_key(&mut self) -> Option<Nibble>;
}

pub trait RandomGenerator: FnMut() -> u8 {}

impl<R: FnMut() -> u8> RandomGenerator for R {}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct BasePlatform<R: RandomGenerator> {
  rand: R,
  screen_frame: ScreenFrame,
  delay_timer: u8,
  sound_timer: u8,
  keyboard: Keyboard,
  last_pressed_key: Option<Nibble>,
}

impl<R: RandomGenerator> BasePlatform<R> {
  pub fn new(rand: R) -> Self {
    Self {
      rand,
      screen_frame: ScreenFrame::new(),
      delay_timer: 0,
      sound_timer: 0,
      keyboard: Keyboard::new(),
      last_pressed_key: None,
    }
  }

  pub fn change_keyboard_state(&mut self, key_index: Nibble, is_down: bool) {
    self.keyboard[key_index].set_is_down(is_down);

    if is_down {
      self.keyboard[key_index].set_is_trackable(true);
      self.last_pressed_key = None;
    } else if self.keyboard[key_index].is_trackable() {
      self.last_pressed_key = Some(key_index);
      self.keyboard[key_index].set_is_trackable(false);
    }
  }

  pub fn get_screen_frame(&self) -> &ScreenFrame {
    &self.screen_frame
  }
}

impl<R: RandomGenerator> Platform for BasePlatform<R> {
  fn get_random_byte(&mut self) -> u8 {
    (self.rand)()
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
      if y_index >= SCREEN_HEIGHT || x_index >= SCREEN_WIDTH {
        return;
      }
      let cur_state = &mut self.screen_frame.0[y_index][x_index];
      *cur_state ^= true;
      was_collision |= !*cur_state;
    });

    was_collision
  }

  fn get_delay_timer(&self) -> u8 {
    self.delay_timer
  }

  fn set_delay_timer(&mut self, value: u8) {
    self.delay_timer = value;
  }

  fn get_sound_timer(&self) -> u8 {
    self.sound_timer
  }

  fn set_sound_timer(&mut self, value: u8) {
    self.sound_timer = value;
  }

  fn is_key_down(&self, key_index: Nibble) -> bool {
    self.keyboard[key_index].is_down()
  }

  fn get_last_pressed_key(&mut self) -> Option<Nibble> {
    self.last_pressed_key.take()
  }
}
