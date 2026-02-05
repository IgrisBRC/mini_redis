use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    println!("mini_redis is alive and listening on port 6379");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                handle_connection(s);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let rstream = stream.try_clone().unwrap();
    let reader = BufReader::new(&rstream);
    let mut reader_lines = reader.lines();

    let line = reader_lines.next().unwrap().unwrap();

    let count = if line.starts_with('*') {
        line[1..].trim().parse::<usize>().unwrap()
    } else {
        panic!("😭");
    };

    let line = reader_lines.next().unwrap().unwrap();
    let term_length = if line.starts_with('$') {
        line[1..].trim().parse::<usize>().unwrap()
    } else {
        panic!("😭");
    };

    //Could rename this as `command` but it could also be an identifier like the key or
    //something
    let term = reader_lines.next().unwrap().unwrap();

    if term.len() != term_length as usize {
        panic!("😭");
    }

    match term.as_str() {
        "PING" => {
            stream.write_all(b"+PONG\r\n").unwrap();
        }
        "GET" => {
            //Since we know that there should exactly be 1 term after this one
            //We can write the code accordingly
            
            if count != 2 {
                panic!("😭");
            }

            let line = reader_lines.next().unwrap().unwrap();
            let key_length = if line.starts_with('$') {
                line[1..].trim().parse::<usize>().unwrap()
            } else {
                panic!("😭");
            };

            let key = reader_lines.next().unwrap().unwrap();

            if key.len() != key_length as usize {
                panic!("😭");
            }

            stream.write_all(b"+OK\r\n").unwrap();
        }
        "SET" => {
            //Same for this except there should be 2 more terms

            if count != 3 {
                panic!("😭");
            }

            let line = reader_lines.next().unwrap().unwrap();
            let key_length = if line.starts_with('$') {
                line[1..].trim().parse::<usize>().unwrap()
            } else {
                panic!("😭");
            };

            let key = reader_lines.next().unwrap().unwrap();

            if key.len() != key_length as usize {
                panic!("😭");
            }

            let line = reader_lines.next().unwrap().unwrap();
            let value_length = if line.starts_with('$') {
                line[1..].trim().parse::<usize>().unwrap()
            } else {
                panic!("😭");
            };

            let value = reader_lines.next().unwrap().unwrap();

            if value.len() != value_length as usize {
                panic!("😭");
            }

            stream.write_all(b"+OK\r\n").unwrap();
        }
        _ => {
            // todo
        }
    }

}

fn validate_and_get_next_term(reader_lines: &mut std::io::Lines<BufReader<&TcpStream>>) -> String {

    let line = reader_lines.next().unwrap().unwrap();
    let term_length = if line.starts_with('$') {
        line[1..].trim().parse::<usize>().unwrap()
    } else {
        panic!("😭");
    };

    let term = reader_lines.next().unwrap().unwrap();

    if term.len() != term_length as usize {
        panic!("😭");
    }

    return term;
}
