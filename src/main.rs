use std::net::TcpListener;

use sqlx::{postgres::PgPoolOptions};
use zero_to_prod::{
    configuration::get_configuration,
    startup::run,
    telementry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configurations");
    let connection = PgPoolOptions::new().connect_lazy_with(configuration.database.with_db());
    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(&address)?;

    run(listener, connection)?.await
}
