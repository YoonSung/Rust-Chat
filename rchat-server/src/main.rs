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

extern crate rand;
use rand::distributions::{IndependentSample, Range};

struct Database {
    groups : HashMap<u64, Group>,
    users : HashMap<u8, User> 
}

struct Group {
    name : String,
    users : Vec<User>
}

struct User {
    name : String,
    id : u64,
    socket : TcpStream
}

/*
fn convert_to_array <'a, T: AsRef<[u8]>>(s: &'a T) -> &'a [u8] {
   s.as_ref()
}
*/

fn generate_random_user_key() -> u64 {
    let range = Range::new(u64::min_value(), u64::max_value());
    let mut rng = rand::thread_rng();
    
    range.ind_sample(&mut rng)
}

fn main() {

    let group = Group {
        name : "rust_day".to_string(),
        users : Vec::new()  
    };
    
    let database = Database {
        groups : HashMap::new(),
        users : HashMap::new()
    };

    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    // accept connections and process them, 64spawning a new thread for each one
    
    loop {
        for stream in listener.incoming() { 
            println!("incoming!!");
            match stream {
                Ok(mut stream) => {
                
                    let user = User {
                            name : "test".to_string(),
                            id : generate_random_user_key(),
                            socket : stream.try_clone().unwrap()
                    };
                    
                    
                
                    thread::spawn(move|| {
                        let mut buffer : [u8; 1024  ] = [0; 1024];
                        stream.read(&mut buffer); // ignore here too
                        
                        
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