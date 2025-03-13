use inquire::{ui::{Color, RenderConfig, StyleSheet, Styled}, Select, Text};
use colored::*;

fn binary_to_decimal(binary: &str) -> u32 {
    let mut decimal_value = 0;
    let mut power = 0;

    // Iterate over the binary string in reverse order
    for digit in binary.chars().rev() {
        let bit = digit.to_digit(2).expect("Invalid binary digit");
        decimal_value += bit * 2u32.pow(power);
        power += 1;
    }

    decimal_value
}

fn decimal_to_binary(mut decimal: u32) -> String {
    let mut binary_string = String::new();

    if decimal == 0 {
        return "0".to_string();
    }

    while decimal > 0 {
        let remainder = decimal % 2;
        binary_string.push_str(&remainder.to_string());
        decimal /= 2;
    }

    binary_string.chars().rev().collect()
}

fn main() {
    let module_style = RenderConfig{
        highlighted_option_prefix: Styled::new(">").with_fg(Color::LightGreen),
        prompt_prefix: Styled::new("").with_fg(Color::LightGreen),
        selected_option: Some(StyleSheet::new().with_fg(Color::LightGreen)),
        canceled_prompt_indicator: Styled::new(""),
        ..RenderConfig::default()
    };

    loop{
        let modules: Vec<&str> = vec!["Decimal > Binär", "Binär > Decimal"];
        let module = Select::new("Was möchten Sie? :", modules)
            .with_render_config(module_style)
            .prompt();

        let choice = match module{
            Ok(module) => module,
            Err(_) => "None"
        };

        match choice{
            "Decimal > Binär" => {
                let message = format!("Geben Sie {} -> ", "a₁".bright_blue());
                let input = Text::new(&message)
                .prompt();
                let number_int = match input {
                    Ok(number) => number,
                    Err(_) => String::from("None")
                };
                let result = decimal_to_binary(number_int.parse().unwrap());
                println!("Ihre Nummer {} ist gleich: {} in Decimal", number_int.bright_green(), result.bright_blue())
            },
            "Binär > Decimal" => {
                let message = format!("Geben Sie Ihre Nummer in {}", "Binär".bright_blue());
                let input = Text::new(&message)
                .prompt();
                let number_str = match input {
                    Ok(number) => number,
                    Err(_) => String::from("None")
                };
                let result = binary_to_decimal(number_str.as_str());
                println!("Ihre Nummer {} ist gleich: {} in Binär", number_str.bright_green(), result.to_string().bright_blue())
            },
            _ => ()
        };

        let continue_options: Vec<&str> = vec!["Ja", "Nein (Schließ program)"];
        let continue_handler = Select::new("Möchten Sie wieder rechnen? :", continue_options)
            .with_render_config(module_style)
            .prompt();

        let should_continue = match continue_handler{
            Ok(i) => i,
            Err(_) => "None"
        };

        match should_continue{
            "Ja" => continue,
            "Nein (Schließ program)" => break,
            _ => break
        }
    }
}