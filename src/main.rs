use clap::Parser;
use std::error::Error;
use tokio::io;
use tokio::net::{TcpListener, TcpStream};

#[derive(Parser)]
struct Opts {
    #[clap(short, long, required = true)]
    local_addr: String,

    #[clap(short, long, required = true)]
    remote_addr: String,
}

async fn handle_client(mut local: TcpStream, remote_addr: String) -> Result<(), Box<dyn Error>> {
    let mut remote = TcpStream::connect(remote_addr).await?;
    let (mut ri, mut ro) = remote.split();
    let (mut li, mut lo) = local.split();

    let client_to_server = io::copy(&mut li, &mut ro);
    let server_to_client = io::copy(&mut ri, &mut lo);

    tokio::try_join!(client_to_server, server_to_client)?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opts: Opts = Opts::parse();
    let local_addr = opts.local_addr.clone();

    let listener = TcpListener::bind(local_addr).await?;
    loop {
        let (socket, _) = listener.accept().await?;
        println!("Accepted connection from {:?}", socket.peer_addr()?);
        let remote_addr = opts.remote_addr.clone(); // Move the value into the closure
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, remote_addr).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
}
