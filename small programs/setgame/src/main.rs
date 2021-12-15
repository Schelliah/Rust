#![allow(dead_code)]

#[derive(PartialEq)]
enum Colors{
    Red,
    Green,
    Blue
}

#[derive(PartialEq)]
enum Shapes {
    Diamond,
    Circle,
    Tilde
}

#[derive(PartialEq)]
enum Filled{
    Completely,
    Hatched,
    Empty
}

struct Card{
    color : Colors,
    shape: Shapes,
    fill: Filled,
    amount: usize
}

impl Card {
    fn get_amount(&self) -> usize{
        return self.amount;
    }
}

fn has_similar_color(card:&Card, other_card:&Card)->bool{
    return card.color == other_card.color;
}

fn has_similar_shape(card:&Card, other_card:&Card)->bool{
    return card.shape == other_card.shape;
}

fn has_similar_fill(card:&Card, other_card:&Card)->bool{
    return card.fill == other_card.fill;
}

fn has_similar_amount(card:&Card, other_card:&Card)->bool{
    return card.amount == other_card.amount;
}

fn is_all_equal_or_different_attribute(card1: &Card, card2: &Card, card3: &Card, f: fn(&Card,&Card)->bool) -> bool{
    let check1 = f(card1,card2); 
    let check2= f(card2,card3);
    if check1 && check2{
        return true;
    } else if !check1 && !check2{
        return true;
    } else {
        return false;
    }
}

fn possible_set(card1: &Card, card2: &Card, card3: &Card) -> bool{
    let color_check = is_all_equal_or_different_attribute(card1,card2,card3,has_similar_color);
    let shape_check = is_all_equal_or_different_attribute(card1,card2,card3,has_similar_shape);
    let fill_check = is_all_equal_or_different_attribute(card1,card2,card3,has_similar_fill);
    let amount_check = is_all_equal_or_different_attribute(card1,card2,card3,has_similar_amount);
    return color_check && amount_check && shape_check && fill_check;
}

fn possible_set_faster_maybe_sometimes(card1: &Card, card2: &Card, card3: &Card) -> bool{
    if !is_all_equal_or_different_attribute(card1,card2,card3,has_similar_color) {return false};
    if !is_all_equal_or_different_attribute(card1,card2,card3,has_similar_shape) {return false};
    if !is_all_equal_or_different_attribute(card1,card2,card3,has_similar_fill) {return false};
    return is_all_equal_or_different_attribute(card1,card2,card3,has_similar_amount);
}

fn main() {
    println!("Hello, world!");
    let card1 = Card {color:Colors::Red, shape:Shapes::Diamond, fill:Filled::Hatched, amount:2 };
    let card2 = Card {color:Colors::Green, shape:Shapes::Diamond, fill:Filled::Hatched, amount:3 };
    let card3 = Card {color:Colors::Blue, shape:Shapes::Diamond, fill:Filled::Hatched, amount:1 };
    if possible_set(&card1, &card2, &card3){
        println!("is a set");
    } else {
        println!("not a set")
    }
}
