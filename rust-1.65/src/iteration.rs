use serde::{Deserialize, Serialize};

const LANG: &str = "rust-1.65";


pub struct Iteration<'a> {
    lang_case: &'a str,
    iteration: u64,
    raw_event: String,
}

#[derive(Serialize, Deserialize)]
struct IterationInner {
    iteration: u64,
}


impl TryFrom<&String> for Iteration<'static> {
    type Error = serde_json::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        serde_json::from_str::<IterationInner>(value.as_str())
            .map(|item| Iteration {
                lang_case: LANG,
                iteration: item.iteration,
                raw_event: value.clone(),
            })
    }
}
