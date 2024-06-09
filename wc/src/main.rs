use clap::Parser;

#[derive(Parser)] // requires `derive` feature
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short = 'w')]
    words: bool,

    #[arg(short = 'c')]
    count: bool,

    #[arg(short = 'l')]
    lines: bool,
    
    #[arg(short = 'm')]
    character: bool,
    
    #[arg(default_value = "")]
    filepath:String,
}
use std::fs;
use std::io;
use std::io::BufRead;
// use std::error::Error;

fn main() -> io::Result<()> {
    let mut contents = String::new();
    let args = Cli::parse();
    if args.filepath != "" {
        let file_path = args.filepath.to_string();
        contents = fs::read_to_string(file_path).expect("Should have been able to read the file");    
    } else {
        let mut buffer = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.   
        let mut handle = stdin.lock(); 
        while handle.read_line(&mut buffer).unwrap_or(0) > 0 {
            contents.push_str(&buffer);
            buffer.clear();
        }
        // stdin.read_to_string(&mut buffer)?;
        // println!("{}",buffer);
        // panic!("nooooooo");
        // for line in buffer.lines() {
        //     contents.push_str(line);
        //     contents.push_str("\r\n");
        // }
        //  = buffer.to_string();
    }
    
    
    let mut message = String::new();
    if args.lines {
        message.push_str(&line_count(&contents).to_string());
        message.push_str("\t");
    }
    if args.words {
        message.push_str(&word_count(&contents).to_string());
        message.push_str("\t");
    }
    if args.count {
        message.push_str(&byte_count(&contents).to_string());
        message.push_str("\t");
    }
    if args.character {
        message.push_str(&character_count(&contents).to_string());
        message.push_str("\t");
    }
    if !args.lines && !args.count && !args.words && !args.character {
        message.push_str(&line_count(&contents).to_string());
        message.push_str("\t");
        message.push_str(&word_count(&contents).to_string());
        message.push_str("\t");
        message.push_str(&byte_count(&contents).to_string());
        message.push_str("\t");
    }
    println!("{} {}",message, args.filepath );
    Ok(())
    
}

fn byte_count(file_contents: &String) -> usize {
    file_contents.bytes().len()
}

fn line_count(file_contents: &String) -> usize {
    file_contents.lines().count()
}

fn word_count(file_contents: &String) -> usize {
    file_contents.split_whitespace().count()
}

fn character_count(file_contents: &String) -> usize {
    file_contents.chars().count()
}