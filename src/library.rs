use std::any::Any;
use dioxus::prelude::*;
use rpgx::{engine::Engine, library::Library, map::Map, pawn::Pawn, prelude::{Coordinates, Effect, Shape, Tile}, scene::Scene};

pub fn use_library(namespaces: Vec<String>) -> Library<Box<dyn Any>> {
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

    for namespace in namespaces {
        let key = format!("sign-ns-{}", namespace);
        let ns = namespace.clone();

        println!("inserting namespace resources");

        library.insert(format!("load-ns-{}", namespace), Box::new(Box::new(move |engine: &mut Engine| {
            let pawn = engine.get_active_scene().unwrap().pawn.clone();
            let ns_scene = Scene {
                name: format!("ns-{}", namespace),
                pawn: Pawn { texture_id: pawn.texture_id, tile: Tile::new(0, Effect::default(), Coordinates { x: 0, y: 0 }, Shape { ..Default::default() } )},
                map: Map {
                    name: format!("ns-{}", namespace),
                    layers: vec![]
                }
            };
            println!("pushing scene");
            engine.push_scene(ns_scene);
        }) as Box<dyn Fn(&mut Engine)>) as Box<dyn Any>);

        library.insert(key, Box::new(Box::new(move || {
            println!("Invoked render closure for sign");
            rsx! {
                div {
                    class: "sign",
                    style: "width: 100%; height: 100%; background-color: rgba(0,0,0,0.7); display: flex; align-items: center; justify-content: center; text-align: center; color: white; border: solid 2px black; border-radius: 5px;",
                    {ns.clone()} // clone here to avoid move
                }
            }.unwrap()
        }) as Box<dyn Fn() -> VNode>) as Box<dyn Any>);
        
    }

    println!("{:?}", library.get_by_key("load-ns-gitlab").is_some());

    library
}
