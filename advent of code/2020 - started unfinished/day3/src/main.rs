use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/*
fn count_trees_line(string:String, location:usize,move_forward:usize) -> usize{
    let string =string[location..location+move_forward+1].to_owned();
    let mut count = 0;
    for i in 0..string.len(){
        println!("{}",string);
        if string[i..i+1].eq("#"){
            count = count + 1 ; 
        }
    }
    return count ;
}
*/

fn count_amount_of_trees(vector:&Vec<String>, steps_forward:usize, steps_down:usize) -> usize{
    let mut count =0;
    println!("{}",vector[0].clone().len());
    let mut i =steps_down;
    let mut location = 0;

     while i< vector.len() {
        location = location +steps_forward;
        let string:String = vector[i].clone();
        println!("{}",location);
            if location>=string.len(){
                location = location % string.len();
                //println!("{}",location);
            }
        if string[location..location+1].eq("#"){
            count = count +1; 
        }
        i=i+steps_down;
        //count = count + count_trees_line(vector[i].clone(), 0+i*3,3);
    }


    return count;
}

fn main() {
    let file = File::open("ii.txt");
    let buf_reader = BufReader::new(file.unwrap());
    let vector:Vec<String>  = buf_reader.lines().collect::<Result<_,_>>().unwrap();
    let counter=count_amount_of_trees(&vector,7,1);
    //let counter=count_amount_of_trees(&vector,1,1) * count_amount_of_trees(&vector,3,1) * count_amount_of_trees(&vector,5,1) * count_amount_of_trees(&vector,7,1) * count_amount_of_trees(&vector,1,2);
    println!("outcome: {}",counter);
    




    
}
