use std::io::Write;
use std::net::UdpSocket;
use std::thread;

const BUFFER_SIZE: usize = 8192;

pub struct Udp {
    pub server_thread: thread::JoinHandle<()>,
}

impl Udp {
    pub fn new(file_path: &str, server_addr: &str, client_addr: &str) -> Self {
        let server_addr = server_addr.to_string();
        let client_addr = client_addr.to_string();
        let file_path = file_path.to_string();

        let handle = thread::spawn(move || {
            let socket = UdpSocket::bind(client_addr).expect("Erro ao criar socket UDP do cliente");
            socket.connect(server_addr).expect("Erro ao conectar no servidor");
            socket.send(b"START").expect("Erro ao enviar handshake");

            let mut file = std::fs::File::create(file_path).expect("Erro ao criar arquivo");
            let mut buffer = [0u8; BUFFER_SIZE];

            loop {
                let bytes_received = socket.recv(&mut buffer).expect("Erro ao receber pacote");
                if bytes_received < BUFFER_SIZE {
                    let resp = String::from_utf8_lossy(&buffer[..bytes_received]);
                    for line in resp.lines() {
                        if line.contains("Resp") {
                            println!("Resposta do servidor: {line}");
                        }
                    }
                }
                file.write_all(&buffer[..bytes_received]).expect("Erro ao escrever arquivo");
            }
        });

        Udp { server_thread: handle }
    }
}
