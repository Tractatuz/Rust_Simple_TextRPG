use std::io;

use super::*;
use crate::base::character::*;
use crate::base::character::player::Player;
use crate::base::character::monster::MonsterTrait;
use crate::base::character::monster::goblin::Goblin;

pub struct BattleScene
{
    monster : Box<dyn MonsterTrait>,
    player : Player,
}

impl BattleScene
{
    pub fn new(player : Player) -> Self 
    {
        return BattleScene
        {
            monster : Box::new(Goblin::new()),
            player,
        }
    }

    fn attack(&mut self)
    {
        self.monster.set_hp(self.monster.get_hp() - (self.player.get_attack() - self.monster.get_defence()));
        if self.monster.is_dead() == false
        {
            self.player.set_hp(self.player.get_hp() - (self.monster.get_attack() - self.player.get_defence()));
        }
    }
}

impl crate::base::Base for BattleScene
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
                match input_number
                {
                    1 => 
                    {
                        if self.monster.is_dead() || self.player.is_dead()
                        {

                        }
                        else 
                        {
                            self.attack(); 
                        }
                         
                    },
                    _ => {},
                }
                return input_number;
            }
            Err(_) => 
            {
                println!("Wrong Input");
                return u16::MAX;
            }
        }
    }

    fn render(&self) 
    {
        println!("===== Battle =====");
        println!("==================");
        if self.monster.is_dead()
        {
            println!("Monster is Dead!");
            println!("You Win!!");
        }
        else
        {
            self.monster.render();
        }
        println!("==================");
        if self.player.is_dead()
        {
            println!("Player is Dead!");
            println!("You Lose!!");
        }
        else
        {
            self.player.render();
        }
        println!("==================");
        if self.monster.is_dead() || self.player.is_dead()
        {
            println!("Menu = 0 : Lobby");
        }
        else 
        {
            println!("Menu = 1 : Attack, 2 : Use Item, 0 : Lobby");
        }
    }
}

impl Scene for BattleScene
{   
    fn get_scene_type(&self) -> SceneType
    {
        return SceneType::Battle;
    }
    fn change_scene(&self, new_scene_type : SceneType) -> Box<dyn Scene>
    {
        match new_scene_type
        {
            SceneType::Lobby =>
            {
                return Box::new(lobbyscene::LobbyScene::new(self.player));
            },
            SceneType::Battle => todo!(),
            SceneType::Shop =>
            {
                return Box::new(shopscene::ShopScene::new(self.player));
            },
        }
    }
}