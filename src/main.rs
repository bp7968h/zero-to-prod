use std::net::TcpListener;

use sqlx::PgPool;
use zero_to_prod::{configuration::get_configuration, startup::run};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configurations");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(&address)?;

    match run(listener, connection) {
        Ok(srv) => {
            println!("Server listening on `{}`", address);
            srv.await
        }
        Err(e) => Err(e)?,
    }
}
