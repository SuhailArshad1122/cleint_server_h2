use std::error::Error;
//use http::{HeaderMap, Request};
use bytes::Bytes;
use h2::server;
//use h2::client;
use tokio::net::{TcpListener, TcpStream};


#[tokio::main]
 async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
     let _ = env_logger::try_init();

    let listener = TcpListener::bind("127.0.0.1:5000").await?;

    println!("\n");
    println!("listening on ..... {:?} .. waiting for client", listener.local_addr());
    println!("\n");

    loop {
        if let Ok((socket, _peer_addr)) = listener.accept().await {
            println!("\n");
            println!("   Socket-------> {:?}", socket);
            println!("\n");
            println!("Peer Addr-------> {:?}", _peer_addr);
            println!("\n");
            tokio::spawn(async move {
                if let Err(e) = handle(socket).await {
                    println!("-------> err={:?}", e);
                }
            });
        } else {println!("-------> No Error")}
    }
}

async fn handle(socket: TcpStream) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut connection = server::handshake(socket).await?;
    println!("\n");
    println!("H2 connection successful....!");
    println!("\n");

    while let Some(result) = connection.accept().await {
        let (request, mut respond) = result?;
        println!("GOT request from client: {:?}", request);
        println!("\n");
        let response = http::Response::new(());

        let mut send = respond.send_response(response, false)?;

        println!("Sending to client >>>>>>>>>> ");
        send.send_data(Bytes::from_static(b"AOA from Server .."), true)?;
    }
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~ H2 connection Ended !!!!!! ~~~~~~~~~~~");

    Ok(())
}