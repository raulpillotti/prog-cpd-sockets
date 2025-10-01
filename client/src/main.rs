mod tcp;
mod udp;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mode = args.get(1).expect("Argumento inválido").as_str();
    let file_path = args.get(2).expect("Argumento inválido");
    let server_addr = args.get(3).expect("Argumento inválido");

    match mode {
        "tcp" => {
            let tcp = tcp::Tcp::new(file_path, server_addr);
            let _ = tcp.server_thread.join();
        },
        "udp" => {
            let client_addr = args.get(4).expect("Argumento inválido");
            let udp = udp::Udp::new(file_path, server_addr, client_addr);
            let _ = udp.server_thread.join();
        }
        _ => panic!("Argumento inválido")
    }
}
