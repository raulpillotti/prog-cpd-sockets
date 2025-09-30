use std::io::Read;
use std::net::UdpSocket;
use std::thread;
use std::fs::File;
use std::time::Instant;

const BUFFER_SIZE: usize = 8192;

pub struct Udp {
    pub server_thread: thread::JoinHandle<()>,
}

impl Udp {
    pub fn new(file_path: &str, client_addr: &str) -> Self {
        let client_addr = client_addr.to_string();
        let file_path = file_path.to_string();

        let handle = thread::spawn(move || {
            let socket = UdpSocket::bind("0.0.0.0:8084").expect("Erro ao iniciar servidor UDP");
            println!("Servidor UDP escutando em 0.0.0.0:8084");

            let mut file = File::open(&file_path).expect("Falha ao abrir arquivo");
            let mut buffer = [0u8; BUFFER_SIZE];
            let start = Instant::now();

            loop {
                let bytes_read = file.read(&mut buffer).expect("Erro ao ler arquivo");
                if bytes_read == 0 {
                    break;
                }
                socket.send_to(&buffer[..bytes_read], &client_addr).expect("Erro ao enviar pacote UDP");
            }

            socket.send_to(b"EOF", &client_addr).expect("Erro ao enviar EOF");
            let elapsed = start.elapsed();
            println!("Arquivo UDP enviado: {} ms", elapsed.as_millis());
        });

        Udp { server_thread: handle }
    }
}
