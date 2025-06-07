use std::any::Any;

use rpgx::{library::Library, prelude::*};

pub fn deployment_map(library: &Library<Box<dyn Any>>, deployment: crate::kube::k8s::Deployment) -> Map {
    let total = deployment.pods.len();
    if total == 0 {
        return Map::new("default".into(), vec![], Coordinates::default());
    }

    // Calculate grid dimensions
    let cols = (total as f64).sqrt().ceil() as usize;

    let mut map = Map::new("default".into(), vec![], Coordinates::default());

    for (i, pod) in deployment.pods.iter().enumerate() {
        let row = i / cols;
        let col = i % cols;

        let ns_map = crate::presets::pod::pod_preset(library, pod.clone());
        let shape = ns_map.get_shape();

        let x_offset = col * shape.width as usize;
        let y_offset = row * shape.height as usize;

        map.merge_at(&ns_map, Coordinates { x: x_offset as u32, y: y_offset as u32 }, None);
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
        0
    );

    

    let hall_shape = Shape {
        width: 9,
        height: 5
    };

    let hall_map = Map::new(
        "hall".into(),
        vec![
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
            ),
            Layer::new(
                "action-back".into(),
                LayerType::Action,
                hall_shape,
                vec![
                    Mask::new(
                        "action-back".into(),
                        Selector::Block((Coordinates { x: 3, y: 4 }, Coordinates { x: 5, y: 4})),
                        Effect {
                            action_id: library.get_id("go_back"),
                            texture_id: library.get_id("floor_1"),
                            ..Default::default()
                        }
                    )
                ],
                5
            )
        ],
        Coordinates::default()
    );

    let merge_offset = Coordinates {
        x: (map.get_shape().width - hall_shape.width) / 2,
        y: map.get_shape().height,
    };

    let center_spawn = Coordinates {
        x: merge_offset.x + hall_shape.width / 2,
        y: merge_offset.y + hall_shape.height - 1,
    };

    map.load_layer(filler_layer);

    map.merge_at(&hall_map, merge_offset, Some(center_spawn));

    

    map
}
