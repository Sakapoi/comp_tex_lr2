use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    pretty_env_logger::init();
    let mut stream = TcpStream::connect("localhost:3333").unwrap();

    println!("Successfully connected to server in port 3333");
    println!(
        r#"
    Меню
    
    eee [имя папки откуда беру инфу] - получить файл со списком хуйни
    ff - отключится от сервера
    qqw - завершить работу сервера.
    "#
    );

    loop {
        let mut buffer = String::new();
        print!("$ ");
        let _ = std::io::stdout().flush();
        std::io::stdin().read_line(&mut buffer).unwrap();

        let binding = buffer.replace('\n', "");
        let msg = binding.as_str();
        dbg!(msg);
        if msg == "ff" {
            break;
        }
        stream.write(msg.as_bytes()).unwrap();
        println!("Sent Hello, awaiting reply...");

        let mut data = [0 as u8; 2048]; // using 6 byte buffer
                                        //let mut data = vec![];

        match stream.read(&mut data) {
            Ok(_) => {
                //let text = from_utf8(&data).unwrap();
                let data = data
                    .to_vec()
                    .iter()
                    .filter(|&&cheto| cheto != 0)
                    .map(|cheto| cheto.to_owned())
                    .collect();
                let x = String::from_utf8(data).unwrap();
                dbg!(&x);
                let text: Vec<String> = serde_json::from_str(&x.trim()).unwrap();
                let mut file = std::fs::File::create("./result.txt").unwrap();
                text.iter().for_each(|chenibyd| {
                    file.write_fmt(format_args!("{chenibyd}\n")).unwrap();
                });
                println!("Unexpected reply: {:?}", text);
            }
            Err(e) => {
                println!("Failed to receive data: {}", e);
            }
        }
    }
    println!("Terminated.");
}
