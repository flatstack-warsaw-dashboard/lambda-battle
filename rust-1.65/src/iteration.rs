use serde::{Deserialize, Serialize};

const LANG: &str = "rust-1.65";

#[derive(Serialize, Deserialize)]
pub struct Iteration {
    pub lang_case: String,
    pub iteration: u64,
    pub raw_event: String,
}

#[derive(Serialize, Deserialize)]
struct IterationInner {
    iteration: u64,
}


impl TryFrom<&String> for Iteration {
    type Error = serde_json::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        serde_json::from_str::<IterationInner>(value.as_str())
            .map(|item| Iteration {
                lang_case: LANG.parse().unwrap(),
                iteration: item.iteration,
                raw_event: value.clone(),
            })
    }
}
