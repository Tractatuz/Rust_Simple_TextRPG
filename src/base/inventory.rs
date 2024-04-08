#[derive(Default, Copy, Clone)]
pub struct Inventory
{

}

impl crate::base::Base for Inventory
{
    fn update(&mut self) -> u16 
    {
        return 0;
    }

    fn render(&self) 
    {
        println!("Inventory...");
    }
}