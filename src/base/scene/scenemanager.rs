use crate::base::scene::*;
use crate::base::scene::lobbyscene::LobbyScene;
use crate::base::character::player::Player;


pub struct SceneManager
{
    current_scene : Box<dyn Scene>,
}

impl SceneManager
{
    pub fn new(new_player : Player) -> Self
    {
        SceneManager
        {
            current_scene : Box::new(LobbyScene::new(new_player)),
        }
    }

    fn change_scene(&mut self, scene_type : SceneType)
    {
        self.current_scene = self.current_scene.change_scene(scene_type);
    }
}

impl Base for SceneManager
{
    fn update(&mut self) -> u16
    {
        let result = self.current_scene.update();
        if result == 0
        {
            match self.current_scene.get_scene_type()
            {
                SceneType::Lobby => return 0,
                _ => 
                {
                    self.change_scene(SceneType::Lobby);
                    return u16::MAX;
                },
            }
        }
        else if result == 1
        {
            match self.current_scene.get_scene_type()
            {
                SceneType::Lobby =>
                {
                    self.change_scene(SceneType::Battle);
                    return u16::MAX;
                },
                _ => {},
            }
        }
        else if result == 2
        {
            match self.current_scene.get_scene_type()
            {
                SceneType::Lobby =>
                {
                    self.change_scene(SceneType::Shop);
                    return u16::MAX;
                },
                _ => {},
            }
        }

        return result;
    }

    fn render(&self)
    {
        self.current_scene.render();
    }
}