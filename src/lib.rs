use crate::utils::models::{J2TRoot, J2TTestCase, KaumaTestsRoot};
use crate::utils::schemas::Schema;
use serde_json::{Value, from_str, json};
use std::collections::HashMap;
use std::process::exit;
use uuid::Uuid;
use valico::json_schema;

mod utils;

pub fn kt2j2t(source_json: &str) -> Result<Value, serde_json::Error> {
    let source_root = match validate_json(from_str(source_json)?, Schema::KaumaTest) {
        true => from_str::<KaumaTestsRoot>(source_json)?,
        false => {
            println!("inputted JSON does not conform with schema");
            exit(1)
        }
    };

    let mut target_testcases: HashMap<String, J2TTestCase> = HashMap::new();

    for (test_name, source_case) in source_root.testcases {
        let expected_result = source_root
            .expected_results
            .get(&test_name)
            .cloned()
            .unwrap_or(Value::Null);

        let target_case = J2TTestCase {
            action: source_case.action,
            arguments: source_case.arguments,
            result: expected_result,
            panic: None,
        };

        target_testcases.insert(Uuid::new_v4().to_string(), target_case);
    }

    let target_root = J2TRoot {
        testcases: target_testcases,
    };

    if !validate_json(json!(target_root), Schema::Json2Test) {
        println!("transformed JSON does not conform with schema");
        exit(1)
    }

    Ok(json!(target_root))
}

fn validate_json(json: Value, schema: Schema) -> bool {
    let mut scope = json_schema::Scope::new();

    let schema = scope
        .compile_and_return(from_str(schema.get()).unwrap(), false)
        .unwrap();

    schema.validate(&json).is_valid()
}

#[cfg(test)]
mod tests {
    use crate::utils::schemas::Schema;
    use crate::{kt2j2t, validate_json};
    use serde_json::from_str;

    #[test]
    fn test_schema_compliance() {
        let example_json = r#"
{
  "title": "Calc Action Testcases",
  "description": "calc: integer representation and calculation (incl. division with negative operands)",
  "authors": ["lalelu"],
  "testcases": {
    "14MO7eegDIBbSA7BkfzmS": {
      "action": "calc",
      "arguments": { "lhs": 1, "op": "-", "rhs": -1 }
    }
  },
  "expectedResults": {
    "14MO7eegDIBbSA7BkfzmS": { "answer": 2 }
  }
  }
    "#;
        let result_json = kt2j2t(example_json).unwrap();

        assert_eq!(validate_json(result_json, Schema::Json2Test), true);
        assert_eq!(
            validate_json(from_str(&example_json).unwrap(), Schema::Json2Test),
            false
        );
    }
}
