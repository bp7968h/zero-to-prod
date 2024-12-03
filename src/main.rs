use zero_to_prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    match run() {
        Ok(srv) => {
            println!("Server listeneing on 8000");
            srv.await
        }
        Err(e) => Err(e)?,
    }
}
