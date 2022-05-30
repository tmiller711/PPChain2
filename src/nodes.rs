use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::{thread, time::Duration};

// only ran on first time startup
pub fn find_nodes()
{
    let mut stream = TcpStream::connect("192.168.0.105:7878").unwrap();
    stream.write(b"new node : 192.168.0.21:7878").unwrap();
    stream.flush().unwrap();

    loop {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let text = String::from_utf8_lossy(&buffer[..]);
        let text = format!("{}{}", text.trim_end_matches("\0"), "\n");

        if text.trim() == "accepted" {
            break;
        }

        let mut nodes_file = File::options().append(true).open("nodes.txt").unwrap();
        nodes_file.write_all(text.trim_start_matches("known node : ").as_bytes()).unwrap();
    }
}

pub fn new_node(msg: &str, mut stream: &TcpStream) {
    // check if node is already in file
    let nodes_file = File::open("nodes.txt").unwrap();
    let reader = BufReader::new(nodes_file);
    let mut already_known = false;

    for line in reader.lines() {
        let line = line.unwrap();
        if line == msg {
            already_known = true;
        } else {
            // send all known nodes
            let node = format!("known node : {}", line);
            stream.write(node.as_bytes()).unwrap();
            stream.flush().unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    }

    // if its not already in file add it
    if already_known == false {
        let mut nodes_file = File::options().append(true).open("nodes.txt").unwrap();
        let msg = format!("{}{}", msg, "\n");
        nodes_file.write_all(msg.as_bytes()).unwrap()
    }
}