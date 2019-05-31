use std::io::prelude::*;
use std::fs::File;

extern crate rand;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file_path;
    if args.len() <= 1 {
        file_path = "list.txt";
    } else {
        file_path = args[1].as_str();
    }

    let items_per_row: u32;
    if args.len() <= 2 {
        items_per_row = 2;
    } else {
        let asked: u32 = args[2].as_str().parse().unwrap();

        if asked <= 0 {
            items_per_row = 2;
        } else {
            items_per_row = asked;
        }
    }

    let generation_limit: u32;
    if args.len() <= 3 {
        generation_limit = 10;
    } else {
        let asked: u32 = args[3].as_str().parse().unwrap();

        if asked <= 0 {
            generation_limit = 10;
        } else {
            generation_limit = asked;
        }
    }

    println!("Cheetah Vortex 1.1.0");
    println!("Source file : {}\n", file_path);

    let mut raw_list = File::open(file_path)
        .expect(&format!("Cannot open the source file {}", file_path));
    let mut raws_buffer: String = String::new();
    raw_list.read_to_string(&mut raws_buffer)
        .expect(&format!("Cannot read the source file {}", file_path));
    let raws = raws_buffer.as_str();

    let list: Vec<&str> = raws
        .split("\n")
        .map(|v| { return v.trim(); })
        .filter(|v| {
            if !v.is_empty() {
                if !v.starts_with("#") {
                    return true;
                }
            }

            return false;
        })
        .collect();

    if list.len() <= 0 {
        panic!("The file {} should not be empty", file_path);
    }

    for _ in 0..generation_limit {
        let mut name: String = "".to_string();
        for _ in 0..items_per_row {
            let id: f64 = rand::random();
            let id: usize = (id * list.len() as f64) as usize;

            if name != "" {
                name.push_str(" ");
            }
            name.push_str(list.get(id).unwrap());
        }
        println!("{}", name);
    }
}
