use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 2048]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(_) => {
            // echo everything!
            let me_die = String::from_utf8(data.to_vec())
                .unwrap()
                .to_lowercase()
                .replace('\0', "");
            let mut timyr = me_die.split(' ');
            //me_die.chars().for_each(|igor| println!("{igor}"));

            match (timyr.next().unwrap(), timyr.next()) {
                ("eee", Some(path)) => {
                    log::info!("ε=ε=ε=ε=ε=ε=┌(;￣◇￣)┘");
                    let spisok_failov = std::fs::read_dir(path)
                        .unwrap()
                        .map(|res| res.map(|e| e.path()))
                        .collect::<Result<Vec<_>, std::io::Error>>()
                        .unwrap();

                    let x = serde_json::to_string(&spisok_failov).unwrap();

                    stream.write(x.as_bytes()).ok();
                    true
                }
                ("qqw", None) => {
                    stream.write("bb, lox".as_bytes()).ok();
                    std::process::exit(100)
                }
                _ => true,
            }
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {
        data.fill(0)
    }
}

fn main() {
    pretty_env_logger::init();
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
