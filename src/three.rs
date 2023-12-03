use std::fs::File;
use std::io::prelude::*;

pub fn main() -> std::io::Result<()> {
    let mut file = File::open("three_input.txt")?;
    // Read the contents of the file into a String
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines: Vec<&str> = contents.lines().collect();
    let mut i = 0;
    let mut valid_sum=0;
    while i < lines.len() {
        let line = lines[i];
        
        let mut j = 0;
        while j < line.len() {
            // println!("{}", j);
            let c = line.chars().nth(j).unwrap();
            let mut num = 0;
            if is_number(c) {
                let mut foundsymbol = symbol(&lines, i as i32 - 1, j as i32 - 1)
                    || symbol(&lines, i as i32 - 1, j as i32)
                    || symbol(&lines, i as i32, j as i32 - 1)
                    || symbol(&lines, i as i32 + 1, j as i32 - 1)
                    || symbol(&lines, i as i32 + 1, j as i32);
                while j < line.len() && is_number(line.chars().nth(j).unwrap()) {
                    foundsymbol = foundsymbol || symbol(&lines, i as i32 - 1, j as i32);
                    foundsymbol = foundsymbol || symbol(&lines, i as i32 + 1, j as i32);

                    num = num * 10 + line.chars().nth(j).unwrap().to_digit(10).unwrap() as i32;
                    j += 1;
                }
                foundsymbol = foundsymbol
                    || symbol(&lines, i as i32 - 1, j as i32 )
                    || symbol(&lines, i as i32 - 1, j as i32-1)
                    || symbol(&lines, i as i32, j as i32 )
                    || symbol(&lines, i as i32 + 1, j as i32 )
                    || symbol(&lines, i as i32 + 1, j as i32-1);

                if foundsymbol{
                    valid_sum+=num;
                }
                // println!("{} {}", num, foundsymbol);
            }
            j+=1;
        }
        i += 1;
    }
    println!("{}", valid_sum);
    Ok(())
}

fn is_symbol(c: char) -> bool {
    !is_number(c) && c != '.' && c != ' ' && c != '\n'
}
fn is_number(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn symbol(lines: &Vec<&str>, i: i32, j: i32) -> bool {
    // println!("{} {}", i, j);
    if i < 0 || j < 0 || i >= lines.len() as i32 || j >= lines[i as usize].len() as i32  {
        return false;
    }
    // println!("{} {} {}", i, j,lines[i as usize].chars().nth(j as usize).unwrap());
    return is_symbol(lines[i as usize].chars().nth(j as usize).unwrap());
}
