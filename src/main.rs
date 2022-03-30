use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let token = env::var("GITHUB_ACCESS_TOKEN")?;
    println!("token: {:?}", token);

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.github.com/graphql")
        .header("Authorization", format!("bearer {}", token))
        .send()
        .await?;

    println!("{:?}", res);
    println!("----------------");
    println!("{:?}", res.text().await?);
    Ok(())
}
