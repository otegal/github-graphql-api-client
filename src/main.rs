use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let token = env::var("GITHUB_ACCESS_TOKEN")?;
    let github_user_name = env::var("GITHUB_USER_NAME")?;

    // 一旦Raw string literalを利用してクエリを組む
    let query = r#"{ "query": "query { viewer { login }}" }"#;

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.github.com/graphql")
        .header("Authorization", format!("bearer {}", token))
        // User-Agentに指定の値を入れない場合403が返ってくる
        // cf. https://docs.github.com/ja/rest/overview/resources-in-the-rest-api#user-agent-required
        .header("User-Agent", github_user_name)
        .body(query)
        .send()
        .await?;

    println!("{:?}", res.text().await?);
    Ok(())
}
