mod app;
mod ddb;
mod models;
mod thumbnail;
mod tiph;

use app::App;
use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use clap::Parser;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app = App::parse();

    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;
    let ddb = aws_sdk_dynamodb::Client::new(&config);
    let lambda = aws_sdk_lambda::Client::new(&config);
    let s3 = aws_sdk_s3::Client::new(&config);
    let http = reqwest::Client::new();

    let _ = app.exec(&ddb, &lambda, &s3, &http).await;
}

#[cfg(test)]
pub mod tests {
}
