pub enum ItemType
{
    EquipItem,
    UseItem,
}

pub trait Item : crate::base::Base
{
    fn get_item_type(&self) -> ItemType;
}