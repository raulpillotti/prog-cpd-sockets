use std::fs::File;
use std::io::{Read, Write, BufReader};
use std::net::TcpStream;
use std::thread;

const BUFFER_SIZE: usize = 8192;

pub struct Tcp {
    pub server_thread: thread::JoinHandle<()>,
}

impl Tcp {
    pub fn new(file_path: &str, server_addr: &str) -> Self {
        let file_path = file_path.to_string();
        let server_addr = server_addr.to_string();

        let handle = thread::spawn(move || {
            let mut file = File::create(&file_path).expect("Erro ao abrir arquivo");
            let mut buffer = [0u8; BUFFER_SIZE];
            let mut stream = TcpStream::connect(&server_addr).expect("Erro ao conectar no servidor");

            loop {
                let bytes = match stream.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => {
                        if n < BUFFER_SIZE {
                            let resp = String::from_utf8_lossy(&buffer[..n]);
                            for line in resp.lines() {
                                if line.contains("Resp") {
                                    println!("Resposta do servidor: {line}");
                                }
                            }
                        }
                        n
                    },
                    Err(e) => {
                        eprintln!("Erro lendo arquivo: {}", e);
                        return;
                    }
                };
                file.write_all(&buffer[..bytes]).expect("Erro ao enviar dados");
            }
        });

        Tcp { server_thread: handle }
    }
}
