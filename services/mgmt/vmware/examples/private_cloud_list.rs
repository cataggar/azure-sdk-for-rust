/*
Lists the private clouds, similar to:
az vmware private-cloud list --query [].id

az extension documentation:
https://docs.microsoft.com/cli/azure/ext/vmware/vmware/private-cloud?view=azure-cli-latest#ext_vmware_az_vmware_private_cloud_list
API documentation:
https://docs.microsoft.com/rest/api/vmware/privateclouds/list

cargo run --package azure_mgmt_vmware --example private_cloud_list
*/

use azure_identity::AzureCliCredential;
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscription_id = std::env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID required");
    let credential = AzureCliCredential::new(None)?;
    let client = azure_mgmt_vmware::Client::builder(credential).build()?;

    let mut count = 0;
    let mut pager = client.private_clouds_client().list_in_subscription(subscription_id).await?;
    while let Some(cloud) = pager.try_next().await? {
        count += 1;
        let resource_id = cloud.tracked_resource.resource.id;
        println!("{count} {resource_id:?}");
    }
    Ok(())
}
