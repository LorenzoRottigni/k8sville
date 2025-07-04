use std::any::Any;
use rpgx::prelude::*;

pub fn building(shape: Shape, texture_id: u32, action_id: u32) -> Map {
    let center_x = shape.width / 2;
    let center_y = shape.height / 2;

    let building_layer = Layer::new(
        "buildings".to_string(),
        vec![Mask::new(
            "logo".to_string(),
            Rect::new(Coordinates::new(0, 0), shape).into_single(),
            vec![
                Effect::Texture(texture_id),
                Effect::Block(Rect::new(
                    Coordinates::new(1,1),
                    Shape::new(2,shape.height.saturating_sub(2))
                )),
                Effect::Block(Rect::new(
                    Coordinates::new(5,1),
                    Shape::new(2,shape.height.saturating_sub(2))
                )),
                Effect::Block(Rect::new(
                    Coordinates::new(3,1),
                    Shape::new(2,5)
                ))
            ],
        )],
        5,
    );

    
    let action_layer = Layer::new(
        "actions".to_string(),
        vec![Mask::new(
            "action_test".to_string(),
            Rect::new(
                Coordinates::new(3, 6),
                Shape::new(2, 1),
            )
            .into_single(),
            vec![Effect::Action(action_id)],
        )],
        6,
    );

    Map::new(
        "base".to_string(),
        vec![
            building_layer,
            action_layer,
        ],
        Coordinates::default(),
    )
}

pub fn namespace_preset(library: &Library<Box<dyn Any>>, namespace: String) -> Map {
    let shape = Shape {
        width: 8,
        height: 10,
    };

    let mut map = building(
        shape,
        library.get_id("namespace_portal").unwrap(),
        library
            .get_id(format!("load-namespace-{}", namespace))
            .unwrap(),
    );

    map.load_layer(rpgx::factory::layer::presets::ground::ground_layer(
        Shape {
            width: 8,
            height: 10,
        },
        library.get_id("floor_1").unwrap(),
    ));
    map.load_layer(rpgx::factory::layer::presets::street::street_layer_around(
        Shape {
            width: 8,
            height: 10,
        },
        library.get_id("floor_2").unwrap(),
    ));
    map.load_layer(rpgx::factory::layer::presets::street::street_layer_around(
        Shape {
            width: 10,
            height: 12,
        },
        library.get_id("floor_3").unwrap(),
    ));

    map.load_layer(Layer::new(
        "sign".into(),
        vec![Mask::new(
            "sign".into(),
            Rect::new(Coordinates::new(3, 0), Shape::new(6, 2)).into_single(),
            vec![Effect::Render(
                library
                    .get_id(format!("sign-namespace-{:}", namespace))
                    .unwrap(),
            )],
        )],
        8,
    ));

    map
}
