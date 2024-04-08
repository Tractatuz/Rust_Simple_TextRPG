use crate::base::*;
use crate::base::scene::scenemanager::SceneManager;
use crate::base::character::player::Player;

pub struct MainGame
{
    scene_manager : SceneManager,
}

impl MainGame 
{
    pub fn new() -> Self 
    {
        let player : Player = Player::new();
        let scene_manager : SceneManager = SceneManager::new(player);
        
        MainGame
        {
            scene_manager,
        }
    }
}

impl Base for MainGame
{
    fn update(&mut self) -> u16 
    {
        return self.scene_manager.update();
    }

    fn render(&self) 
    {
        crate::clear_console();
        self.scene_manager.render();
    }
}
