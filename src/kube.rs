use std::path::PathBuf;
use kube::{Api, Client, Config, api::ListParams};
use kube::config::Kubeconfig;
use k8s_openapi::api::core::v1::{Namespace, Pod};
use k8s_openapi::api::apps::v1::Deployment;

pub async fn fetch_k8s_data(
    kubeconfig_path: PathBuf,
) -> anyhow::Result<(Vec<String>, Vec<(String, String)>, Vec<(String, String)>)> {
    let kubeconfig = Kubeconfig::read_from(kubeconfig_path)?;
    let config = Config::from_custom_kubeconfig(kubeconfig, &Default::default()).await?;
    let client = Client::try_from(config)?;

    let mut namespaces = vec![];
    let mut deployments = vec![];
    let mut pods = vec![];

    let ns_api: Api<Namespace> = Api::all(client.clone());
    for ns in ns_api.list(&ListParams::default()).await?.items {
        if let Some(ns_name) = ns.metadata.name.clone() {
            namespaces.push(ns_name.clone());

            // Deployments
            let deploy_api: Api<Deployment> = Api::namespaced(client.clone(), &ns_name);
            if let Ok(deploys) = deploy_api.list(&ListParams::default()).await {
                for d in deploys.items {
                    if let Some(name) = d.metadata.name {
                        deployments.push((ns_name.clone(), name));
                    }
                }
            }

            // Pods
            let pods_api: Api<Pod> = Api::namespaced(client.clone(), &ns_name);
            if let Ok(pod_list) = pods_api.list(&ListParams::default()).await {
                for p in pod_list.items {
                    if let Some(name) = p.metadata.name {
                        pods.push((ns_name.clone(), name));
                    }
                }
            }
        }
    }

    Ok((namespaces, deployments, pods))
}