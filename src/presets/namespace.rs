use rpgx::{library::ResourceLibrary, prelude::*};

pub fn namespace_map(library: &ResourceLibrary, _namespace: String) -> Map {
    let mut map = rpgx::factory::map::presets::building::building_2x3(
        Shape {
            width: 4,
            height: 6,
        },
        library.get_key_id("building_1"),
        library.get_key_id("consolelog"),
    );
    map.load_layer(rpgx::factory::layer::presets::ground::ground_layer(
        Shape {
            width: 6,
            height: 8,
        },
        library.get_key_id("floor_1"),
    ));
    map.load_layer(rpgx::factory::layer::presets::street::street_layer_around(
        Shape {
            width: 4,
            height: 6,
        },
        library.get_key_id("floor_2"),
    ));
    map.load_layer(rpgx::factory::layer::presets::street::street_layer_around(
        Shape {
            width: 6,
            height: 8,
        },
        library.get_key_id("floor_2"),
    ));

    map
}