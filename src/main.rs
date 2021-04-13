use std::io::{Error, BufReader, BufRead};
use std::ops::Range;

fn printer(buffer:&mut BufReader<std::fs::File>) -> Result<(), std::io::Error> {
    let mut string = String::new();
    // let mut size;
    // let comulative_size;
    while 0 < buffer.read_line(&mut string)?{
        print!("{}", string);
        string.clear();
    }
    eprintln!();
    Ok(())
}

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    // args.remove(0);
    dbg!(&args);
    let mut arg;
    for i in 1..args.len() {
        arg = &args[i];
        match std::fs::File::open(&arg) {
            Ok(file) => {
                println!("file \"{}\" opened", arg);
                let mut buffer = BufReader::new(file);
                match printer(&mut buffer){
                    Ok(_) => {}
                    Err(_) => {eprintln!("Not an UTF-8 text file.")}
                }
            }
            Err(_) => {}
        }
    }
}