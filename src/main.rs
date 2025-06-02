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

#[component]
fn App() -> Element {
    let kube_data = use_resource(|| async {
        kube::fetch_k8s_data(PathBuf::from(r#"C:\Users\loren\Documents\credentials\kubeconfig-test.yml"#)).await
    });

    let boxed_fn: Box<dyn Fn() -> Element> = Box::new(|| rsx! { div { class: "test", "Hello from box!" } });

    match &*kube_data.read_unchecked() {
        Some(Ok((namespaces, services, pods))) => {
            let k8s_library = use_signal(|| library::use_library());
            let map = maps::default::default_map(&k8s_library.read(), namespaces);

            match map.get_base_layer() {
                Some(layer) => {
                    if let Some(tile) = layer.get_tile_at(Coordinates { x: 0, y: 0 }) {
                        let pawn = Pawn {
                            tile,
                            texture_id: k8s_library.read().get_id("character_1").unwrap(),
                        };
                        let scene = Scene::new("default".into(),map,pawn);
                        let engine = use_signal(|| Engine::new(scene));
                        rsx! {
                            div { class: "cluster",
                                rpgx_dioxus::components::engine::Engine {
                                    engine: engine.clone(),
                                    library: k8s_library.clone(),
                                    square_size: 32,
                                }
                            }
                            {boxed_fn()}
                        }
                    } else {
                        rsx! { div { "no base tile" } }
                    }
                }
                None => rsx! { div { "no base layer" } },
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
