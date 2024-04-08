use super::*;

pub trait UseItem : Item
{
    fn get_item_type(&self) -> ItemType
    {
        return ItemType::UseItem;
    }

    fn use_item();
}