use crate::character::*;

pub struct Goblin
{
    name : &'static str,
    hp : i32,
    max_hp : i32,
    attack : i32,
    defence : i32,
}

impl Goblin
{
    pub fn new() -> Self 
    {
        return Goblin
        {
            name : "Goblin",
            hp : 50,
            max_hp : 50,
            attack : 7,
            defence : 3,
        }
    }
}

impl super::MonsterTrait for Goblin
{
}

impl crate::character::CharacterTrait for Goblin
{
    fn get_name(&self) -> &str { &self.name }

    fn set_attack(&mut self, new_attack : i32) { self.attack = new_attack; }
    fn get_attack(&self) -> i32 { self.attack }

    fn set_defence(&mut self, new_defence : i32) { self.defence = new_defence; }
    fn get_defence(&self) -> i32 { self.defence }
    
    fn set_hp(&mut self, new_hp : i32) { self.hp = new_hp; }
    fn get_hp(&self) -> i32 { self.hp }
    
    fn get_max_hp(&self) ->i32 { self.max_hp }
}

impl crate::base::Base for Goblin
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