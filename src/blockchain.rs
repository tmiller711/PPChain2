use std::io::{prelude::*, self, Read, Write};
use crate::nodes::{return_nodes};
use std::net::TcpStream;
use walkdir::WalkDir;
use std::fs::File;
use std::fs;
use std::{thread, time::Duration};

pub fn blockchain_startup(mut stream: TcpStream) {
    let blockheight = WalkDir::new("blocks").into_iter().count();
    let msg = format!("current height : {}", blockheight - 1);

    stream.write(msg.as_bytes()).unwrap();
    stream.flush().unwrap();

    // receive files
    loop {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let text = String::from_utf8_lossy(&buffer[..]);
        let text = text.trim_end_matches("\0");

        if text == "!DICONNECT" {
            break;
        }

        // handle creating files and populating them
        if text.contains("file name : ") {
            let path = format!("blocks/{}",text.trim_start_matches("file name : "));
            let mut file = File::create(path).unwrap();

            let mut buffer = [0; 10000];
            stream.read(&mut buffer).unwrap();
            let block_data = String::from_utf8_lossy(&buffer[..]);
            let block_data = block_data.trim_end_matches("\0");
            file.write_all(block_data.as_bytes()).unwrap();
        }

        if text != "" {
            println!("{:?}", text);
        }
    }
}

pub fn send_blocks(mut stream: TcpStream, start_block: i32) {
    // send all the blocks starting at 'start_block' and ending at the most current one
    for file in fs::read_dir("blocks").unwrap() {
        let file = file.unwrap().path().display().to_string();
        let block_num = file.trim_start_matches("blocks\\").trim_end_matches(".txt").parse::<i32>().unwrap();
        
        if block_num >= start_block {
            // send file name
            let block_id = format!("file name : {}.txt", block_num);
            println!("Sending {:?}", block_id);
            stream.write(block_id.as_bytes()).unwrap();
            stream.flush().unwrap();
            thread::sleep(Duration::from_millis(500));

            // send contents of file
            let mut block_file = File::open(file).unwrap();
            std::io::copy(&mut block_file, &mut stream).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    }
    stream.write(b"!DICONNECT").unwrap();
    stream.flush().unwrap();
}