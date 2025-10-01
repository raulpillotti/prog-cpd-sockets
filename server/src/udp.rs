use std::io::Read;
use std::net::UdpSocket;
use std::thread;
use std::fs::File;
use std::time::Instant;

const BUFFER_SIZE: usize = 1024 * 16;

pub struct Udp {
    pub server_thread: thread::JoinHandle<()>,
}

impl Udp {
    pub fn new(file_path: &str, addr: &str) -> Self {
        let addr = addr.to_string();
        let file_path = file_path.to_string();

        let handle = thread::spawn(move || {
            let socket = UdpSocket::bind(&addr).expect("Erro ao iniciar servidor UDP");
            println!("Servidor UDP escutando em {addr}");

            let mut file = File::open(&file_path).expect("Falha ao abrir arquivo");
            let mut buffer = [0u8; BUFFER_SIZE];
            let (_bytes, client_addr) = socket.recv_from(&mut buffer).expect("Erro recebendo handshake");
            println!("Cliente conectado: {}", client_addr);

            let start = Instant::now();
            loop {
                let bytes_read = file.read(&mut buffer).expect("Erro ao ler arquivo");
                if bytes_read == 0 {
                    break;
                }
                socket.send_to(&buffer[..bytes_read], &client_addr).expect("Erro ao enviar pacote UDP");
            }
            let elapsed = start.elapsed();
            let msg = format!("\nResp: {:?} ms", elapsed.as_millis());
            socket.send_to(msg.as_bytes(), &client_addr).expect("Erro ao enviar mensagem final");
            println!("Arquivo UDP enviado: {} ms", elapsed.as_millis());
        });

        Udp { server_thread: handle }
    }
}
