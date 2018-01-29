extern crate getopts;

use std::{env, process};
use getopts::Options;
use std::fs::File;
use std::io::{BufReader, Read, BufRead, SeekFrom, Seek, stdin};
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
    if cmd_args.free.is_empty(){
        tail_stdin(line_number);
    }else{
        let file = cmd_args.free[0].clone();
        tail(&file, line_number);
    }
}

fn print_usage(program: &str, options: &Options){
    let brief = format!("Usage: {} [options] FILE", program);
    print!("{}", options.usage(&brief));
    process::exit(0);
}

fn tail_stdin(count: u64){
    let stdin = stdin();
    let mut line_strs :Vec<String> = Vec::new();
    for line in stdin.lock().lines(){
        line_strs.push(match line{
            Err (why) => panic!("Cannot read strin! cause:{}", Error::description(&why)),
            Ok(l) => l
        });
    }
    let mut result = String::new();
    let end_line = line_strs.len() as u64;
    let start_line = if (end_line) > count {
                        end_line - count
                     }else{
                        0
                     };
    for n in start_line..end_line {
        result += &line_strs[n as usize][..];
        result += "\n";
    }
    print_result(result);
}

const BUF_SIZE :usize = 1024;

fn tail(path: &String, count: u64){
    let file = match File::open(path){
        Err (why) => panic!("Cannot open file! file:{} cause:{}", path, Error::description(&why)),
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
    let mut read_start = if (f_size -2) > BUF_SIZE as u64 {
                            f_size - 2 - BUF_SIZE as u64
                         }else{
                            0
                         };
    let mut buf = [0;BUF_SIZE];
    'outer: loop {
        match reader.seek(SeekFrom::Start(read_start)){
            Err(why) => panic!("Cannot move offset! offset:{} cause:{}", current_pos, why),
            Ok(_) => current_pos
        };
        let b = match reader.read(&mut buf){
            Err(why) => panic!("Cannot read offset byte! offset:{} cause:{}", current_pos, why),
            Ok(b) => b
        };
        for i in 0..b{
            if buf[b-(i+1)] == 0xA {
                line_count += 1;
            }
           // println!("{}, {}", line_count, i);
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
        read_start = if read_start > BUF_SIZE as u64 {
                        read_start - BUF_SIZE as u64
                     }else{
                        0
                     }
    }
    //println!("last pos :{}", current_pos);
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