pub enum Schema {
    Json2Test,
    KaumaTest,
}

impl Schema {
    pub fn get<'a>(self) -> &'a str {
        match self {
            Schema::Json2Test => JSON2TEST_SCHEMA,
            Schema::KaumaTest => KT_SCHEMA,
        }
    }
}

pub const JSON2TEST_SCHEMA: &str = r#"
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://raw.githubusercontent.com/niri81/json2tests-rs/refs/heads/main/schema.json",
  "title": "Test Cases Schema",
  "description": "Schema for a collection of named test vectors.",
  "type": "object",
  "properties": {
    "testcases": {
      "description": "An object where keys are the test names and values are the test vectors.",
      "type": "object",
      "additionalProperties": {
        "title": "Test Vector",
        "type": "object",
        "properties": {
          "action": {
            "description": "The operation to be performed, e.g., 'add'.",
            "type": "string"
          },
          "arguments": {
            "description": "The input data for the action.",
            "additionalProperties": true
          },
          "result": {
            "description": "The expected result of the action."
          },
          "panic": {
            "description": "The test is expected to panic. When supplied with a string, the panic message will be checked against it.",
            "type": ["boolean", "string"]
          }
        },
        "required": ["action"],
        "additionalProperties": false
      },
      "minProperties": 1
    }
  },
  "required": ["testcases"],
  "additionalProperties": false
}
"#;

pub const KT_SCHEMA: &str = r#"
{
  "$schema": "http://json-schema.org/draft-07/schema",
  "$id": "https://raw.githubusercontent.com/sarsum/TINF23CS-kauma-tests/refs/heads/main/schema.json",
  "title": "Test Vectors",
  "description": "Test Vector including input and expected result to test the kauma application",
  "additionalProperties": false,
  "type": "object",
  "properties": {
    "title": { "type": "string" },
    "description": { "type": "string" },
    "authors": { "type": "array", "items": { "type": "string" } },
    "testcases": {
      "type": "object",
      "minProperties": 1,
      "description": "Map of testcase name -> testcase",
      "additionalProperties": {
        "type": "object",
        "additionalProperties": false,
        "properties": {
          "action": { "type": "string" },
          "arguments": {
            "type": "object"
          }
        },
        "required": ["action", "arguments"]
      }
    },
    "expectedResults": {
        "type": "object",
        "properties": {
            "responses": {
                "type": "object",
                "minProperties": 1,
                "description": "Results of testcases. Sometimes for each testcase a result, sometimes one result computed by multiple actions.",
                "additionalProperties": {
                  "type": ["object", "null"]
                }
            }
        }
    }
  },
  "required": ["title", "description", "testcases", "expectedResults"]
}
"#;
