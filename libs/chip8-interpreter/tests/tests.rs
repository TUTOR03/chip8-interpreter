use chip8_interpreter::{BasePlatform, Chip8BaseExecutable, Interpreter, ScreenFrame};

const TRUE_PIXEL: &str = "@";
const FALSE_PIXEL: &str = ".";

fn assert_display_result(image: &[u8], expected: &str, instruction_count: usize) {
  let base_platform = BasePlatform::new();
  let executable = Chip8BaseExecutable::new(image);
  let mut interpreter = Interpreter::new(base_platform, executable);
  for _ in 0..instruction_count {
    interpreter.run_one_instruction().unwrap();
  }
  assert_screen_frame_eq(&interpreter.platform().screen_frame, expected);
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
    include_str!("./results/chip8-splash-screen.txt"),
    39,
  );
}

#[test]
fn test_ibm_logo() {
  assert_display_result(
    include_bytes!("./images/ibm-logo.ch8"),
    include_str!("./results/ibm-logo.txt"),
    20,
  );
}

#[test]
fn test_corax_plus() {
  assert_display_result(
    include_bytes!("./images/corax-plus.ch8"),
    include_str!("./results/corax-plus.txt"),
    284,
  );
}

#[test]
fn test_4_flags() {
  assert_display_result(
    include_bytes!("./images/4-flags.ch8"),
    include_str!("./results/4-flags.txt"),
    1000,
  );
}
