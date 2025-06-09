use std::any::Any;
use rpgx::{library::{self, Library}, prelude::*};
use crate::presets::namespace::namespace_preset;

pub fn cluster_map(library: &Library<Box<dyn Any>>, namespaces: &Vec<crate::kube::k8s::Namespace>) -> Map {
    let total = namespaces.len();
    if total == 0 {
        return Map::new("default".into(), vec![], Coordinates::default());
    }

    let cols = (total as f64).sqrt().ceil() as usize;
    let rows = ((total + cols - 1) / cols) as usize;
    let mut map = Map::new("default".into(), vec![], Coordinates::default());

    // Step 1: Precompute all namespace maps and determine largest shape
    let mut ns_maps = vec![];
    let mut max_width = 0;
    let mut max_height = 0;

    for ns in namespaces {
        let ns_map = namespace_preset(library, ns.name.clone());
        max_width = max_width.max(ns_map.get_shape().width);
        max_height = max_height.max(ns_map.get_shape().height);
        ns_maps.push(ns_map);
    }

    let full_rows = total / cols;
    let remainder = total % cols;

    // Step 2: Fill full rows normally
    let mut padded_maps: Vec<Option<Map>> = vec![];
    for map in ns_maps.iter().take(full_rows * cols) {
        padded_maps.push(Some(map.clone()));
    }

    // Step 3: Custom zig-zag fill for last row
    if remainder > 0 {
        let mut last_row = vec![None; cols];
        let remaining_maps = &ns_maps[full_rows * cols..];
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

    // Step 4: Merge all maps
    for (i, maybe_map) in padded_maps.into_iter().enumerate() {
        let row = (i / cols) as u32;
        let col = (i % cols) as u32;

        if let Some(ns_map) = maybe_map {
            let x_offset = col * max_width;
            let y_offset = row * max_height;
            map.merge_at(&ns_map, Coordinates { x: x_offset, y: y_offset }, None);
        }
    }

    // Step 5: Add filler layer
    let filler_layer = Layer::new(
        "filler".into(),
        LayerType::Texture,
        map.get_shape(),
        vec![Mask::new(
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
        )],
        0,
    );
    map.load_layer(filler_layer);

    // Step 6: Add hall map at the bottom
    let hall_shape = Shape { width: 9, height: 5 };
    let hall_map = Map::new(
        "hall".into(),
        vec![Layer::new(
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
        )],
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
