use super::colored;
use super::COLOR;
use tracing;

pub enum Level {
    Info,
    Error,
}

pub trait ColorTrait {
    fn color(&self) -> COLOR;
}
pub struct Color(pub COLOR);
impl ColorTrait for Color {
    fn color(&self) -> COLOR {
        self.0
    }
}
pub struct Tag<'a>(pub &'a str);
pub struct Text<'a>(pub &'a str);

pub fn log(l: Level, c: impl ColorTrait, tag: Tag, message: Text) {
    let color = c.color();

    match l {
        Level::Info => {
            tracing::info!("{}: {}", colored(color, tag.0), message.0);
        }
        Level::Error => {
            tracing::error!("{}: {}", colored(color, tag.0), message.0);
        }
    }
}
