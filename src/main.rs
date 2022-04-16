fn main() -> Result<(), Box<dyn std::error::Error>> {
    http_client::start();

    Ok(())
}
