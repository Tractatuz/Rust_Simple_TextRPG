pub mod maingame;
pub mod character;
pub mod scene;
pub mod inventory;
pub mod item;

pub trait Base
{
    fn update(&mut self) -> u16;
    fn render(&self);
}

