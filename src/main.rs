extern crate getopts;

use std::{env, process};
use getopts::Options;
use std::fs::File;
use std::io::{BufReader, Read, SeekFrom, Seek};
use std::error::Error;

fn main(){
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut options = Options::new();
    options.optopt("n", "", "number of lines", "NUMS");
    options.optflag("h", "", "print help");
    let cmd_args = match options.parse(&args[1..]) {
        Err(why) => panic!("Cannot parse command args :{}", why),
        Ok(p) => p,
    };
    if cmd_args.opt_present("h") {
        print_usage(&program, &options);
    }
    let line_number = if cmd_args.opt_present("n") {
        let str_num = match cmd_args.opt_str("n"){
            None => panic!("specify line number!"),
            Some(num) => num
        };
        match str_num.trim().parse(){
             Err(_) => panic!("specify line number!"),
             Ok(num) => num
         }
    }else{
        10
    };
    let file = match args.last().clone(){
        None => panic!("specify file"),
        Some(file) => file
    };
    tail(file, line_number);
}

fn print_usage(program: &str, options: &Options){
    let brief = format!("Usage: {} [options] FILE", program);
    print!("{}", options.usage(&brief));
    process::exit(0);
}

const BUF_SIZE :usize = 1024;

fn tail(path: &str, count: u64){
    let file = match File::open(path){
        Err (why) => panic!("Cannot open file! :{}", Error::description(&why)),
        Ok(file) => file
    };
    let f_metadata = match file.metadata(){
        Err(why) => panic!("Cannot read file metadata :{}", Error::description(&why)),
        Ok(data) => data
    };
    let f_size = f_metadata.len();
    //println!("file size is {} bytes", f_size);
    if f_size == 0 {
        process::exit(0);
    }
    let mut reader = BufReader::new(file);

    let mut line_count = 0;
    let mut current_pos = f_size - 2;
    let mut buf = [0;BUF_SIZE];
    'outer: loop {
        if current_pos == 0 {
            break;
        }
        match reader.seek(SeekFrom::Start(current_pos)){
            Err(why) => panic!("Cannot move offset! offset:{} cause:{}", current_pos, why),
            Ok(_) => current_pos
        };
        match reader.read(&mut buf){
            Err(why) => panic!("Cannot read offset byte! offset:{} cause:{}", current_pos, why),
            Ok(_) => current_pos
        };
        for i in 0..BUF_SIZE{
            if buf[i] == 0xA {
                line_count += 1;
            }
            //println!("{}", line_count);
            if line_count == count {
                break 'outer;
            }

            current_pos -= 1;
            //println!("{}", current_pos);
            if current_pos <= 0 {
                current_pos = 0;
                break 'outer;
            }
        }
    }
    current_pos += 1;
    match reader.seek(SeekFrom::Start(current_pos)){
        Err(why) => panic!("Cannot read offset byte! offset:{} cause:{}", current_pos, why),
        Ok(_) => current_pos
    };
    let mut buf_str = String::new();
    match reader.read_to_string(&mut buf_str){
        Err(why) => panic!("Cannot read offset byte! offset:{} cause:{}", current_pos, why),
        Ok(_) => current_pos
    };
    print_result(buf_str);
}

fn print_result(disp_str: String){
    print!("{}", disp_str);
}