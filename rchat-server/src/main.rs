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

#[derive(Debug)]
struct Database<'a> {
    //groups : Rc<RefCell<HashMap<u64, Group>>>
    groups : &'a mut HashMap<u64, Group<'a>>
}

#[derive(Debug)]
struct Group<'a> {
    name : String,
    users : &'a mut Vec<User>
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
    //[START] only 1 group. it's code for extension
    
    let mut group_map : HashMap<u64, Group> = HashMap::new();
    //let group_map : Rc<RefCell<HashMap<u64, Group>>> = Rc::new(RefCell::new(HashMap::new()));
    //let mut database : MutexArc<Database> = MutexArc::new(Database { groups : group_map });
    let mut database  : Database = Database { groups : &mut group_map };
    let mut shared_database = Arc::new(database);
    
    
    let global_group_key = generate_random_group_key();
    /*
    let mut group : Group = Group {
                            name : "rust_day".to_string(),
                            users : &mut Vec::new()  
                        };
                        
    shared_database.groups.insert(global_group_key, group);
    //[END] only 1 group. it's code for extension
    */

    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    
    loop {
        for stream in listener.incoming() { 
            println!("incoming!!");
            match stream {
                Ok(stream) => {
                    let cloned_stream = stream.try_clone().unwrap();
                    let cloned_database = shared_database.clone();

                    thread::spawn(move|| {
                        execute(cloned_stream, cloned_database, &global_group_key.clone());
                    });    
                },  
                Err(e) => { /* connection failed */ }
            }
        }
    }
        
    // close the socket server
    // drop(listener);
}

fn execute(mut stream : TcpStream, shared_database : Arc<Database>, global_group_key : &u64) {
    //[START] TODO extract from db OR register to db

    let user = User {
            name : "test".to_string(),
            id : generate_random_user_key(),
            socket : stream.try_clone().unwrap(),
            groups : vec![*global_group_key]
    };    
    
    //shared_database.groups.get(global_group_key).unwrap().users.push(user);
    //let users = shared_database.groups.get(global_group_key).unwrap();
    let users = shared_database.groups.get(global_group_key);
    println!("database : {:?}", shared_database);
    println!("users : {:?}", users);    

    
    //[END]
    
    let reader = thread::spawn(move|| {
        loop {
            let mut buffer : [u8; 1024] = [0; 1024];
            stream.read(&mut buffer);

            let protocol = to_protocol(&buffer);
            
            //println!("protocol : {:?}", prot);
            println!("채팅 : [  {:?}  ]\n", protocol.body);
            
            //let group_users = shared_database.groups.get(&global_group_key).unwrap();
            //println!("{:?}", group_users);
            
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