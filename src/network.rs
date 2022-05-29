use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::fs::File;

// run name is due to change. Possibly make it main but IDK
pub fn run()
{
    let listener = TcpListener::bind("192.168.0.21:7878").unwrap();

    // if it's the first time booting up in a while
    findnodes();
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn findnodes()
{
    // send "new node : IP" to a node and have them send back all they nodes they know
    // have it connect to a seed node (probably my main computer)
    let mut stream = TcpStream::connect("192.168.0.105:7878").unwrap();
    stream.write(b"new node : 192.168.0.21:7878").unwrap();
    stream.flush().unwrap();
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let msg = String::from_utf8_lossy(&buffer[..]);
    let msg = msg.trim_end_matches("\0");

    println!("{msg}");

    // do if statements to see what the msg is and see what the network should do in response
    if msg.contains("new node") {
        newnode(msg.trim_start_matches("new node : "), &stream);
        stream.write(b"accepted").unwrap();
        stream.flush().unwrap();
    }
    if msg.contains("node") {
        let mut nodesfile = File::options().append(true).open("nodes.txt").unwrap();
        nodesfile.write_all(msg.trim_start_matches("node : ").as_bytes()).unwrap()
    }
} 

fn newnode(msg: &str, mut stream: &TcpStream) {
    // check if node is already in file
    let nodesfile = File::open("nodes.txt").unwrap();
    let reader = BufReader::new(nodesfile);
    let mut alreadyknown = false;

    for line in reader.lines() {
        let line = line.unwrap();
        if line == msg {
            println!("alerady here nigga");
            alreadyknown = true;
        } else {
            // send all known nodes
            let node = format!("node : {}", line);
            stream.write(node.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }

    // if its not already in file add it
    if alreadyknown == false {
        let mut nodesfile = File::options().append(true).open("nodes.txt").unwrap();
        let msg = format!("{}{}", msg, "\n");
        nodesfile.write_all(msg.as_bytes()).unwrap()
    }
}