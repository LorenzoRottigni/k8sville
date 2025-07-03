use std::any::Any;
use rpgx::{library::Library, prelude::*};

pub fn deployment_map(library: &Library<Box<dyn Any>>, deployment: crate::kube::k8s::Deployment) -> Map {
    let total = deployment.pods.len();
    if total == 0 {
        return Map::new("default".into(), vec![], Coordinates::default());
    }

    let cols = (total as f64).sqrt().ceil() as usize;
    let rows = ((total + cols - 1) / cols) as usize;
    let mut map = Map::new("default".into(), vec![], Coordinates::default());

    let mut deploy_maps = vec![];
    let mut max_width = 0;
    let mut max_height = 0;

    for deployment in &deployment.pods {
        let deploy_map = crate::presets::pod::pod_preset(library, deployment.clone());
        max_width = max_width.max(deploy_map.get_shape().width);
        max_height = max_height.max(deploy_map.get_shape().height);
        deploy_maps.push(deploy_map);
    }

    let full_rows = total / cols;
    let remainder = total % cols;

    // Build padded map grid with custom last row layout
    let mut padded_maps: Vec<Option<Map>> = vec![];

    // Fill full rows as usual
    for map in deploy_maps.iter().take(full_rows * cols) {
        padded_maps.push(Some(map.clone()));
    }

    // Handle custom alignment for last row
    if remainder > 0 {
        let mut last_row = vec![None; cols];
        let remaining_maps = &deploy_maps[full_rows * cols..];
        for (i, map) in remaining_maps.iter().enumerate() {
            let col = if i % 2 == 0 {
                // 0 → 0, 2 → 1, 4 → 2, ...
                i / 2
            } else {
                // 1 → last, 3 → last - 1, 5 → last - 2, ...
                cols - 1 - (i / 2)
            };
            last_row[col] = Some(map.clone());
        }
        padded_maps.extend(last_row);
    }

    // Merge all maps into final map
    for (i, maybe_map) in padded_maps.into_iter().enumerate() {
        let row = (i / cols) as u32;
        let col = (i % cols) as u32;

        if let Some(deploy_map) = maybe_map {
            let x_offset = col * max_width;
            let y_offset = row * max_height;
            map.merge_at(&deploy_map, Coordinates { x: x_offset, y: y_offset }, None);
        }
    }

    // Add filler
    let filler_layer = Layer::new(
        "filler".into(),
        vec![Mask::new(
            "filler".into(),
            Rect::new(Coordinates::default(), map.get_shape()).into_many(),
            vec![Effect::Texture(library.get_id("floor_1").unwrap())]
        )],
        0,
    );
    map.load_layer(filler_layer);

    // Add hall
    let hall_shape = Shape { width: 9, height: 5 };
    let hall_map = Map::new(
        "hall".into(),
        vec![
            Layer::new(
                "hall".into(),
                vec![Mask::new(
                    "hall-ground".into(),
                    Rect::new(Coordinates::default(), hall_shape).into_many(),
                    vec![Effect::Texture(library.get_id("floor_1").unwrap())]
                )],
                1,
            ),
            Layer::new(
                "action-back".into(),
                vec![Mask::new(
                    "action-back".into(),
                    Rect::new(Coordinates::new(3,4), Shape::new(2,1)).into_many(),
                    vec![Effect::Texture(library.get_id("floor_3").unwrap()), Effect::Action(library.get_id("go_back").unwrap())]
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
