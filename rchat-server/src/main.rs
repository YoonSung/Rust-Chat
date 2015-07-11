// --------------------------------------------------------------------------------------- Common
extern crate bincode;
extern crate rustc_serialize;

#[derive(Debug, RustcEncodable, RustcDecodable)]
enum MessageType {
    Chat,
    System,
    Id
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
struct Protocol {
    len : u32,
    message_type : MessageType,
    group_id : u64,
    body : String
}

fn from_protocol(protocol : Protocol) -> Vec<u8> {
    bincode::encode(&protocol, bincode::SizeLimit::Infinite).unwrap()
}

fn to_protocol(stream_data : &[u8]) -> Protocol {
    bincode::decode(&stream_data).unwrap()
}

// --------------------------------------------------------------------------------------- Server

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::collections::HashMap;

struct Database {
    group : HashMap<u64, Group>,
    user : HashMap<u8, User> 
}

struct Group {
    name : String,
    users : Vec<User>
}

struct User {
    name : String,
    id : u64
}

fn handle_client(stream: TcpStream) {
    //println!("{}", stream.peer_addr().unwrap());
    //println!("client in!!");
    let buffer = [0; 128];
    
    stream.read(&mut buffer); // ignore here too
    
    println!("{:?}", buffer);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    // accept connections and process them, 64spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => { /* connection failed */ }
        }
    }
    
    // close the socket server
    // drop(listener);
}


// --------------------------------------------------------------------------------------- Client
use std::io::prelude::*;
//use std::net::TcpStream;


//#[test]
fn client() {
    let mut stream = TcpStream::connect("127.0.0.1:9000").unwrap();

    let protocol = Protocol (len = 33,
                        message_type = MessageType::Chat,
                        group_id = 123123,
                        body = "testest".to_string());

    // ignore the Result
    //let _ = stream.write(&[1]);
    let _ = stream.write(&protocol);
    let _ = stream.read(&mut [0; 128]); // ignore here too

    println!("Hello, world!");
}


use std::string::String;
#[test]
fn test1() {
    let string = "foo";
    println!("{:?}", string.as_bytes());
}