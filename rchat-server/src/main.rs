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

#[allow(unused_variables)]
fn from_protocol(protocol : Protocol) -> Vec<u8> {
    match bincode::encode(&protocol, bincode::SizeLimit::Infinite) {
        Ok(v) => v,
        Err(e) => {
            panic!("protocol encode fail!");
        }
    }
}

#[allow(unused_variables)]
fn to_protocol(stream_data : &[u8]) -> Protocol {
    match bincode::decode(stream_data) {
        Ok(v) => v, 
        Err(e) => {
            panic!("protocol decode fail!");
        }
    }
}

// --------------------------------------------------------------------------------------- Server

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::collections::HashMap;

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

extern crate rand;
use rand::distributions::{IndependentSample, Range};

const TEST_GLOBAL_GROUP_KEY : u64 = 123123123;

#[derive(Debug)]
struct Database {
    groups : HashMap<u64, Group>
}

#[derive(Debug)]
struct Group {
    name : String,
    users : Vec<User>
}

#[derive(Debug)]
struct User {
    name : String,
    id : u8,
    socket : TcpStream,
    groups : Vec<u64>
}

/*
fn convert_to_array <'a, T: AsRef<[u8]>>(s: &'a T) -> &'a [u8] {
   s.as_ref()
}
*/

fn generate_random_user_key() -> u8 {
    let range = Range::new(u8::min_value(), u8::max_value());
    let mut rng = rand::thread_rng();
    range.ind_sample(&mut rng)
}

fn generate_random_group_key() -> u64 {
    let range = Range::new(u64::min_value(), u64::max_value());
    let mut rng = rand::thread_rng();
    range.ind_sample(&mut rng)
}

fn main() {

    //TODO use real db
    
    //Make Group
    let mut group_map : HashMap<u64, Group> = HashMap::new();
    let group : Group = Group {
                            name : "rust_day".to_string(),
                            users : Vec::<User>::new()  
                    };
                
    //Add Initial Group    
    group_map.insert(TEST_GLOBAL_GROUP_KEY, group); 
    
    //Database Setting
    let database  : Database = Database { groups : group_map };                
    let shared_database = Arc::new(database);
    
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    
    loop {
        for stream in listener.incoming() { 
            println!("incoming!!");
            match stream {
                Ok(stream) => {
                    let cloned_stream = stream.try_clone().unwrap();
                    let cloned_database = shared_database.clone();
                    
                    thread::spawn(move|| {    
                        execute(cloned_stream, cloned_database);
                    });    
                },  
                Err(e) => { /* connection failed */ }
            }
        }
    }
        
    // close the socket server
    // drop(listener);
}

fn execute(mut stream : TcpStream, cloned_database : Arc<Database>) {
    //[START] TODO extract from db OR register to db
    let user = User {
            name : "test".to_string(),
            id : generate_random_user_key(),
            socket : stream.try_clone().unwrap(),
            groups : vec![TEST_GLOBAL_GROUP_KEY]
    };    
    
    let users = cloned_database.groups.get(&TEST_GLOBAL_GROUP_KEY);
    //TODO DELETE Test
    //println!("database : {:?}", cloned_database);
    //println!("users : {:?}", users);    
    //[END]
    
    create_stream_reader(stream, cloned_database.clone());
}

fn create_stream_reader(mut stream : TcpStream, cloned_database : Arc<Database>) {
    let reader = thread::spawn(move|| {
        loop {
            let mut buffer : [u8; 1024] = [0; 1024];
            stream.read(&mut buffer);

            let protocol = to_protocol(&buffer);
            
            //println!("protocol : {:?}", prot);
            println!("채팅 : [  {:?}  ]\n", protocol.body);
            
            let group_users = cloned_database.groups.get(&TEST_GLOBAL_GROUP_KEY).unwrap();
            println!("{:?}", group_users);
            
            // Test, Make Eco Server
            let byte_vec = from_protocol(protocol);
            stream.write(&byte_vec);
            stream.flush();
        }
    });
    
    reader.join().unwrap();
}

// --------------------------------------------------------------------------------------- Client
use std::io::prelude::*;
//use std::net::TcpStream;
use std::io;

#[test]
fn client() {
    let mut stream = TcpStream::connect("127.0.0.1:9000").unwrap();
    let mut reader_stream = stream.try_clone().unwrap();

    let mut user_nickname = String::new();
    
    print!("채팅방에서 사용할 닉네임을 입력해주세요 : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_nickname);
    
    let writer = thread::spawn(move|| {
        loop {
            // Write
            let mut message = String::new();
            print!("{} : ", user_nickname);
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut message);
            
            let protocol = Protocol {
                            len : 33,
                            message_type : MessageType::Chat,
                            group_id : 123123,
                            body : message};
    
            let byte_vec = from_protocol(protocol);
            stream.write(&byte_vec);
            stream.flush().unwrap();
        }
    });
    
    
    let reader = thread::spawn(move|| {
        loop {
            // Read
            let mut buffer : [u8; 1024  ] = [0; 1024];
            reader_stream.read(&mut buffer); // ignore here too
            let protocol = to_protocol(&buffer);
            
            //println!("protocol : {:?}", prot);
            println!("===> {}",protocol.body);
            io::stdout().flush().unwrap();
        }
    });
    
    writer.join().unwrap();
    reader.join().unwrap();   
}