use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;




pub fn main() -> std::io::Result<()> {
    let mut file = File::open("three_input.txt")?;
    // Read the contents of the file into a String
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines: Vec<&str> = contents.lines().collect();
    let mut i = 0;
    let mut map:HashMap<String,i32>=HashMap::new();
    let mut freqs:HashMap<String,i32>=HashMap::new();
    while i < lines.len() {
        let line = lines[i];

        let mut j = 0;
        while j < line.len() {
            // println!("{}", j);
            let c = line.chars().nth(j).unwrap();
            let mut num = 0;
            if is_number(c) {
                let mut dupj = j;

                while dupj < line.len() && is_number(line.chars().nth(dupj).unwrap()) {
                    num = num * 10 + line.chars().nth(dupj).unwrap().to_digit(10).unwrap() as i32;
                    dupj += 1;
                }

                symbol(&mut freqs,&mut map,num, &lines, i as i32 - 1, j as i32 - 1);
                symbol(&mut freqs,&mut map,num, &lines, i as i32, j as i32 - 1);
                symbol(&mut freqs,&mut map,num, &lines, i as i32 + 1, j as i32 - 1);
                
                for k in j..dupj {
                    symbol(&mut freqs,&mut map,num, &lines, i as i32 - 1, k as i32);
                    symbol(&mut freqs,&mut map,num, &lines, i as i32 + 1, k as i32);
                }
                j = dupj;
                symbol(&mut freqs,&mut map,num, &lines, i as i32 - 1, j as i32);
                symbol(&mut freqs,&mut map,num, &lines, i as i32, j as i32);
                symbol(&mut freqs,&mut map,num, &lines, i as i32 + 1, j as i32);
                
                // println!("{} {}", num, foundsymbol);
            }
            j += 1;
        }
        i += 1;
    }
    //print map
    let mut sum=0;
    for (key, value) in &map {
        if *freqs.get(key).unwrap() != 2 as i32{
            continue;
        }
        println!("{}: {}", key, value);
        sum+=value;
    }
    println!("sum: {}",sum);
    Ok(())
}

fn is_gear(c: char) -> bool {
    c == '*'
}
fn is_number(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn symbol(freqs:&mut HashMap<String,i32>, map:&mut HashMap<String,i32> ,num: i32, lines: &Vec<&str>, i: i32, j: i32) {
    // println!("{} {} ___ {}", i, j,num);
    if i < 0 || j < 0 || i >= lines.len() as i32 || j >= lines[i as usize].len() as i32 {
        return;
    }
    // println!("{} {} {}", i, j,lines[i as usize].chars().nth(j as usize).unwrap());
    if is_gear(lines[i as usize].chars().nth(j as usize).unwrap()) {
        let key=i.to_string()+"_"+&j.to_string();
        // println!("{} {}",key,num);
        if map.contains_key(&key){
            map.insert(key.clone(),map.get(&key).unwrap()*num); 
            freqs.insert(key.clone(),freqs.get(&key).unwrap()+1);
        }
        else{
            map.insert(key.clone(),num);
            freqs.insert(key.clone(),1);
        }
    }
}
