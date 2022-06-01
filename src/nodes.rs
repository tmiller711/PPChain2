use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::{thread, time::Duration};

// only ran on first time startup
pub fn node_bootstrap(mut stream: TcpStream)
{
    stream.write(b"new node : 192.168.0.105:7878").unwrap();
    stream.flush().unwrap();

    loop {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let text = String::from_utf8_lossy(&buffer[..]);
        let text = format!("{}{}", text.trim_end_matches("\0"), "\n");

        if text.trim() == "accepted" {
            println!("breaking out of node bootstrap loop");
            break;
        }

        let mut nodes_file = File::options().append(true).open("nodes.txt").unwrap();
        nodes_file.write_all(text.trim_start_matches("*node : ").as_bytes()).unwrap();
    }

    // loop through all nodes in nodes.txt and send '*node : local_ip' to them
    let nodes_file = File::open("nodes.txt").unwrap();
    let reader = BufReader::new(nodes_file);
    for line in reader.lines() {
        let line = line.unwrap();
        let msg = format!("*node : {}", "local_ip");
        println!("Sending: {} to {}", msg, line);

        // commented out because there are no other nodes besides the two
        // let mut stream = TcpStream::connect(line).unwrap();
        // thread::sleep(Duration::from_millis(700));
        // stream.write(msg.as_bytes()).unwrap();
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
            let node = format!("*node : {}", line);
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

pub fn check_for_node(node: &str) -> bool {
    let nodes_file = File::open("nodes.txt").unwrap();
    let reader = BufReader::new(nodes_file);
    for line in reader.lines() {
        let line = line.unwrap();
            if line == node {
                return true;
            }
    }
    false
}

pub fn return_nodes() -> Vec<String> {
    let mut node_list = Vec::new();
    let nodes_file = File::open("nodes.txt").unwrap();
    let reader = BufReader::new(nodes_file);
    for line in reader.lines() {
        let line = line.unwrap();
        node_list.push(line);
    }
    node_list
}