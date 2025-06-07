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

    map.load_layer(filler_layer);

    // let hall = Map {
    //     name: "hall".to_string(),
    //     layers: vec![
    //         Layer::new(
    //             "hall".into(),
    //             LayerType::Texture,
    //             Shape { width: 6, height: 10 },
    //             vec![
    //                 Mask::new(
    //                     "hall".into(),
    //                     Selector::Block((Coordinates { x: 0, y: 0 }, Coordinates { x: 5, y: 9 })),
    //                     Effect {
    //                         texture_id: Some(2),
    //                         ..Default::default()
    //                     }
    //                 )
    //             ],
    //             2
// 
    //         )
    //     ]
    // };
    // let curr_shape = map.get_shape().clone();
// 
    // map.merge_at(&hall, Coordinates { x: curr_shape.width / 2, y: curr_shape.height - 1 });

    map
}
