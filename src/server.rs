use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 3]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(_) => {
            // echo everything!
            let me_die = String::from_utf8(data.to_vec()).unwrap();
            println!(
                "Я Пидорас, {} {}\n{me_die}/qqw",
                String::from_utf8(data.to_vec()).unwrap(),
                format!("{me_die}") == format!("{me_die}")
            );
            println!("cyka, pochemy ne rabotaet nixyя");
            me_die.chars().for_each(|igor| println!("{igor}"));
            match me_die.trim_end() {
                "шо с ебалом?" => println!("NE xyeta"),
                oleg => println!("XYETA, {oleg}"),
            };
            if me_die.to_lowercase() == "qqw".to_string().to_lowercase() {
                println!("Люблю сосать");
                let spisok_failov = std::fs::read_dir(".")
                    .unwrap()
                    .map(|res| res.map(|e| e.path()))
                    .collect::<Result<Vec<_>, std::io::Error>>()
                    .unwrap();

                let x = serde_json::to_string(&spisok_failov).unwrap();

                stream.write(x.as_bytes()).unwrap();
            }
            //println!("ok");
            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
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
