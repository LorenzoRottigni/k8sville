use std::any::Any;
use rpgx::{library::Library, prelude::*};

pub fn deployment_map(library: &Library<Box<dyn Any>>, deployment: crate::kube::k8s::Deployment) -> Map {
    let total = deployment.pods.len();
    if total == 0 {
        return Map::new("default".into(), vec![], Coordinates::default());
    }

    let cols = (total as f64).sqrt().ceil() as usize;
    let rows = (total + cols - 1) / cols;

    let mut pod_maps = vec![];
    let mut max_width = 0;
    let mut max_height = 0;

    for pod in &deployment.pods {
        let pod_map = crate::presets::pod::pod_preset(library, pod.clone());
        max_width = max_width.max(pod_map.get_shape().width);
        max_height = max_height.max(pod_map.get_shape().height);
        pod_maps.push(pod_map);
    }

    let full_rows = total / cols;
    let remainder = total % cols;

    // Pad maps with None for alignment
    let mut padded_maps: Vec<Option<Map>> = pod_maps.iter().take(full_rows * cols).cloned().map(Some).collect();

    if remainder > 0 {
        let mut last_row = vec![None; cols];
        let remaining_maps = &pod_maps[full_rows * cols..];
        for (i, map) in remaining_maps.iter().enumerate() {
            let col = if i % 2 == 0 {
                i / 2
            } else {
                cols - 1 - (i / 2)
            };
            last_row[col] = Some(map.clone());
        }
        padded_maps.extend(last_row);
    }

    let mut map = Map::new("default".into(), vec![], Coordinates::default());

    for (i, maybe_map) in padded_maps.into_iter().enumerate() {
        let row = (i / cols) as u32;
        let col = (i % cols) as u32;

        if let Some(pod_map) = maybe_map {
            let x_offset = col * max_width;
            let y_offset = row * max_height;
            map.merge_at(&pod_map, Coordinates { x: x_offset, y: y_offset }, None);
        }
    }

    let filler_layer = Layer::new(
        "filler".into(),
        LayerType::Texture,
        map.get_shape(),
        vec![
            Mask::new(
                "filler".into(),
                Selector::Block((
                    Coordinates { x: 0, y: 0 },
                    Coordinates {
                        x: map.get_shape().width,
                        y: map.get_shape().height,
                    },
                )),
                Effect {
                    texture_id: library.get_id("floor_1"),
                    ..Default::default()
                },
            )
        ],
        0,
    );

    map.load_layer(filler_layer);

    let hall_shape = Shape {
        width: 9,
        height: 5,
    };

    let hall_map = Map::new(
        "hall".into(),
        vec![
            Layer::new(
                "hall".into(),
                LayerType::Texture,
                hall_shape,
                vec![Mask::new(
                    "hall-ground".into(),
                    Selector::Block((
                        Coordinates { x: 0, y: 0 },
                        Coordinates {
                            x: hall_shape.width,
                            y: hall_shape.height,
                        },
                    )),
                    Effect {
                        texture_id: library.get_id("floor_1"),
                        ..Default::default()
                    },
                )],
                1,
            ),
            Layer::new(
                "action-back".into(),
                LayerType::Action,
                hall_shape,
                vec![Mask::new(
                    "action-back".into(),
                    Selector::Block((
                        Coordinates { x: 3, y: 4 },
                        Coordinates { x: 5, y: 4 },
                    )),
                    Effect {
                        action_id: library.get_id("go_back"),
                        texture_id: library.get_id("floor_3"),
                        ..Default::default()
                    },
                )],
                5,
            ),
        ],
        Coordinates::default(),
    );

    let merge_offset = Coordinates {
        x: (map.get_shape().width - hall_shape.width) / 2,
        y: map.get_shape().height,
    };

    let center_spawn = Coordinates {
        x: merge_offset.x + hall_shape.width / 2,
        y: merge_offset.y + hall_shape.height - 1,
    };

    map.merge_at(&hall_map, merge_offset, Some(center_spawn));

    map
}
