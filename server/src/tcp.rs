use std::net::TcpListener;
use std::io::{Read, Write};
use std::thread;
use std::fs::File;
use std::time::Instant;

const BUFFER_SIZE: usize = 8192;

pub struct Tcp {
    pub server_thread: thread::JoinHandle<()>,
}

impl Tcp {
    pub fn new(file_path: &str) -> Self {
        let file_path = file_path.to_string();
        let handle = thread::spawn(move || {
            let listener = TcpListener::bind("127.0.0.1:8084").expect("Falha ao iniciar servidor TCP");
            println!("Servidor TCP escutando em 127.0.0.1:8084");

            loop {
                if let Ok((mut client_socket, addr)) = listener.accept() {
                    println!("Cliente conectado: {}", addr);

                    let mut file = File::open(&file_path).expect("Falha ao abrir arquivo");
                    let mut buffer = [0u8; BUFFER_SIZE];

                    let start = Instant::now();

                    loop {
                        let bytes_read = file.read(&mut buffer).expect("Erro ao ler arquivo");
                        if bytes_read == 0 {
                            break;
                        }
                        client_socket.write_all(&buffer[..bytes_read]).expect("Erro ao enviar arquivo");
                    }

                    let elapsed = start.elapsed();
                    let msg = format!("\nResp: {:?} ms", elapsed.as_millis());
                    client_socket.write_all(msg.as_bytes()).expect("erro enviando mensagem");
                }
            }
        });

        Tcp { server_thread: handle }
    }
}
