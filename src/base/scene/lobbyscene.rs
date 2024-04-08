use std::io;

use super::*;
use crate::base::character::*;
use crate::base::character::player::Player;


pub struct LobbyScene
{
    player : Player,
}

impl LobbyScene
{
    pub fn new(player : Player) -> Self 
    {
        return LobbyScene
        {
            player,
        }
    }
}

impl crate::base::Base for LobbyScene
{
    fn update(&mut self) -> u16 
    {
        if self.player.is_dead()
        {
            self.player.resurrect();
        }

        let mut input_string : String = String::new();
        io::stdin().read_line(&mut input_string).expect("Fail to read line!");
        
        // To Trim '\n'
        let input : Result<u16, _> = input_string.trim().parse();
        match input 
        {
            Ok(input_number) => 
            {
                return input_number;
            }
            Err(_) => 
            {
                return u16::MAX;
            }
        }
    }

    fn render(&self) 
    {
        println!("Welcome To Text RPG!");
        println!("====================");
        println!("Menu = 1 : Battle , 2 : Shop , 0 : Quit");
    }
}

impl crate::scene::Scene for LobbyScene
{   
    fn get_scene_type(&self) -> SceneType
    {
        return SceneType::Lobby;
    }
    
    fn change_scene(&self, new_scene_type : SceneType) -> Box<dyn Scene>
    {
        match new_scene_type
        {
            SceneType::Lobby => todo!(),
            SceneType::Battle =>
            {
                return Box::new(battlescene::BattleScene::new(self.player));
            },
            SceneType::Shop =>
            {
                return Box::new(shopscene::ShopScene::new(self.player));
            },
        }
    }
}