use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::{self, BufRead};  

fn main() {
    let  file =  File::open("input.txt").expect("can't open file");
    let mut buf_reader = BufReader::new(file);
    let years:Vec<usize>=buf_reader.lines().map(|x|x.unwrap().parse::<usize>().unwrap()).collect();
    let mut result:usize=0;
    println!("{:?}", years);

    /*
    for i in 0..years.len(){
        for j in 1..years.len(){
            if years[i]+years[j]==2020 && i!=j {
                result= years[i]*years[j];
                break;
            }
        }
    }

    println!("{}",result);*/

    for i in 0..years.len(){
        for j in 0..years.len(){
            for x in 0..years.len(){
                if years[i]+years[j]+years[x]==2020 && i!=j && j!=x && i!=x {
                    result= years[i]*years[j]*years[x];
                    break;
                }
            }

        }
    }

    println!("{}",result);

}
