use s2n_quic::{client::Connect, Client};
use std::{net::SocketAddr, path::Path};


#[tokio::main]
async fn main() {
    let client_result = Client::builder()
        .with_tls(Path::new("./server/ca-cert.pem"))
        .unwrap()
        .with_io("127.0.0.1:0")
        .unwrap().start();
    let addr: SocketAddr = "127.0.0.1:4443".parse().unwrap();
    
    match client_result {
        Ok(client) => {
            let connect = Connect::new(addr).with_server_name("localhost");
            let mut conn     =client.connect(connect).await.unwrap();
            
            conn.keep_alive(true).unwrap();
            
            let stream = conn.open_bidirectional_stream().await.unwrap();
            let (mut receive_stream, mut send_stream) = stream.split();
            send_stream.send_data(bytes::Bytes::from("hallo")).unwrap();
            
            while let Some(chunk) = receive_stream.receive().await.unwrap() {
                println!("received: {:?}", chunk);
            }
        },
        Err(err) => {print!("error {} ",err);},
    }
        
}


