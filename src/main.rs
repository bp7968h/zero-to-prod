use std::net::TcpListener;

use zero_to_prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let rand_port = listener.local_addr().unwrap().port();

    match run(listener) {
        Ok(srv) => {
            println!("Server listeneing on {}", rand_port);
            srv.await
        }
        Err(e) => Err(e)?,
    }
}
