use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct KaumaTestsTestCase {
    pub action: String,
    pub arguments: Value,
}

#[derive(Debug, Deserialize)]
pub struct KaumaTestsRoot {
    pub testcases: HashMap<String, KaumaTestsTestCase>,
    #[serde(rename = "expectedResults")]
    pub expected_results: HashMap<String, Value>,
}

// Structures for the destination scheme for json2tests-rs

#[derive(Debug, Serialize)]
pub struct J2TTestCase {
    pub action: String,
    pub arguments: Value,

    pub result: Value,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub panic: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct J2TRoot {
    pub testcases: HashMap<String, J2TTestCase>,
}
