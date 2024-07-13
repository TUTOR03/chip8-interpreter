use std::time::Duration;

use crate::{errors::InterpreterError, Executable, Interpreter, Platform};

pub struct ControlledInterpreter<P: Platform> {
  interpreter: Interpreter<P>,
  delay_timer: Timer,
  sound_timer: Timer,
  instruction_timer: Timer,
}

struct Timer {
  cur_time: Duration,
  interval: Duration,
}

impl Timer {
  pub fn new(interval: Duration) -> Self {
    Self {
      interval,
      cur_time: Duration::ZERO,
    }
  }

  pub fn time_until_tick(&self) -> Duration {
    self.interval.saturating_sub(self.cur_time)
  }

  pub fn add_time(&mut self, value: Duration) -> bool {
    self.cur_time = self.cur_time.saturating_add(value);
    if self.cur_time >= self.interval {
      self.cur_time = self.cur_time.saturating_sub(self.interval);
      return true;
    }
    false
  }
}

pub const DEFAULT_INSTRUCTION_DURATION: Duration = Duration::from_millis(2);
pub const DEFAULT_DELAY_TIMER_DURATION: Duration = Duration::from_millis(16);
pub const DEFAULT_SOUND_TIMER_DURATION: Duration = Duration::from_millis(16);

impl<P: Platform> ControlledInterpreter<P> {
  pub fn new<E: Executable>(
    platform: P,
    executable: E,
    instruction_duration: Duration,
    delay_timer_duration: Duration,
    sound_timer_duration: Duration,
  ) -> Self {
    Self {
      interpreter: Interpreter::new::<E>(platform, executable),
      delay_timer: Timer::new(delay_timer_duration),
      sound_timer: Timer::new(sound_timer_duration),
      instruction_timer: Timer::new(instruction_duration),
    }
  }

  pub fn get_platform_mut(&mut self) -> &mut P {
    self.interpreter.get_platform_mut()
  }

  pub fn simulate_one_instruction(&mut self) -> Result<(), InterpreterError> {
    self.simulate_duration(self.instruction_timer.time_until_tick())
  }

  pub fn simulate_duration(&mut self, duration: Duration) -> Result<(), InterpreterError> {
    let mut remaining = duration;

    while remaining > Duration::ZERO {
      let next_tick_duration = *[
        remaining,
        self.delay_timer.time_until_tick(),
        self.sound_timer.time_until_tick(),
        self.instruction_timer.time_until_tick(),
      ]
      .iter()
      .min()
      .unwrap();

      if self.delay_timer.add_time(next_tick_duration) {
        let platform = self.get_platform_mut();
        platform.set_delay_timer(platform.get_delay_timer().saturating_sub(1));
      }

      if self.sound_timer.add_time(next_tick_duration) {
        let platform = self.get_platform_mut();
        platform.set_sound_timer(platform.get_sound_timer().saturating_sub(1));
      }

      if self.instruction_timer.add_time(next_tick_duration) {
        self.interpreter.run_next()?;
      }

      remaining = remaining.saturating_sub(next_tick_duration);
    }
    Ok(())
  }
}
