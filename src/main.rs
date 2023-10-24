mod sso;

use aws_credential_types::provider::SharedCredentialsProvider;
use aws_types::region::Region;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    profile: String,
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let args = Args::parse();

    let sso_povider = sso::SSOProvider::new().populate(Some(&args.profile)).await;
    let region: String = sso_povider.region().await.to_owned();

    let aws_config = aws_config::SdkConfig::builder()
        .credentials_provider(SharedCredentialsProvider::new(sso_povider))
        .region(Region::new(region))
        .build();

    let sts = aws_sdk_sts::Client::new(&aws_config);
    let caller_identity = sts.get_caller_identity().send().await?;

    println!("sts::get_caller_identity = {:?}", caller_identity);

    Ok(())
}
