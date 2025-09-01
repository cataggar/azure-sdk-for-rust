/*
Lists the private clouds, similar to:
az vmware private-cloud list --query [].id

az extension documentation:
https://docs.microsoft.com/cli/azure/ext/vmware/vmware/private-cloud?view=azure-cli-latest#ext_vmware_az_vmware_private_cloud_list
API documentation:
https://docs.microsoft.com/rest/api/vmware/privateclouds/list

cargo run --package azure_mgmt_vmware --example private_cloud_list
*/

use azure_core::http::Url;
use azure_identity::AzureCliCredential;
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscription_id = &std::env::var("AZURE_SUBSCRIPTION_ID").expect("AZURE_SUBSCRIPTION_ID required");
    let credential = AzureCliCredential::new(None)?;
    let client = azure_mgmt_vmware::Client::builder(credential).build()?;

    let mut count = 0;
    let clouds_client = client.private_clouds_client();
    let mut cloud_pager = clouds_client.list_in_subscription(subscription_id).pager()?;
    while let Some(cloud) = cloud_pager.try_next().await? {
        count += 1;
        let resource_id = cloud.tracked_resource.resource.id;
        println!("{count} {resource_id:?}");

        if let Some(resource_id) = resource_id {
            let resource_group_name = get_resource_group(&resource_id);
            let private_cloud_name = cloud.tracked_resource.resource.name;
            match (resource_group_name, private_cloud_name) {
                (Some(resource_group_name), Some(private_cloud_name)) => {
                    println!("  Resource Group: {resource_group_name}, Private Cloud: {private_cloud_name}");

                    // get cloud
                    let cloud = clouds_client.get(subscription_id, resource_group_name, private_cloud_name).await?;
                    println!("  Cloud: {cloud:?}");

                    // get clusters
                    // let mut cluster_pager = client
                    //     .clusters_client()
                    //     .list(subscription_id, resource_group_name, private_cloud_name)
                    //     .pager()?;
                    // let mut cluster_count = 0;
                    // while let Some(cluster) = cluster_pager.try_next().await? {
                    //     cluster_count += 1;
                    //     println!("  Cluster: {cluster:?}");
                    // }
                    // println!("  Total Clusters: {cluster_count}");
                }
                _ => {}
            }
        }
    }

    // get cluster
    // let cluster_id = std::env::var("AZURE_VMWARE_CLUSTER_ID").expect("AZURE_VMWARE_CLUSTER_ID required");
    // let cluster = client.private_clouds_client().clusters_client().get(cluster_id).await?;
    // println!("Cluster: {cluster:?}");

    // scale up cluster by adding 1

    Ok(())
}

fn get_next_segment(url: &Url, segment_name: &str) -> Option<String> {
    url.path_segments().and_then(|mut segments| {
        while let Some(segment) = segments.next() {
            if segment == segment_name {
                return segments.next().map(|s| s.into());
            }
        }
        None
    })
}

fn get_resource_group(resource_id: &str) -> Option<String> {
    Url::parse("r:/")
        .ok()
        .and_then(|url| url.join(&resource_id).ok())
        .and_then(|url| get_next_segment(&url, "resourceGroups"))
}
