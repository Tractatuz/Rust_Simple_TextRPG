use super::*;

pub trait EquipItem : Item
{
    fn get_item_type(&self) -> ItemType
    {
        return ItemType::EquipItem;
    }

    fn equip_item();
}