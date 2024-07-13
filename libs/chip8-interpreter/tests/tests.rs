use chip8_interpreter::{
  Address, BaseExecutable, BasePlatform, ControlledInterpreter, ScreenFrame,
  DEFAULT_DELAY_TIMER_DURATION, DEFAULT_INSTRUCTION_DURATION, DEFAULT_SOUND_TIMER_DURATION,
};

const TRUE_PIXEL: &str = "@";
const FALSE_PIXEL: &str = ".";

fn assert_display_result(
  image: &[u8],
  load_point: Address,
  expected: &str,
  instruction_count: usize,
) {
  let base_platform = BasePlatform::new(rand::random);
  let executable = BaseExecutable::new(image, load_point);
  let mut interpreter = ControlledInterpreter::new(
    base_platform,
    executable,
    DEFAULT_INSTRUCTION_DURATION,
    DEFAULT_DELAY_TIMER_DURATION,
    DEFAULT_SOUND_TIMER_DURATION,
  );
  for _ in 0..instruction_count {
    interpreter.simulate_one_instruction().unwrap();
  }
  assert_screen_frame_eq(interpreter.get_platform_mut().get_screen_frame(), expected);
}

fn assert_screen_frame_eq(screen_frame: &ScreenFrame, raw_expected: &str) {
  let expected: String = raw_expected
    .trim()
    .split('\n')
    .map(|row| row.trim())
    .collect::<Vec<_>>()
    .join("\n");

  let frame_factual: String = screen_frame
    .iter_rows()
    .map(|row| {
      row
        .iter()
        .map(|value| match *value {
          true => TRUE_PIXEL,
          false => FALSE_PIXEL,
        })
        .collect::<String>()
    })
    .collect::<Vec<_>>()
    .join("\n");

  if expected != frame_factual {
    panic!(
      "Wrong frame state\nexpected:\n{}\nfactual:\n{}",
      expected, frame_factual
    );
  }
}

#[test]
fn test_chip8_splash_screen() {
  assert_display_result(
    include_bytes!("./images/chip8-splash-screen.ch8"),
    Address::new::<0x200>(),
    include_str!("./results/chip8-splash-screen.txt"),
    39,
  );
}

#[test]
fn test_ibm_logo() {
  assert_display_result(
    include_bytes!("./images/ibm-logo.ch8"),
    Address::new::<0x200>(),
    include_str!("./results/ibm-logo.txt"),
    20,
  );
}

#[test]
fn test_corax_plus() {
  assert_display_result(
    include_bytes!("./images/corax-plus.ch8"),
    Address::new::<0x200>(),
    include_str!("./results/corax-plus.txt"),
    284,
  );
}

#[test]
fn test_flags() {
  assert_display_result(
    include_bytes!("./images/flags.ch8"),
    Address::new::<0x200>(),
    include_str!("./results/flags.txt"),
    1000,
  );
}

#[test]
fn test_quirks() {
  let raw_image = include_bytes!("./images/quirks.ch8");
  let mut target_image = [0u8; 4096];
  let mem_start = 0x200;
  let mem_end = raw_image.len() + 0x200;
  target_image[mem_start..mem_end].copy_from_slice(raw_image);
  target_image[0x1FF] = 1;

  assert_display_result(
    &target_image,
    Address::new::<0x0>(),
    include_str!("./results/quirks.txt"),
    2485,
  );
}
