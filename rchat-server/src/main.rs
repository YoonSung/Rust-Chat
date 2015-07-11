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
    match bincode::decode(stream_data) {
        Ok(v) => {
            println!("{:?}", v);
            v
        }, 
        Err(e) => {
            println!("{:?}", e);
            panic!();
        }
    }
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

/*
fn convert_to_array <'a, T: AsRef<[u8]>>(s: &'a T) -> &'a [u8] {
   s.as_ref()
}
*/

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    // accept connections and process them, 64spawning a new thread for each one
    
    loop {
        for stream in listener.incoming() {
            println!("incoming!!");
            match stream {
                Ok(mut stream) => {
                    thread::spawn(move|| {
                        let mut buffer : [u8; 1024] = [0; 1024];
                        stream.read(&mut buffer); // ignore here too
                        
                        //let prot : Protocol = bincode::decode(&buffer).unwrap();
                        let prot = to_protocol(&buffer);
                        
                        println!("protocol : {:?}", prot);
                    });
                }
                Err(e) => { /* connection failed */ }
            }
        }
    }
    
    
    // close the socket server
    // drop(listener);
}


// --------------------------------------------------------------------------------------- Client
use std::io::prelude::*;
//use std::net::TcpStream;

#[test]
fn client() {
    let mut stream = TcpStream::connect("127.0.0.1:9000").unwrap();

    let protocol = Protocol {len : 33,
                        message_type : MessageType::Chat,
                        group_id : 123123,
                        body : "testestasdasdfasdf".to_string()};

    let byte_vec = from_protocol(protocol);
    stream.write(&byte_vec);
    stream.flush();
}

use std::string::String;
//#[test]
fn test1() {
    let string = "foo";
    println!("{:?}", string.as_bytes());
}

//#[test]
fn test2() {
    let s1 = vec![1,2,3];
    is_hello(&s1);
}

fn is_array(arr : String) {
    print!("Success!");
}

//fn is_hello<'a, T: AsRef<[u8]>>(s: T) -> &'a [u8] {
fn is_hello<'a, T: AsRef<[u8]>>(s: &'a T) -> &'a [u8] {
   //println!("{:?}",s.as_ref());
   s.as_ref()
}