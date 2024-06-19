use bincode::{deserialize_from, serialize_into};
use clap::Parser;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::{metadata, File};
use std::io::{BufReader, BufWriter};

#[derive(Parser, Debug)]
#[command(version,about,long_about = None)]
struct Args {
    #[clap(short, long)]
    del: Option<usize>,
    #[clap(short, long)]
    // This means that the if_add parameter can be specified on the command line via -i or --if_add.
    todo: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Date {
    words: Vec<String>,
    mark: Vec<bool>,
}

impl Date {
    fn new() -> Date {
        Date { 
            words: Vec::new() ,
            mark: Vec::new(),
        }
    }
}






fn get_emoji() -> String {
    let emoji_ranges = [
        '\u{1F600}'..='\u{1F64F}', // Emoticons
        '\u{1F300}'..='\u{1F5FF}', // Miscellaneous Symbols and Pictographs
        '\u{1F680}'..='\u{1F6FF}', // Transport and Map Symbols
        '\u{1F700}'..='\u{1F77F}', // Alchemical Symbols
        '\u{1F780}'..='\u{1F7FF}', // Geometric Shapes Extended
        '\u{1F800}'..='\u{1F8FF}', // Supplemental Arrows-C
        '\u{1F900}'..='\u{1F9FF}', // Supplemental Symbols and Pictographs
        '\u{1FA00}'..='\u{1FA6F}', // Chess Symbols
        '\u{1FA70}'..='\u{1FAFF}', // Symbols and Pictographs Extended-A
        '\u{1F004}'..='\u{1F0CF}', // Miscellaneous Symbols and Arrows
    ];
    // 创建一个随机数生成器
    let mut rng = rand::thread_rng();

    // 随机选择一个 emoji 范围
    let range = rng.gen_range(0..emoji_ranges.len());
    let emoji_range = &emoji_ranges[range];

    // 在选定的范围内生成一个随机 emoji
    let emoji = rng.gen_range(emoji_range.clone());

    emoji.to_string()
}

fn read_word_list_file() -> Result<Date, Box<dyn std::error::Error>> {
    match metadata("words.bin") {
        Ok(_) => (),
        Err(_) => {
            let mut originlist = Date::new();
            originlist.words.push("to Do".to_string());
            originlist.mark.push(false);
            write_word_list_file(originlist)?;
        }
    }

    let file = File::open("words.bin")?;
    let mut reader = BufReader::new(file);
    let loaded_data: Date = deserialize_from(&mut reader)?;

    Ok(loaded_data)
}

fn write_word_list_file(words: Date) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("words.bin")?;
    let mut writer = BufWriter::new(file);
    serialize_into(&mut writer, &words)?;
    Ok(())
}

fn add_to_do(newtask: String) -> Result<(), Box<dyn std::error::Error>> {
    match read_word_list_file() {
        Ok(loaded_data) => {
            let mut to_do_list = loaded_data;
            to_do_list.words.push(newtask);
            to_do_list.mark.push(false);
            write_word_list_file(to_do_list)?;
        }
        Err(e) => {
            eprintln!("Error reading file:{}", e);
        }
    }
    Ok(())
}

fn del_task(num: usize) -> Result<(), Box<dyn std::error::Error>> {
    match read_word_list_file() {
        Ok(loaded_data) => {
            let mut to_do_list = loaded_data;
            to_do_list.words.remove(num);
            write_word_list_file(to_do_list)?;
        }
        Err(e) => {
            eprintln!("Error reading file:{}", e);
        }
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    if !args.todo.is_none() {
        let task = args.todo.unwrap();
        match add_to_do(task) {
            Ok(_) => println!("Add successfully"),
            Err(e) => eprintln!("Error add task:{}", e),
        }
    }

    if !args.del.is_none() {
        let idx = args.del.unwrap();
        match del_task(idx) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }

    match read_word_list_file() {
        Ok(loaded_data) => {
            let mut idx = 0;
            for i in &loaded_data.words {
                if idx == 0 {
                    println!("{} {}", get_emoji(), i);
                }else {
                    println!("[{}]:{} {}",idx,get_emoji(),i);
                }
                idx += 1;
            }
        }
        Err(e) => {
            eprintln!("Error load :{}", e);
        }
    }
}
