use std::net::TcpListener;

use sqlx::PgPool;
use zero_to_prod::{
    configuration::get_configuration, 
    startup::run, 
    telementry::{get_subscriber, init_subscriber}
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into());
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configurations");
    let connection = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(&address)?;

    run(listener, connection)?.await
}
