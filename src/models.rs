use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subject {
    pub name: String,
}

impl Subject {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct File {
//     pub name: String,
//     pub subject: String,
//     pub path: String,
// }
