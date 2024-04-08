use std::io;


use super::*;
use crate::base::item::Item;
use crate::base::character::player::Player;

pub struct ShopScene
{
    player : Player,
    items : Vec<Box<dyn Item>>,
}

impl ShopScene
{
    pub fn new(player : Player) -> Self 
    {
        return ShopScene
        {
            player,
            items : vec![],
        }
    }
}

impl crate::base::Base for ShopScene
{
    fn update(&mut self) -> u16 
    {
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
        println!("Welcome To Shop");
        self.player.get_inventory().render();
        for item in &self.items
        {
            item.render();
        }
        println!("1 ~ 9 : Buy Item, 0 : Lobby");
    }
}

impl crate::scene::Scene for ShopScene
{   
    fn get_scene_type(&self) -> SceneType
    {
        return SceneType::Shop;
    }

    fn change_scene(&self, new_scene_type : SceneType) -> Box<dyn Scene>
    {
        match new_scene_type
        {
            SceneType::Lobby =>
            {
                return Box::new(lobbyscene::LobbyScene::new(self.player));
            },
            SceneType::Battle =>
            {
                return Box::new(battlescene::BattleScene::new(self.player));
            },
            SceneType::Shop => todo!(),
        }
    }
}