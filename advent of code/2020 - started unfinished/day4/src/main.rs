use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

const REQUIRED_FIELDS : & [& str] = &["byr","iyr","eyr","hgt","hcl","ecl","pid"];
const POSSIBLE_EYE_COLOR : & [&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let passport_vector:Vec<&str> =input.split("\n\n").collect();
    let mut count = 0;

    for i in 0..passport_vector.len(){
        let entry= passport_vector[i].replace("\n"," ");
        let entries:HashMap<&str, &str>   =entry.split(" ").map(|s| s.split_at(s.find(":").unwrap())).map(|(key, val)| (key, &val[1..])).collect();
        if REQUIRED_FIELDS.iter().all(|item| entries.contains_key(item)) && entries.iter().all(|(key, value)| check_if_correct(key , value)){
               count = count +1;
        }  //entries.iter().all(|(key, value)| check_if_correct(key , value));
        //let entries:HashSet<_>=entry.split(" ").map(|s| &s[0..3]).collect::<HashSet<_>>();
        //if REQUIRED_FIELDS.iter().all(|item| entries.contains(item)){
        //    count = count +1;
        //}
    }

    println!("{}",count);
}

fn check_if_correct(key:&str, value:&str) -> bool{
    match key {
        "byr" => return value.parse::<isize>().unwrap() - 1920 <= 82,
        "iyr" => return value.parse::<isize>().unwrap() - 2010 <= 10,
        "eyr" => return value.parse::<isize>().unwrap() - 2020 <= 10,
        "cid" => return true,
        "pid" => value.len() == 9 ,//&& value.parse::<usize>().unwrap(),
        "hgt" => if value[value.len()-2..].eq("cm") {
                    println!("1");
                    return value[..value.len()-2].parse::<isize>().unwrap() - 150 <= 43
        }        else if value[value.len()-2..].eq("in") {
                    println!("2");
                    return value[..value.len()-2].parse::<isize>().unwrap() - 49 <= 27
        }        else {
                    return false;
                    //panic!("not correct height measure")
        },
        "ecl" => POSSIBLE_EYE_COLOR.iter().filter(|eye_color| eye_color.to_owned()[0..].eq(value) ).count()==1,
        "hcl" => value.to_owned()[0..1].eq("#") && value.len()==7,
        _ => panic!("wrong key"),
    }
}