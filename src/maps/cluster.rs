use std::any::Any;
use rpgx::{library::{self, Library}, prelude::*};
use crate::presets::namespace::namespace_preset;

pub fn cluster_map(
    library: &Library<Box<dyn Any>>,
    namespaces: &Vec<crate::kube::k8s::Namespace>,
) -> Map {
    let total = namespaces.len();
    if total == 0 {
        return Map::new("default".into(), vec![], Coordinates::default());
    }

    // Precompute namespace maps and max shape
    let mut ns_maps = vec![];
    let mut max_width = 0;
    let mut max_height = 0;

    for ns in namespaces {
        let ns_map = namespace_preset(library, ns.name.clone());
        max_width = max_width.max(ns_map.get_shape().width);
        max_height = max_height.max(ns_map.get_shape().height);
        ns_maps.push(ns_map);
    }

    // Determine usable columns (left + right of central column)
    let mut usable_cols = (total as f64).sqrt().floor() as usize;

    // Ensure at least 1 left + 1 right column
    usable_cols = usable_cols.max(2);

    // Enforce even usable_cols → odd total_cols = usable_cols + 1 (center gap)
    if usable_cols % 2 != 0 {
        usable_cols += 1;
    }

    let total_cols = usable_cols + 1;
    let center_col = usable_cols / 2;

    // Number of rows required
    let rows = (total + usable_cols - 1) / usable_cols;

    let mut map = Map::new("default".into(), vec![], Coordinates::default());

    let mut placed = 0;

    for row in 0..rows {
        for col_in_row in 0..usable_cols {
            if placed >= ns_maps.len() {
                break;
            }

            // Map usable column index to real column index (skip center_col)
            let real_col = if col_in_row < center_col {
                col_in_row
            } else {
                col_in_row + 1
            };

            let x_offset = real_col as u32 * max_width;
            let y_offset = row as u32 * max_height;

            let offset = Coordinates { x: x_offset, y: y_offset };
            map.merge_at(&ns_maps[placed], offset, None);
            placed += 1;
        }
    }

    // Filler background layer
    let filler_layer = Layer::new(
        "filler".into(),
        vec![Mask::new(
            "filler".into(),
            Rect::from_shape(map.get_shape()).into_many(),
            vec![Effect::Texture(library.get_id("floor_1").unwrap())],
        )],
        0,
    );
    map.load_layer(filler_layer);


    let k8sville_logo = Layer::new(
        "k8sville_logo".into(),
        vec![Mask::new(
            "k8sville_logo".into(),
            Rect::new(
                Coordinates::new(
                    (map.get_shape().width / 2) - 4,
                    map.get_shape().height - 14,
                ),
                Shape::new(8,10),
            ).into_single(),
            vec![Effect::Texture(library.get_id("k8sville_logo").unwrap())]
        )],
        5
    );
    map.load_layer(k8sville_logo);

    // Add hall at bottom
    let hall_shape = Shape { width: 9, height: 5 };
    let hall_map = Map::new(
        "hall".into(),
        vec![Layer::new(
            "hall".into(),
            vec![Mask::new(
                "hall-ground".into(),
                Rect::new(Coordinates::default(), hall_shape).into_many(),
                vec![Effect::Texture(library.get_id("floor_1").unwrap())],
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
