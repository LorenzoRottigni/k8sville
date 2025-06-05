use std::any::Any;
use dioxus::prelude::*;
use rpgx::{engine::Engine, library::Library, map::Map, pawn::Pawn, prelude::{Coordinates, Effect, Shape, Tile}, scene::Scene};

pub fn use_library(namespaces: Vec<crate::kube::k8s::Namespace>) -> Library<Box<dyn Any>> {
    let mut library: Library<Box<dyn Any>> = Library::new();

    // Platform-agnostic logger
    fn log_message(message: &str) {
        // #[cfg(target_arch = "wasm32")]
        // web_sys::console::log_1(&message.into());

        #[cfg(not(target_arch = "wasm32"))]
        println!("{message}");
    }

    library.insert(
        "floor_1",
        Box::new("https://s3.rottigni.tech/rpgx/spaceship_floor_1.webp".to_string()),
    );
    library.insert(
        "floor_2",
        Box::new("https://s3.rottigni.tech/rpgx/spaceship_floor_2.webp".to_string()),
    );
    library.insert(
        "floor_3",
        Box::new("https://s3.rottigni.tech/rpgx/spaceship_floor_3.webp".to_string()),
    );
    library.insert(
        "building_1",
        Box::new("https://s3.rottigni.tech/rpgx/processor_8.webp".to_string()),
    );
    library.insert(
        "building_2",
        Box::new("https://s3.rottigni.tech/rpgx/processor_9.webp".to_string()),
    );
    library.insert(
        "deployment",
        Box::new("https://s3.rottigni.tech/k8sville/k8sville_deployment.webp".to_string()),
    );
    library.insert(
        "namespace",
        Box::new("https://s3.rottigni.tech/k8sville/k8sville_namespace.webp".to_string()),
    );
    library.insert(
        "pod",
        Box::new("https://s3.rottigni.tech/k8sville/k8sville_pod.webp".to_string()),
    );
    library.insert(
        "portal_1",
        Box::new("https://s3.rottigni.tech/rpgx/portal_1.webp".to_string()),
    );
    library.insert(
        "character_1",
        Box::new("https://s3.rottigni.tech/rpgx/character_1.webp".to_string()),
    );
    // Platform-agnostic action
    library.insert("consolelog", Box::new(Box::new(|| {
        println!("callback");
        log_message("Hello from Rust!");
    }) as Box<dyn Fn()>) as Box<dyn Any>);

    library.insert("sign", Box::new(Box::new(|| {
        println!("Inserting a sign");
        rsx!{
            div {
                class: "sign",
                style: "width: 100%; height: 100%; background-color: red;",
                "this is sign"
            }
        }
    })));

    library.insert("go_back", Box::new(Box::new(move |engine: &mut Engine| {
        engine.pop_scene();
    }) as Box<dyn Fn(&mut Engine)>) as Box<dyn Any>);
    
    for namespace in &namespaces {
        let namespace_name = namespace.name.clone();

        for deployment in &namespace.deployments {
            let deployment_name = deployment.name.clone();

            for pod in &deployment.pods {
                let pod_name = pod.name.clone();

                // Insert pod scene loader
                {
                    let key = format!("load-pod-{}", pod_name);
                    let map = crate::maps::pod::pod_map(&library);
                    let name_for_scene = pod_name.clone();
                    library.insert(
                        key,
                        Box::new(Box::new(move |engine: &mut Engine| {
                            let pawn = engine.get_active_scene().unwrap().pawn.clone();
                            let ns_scene = Scene {
                                name: format!("pod-{}", name_for_scene),
                                pawn: Pawn {
                                    texture_id: pawn.texture_id,
                                    tile: Tile::new(
                                        0,
                                        Effect::default(),
                                        Coordinates { x: 0, y: 0 },
                                        Shape::default(),
                                    ),
                                },
                                map: map.clone(),
                            };
                            engine.push_scene(ns_scene);
                        }) as Box<dyn Fn(&mut Engine)>) as Box<dyn Any>,
                    );
                }

                // Insert pod sign renderer
                {
                    let key = format!("sign-pod-{}", pod_name);
                    library.insert(
                        key,
                        Box::new(Box::new(move || {
                            rsx! {
                                div {
                                    class: "sign",
                                    style: "width: 100%; height: 100%; background-color: rgba(0,0,0,0.7); display: flex; align-items: center; justify-content: center; text-align: center; color: white; border: solid 2px black; border-radius: 5px;",
                                    {pod_name.clone()}
                                }
                            }.unwrap()
                        }) as Box<dyn Fn() -> VNode>) as Box<dyn Any>,
                    );
                }
            }
            
            // Insert deployment scene loader
            {
                let key = format!("load-deployment-{}", deployment_name);
                let map = crate::maps::deployment::deployment_map(&library, deployment.clone());
                let name_for_scene = deployment_name.clone();
                library.insert(
                    key,
                    Box::new(Box::new(move |engine: &mut Engine| {
                        let pawn = engine.get_active_scene().unwrap().pawn.clone();
                        let deployment_scene = Scene {
                            name: format!("deployment-{}", name_for_scene),
                            pawn: Pawn {
                                texture_id: pawn.texture_id,
                                tile: Tile::new(
                                    0,
                                    Effect::default(),
                                    Coordinates { x: 0, y: 0 },
                                    Shape::default(),
                                ),
                            },
                            map: map.clone(),
                        };
                        engine.push_scene(deployment_scene);
                    }) as Box<dyn Fn(&mut Engine)>) as Box<dyn Any>,
                );
            }

            // Insert deployment sign renderer
            {
                let key = format!("sign-deployment-{}", deployment_name);
                library.insert(
                    key,
                    Box::new(Box::new(move || {
                        rsx! {
                            div {
                                class: "sign",
                                style: "width: 100%; height: 100%; background-color: rgba(0,0,0,0.7); display: flex; align-items: center; justify-content: center; text-align: center; color: white; border: solid 2px black; border-radius: 5px;",
                                {deployment_name.clone()}
                            }
                        }.unwrap()
                    }) as Box<dyn Fn() -> VNode>) as Box<dyn Any>,
                );
            }

            
        }
        
        // Insert namespace scene loader
        {
            let key = format!("load-namespace-{}", namespace_name);
            let map = crate::maps::namespace::namespace_map(&library, namespace.clone());
            let name_for_scene = namespace_name.clone();
            library.insert(
                key,
                Box::new(Box::new(move |engine: &mut Engine| {
                    let pawn = engine.get_active_scene().unwrap().pawn.clone();
                    let ns_scene = Scene {
                        name: format!("ns-{}", name_for_scene),
                        pawn: Pawn {
                            texture_id: pawn.texture_id,
                            tile: Tile::new(
                                0,
                                Effect::default(),
                                Coordinates { x: 0, y: 0 },
                                Shape::default(),
                            ),
                        },
                        map: map.clone(),
                    };
                    engine.push_scene(ns_scene);
                }) as Box<dyn Fn(&mut Engine)>) as Box<dyn Any>,
            );
        }

        // Insert namespace sign renderer
        {
            let key = format!("sign-namespace-{}", namespace_name);
            library.insert(
                key,
                Box::new(Box::new(move || {
                    rsx! {
                        div {
                            class: "sign",
                            style: "width: 100%; height: 100%; background-color: rgba(0,0,0,0.7); display: flex; align-items: center; justify-content: center; text-align: center; color: white; border: solid 2px black; border-radius: 5px;",
                            {namespace_name.clone()}
                        }
                    }.unwrap()
                }) as Box<dyn Fn() -> VNode>) as Box<dyn Any>,
            );
        }
    }


    library
}
