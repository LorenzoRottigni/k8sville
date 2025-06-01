use std::any::Any;

use rpgx::{library::Library, prelude::*};

pub fn namespace_map(library: &Library<Box<dyn Any>>, _namespace: String) -> Map {
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

    map
}