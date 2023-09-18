use std::fs;
use std::path::Path;

use crate::{Error, Trigger};

pub const EXTENSION: &str = ".yml";

pub fn load_trigger(path: &Path) -> Result<Trigger, Error> {
    let file = fs::File::open(path)?;
    Trigger::from_yaml(file)
}
