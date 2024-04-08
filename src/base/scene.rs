mod battlescene;
mod lobbyscene;
mod shopscene;
pub mod scenemanager;

use crate::base::*;

pub enum SceneType
{
    Lobby,
    Battle,
    Shop,
}

pub trait Scene : Base
{
    fn get_scene_type(&self) -> SceneType;
    fn change_scene(&self, new_scene_type : SceneType) -> Box<dyn Scene>;
}