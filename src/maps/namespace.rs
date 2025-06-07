use std::any::Any;
use rpgx::{library::Library, prelude::*};

pub fn namespace_map(library: &Library<Box<dyn Any>>, namespace: crate::kube::k8s::Namespace) -> Map {
    let total = namespace.deployments.len();
    if total == 0 {
        return Map::new("default".into(), vec![], Coordinates::default());
    }

    let cols = (total as f64).sqrt().ceil() as usize;
    let rows = ((total + cols - 1) / cols) as usize;
    let mut map = Map::new("default".into(), vec![], Coordinates::default());

    // Precompute all deployment maps to determine the largest shape
    let mut ns_maps = vec![];
    let mut max_width = 0;
    let mut max_height = 0;

    for deployment in &namespace.deployments {
        let ns_map = crate::presets::deployment::deployment_preset(library, deployment.clone());
        max_width = max_width.max(ns_map.get_shape().width);
        max_height = max_height.max(ns_map.get_shape().height);
        ns_maps.push(Some(ns_map));
    }

    // Pad to full grid with `None`
    while ns_maps.len() < rows * cols {
        ns_maps.push(None);
    }

    for (i, maybe_map) in ns_maps.into_iter().enumerate() {
        let row = (i / cols) as u32;
        let col = (i % cols) as u32;

        if let Some(ns_map) = maybe_map {
            let x_offset = col * max_width;
            let y_offset = row * max_height;
            map.merge_at(&ns_map, Coordinates { x: x_offset, y: y_offset }, None);
        }
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
        width: 8,
        height: 4
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
            )
        ],
        Coordinates::default()
    );

    map.merge_at(&hall_map, Coordinates { x: (map.get_shape().width - hall_shape.width) / 2, y: map.get_shape().height }, None);

    map.load_layer(filler_layer);

    map
}
