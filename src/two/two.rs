
use std::fs::File;
use std::cmp;
use std::io::prelude::*;
fn main()->std::io::Result<()>{

    let red=12;
    let blue=14;
    let green=13;

    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.lines();

    let mut sum=0;
    for line in lines{
        sum+=is_possible(line.to_string()+",", red, blue, green);
    }
    println!("{}", sum);
    
    Ok(())
}

//returns the game id if possible else returns 0
fn is_possible(line:String, red:i32, blue:i32, green:i32)->i32{
    let tokens=line.split(" ").collect::<Vec<&str>>();
    let id=tokens[1][0..tokens[1].len()-1].parse::<i32>().unwrap();
    let mut max_red=0;
    let mut max_blue=0;
    let mut max_green=0;
    let mut i=2;
    while true{
        if i>=tokens.len(){
            break;
        }
        let qty=tokens[i].parse::<i32>().unwrap();
        let color=tokens[i+1][0..tokens[i+1].len()-1].to_string();
        
        if color.starts_with("red"){
            // if qty>red{
            //     return 0;
            // }
            max_red=cmp::max(max_red, qty);

        }else if color.starts_with("blue"){
            // if qty>blue{
            //     return 0;
            // }
            max_blue=cmp::max(max_blue, qty);
        }else if color.starts_with("green"){
            // if qty>green{
            //     return 0;
            // }
            max_green=cmp::max(max_green, qty);
        }
        i+=2;
    }
    println!("{:?}", tokens);
    return max_red*max_blue*max_green;
}