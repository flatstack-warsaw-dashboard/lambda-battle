use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use aws_sdk_dynamodb::{Client};
use aws_sdk_dynamodb::error::{GetItemError, PutItemError};
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::output::{PutItemOutput};
use aws_sdk_dynamodb::types::SdkError;
use crate::iteration::Iteration;

pub enum PutIterationErrors {
    TableNameNotSet,
    SaveError(SdkError<PutItemError>),
}

impl Display for PutIterationErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PutIterationErrors::TableNameNotSet => write!(f, "Table name env variable is not set"),
            PutIterationErrors::SaveError(sdk_error) => write!(f, "Saving item iteration error {}", sdk_error),
        }
    }
}

pub async fn add_iteration(client: &Client, iteration: &Iteration) -> Result<PutItemOutput, PutIterationErrors> {
    let table_name = std::env::var("TABLE");
    if table_name.is_err() {
        return Err(PutIterationErrors::TableNameNotSet);
    }

    let lang_case = AttributeValue::S(iteration.lang_case.clone());
    let iteration_n = AttributeValue::N(iteration.iteration.to_string());
    let raw_event = AttributeValue::S(iteration.raw_event.clone());

    client
        .put_item()
        .table_name(table_name.unwrap())
        .item("langCase", lang_case)
        .item("iteration", iteration_n)
        .item("raw_event", raw_event)
        .send()
        .await
        .map_err(|error| PutIterationErrors::SaveError(error))
}

#[derive(Debug)]
pub enum FindIterationError {
    TableNameNotSet,
    FindItemError(SdkError<GetItemError>),
    MapResultError(MapValueError),
}

impl Display for FindIterationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FindIterationError::TableNameNotSet => write!(f, "Table name env variable is not set"),
            FindIterationError::FindItemError(sdk_error) => write!(f, "previous item is not founded {}", sdk_error),
            FindIterationError::MapResultError(error) => write!(f, "map result error {}", error)
        }
    }
}

pub async fn find_iteration(client: &Client, iteration: &Iteration) -> Result<Option<Iteration>, FindIterationError> {
    let table_name = std::env::var("TABLE");
    if table_name.is_err() {
        return Err(FindIterationError::TableNameNotSet);
    }

    let lang = AttributeValue::S(iteration.lang_case.clone());
    let iteration_n = AttributeValue::N((iteration.iteration - 1).to_string());

    client
        .get_item()
        .table_name(table_name.unwrap())
        .key("langCase", lang)
        .key("iteration", iteration_n)
        .send()
        .await
        .map_err(|error| FindIterationError::FindItemError(error))
        .and_then(|raw| {
            if raw.item.is_none() {
                return Ok(None);
            }
            match map_result(raw.item().unwrap()) {
                Ok(val) => Ok(Some(val)),
                Err(err) => Err(FindIterationError::MapResultError(err))
            }
        })
}


fn map_result(item: &HashMap<String, AttributeValue>) -> Result<Iteration, MapValueError> {
    Ok(Iteration {
        lang_case: get_lang_value(item)?,
        raw_event: get_raw_event_value(item)?.clone(),
        iteration: get_iteration_value(item)?,
    })
}

#[derive(Debug)]
pub enum MapValueError {
    KeyMissing(String),
    UnexpectedReturnValueType(String, AttributeValue),
    ParsingU64Error(String),
}

impl Display for MapValueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MapValueError::KeyMissing(key) => write!(f, "missing {} from dynamodb response", key),
            MapValueError::UnexpectedReturnValueType(key, _) => write!(f, "unexpected {} value type returned from dynamodb", key),
            MapValueError::ParsingU64Error(key) => write!(f, "error parsing {}", key)
        }
    }
}

fn get_lang_value(result: &HashMap<String, AttributeValue>) -> Result<String, MapValueError> {
    let lang_value = result.get("langCase");
    if lang_value.is_none() {
        return Err(MapValueError::KeyMissing("langCase".parse().unwrap()));
    }

    match lang_value.unwrap() {
        AttributeValue::S(val) => Ok(val.clone()),
        _ => Err(MapValueError::UnexpectedReturnValueType("langCase".parse().unwrap(), lang_value.unwrap().clone()))
    }
}

fn get_iteration_value(result: &HashMap<String, AttributeValue>) -> Result<u64, MapValueError> {
    let lang_value = result.get("iteration");
    if lang_value.is_none() {
        return Err(MapValueError::KeyMissing("iteration".parse().unwrap()));
    }

    match lang_value.unwrap() {
        AttributeValue::N(val) => {
            match val.parse::<u64>() {
                Ok(value) => Ok(value),
                Err(_) => Err(MapValueError::ParsingU64Error("iteration".parse().unwrap()))
            }
        }
        _ => Err(MapValueError::UnexpectedReturnValueType("iteration".parse().unwrap(), lang_value.unwrap().clone()))
    }
}

fn get_raw_event_value(result: &HashMap<String, AttributeValue>) -> Result<&String, MapValueError> {
    let lang_value = result.get("raw_event");
    if lang_value.is_none() {
        return Err(MapValueError::KeyMissing("raw_event".parse().unwrap()));
    }

    match lang_value.unwrap() {
        AttributeValue::S(val) => Ok(val),
        _ => Err(MapValueError::UnexpectedReturnValueType("raw_event".parse().unwrap(), lang_value.unwrap().clone()))
    }
}
