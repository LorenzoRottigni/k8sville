use std::path::PathBuf;

use dioxus::prelude::*;
use rpgx::prelude::*;

pub mod kube;
pub mod maps;
pub mod presets;
pub mod library;

fn main() {
    dioxus::launch(App);
}

static CSS: Asset = asset!("/assets/main.css");

#[component]
fn App() -> Element {
    let kube_data = use_resource(|| async {
        kube::fetch_k8s_data(PathBuf::from(r#"C:\Users\loren\Documents\credentials\kubeconfig-test.yml"#)).await
    });

    match &*kube_data.read_unchecked() {
        Some(Ok(namespaces)) => {
            let k8s_library = use_signal(|| library::use_library(namespaces.clone()));
            let map = maps::cluster::cluster_map(&k8s_library.read(), namespaces);
            // let map = maps::namespace::namespace_map(&k8s_library.read(), &namespaces.get(0).unwrap().deployments);

            let mut scene = Scene::new("default".into(),map,None);
            scene.load_pawn(k8s_library.read().get_id("character_1").unwrap());
            let engine = use_signal(|| Engine::new(scene));
            rsx! {
                document::Stylesheet { href: CSS },
                div { class: "cluster",
                    rpgx_dioxus::components::engine::Engine {
                        engine: engine.clone(),
                        library: k8s_library.clone(),
                        square_size: 32,
                    }
                }
            }
        }
        Some(Err(err)) => {
            rsx! { div { "Failed to fetch K8s data: {err}" } }
        }
        None => {
            rsx! { div { "Loading..." } }
        }
    }
}
