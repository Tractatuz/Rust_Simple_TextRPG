use crate::base::inventory::*;
use crate::base::character::*;

#[derive(Default, Copy, Clone)]
pub struct Player
{
    name : &'static str,
    hp : i32,
    max_hp : i32,
    attack : i32,
    defence : i32,
    inventory : Inventory,
}

impl Player 
{
    pub fn new() -> Self 
    {
        Player 
        {
            name : "Player",
            hp : 100,
            max_hp : 100,
            attack : 10,
            defence : 5,
            inventory : Inventory {}
        }
    }

    pub fn get_inventory(&self) -> &Inventory { &self.inventory }
}

impl crate::base::Base for Player
{    
    fn update(&mut self) -> u16 {
        todo!()
    }

    fn render(&self)
    {
        println!("{}", self.get_name());
        println!("hp : {}", self.get_hp());
        println!("attack : {}", self.get_attack());
        println!("defence : {}", self.get_defence());
    }
}

impl super::CharacterTrait for Player
{
    fn get_name(&self) -> &str { &self.name }
    
    fn set_attack(&mut self, new_attack : i32) { self.attack = new_attack; }
    fn get_attack(&self) -> i32 { return self.attack; }

    fn set_defence(&mut self, new_defence : i32) { self.defence = new_defence; }
    fn get_defence(&self) -> i32 { return self.defence; }
    
    fn set_hp(&mut self, new_hp : i32) { self.hp = new_hp; }
    fn get_hp(&self) -> i32 { return self.hp; }
    
    fn get_max_hp(&self) ->i32 { return self.max_hp; }
}