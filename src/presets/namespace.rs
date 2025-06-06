use std::any::Any;
use dioxus::prelude::*;
use rpgx::{library::Library, prelude::*};

pub fn building(shape: Shape, texture_id: i32, action_id: i32) -> Map {
    // let base_layer = Layer::base("base".to_string(), shape, vec![]);
    let building_layer = Layer::new(
        "buildings".to_string(),
        LayerType::Block,
        shape,
        vec![Mask {
            name: "logo".to_string(),
            effect: rpgx::prelude::Effect {
                texture_id: Some(texture_id),
                block: true,
                group: true,
                shrink: Some((
                    Coordinates { x: 1, y: 1 },
                    Coordinates {
                        x: shape.width - 2,
                        y: shape.height - 2,
                    },
                )),
                ..Default::default()
            },
            selector: Selector::Block((
                Coordinates { x: 0, y: 0 },
                Coordinates {
                    x: shape.width,
                    y: shape.height,
                },
            )),
        }],
        5,
    );

    let (start_x, end_x) = if shape.width % 2 == 0 {
        let mid_left = shape.width / 2;
        let mid_right = shape.width / 2;
        (mid_left, mid_right)
    } else {
        let mid = shape.width / 2;
        (mid, mid + 1)
    };
    let bottom_y = shape.height - 1;

    let action_layer = Layer::new(
        "actions".to_string(),
        LayerType::Action,
        shape,
        vec![Mask {
            name: "action_test".to_string(),
            effect: rpgx::prelude::Effect {
                action_id: Some(action_id),
                ..Default::default()
            },
            selector: Selector::Block((
                Coordinates {
                    x: start_x,
                    y: bottom_y,
                },
                Coordinates {
                    x: end_x + 1,
                    y: bottom_y + 1,
                },
            )),
        }],
        6,
    );

    Map::new(
        "base".to_string(),
        vec![
            // base_layer.clone(),
            building_layer.clone(),
            action_layer.clone(),
        ],
    )
}


pub fn namespace_preset(library: &Library<Box<dyn Any>>, namespace: String) -> Map {
    let mut map = building(
        Shape {
            width: 10,
            height: 8,
        },
        library.get_id("namespace").unwrap(),
        library.get_id(format!("load-namespace-{}", namespace)).unwrap(),
    );
    map.load_layer(rpgx::factory::layer::presets::ground::ground_layer(
        Shape {
            width: 11,
            height: 8,
        },
        library.get_id("floor_1").unwrap(),
    ));
    map.load_layer(rpgx::factory::layer::presets::street::street_layer_around(
        Shape {
            width: 11,
            height: 8,
        },
        library.get_id("floor_2").unwrap(),
    ));
    map.load_layer(rpgx::factory::layer::presets::street::street_layer_around(
        Shape {
            width: 13,
            height: 10,
        },
        library.get_id("floor_3").unwrap(),
    ));
    

    map.load_layer(Layer::new("sign".into(), LayerType::Block, Shape { width: 10, height: 12 }, vec![
        Mask {
            name: "sign".into(),
            effect: rpgx::prelude::Effect { render_id: library.get_id(format!("sign-namespace-{:}", namespace)), group: true, ..Default::default() },
            selector: Selector::Block((Coordinates { x: 6, y: 2 }, Coordinates { x: 9, y: 3 }))
        }
    ], 8 ));

    

    map
}

/*
pub fn namespace_map(namespace: String, texture_id: i32, action_id: i32) -> Map {
    let mut map = Map {
        name: format!("namespace-{}", namespace),
        layers: vec![]
    };
    let shape = Shape {
        width: 10,
        height: 10,
    };
    map.load_layer(rpgx::factory::layer::presets::ground::ground_layer(
        shape,
        texture_id,
    ));
    map.load_layer(Layer::new(
        "actions".to_string(),
        LayerType::Action,
        shape,
        vec![Mask {
            name: "action_test".to_string(),
            effect: rpgx::prelude::Effect {
                action_id: Some(action_id),
                texture_id: Some(2),
                ..Default::default()
            },
            selector: Selector::Block((
                Coordinates {
                    x: shape.width / 2,
                    y: shape.height -1 ,
                },
                Coordinates {
                    x: (shape.width / 2) + 1,
                    y: shape.height -1 ,
                },
            )),
        }],
        6,
    ));

    map
    
} */