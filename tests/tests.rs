use chip8_interpreter::{BasePlatform, Chip8BaseExecutable, Interpreter};

fn assert_display_result(image: &[u8], instruction_count: usize, expected: &str) {
  let base_platform = BasePlatform::new();
  let executable = Chip8BaseExecutable::new(include_bytes!("./images/chip8-splash-screen.ch8"));
  let interpreter = Interpreter::new(base_platform, executable);
}

fn test_chip8_splash_screen() {}
