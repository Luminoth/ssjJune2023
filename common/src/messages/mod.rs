pub mod duel;

use serde::Serialize;

pub trait Message: Serialize + Into<String> {}
