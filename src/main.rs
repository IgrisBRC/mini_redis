use std::net::TcpListener;

use mini_redis::handle_connection::handle_connection;
use mini_redis::memory_database::MemoryDatabase;

fn main() {
    let mut database = MemoryDatabase::new("IgrisDB");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    println!("mini_redis is alive and listening on port 6379");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => match handle_connection(s, &mut database) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e)
                }
            },
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
