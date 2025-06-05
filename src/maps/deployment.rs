use std::any::Any;

use rpgx::{library::Library, prelude::*};
use crate::presets::namespace::namespace_preset;

pub fn deployment_map(library: &Library<Box<dyn Any>>, pods: &Vec<String>) -> Map {

    Map::new("default".into(), vec![])

}
