static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[tokio::main]
async fn main() {
    let client = reqwest::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();
    let res = client
        .get("https://www.rust-lang.org")
        .send()
        .await
        .unwrap();
    println!("{:?}", res)
}
