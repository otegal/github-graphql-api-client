use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let token = env::var("GITHUB_ACCESS_TOKEN")?;
    println!("token: {:?}", token);

    Ok(())
}
