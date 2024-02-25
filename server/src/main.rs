

use s2n_quic::Server;
use std::path::Path;
use log::{debug, warn};
use env_logger::Env;
#[tokio::main]
async fn main() {
  
  env_logger::init_from_env(Env::default().default_filter_or("trace"));
  let mut server =  Server::builder()
  .with_tls((Path::new("ca-cert.pem"), Path::new("ca-key.pem"))).unwrap()
  .with_io("127.0.0.1:4443").unwrap()
  .start().unwrap();

  while let Some(mut connection) = server.accept().await {
    let _ = connection.keep_alive(true);
      while let Ok(r) = connection.accept_bidirectional_stream().await {
          match r {
              Some(mut rstream)=>{
                
                while let Some(chunk) = rstream.receive().await.unwrap() {
                  debug!("received: {:?}", chunk);
                }
                debug!("send");
                rstream.send("test".into()).await.unwrap()
                
              },
              None=> {
                warn!("empty stream")
              }
          }
      }
  }
}
 

 