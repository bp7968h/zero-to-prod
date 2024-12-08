use std::net::TcpListener;

use zero_to_prod::{configuration::get_configuration, run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configurations");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(&address)?;

    match run(listener) {
        Ok(srv) => {
            println!("Server listening on `{}`", address);
            srv.await
        }
        Err(e) => Err(e)?,
    }
}
