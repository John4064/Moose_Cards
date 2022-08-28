use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UnitCard {
    id: i32,
    card_type: String,
    name: String,
    description: String,
    unit_stats: Stats
}

#[derive(Deserialize, Serialize, Debug)]
struct Stats{
    hp: i32,
    power: i32,
    speed: i32
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ItemCard{
    id: i32,
    card_type: String,
    item_name: String,
    item_description : String,
    item_stats: Stats
}

impl UnitCard{
    //Getter
    pub fn get_name(&self) -> &String{
        return &self.name;
    }

    pub fn get_id(&self)-> &i32{
        return &self.id;
    }

    pub fn get_type(&self) -> &String{
        return &self.card_type;
    }

    pub fn get_description(&self) -> &String{
        return &self.description;
    }

    pub fn get_stats(&self) -> &Stats{
        return &self.unit_stats;
    }
}

impl ItemCard{
    //Getters
    pub fn get_name(&self) -> &String{
        return &self.item_name;
    }

    pub fn get_id(&self)-> &i32{
        return &self.id;
    }

    pub fn get_type(&self) -> &String{
        return &self.card_type;
    }
    pub fn get_description(&self) -> &String{
        return &self.item_description;
    }
    pub fn get_stats(&self) -> &Stats{
        return &self.item_stats;
    }
    //Setters(IN PROGRESS)
}
impl Stats{
    // hp: i32,
    // power: i32,
    // speed: i32
    pub fn get_hp(&self)-> &i32{
        return &self.hp;
    }
    pub fn get_power(&self)-> &i32{
        return &self.power;
    }
    pub fn get_speed(&self)-> &i32{
        return &self.speed;
    }
}

#[derive(Builder, Debug)]
pub struct test{
    pub id: i32,
    pub name : String
}