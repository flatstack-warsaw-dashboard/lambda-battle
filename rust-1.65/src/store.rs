use std::collections::HashMap;
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
    MapResultError(MapResultError),
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
            if raw.item().is_none() {
                return Ok(None);
            }
            match map_result(raw.item().unwrap()) {
                Ok(val) => Ok(Some(val)),
                Err(err) => Err(FindIterationError::MapResultError(err))
            }
        })
}

#[derive(Debug)]
pub enum MapResultError {
    MapValueError(MapValueError),
}

fn map_result(item: &HashMap<String, AttributeValue>) -> Result<Iteration, MapResultError> {
    let lang = get_lang_value(item);
    if lang.is_err() {
        return Err(MapResultError::MapValueError(lang.err().unwrap()));
    }

    let iteration_n = get_iteration_value(item);
    if iteration_n.is_err() {
        return Err(MapResultError::MapValueError(iteration_n.err().unwrap()));
    }

    let raw_event = get_raw_event_value(item);
    if raw_event.is_err() {
        return Err(MapResultError::MapValueError(raw_event.err().unwrap()));
    }

    Ok(Iteration {
        lang_case: lang.unwrap(),
        raw_event: raw_event.unwrap(),
        iteration: iteration_n.unwrap(),
    })
}

#[derive(Debug)]
pub enum MapValueError {
    KeyMissing(String),
    UnexpectedReturnValueType(String),
    ParsingValueError(String),
}

fn get_lang_value(result: &HashMap<String, AttributeValue>) -> Result<String, MapValueError> {
    let lang_value = result.get("langCase");
    if lang_value.is_none() {
        return Err(MapValueError::KeyMissing("langCase".parse().unwrap()));
    }

    match lang_value.unwrap() {
        AttributeValue::S(val) => Ok(val.clone()),
        _ => Err(MapValueError::UnexpectedReturnValueType("langCase".parse().unwrap()))
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
                Err(_) => Err(MapValueError::ParsingValueError("iteration".parse().unwrap()))
            }
        }
        _ => Err(MapValueError::UnexpectedReturnValueType("iteration".parse().unwrap()))
    }
}

fn get_raw_event_value(result: &HashMap<String, AttributeValue>) -> Result<String, MapValueError> {
    let lang_value = result.get("raw_event");
    if lang_value.is_none() {
        return Err(MapValueError::KeyMissing("raw_event".parse().unwrap()));
    }

    match lang_value.unwrap() {
        AttributeValue::S(val) => Ok(val.clone()),
        _ => Err(MapValueError::UnexpectedReturnValueType("raw_event".parse().unwrap()))
    }
}
