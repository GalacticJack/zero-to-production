use zero_to_production::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    run()?.await
}
