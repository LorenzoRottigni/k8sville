use std::any::Any;
use dioxus::prelude::*;
use rpgx::{library::Library, prelude::*};

pub fn namespace_map(library: &Library<Box<dyn Any>>, namespace: String) -> Map {
    let mut map = rpgx::factory::map::presets::building::building_2x3(
        Shape {
            width: 4,
            height: 6,
        },
        library.get_id("building_1").unwrap(),
        library.get_id("consolelog").unwrap(),
    );
    map.load_layer(rpgx::factory::layer::presets::ground::ground_layer(
        Shape {
            width: 6,
            height: 8,
        },
        library.get_id("floor_1").unwrap(),
    ));
    map.load_layer(rpgx::factory::layer::presets::street::street_layer_around(
        Shape {
            width: 4,
            height: 6,
        },
        library.get_id("floor_2").unwrap(),
    ));
    map.load_layer(rpgx::factory::layer::presets::street::street_layer_around(
        Shape {
            width: 6,
            height: 8,
        },
        library.get_id("floor_2").unwrap(),
    ));

    // insert dynamic sign and read it directly.
    
    let key: String = format!("sign-ns-{:}", namespace);
    println!("Inserting sign {:?}", &key.clone());
    

    map.load_layer(Layer::new("sign".into(), LayerType::Block, Shape { width: 8, height: 10 }, vec![
        Mask {
            name: "sign".into(),
            effect: rpgx::prelude::Effect { render_id: library.get_id(key), group: true, ..Default::default() },
            selector: Selector::Block((Coordinates { x: 0, y: 0 }, Coordinates { x: 2, y: 2 }))
        }
    ], 8 ));

    

    map
}