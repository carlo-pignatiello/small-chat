use tokio::{net::TcpListener, io::{AsyncWriteExt, BufReader, AsyncBufReadExt}, sync::broadcast};

struct User {
    id: Option<i32>,
    name: String
}

#[tokio::main]
async fn main(){
    let tpc_listener = TcpListener::bind("localhost:8000").await.unwrap();
    let (tx, _rx) = broadcast::channel(10);
    loop {
        let (mut socket, addr) = tpc_listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        let mut client_n = 0;
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
            client_n = client_n + 1;
            println!("Hi, insert your name");
            let new_user_name = reader.read_line(&mut line);
            let new_user = User{
                id: Some(client_n),
                name: new_user_name.await.unwrap().to_string()
            };
            println!("Hi {}, your id is: {:?}", new_user.name,  new_user.id);
            loop {  
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear()
                    }
                    result = rx.recv() => {
                        let (mess, other_addr) = result.unwrap();
                        if addr != other_addr {   
                            let msg = format!("Client n. {}: {}", client_n, mess);
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                } 
            }    
        });
    }
}