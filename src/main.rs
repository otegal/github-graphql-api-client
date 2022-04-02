use dotenv::dotenv;
use std::env;
use graphql_client::GraphQLQuery;
use graphql_client::reqwest::post_graphql;
use reqwest::Client;

#[allow(clippy::upper_case_acronyms)]
type URI = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.docs.graphql",
    query_path = "query.graphql",
    response_derives = "Debug"
)]
struct RepoView;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.docs.graphql",
    query_path  = "query.graphql",
    response_derives = "Debug"
)]
struct Viewer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let token = env::var("GITHUB_ACCESS_TOKEN")?;
    let github_user_name = env::var("GITHUB_USER_NAME")?;

    // // 一旦Raw string literalを利用してクエリを組む
    // let query = r#"{ "query": "query { viewer { login }}" }"#;
    //
    // let client = reqwest::Client::new();
    // let res = client
    //     .post("https://api.github.com/graphql")
    //     .header("Authorization", format!("bearer {}", token))
    //     // User-Agentに指定の値を入れない場合403が返ってくる
    //     // cf. https://docs.github.com/ja/rest/overview/resources-in-the-rest-api#user-agent-required
    //     .header("User-Agent", github_user_name)
    //     .body(query)
    //     .send()
    //     .await?;
    //
    // println!("{:?}", res.text().await?);

    let client = Client::builder()
        // User-Agentに指定の値を入れない場合403が返ってくる
        // cf. https://docs.github.com/ja/rest/overview/resources-in-the-rest-api#user-agent-required
        .user_agent(github_user_name)
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("bearer {}", token))?
            ))
            .collect(),
        )
        .build()?;

    // let variables = repo_view::Variables{
    //     owner: "otegal".into(),
    //     name:  "github-graphql-api-client".into(),
    // };
    // let res = post_graphql::<RepoView, _>(&client, "https://api.github.com/graphql", variables).await?;

    let res = post_graphql::<Viewer, _>(&client, "https://api.github.com/graphql", viewer::Variables{}).await?;

    println!("{:?}", res);
    Ok(())
}
