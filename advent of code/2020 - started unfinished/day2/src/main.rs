use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

struct State {
    a: i32,
    b: i32,
    result_add: i32,
    result_subtract: i32
}

fn parse_line(line:String) -> (usize,usize, String, String) {
    let mut split = line.split_whitespace();
    let mut rule = split.next().unwrap().split('-');
    let begin = rule.next().unwrap().parse::<usize>().unwrap();
    let end = rule.next().unwrap().parse::<usize>().unwrap();
    let letter = split.next().unwrap()[0..1].to_owned(); 
    let password = split.next().unwrap().to_owned();

    return (begin,end,letter,password);
}

fn is_proper_pass2(line:String) -> bool {
    let (begin,end,letter,password)=parse_line(line);
    return password[begin-1..begin].eq(&letter) && !password[end-1..end].eq(&letter) || !password[begin-1..begin].eq(&letter) && password[end-1..end].eq(&letter) ;

}

fn is_proper_pass(line:String) -> bool {
    let (begin,end,letter,password)=parse_line(line);
    let mut count:usize = 0;

    for i in 0..password.len(){
        if password[i..i+1].eq(&letter) {
            count=count+1;
        }
    }

    return count>=begin && count<=end;
}


fn get_correct_passwords(vector:Vec<String>) -> usize{
    let mut count =0;
    for i in 0..vector.len(){
        if is_proper_pass2(vector[i].clone()){
            count=count+1;
        }
    }
    return count;
}



fn main() {
    let file = File::open("input.txt");
    let buf_reader = BufReader::new(file.unwrap());
    let vector:Vec<String>=buf_reader.lines().collect::<Result<_,_>>().unwrap();
    let i=get_correct_passwords(vector);
    print!("{}",i);

    //buf_reader.lines().next()

    


}
