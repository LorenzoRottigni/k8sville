use std::path::PathBuf;
use kube::{Api, Client, Config, api::ListParams};
use kube::config::Kubeconfig;
use k8s_openapi::api::core::v1::{Namespace as K8sNamespace, Pod as K8sPod};
use k8s_openapi::api::apps::v1::Deployment as K8sDeployment;

pub mod k8s {
    #[derive(Debug, Clone)]
    pub struct Namespace {
        pub name: String,
        pub deployments: Vec<Deployment>,
    }

    #[derive(Debug, Clone)]
    pub struct Deployment {
        pub name: String,
        pub pods: Vec<Pod>,
    }

    #[derive(Debug, Clone)]
    pub struct Pod {
        pub name: String,
    }
}

pub async fn fetch_k8s_data(
    kubeconfig_path: PathBuf,
) -> anyhow::Result<Vec<k8s::Namespace>> {
    let kubeconfig = Kubeconfig::read_from(kubeconfig_path)?;
    let config = Config::from_custom_kubeconfig(kubeconfig, &Default::default()).await?;
    let client = Client::try_from(config)?;

    let ns_api: Api<K8sNamespace> = Api::all(client.clone());
    let ns_list = ns_api.list(&ListParams::default()).await?;

    let mut result = Vec::new();

    for ns in ns_list.items {
        if let Some(ns_name) = ns.metadata.name.clone() {
            let mut deployments_vec = Vec::new();

            let deploy_api: Api<K8sDeployment> = Api::namespaced(client.clone(), &ns_name);
            if let Ok(deployments) = deploy_api.list(&ListParams::default()).await {
                for d in deployments.items {
                    if let Some(deploy_name) = d.metadata.name.clone() {
                        let pods_api: Api<K8sPod> = Api::namespaced(client.clone(), &ns_name);
                        let mut pods_vec = Vec::new();

                        if let Ok(pods) = pods_api.list(&ListParams::default()).await {
                            for p in pods.items {
                                if let Some(owners) = &p.metadata.owner_references {
                                    if owners.iter().any(|owner| {
                                        owner.kind == "ReplicaSet" && owner.name.starts_with(&deploy_name)
                                    }) {
                                        if let Some(pod_name) = p.metadata.name.clone() {
                                            pods_vec.push(k8s::Pod { name: pod_name });
                                        }
                                    }
                                }
                            }
                        }

                        deployments_vec.push(k8s::Deployment {
                            name: deploy_name,
                            pods: pods_vec,
                        });
                    }
                }
            }

            result.push(k8s::Namespace {
                name: ns_name,
                deployments: deployments_vec,
            });
        }
    }

    Ok(result)
}
