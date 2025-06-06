use std::any::Any;

use rpgx::{library::Library, prelude::*};

pub fn namespace_map(library: &Library<Box<dyn Any>>, namespace: crate::kube::k8s::Namespace) -> Map {
    let total = namespace.deployments.len();
    if total == 0 {
        return Map::new("default".into(), vec![]);
    }

    let cols = (total as f64).sqrt().ceil() as usize;
    let rows = ((total + cols - 1) / cols) as usize;
    let mut map = Map::new("default".into(), vec![]);

    // Precompute all deployment maps to determine the largest shape
    let mut ns_maps = vec![];
    let mut max_width = 0;
    let mut max_height = 0;

    for deployment in &namespace.deployments {
        let ns_map = crate::presets::deployment::deployment_preset(library, deployment.clone());
        let shape = ns_map.get_shape();
        max_width = max_width.max(shape.width);
        max_height = max_height.max(shape.height);
        ns_maps.push(Some(ns_map));
    }

    // Pad to full grid with `None`
    while ns_maps.len() < rows * cols {
        ns_maps.push(None);
    }

    for (i, maybe_map) in ns_maps.into_iter().enumerate() {
        let row = (i / cols) as i32;
        let col = (i % cols) as i32;

        if let Some(ns_map) = maybe_map {
            let x_offset = col * max_width;
            let y_offset = row * max_height;
            map.merge_at(&ns_map, Coordinates { x: x_offset, y: y_offset });
        }
    }

    map
}
