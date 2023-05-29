use dialoguer::Select;
use languages::languages::calculator::{interpreter, lexer, parser, CalculatorError};
use std::io::{stdin, stdout, Write};

pub fn etop() {
    let languages = ["Calculator"];

    let selection = Select::new()
        .with_prompt("Please select a language")
        .default(0)
        .items(&languages[..])
        .interact()
        .unwrap();

    let language = &languages[selection];
    println!("Language chosen: {}", language);

    loop {
        print!("etop # ");
        stdout().flush().expect("flush error");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input_clone = input.clone().trim().to_string();

        let result = match *language {
            "Calculator" => match etop_calculator(input_clone.as_str()) {
                Ok(str_result) => str_result,
                Err(e) => match e {
                    CalculatorError::Parser(parser_error) => {
                        format!("Parser error: {:?}", parser_error)
                    }
                    CalculatorError::Lexer(lexer_error) => {
                        format!("Lexer error: {:?}", lexer_error)
                    }
                    CalculatorError::Interpreter(interpreter_error) => {
                        format!("Interpreter error: {:?}", interpreter_error)
                    }
                },
            },
            _ => String::from(""),
        };
        println!("- : {}", result);
    }
}

fn etop_calculator(input: &str) -> Result<String, CalculatorError> {
    let lexed = lexer::lexer(input, 0).map_err(CalculatorError::Lexer)?;
    let parsed = parser::parser(lexed).map_err(CalculatorError::Parser)?;
    let interpreted = interpreter::interpreter(parsed).map_err(CalculatorError::Interpreter)?;
    Ok(interpreted.to_string())
}
