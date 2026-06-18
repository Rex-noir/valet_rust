use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub php: HashMap<String, String>,
}
