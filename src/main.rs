use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::env;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let response = match request.lines().next().unwrap() {
        "GET / HTTP/1.1" => "HTTP/1.1 200 OK\r\n\r\nHello",
        "GET /crash HTTP/1.1" => "HTTP/1.1 200 OK\r\n\r\nCrash",      
        _ => "HTTP/1.1 404 NOT FOUND\r\n\r\n",
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    if response.contains("Crash") {
        stream.shutdown(std::net::Shutdown::Both).unwrap();
        println!("Serveur arrêté par la route /crash");
        std::process::exit(0);
    }
            
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let port = if args.len() > 1 {
        args[1].parse().unwrap_or(8080)
    } else {
        8080
    };

    let address = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&address).unwrap();
    println!("Serveur en écoute sur http://{}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Erreur de connexion: {}", e);
            }
        }
    }
}
