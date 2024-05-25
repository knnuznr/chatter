use std::{
    io::{self, BufRead, Read, Write},
    net::{TcpListener, TcpStream},
    process::exit,
    thread,
};

pub struct Server {}

impl Server {
    pub fn start_server() {
        let listener = TcpListener::bind("127.0.0.1:7007").expect("Failed to bind to address");
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| {
                        handle_client(stream);
                    });
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}
fn handle_client(mut stream: TcpStream) {
    let mut reader_stream = stream.try_clone().expect("Failed to clone stream");
    let mut buffer = [0; 1024];

    thread::spawn(move || loop {
        let n = reader_stream
            .read(&mut buffer)
            .expect("Failed to read from client");
        if n == 0 {
            println!("Connection closed by client.");
            break;
        }

        println!("Received: {}", String::from_utf8_lossy(&buffer[..n]));
    });

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input = line.expect("Failed to read from console") + "\n";
        if input.trim().is_empty() {
            println!("No input provided. Exiting...");
            exit(1);
        }

        stream
            .write(input.as_bytes())
            .expect("Failed to write to client");
    }
}
