use azure_device_update::DeviceUpdateClient;
use azure_identity::token_credentials::{ClientSecretCredential, TokenCredentialOptions};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = env::var("CLIENT_ID").expect("Missing CLIENT_ID environment variable.");
    let client_secret =
        env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET environment variable.");
    let tenant_id = env::var("TENANT_ID").expect("Missing TENANT_ID environment variable.");
    let device_update_url =
        env::var("DEVICE_UPDATE_URL").expect("Missing DEVICE_UPDATE_URL environment variable.");
    let instance_id = env::var("DEVICE_UPDATE_INSTANCE_ID")
        .expect("Missing DEVICE_UPDATE_INSTANCE_ID environment variable.");
    let import_json = env::var("IMPORT_VALUE").expect("Missing IMPORT_VALUE environment variable.");

    let creds = ClientSecretCredential::new(
        tenant_id,
        client_id,
        client_secret,
        TokenCredentialOptions::default(),
    );
    let mut client = DeviceUpdateClient::new(&device_update_url, &creds)?;

    let import_update_response = client.import_update(&instance_id, import_json).await?;
    dbg!(&import_update_response);

    Ok(())
}
