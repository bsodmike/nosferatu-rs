type COLOR = (i32, i32, i32);
pub static YELLOW: COLOR = (250, 189, 47);
#[allow(dead_code)]
pub static GREEN: COLOR = (184, 187, 38);
#[allow(dead_code)]
pub static ORANGE: COLOR = (199, 100, 42);
#[allow(dead_code)]
pub static RED: COLOR = (250, 64, 46);

pub mod logger;

pub fn colored(color: (i32, i32, i32), text: &str) -> String {
    let (r, g, b) = color;
    return format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, text);
}
