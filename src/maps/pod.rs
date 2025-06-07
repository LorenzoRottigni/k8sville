use std::any::Any;

use rpgx::{library::Library, prelude::*};

pub fn pod_map(library: &Library<Box<dyn Any>>) -> Map {
    Map::new("default".into(), vec![], Coordinates::default())
}
