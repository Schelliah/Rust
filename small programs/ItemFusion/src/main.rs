use std::fs;
//program to find quickest or cheapest way to get item, given item and item fusion data

#[derive(Clone, PartialEq,PartialOrd)]
pub struct ShopItem{
    shop_location:isize,  
    price:f64
}

#[derive(Clone)]
pub struct Item{
    item_name:String,
    shops:Vec<ShopItem>,
    forage_locations:Vec<isize>,
    possible_fusion_list:Vec<Fusion>
}

#[derive(Clone)]
pub struct Fusion{
    result:Item,
    item1:Item,
    item2:Item,
}

impl Item{
    pub fn set_fusion(&mut self,fusion_list:Vec<Fusion>){ //insert all fusions, save only fusions with which are possible for item, exclude fusions which have Item as ingredient
        self.possible_fusion_list=fusion_list.into_iter().filter(|fusion|fusion.result.item_name.eq(&self.item_name) && !fusion.item1.item_name.eq(&self.item_name) && !fusion.item2.item_name.eq(&self.item_name)).collect()
    }
    pub fn is_forageable(&self) -> bool{
        return !self.forage_locations.is_empty();
    }

    pub fn is_purchasable(&self) ->bool{
        return !self.shops.is_empty();
    }

    pub fn craftable(&self)-> bool{
        return !self.possible_fusion_list.is_empty();
    }

    //earliest and cheapest assuming you don't craft
    pub fn get_earliest_location(& self) -> isize{
        let min:isize = self.forage_locations.clone().into_iter().min().unwrap();
        return min;
    }

    pub fn get_cheapest_shop(&mut self)  -> isize{
        let min:ShopItem = self.shops.iter().min_by(|a, b| a.price.partial_cmp(&b.price).unwrap()).cloned().unwrap();
        return min.shop_location;
    }
}

pub fn convert_int_to_locationstring(location_number:isize) ->String{
    match location_number{
        0=> String::from("Viridian"),
        1=> String::from("Pewter"),
        2=> String::from("Vermillion"),
        3=> String::from("Saffron"),
        _=> panic!("wrong location number")
    }
}



fn main() {
    //let file_location:String = String::from("item-fusion-data.txt");
    let file_location:& str = "item-fusion-data.txt";
    let mut store_items:Vec<Vec<String>>=read_input(file_location);
    let mut item_list:Vec<Item> = Vec::new();
    
}

fn get_vector_items(input:String) -> Vec<String>{
    let mut items_vector:Vec<String>= input.split("\r\n").map(String::from).collect();
    items_vector.remove(0);
    items_vector.remove(items_vector.len()-1);
    return items_vector;
}

fn read_input(file_location:&str) -> Vec<Vec<String>>{
    let input:String = fs::read_to_string(file_location).unwrap().replace("\r\n\r\n", "\r\n").replace("*****\r\n", "");
    let input_vector: Vec<String> = input.split(":").map(String::from).collect();
    println!("{:?}",input_vector[7]);//4 -8 5 stores, index 9 is fusions
    println!("{}",input_vector.len());
    let mut store_items:Vec<Vec<String>> = Vec::new();

    for i in 4..9 {
        store_items.push(get_vector_items(input_vector[i].clone()))
    }

    return store_items;
}
