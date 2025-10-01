Resultados:

TCP: o arquivo é transferido normalmente e funciona

Buffer 1KB -> ~40 ms  
Buffer 8KB -> ~30 ms  
Buffer 16KB -> ~60 ms

UDP: o arquivo vem corrompido, acho improvável que está dropando pacotes pois estou testando local, mas não vai dar tempo de corrigir se for o caso.

Buffer 1KB -> ~50 ms  
Buffer 8KB -> ~12 ms  
Buffer 16KB -> ~8ms

Utilização:

Para compilar é necessário instalar o compilador do rust: https://rust-lang.org/tools/install/

Para executar

Servidor (/server)

TCP:

cargo run -- tcp {caminho_arquivo_entrada} {endereço}  
ex: cargo run -- tcp /home/raul/faculdade/comp-paralela/server/file.mp4 127.0.0.1:8084

UDP:

cargo run -- udp {caminho_arquivo_entrada} {endereço}  
ex: cargo run -- udp /home/raul/faculdade/comp-paralela/server/file.mp4 0.0.0.0:8084

Cliente (/client)

TCP:

cargo run -- tcp {caminho_arquivo_saida} {endereço_servidor}  
ex: cargo run -- tcp /home/raul/faculdade/comp-paralela/client/dest.mp4 127.0.0.1:8084

UDP:

cargo run -- udp {caminho_arquivo_saida} {endereço_servidor} {endereço_cliente}  
ex: cargo run -- udp /home/raul/faculdade/comp-paralela/client/dest.mp4 127.0.0.1:8084 127.0.0.1:8085
