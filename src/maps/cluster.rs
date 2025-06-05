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

    map
}
