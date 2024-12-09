use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct ImageSet {
    lowest_to_highest_res: Vec<String>,
}

impl ImageSet {
    pub fn new(lowest_to_highest_res: Vec<String>) -> Self {
        Self {
            lowest_to_highest_res,
        }
    }

    pub fn empty() -> Self {
        Self {
            lowest_to_highest_res: Vec::new(),
        }
    }

    pub fn to_highest_res(&self) -> &str {
        let maybe_src = self.lowest_to_highest_res.last();
        match maybe_src {
            Some(src) => src,
            None => "",
        }
    }
}
