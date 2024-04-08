pub mod monster;
pub mod player;

pub trait CharacterTrait : super::Base
{
    fn get_name(&self) -> &str;
    fn set_hp(&mut self, new_hp : i32); 
    fn get_hp(&self) -> i32;
    fn get_max_hp(&self) ->i32;
    fn set_attack(&mut self, new_attack : i32); 
    fn get_attack(&self) -> i32;
    fn set_defence(&mut self, new_defence : i32); 
    fn get_defence(&self) -> i32;
    fn is_dead(&self) -> bool { return self.get_hp() <= 0; }
    fn resurrect(&mut self) { self.set_hp(self.get_max_hp()); }
}