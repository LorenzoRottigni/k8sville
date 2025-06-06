use std::any::Any;

use rpgx::{library::Library, prelude::*};
use crate::presets::namespace::namespace_preset;

pub fn cluster_map(library: &Library<Box<dyn Any>>, namespaces: &Vec<crate::kube::k8s::Namespace>) -> Map {
    let total = namespaces.len();
    if total == 0 {
        return Map::new("default".into(), vec![]);
    }

    // Calculate grid dimensions
    let cols = (total as f64).sqrt().ceil() as usize;
    // let rows = (total + cols - 1) / cols;

    let mut map = Map::new("default".into(), vec![]);

    for (i, namespace) in namespaces.iter().enumerate() {
        let row = (i / cols) as i32;
        let col = (i % cols) as i32;

        let ns_map = namespace_preset(library, namespace.name.clone());
        let shape = ns_map.get_shape();

        // Calculate position to merge the namespace map
        let x_offset = col * shape.width;
        let y_offset = row * shape.height;

        map.merge_at(&ns_map, Coordinates { x: x_offset, y: y_offset });
    }

    let filler_layer = Layer::new(
        "filler".into(),
        LayerType::Texture,
        map.get_shape(),
        vec![
            Mask::new(
                "filler".into(),
                Selector::Block((Coordinates { x: 0, y: 0 }, Coordinates { x: map.get_shape().width, y: map.get_shape().height })),
                Effect {
                    texture_id: Some(2),
                    ..Default::default()
                }
            )
        ],
        -1
    );

    let hall_shape = Shape {
        width: 8,
        height: 4
    };

    let hall_map = Map {
        name: "hall".into(),
        layers: vec![
            Layer::new(
                "hall".into(),
                LayerType::Texture,
                hall_shape,
                vec![
                    Mask::new(
                        "hall-ground".into(),
                        Selector::Block((Coordinates { x: 0, y: 0 }, Coordinates { x: hall_shape.width, y: hall_shape.height })),
                        Effect {
                            texture_id: Some(3),
                            ..Default::default()
                        }
                    )
                ],
                1
            )
        ]
    };

    map.merge_at(&hall_map, Coordinates { x: (map.get_shape().width - hall_shape.width) / 2, y: map.get_shape().height });

    map.load_layer(filler_layer);

    map
}
