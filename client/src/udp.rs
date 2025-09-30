use std::io::Write;
use std::net::UdpSocket;
use std::thread;
use std::time::Instant;

const BUFFER_SIZE: usize = 8192;

pub struct Udp {
    pub server_thread: thread::JoinHandle<()>,
}

impl Udp {
    pub fn new() -> Self {
        let handle = thread::spawn(move || {
            let socket = UdpSocket::bind("127.0.0.1:8084").expect("Erro ao iniciar servidor UDP");
            println!("Servidor UDP escutando em 127.0.0.1:8084");

            let mut buffer = [0u8; BUFFER_SIZE];
            let mut file = std::fs::File::create("dest.mp4").expect("Falha ao criar arquivo de destino");

            let start = Instant::now();

            loop {
                let (bytes_recv, src_addr) = socket.recv_from(&mut buffer).expect("Erro ao receber pacote UDP");

                if bytes_recv == 3 && &buffer[..3] == b"EOF" {
                    let elapsed = start.elapsed();
                    let msg = format!("{} ms", elapsed.as_millis());
                    let _ = socket.send_to(msg.as_bytes(), src_addr);
                    println!("Arquivo UDP recebido em {} ms", elapsed.as_millis());
                    break;
                }

                file.write_all(&buffer[..bytes_recv])
                    .expect("Erro ao escrever no arquivo");
            }
        });

        Udp { server_thread: handle }
    }
}
