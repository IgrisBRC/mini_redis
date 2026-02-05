use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::{TcpListener, TcpStream},
};

use mini_redis::MemoryDatabase;

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

fn handle_connection(rstream: TcpStream, db: &mut MemoryDatabase) -> Result<(), String> {
    let mut wstream = BufWriter::new(&rstream);
    let reader = BufReader::new(&rstream);
    let mut reader_lines = reader.lines();

    loop {
        let line = reader_lines
            .next()
            .ok_or("Connection closed by client?")?
            .map_err(|_| "Failed to read line")?;

        let count = if line.starts_with('*') {
            line[1..]
                .trim()
                .parse::<usize>()
                .map_err(|_| "Failed to parse")?
        } else {
            write_to_wstream(&mut wstream, b"-ERR Protocol Error\r\n")?;
            continue;
        };

        let mut count_ledger: i32 = count as i32;

        let term = match validate_and_get_next_term(&mut reader_lines, &mut count_ledger) {
            Ok(t) => t,
            Err(e) => {
                write_to_wstream(&mut wstream, format!("{}\r\n", e).as_bytes())?;
                continue;
            }
        };

        match term.to_uppercase().as_str() {
            "PING" => {
                write_to_wstream(&mut wstream, b"+PONG\r\n")?;
            }
            "GET" => {
                if count != 2 {
                    write_to_wstream(&mut wstream, b"-ERR Protocol Error\r\n")?;
                    cleanup(&mut count_ledger, &mut reader_lines);
                    continue;
                }

                let key = match validate_and_get_next_term(&mut reader_lines, &mut count_ledger) {
                    Ok(t) => t,
                    Err(e) => {
                        write_to_wstream(&mut wstream, format!("{}\r\n", e).as_bytes())?;
                        continue;
                    }
                };

                if let Some(value_bytes) = db.get(&key) {
                    let header = format!("${}\r\n", value_bytes.len());

                    wstream
                        .write_all(header.as_bytes())
                        .map_err(|_| "Failed to write to stream")?;
                    wstream
                        .write_all(value_bytes)
                        .map_err(|_| "Failed to write to stream")?;
                    wstream
                        .write_all(b"\r\n")
                        .map_err(|_| "Failed to write to stream")?;

                    wstream.flush().map_err(|_| "Failed to flush stream")?;
                } else {
                    write_to_wstream(&mut wstream, b"$-1\r\n")?;
                }
            }
            "SET" => {
                if count != 3 {
                    write_to_wstream(&mut wstream, b"-ERR Protocol Error\r\n")?;
                    cleanup(&mut count_ledger, &mut reader_lines);
                    continue;
                }

                let key = match validate_and_get_next_term(&mut reader_lines, &mut count_ledger) {
                    Ok(t) => t,
                    Err(e) => {
                        write_to_wstream(&mut wstream, format!("{}\r\n", e).as_bytes())?;
                        continue;
                    }
                };

                let value = match validate_and_get_next_term(&mut reader_lines, &mut count_ledger) {
                    Ok(t) => t,
                    Err(e) => {
                        wstream
                            .write_all(format!("{}\r\n", e).as_bytes())
                            .map_err(|_| "Failed to write to stream")?;
                        wstream.flush().map_err(|_| "Failed to flush stream")?;

                        continue;
                    }
                };

                db.insert(&key, value.into_bytes());

                write_to_wstream(&mut wstream, b"+OK\r\n")?;
            }
            "COMMAND" => {
                write_to_wstream(&mut wstream, b"*0\r\n")?;
            }

            _ => {
                let err_msg = format!("-ERR Unknown command {}\r\n", term);
                write_to_wstream(&mut wstream, err_msg.as_bytes())?;
            }
        }

        cleanup(&mut count_ledger, &mut reader_lines);
    }
}

fn write_to_wstream(wstream: &mut BufWriter<&TcpStream>, buf: &[u8]) -> Result<(), String> {
    wstream
        .write_all(buf)
        .map_err(|_| "Failed to write to stream")?;
    wstream.flush().map_err(|_| "Failed to flush stream")?;

    Ok(())
}

fn cleanup(count_ledger: &mut i32, reader_lines: &mut std::io::Lines<BufReader<&TcpStream>>) {
    while *count_ledger > 0 {
        let _ = reader_lines.next();
        let _ = reader_lines.next();
        *count_ledger -= 1;
    }
}

fn validate_and_get_next_term(
    reader_lines: &mut std::io::Lines<BufReader<&TcpStream>>,
    count_ledger: &mut i32,
) -> Result<String, String> {
    let line = reader_lines
        .next()
        .ok_or("Connnection closed unexpectedly?")?
        .map_err(|_| "Failed to read line")?;

    let term_length = if line.starts_with('$') {
        line[1..].trim().parse::<usize>().map_err(|_| "Parsing error of some kind idk, don't blame me I just used rust's `parse` function, it just returned an error, okay? And no I am not gonna write my own parse function.")?
    } else {
        let _ = reader_lines
            .next()
            .ok_or("Connnection closed unexpectedly?")?
            .map_err(|_| "Failed to read line")?;
        *count_ledger -= 1;

        Err("-ERR Protocol Error: ")?
    };

    let term = reader_lines
        .next()
        .ok_or("Connnection closed unexpectedly?")?
        .map_err(|_| "Failed to read line")?;

    *count_ledger -= 1;

    if term.len() != term_length as usize {
        Err("-ERR Protocol Error: ")?
    }

    Ok(term)
}
