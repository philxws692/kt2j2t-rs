use kt2j2t_rs::kt2j2t;

fn main() {
    let input_file_arg = std::env::args().nth(1).unwrap();
    let input_file = std::fs::read_to_string(input_file_arg).unwrap();

    let output_file_arg = std::env::args().nth(2).unwrap();

    match kt2j2t(&input_file) {
        Ok(transformed_json) => {
            println!("JSON was successfully transformed");
            std::fs::write(output_file_arg, serde_json::to_string_pretty(&transformed_json).unwrap()).unwrap();
        }
        Err(e) => {
            eprintln!("error with transforming json: {}", e);
        }
    }
}
