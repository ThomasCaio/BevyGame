use crate::combat::DamageType;

#[derive(Debug, Clone)]
pub struct Equipments {
    pub mainhand: Option<Item>,
    pub offhand: Option<Item>,
    pub neck: Option<Item>,
    pub head: Option<Item>,
    pub chest: Option<Item>,
    pub legs: Option<Item>,
    pub boots: Option<Item>,
    pub leftfinger: Option<Item>,
    pub rightfinger: Option<Item>,
}

impl Equipments {
    pub fn get_attributes(&self, attribute: AttributeType) -> f32 {
        let mut total_value = 0.;
        let equipments = vec![
            self.mainhand.as_ref(), 
            self.offhand.as_ref(), 
            self.neck.as_ref(), 
            self.head.as_ref(), 
            self.chest.as_ref(), 
            self.legs.as_ref(), 
            self.boots.as_ref(),
            self.leftfinger.as_ref(),
            self.rightfinger.as_ref(),
        ];
        for item in equipments {
            if let Some(t) = item {
                for attr in &t.attributes {
                    // println!("{:?} - {:?}", attr.attribute_type, attribute);
                    if attr.attribute_type == attribute {
                        total_value += attr.value;
                    }
                }
            }
        }
        total_value
    }
}

impl Default for Equipments {
    fn default() -> Self {
        Equipments {
            mainhand: None,
            offhand: None,
            neck: None,
            head: None,
            chest: None,
            legs: None,
            boots: None,
            leftfinger: None,
            rightfinger: None,
        }
    }
}

impl IntoIterator for Equipments {
    type Item = Option<Item>;
    type IntoIter = EquipmentsIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        EquipmentsIntoIterator {
            equipments: self,
            index: 0,
        }
    }
}

pub struct EquipmentsIntoIterator {
    equipments: Equipments,
    index: usize,
}

impl Iterator for EquipmentsIntoIterator {
    type Item = Option<Item>;

    fn next(&mut self) -> Option<Option<Item>> {
        let result = match self.index {
            0 => self.equipments.mainhand.clone(),
            1 => self.equipments.offhand.clone(),
            2 => self.equipments.neck.clone(),
            3 => self.equipments.head.clone(),
            4 => self.equipments.chest.clone(),
            5 => self.equipments.legs.clone(),
            6 => self.equipments.boots.clone(),
            7 => self.equipments.leftfinger.clone(),
            8 => self.equipments.rightfinger.clone(),
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}

#[derive(Debug, Clone)]
pub enum ItemSlot {
    MainHand,
    OffHand,
    Neck,
    Head,
    Chest,
    Legs,
    Boots,
    LeftFinger,
    RightFinger,
}


#[derive(Debug, Clone)]
pub struct Item{
    pub title: String,
    pub description: String,
    pub slot: ItemSlot,
    pub attributes: Vec<Attribute>,
}

// TODO: ARRUMAR O ESQUEMA DE CRIAÇÃO DE ITEMS (COLETA DE DADOS)!

impl Item {
    pub fn new(title: &str) -> Option<Item> {
        match title {
            "Sword" => Some(Item{title: "Sword".to_string(), description: "Iron small sword".to_string(), slot: ItemSlot::MainHand, attributes: vec![Attribute{value: 10., attribute_type: AttributeType::Damage(DamageType::Fire) }]}),
            "Leather Armor" => Some(Item{title: "Leather Armor".to_string(), description: "Light armor".to_string(), slot: ItemSlot::Chest, attributes: vec![Attribute{value: 10., attribute_type: AttributeType::Defense }]}),
            "Wooden Shield" => Some(Item{title: "Wooden Shield".to_string(), description: "Shield made with wooden".to_string(), slot: ItemSlot::OffHand, attributes: vec![Attribute{value: 10., attribute_type: AttributeType::Resistance(DamageType::Physical) }]}),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Attribute{
    pub value: f32,
    pub attribute_type: AttributeType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttributeType {
    Damage(DamageType),
    Defense,
    Resistance(DamageType),
    Evasion,
    Block,
    MaxHealth,
    MaxMana,
}