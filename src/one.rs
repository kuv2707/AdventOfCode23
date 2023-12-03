// use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

pub fn main() -> std::io::Result<()> {
    // Open the file
    let mut file = File::open("one_input.txt")?;
    // Read the contents of the file into a String
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines = contents.lines();

    let mut sum: i32 = 0;

    for lll in lines {
        let mut i = 0;
        let mut nums = Vec::new();
        let a="    ";
        let line_raw=lll.to_string()+a;
        while i < line_raw.len() - 4 {
            let substr = line_raw.get(i..i + 5).unwrap();
            if is_number(line_raw.get(i..i + 1).unwrap()) {
                nums.push(line_raw[i..i + 1].parse::<i32>().unwrap());
                i += 1;
                continue;
            }
            if substr.starts_with("one") {
                nums.push(1);
            } else if substr.starts_with("two") {
                nums.push(2);
            } else if substr.starts_with("three") {
                nums.push(3);
            } else if substr.starts_with("four") {
                nums.push(4);
            } else if substr.starts_with("five") {
                nums.push(5);
            } else if substr.starts_with("six") {
                nums.push(6);
            } else if substr.starts_with("seven") {
                nums.push(7);
            } else if substr.starts_with("eight") {
                nums.push(8);
            } else if substr.starts_with("nine") {
                nums.push(9);
            } else if substr.starts_with("zero") {
                nums.push(0);
            }
            i += 1;
        }
        // println!("{:?}", nums);
        //take first and last digits from nums and make a two digit number
        let first = nums[0];
        let last = nums[nums.len() - 1];
        let summ = first * 10 + last;
        // println!("{}", summ);
        sum=sum+summ;
    }
    println!("--> {}", sum);
    Ok(())
}

fn is_number(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}
