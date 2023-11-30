use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;

pub const DEFAULT_REGION: &str = "eu-central-1";

pub async fn access_aws() -> Client {
    let region_provider = RegionProviderChain::first_try(DEFAULT_REGION);
    let config = aws_config::from_env()
        .credentials_provider(
            aws_config::profile::ProfileFileCredentialsProvider::builder()
                .profile_name("devnet")
                .build(),
        )
        .region(region_provider)
        .load()
        .await;
    Client::new(&config)
}
