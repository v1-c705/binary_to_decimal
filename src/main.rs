use inquire::{ui::{Color, RenderConfig, StyleSheet, Styled}, Select, Text};
use colored::*;

fn binary_to_decimal(binary: &str) -> u128 {
    let mut decimal_value = 0;
    let mut power = 0;

    for digit in binary.chars().rev() {
        let bit = digit.to_digit(2).expect("Invalid binary digit") as u128;
        decimal_value += bit * 2u128.pow(power);
        power += 1;
    }

    decimal_value
}

fn decimal_to_binary(mut decimal: u128) -> String {
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
        answer: StyleSheet::new().with_fg(Color::DarkGrey),
        canceled_prompt_indicator: Styled::new(""),
        help_message: StyleSheet::new().with_fg(Color::DarkGrey),
        ..RenderConfig::default()
    };

    loop{
        let modules: Vec<&str> = vec!["Decimal nach Binär", "Binär nach Decimal"];
        let module = Select::new("Was rechnen wir? :", modules)
            .with_render_config(module_style)
            .with_help_message("↑↓ zum Bewegen")
            .prompt();

        let choice = match module{
            Ok(module) => module,
            Err(_) => {
                println!("\x1B[F\x1B[2K\x1B[F");
                ""
            }
        };

        match choice{
            "Decimal nach Binär" => {
                let message = format!("Geben Sie Ihre Nummer in {}", "Decimal".bright_green());
                let input = Text::new(&message)
                .prompt();
                let number_int = match input {
                    Ok(number) => number,
                    Err(_) => {
                        println!("\x1B[F\x1B[2K\x1B[F\x1B[2K\x1B[F");
                        continue
                    }
                };
                
                let number_int = match number_int.parse::<u128>(){
                    Ok(x) => x,
                    Err(_) => {
                        println!("\x1B[F\x1B[2K\x1B[F\x1B[2K\x1B[F");
                        continue
                    }
                };

                let result = decimal_to_binary(number_int);
                println!("\x1B[2K\x1B[F\x1B[2K\x1B[F\x1B[2K\x1B[F");
                println!("{} ist gleich: {} in Binär", number_int.to_string().bright_green(), result.bright_green());
                println!("{}", "----------------------------------".bright_green());
            },
            "Binär nach Decimal" => {
                let message = format!("Geben Sie Ihre Nummer in {}", "Binär".bright_green());
                let input = Text::new(&message)
                .prompt();

                let number_str = match input {
                    Ok(number) => number,
                    Err(_) => {
                        println!("\x1B[F\x1B[2K\x1B[F\x1B[2K\x1B[F");
                        continue
                    }
                };

                if number_str.chars().all(|c| c == '1' || c == '0') == false{
                    println!("\x1B[F\x1B[2K\x1B[F\x1B[2K\x1B[F");
                        continue
                }

                let result = binary_to_decimal(number_str.to_string().as_str());
                println!("\x1B[2K\x1B[F\x1B[2K\x1B[F\x1B[2K\x1B[F");
                println!("{} ist gleich: {} in Decimal", number_str.bright_green(), result.to_string().bright_green());
                println!("{}", "----------------------------------".bright_green());
            },
            _ => ()
        };

        let continue_options: Vec<&str> = vec!["Ja", "Nein (Schließ program)"];
        let continue_handler = Select::new("Möchten Sie weiterhin rechnen? :", continue_options)
            .with_render_config(module_style)
            .with_help_message("↑↓ zum Bewegen")
            .prompt();

        let should_continue = match continue_handler{
            Ok(x) => x,
            Err(_) => ""
        };

        match should_continue{
            "Ja" => {
                println!("\x1B[F\x1B[2K\x1B[F");
                continue
            }
            "Nein (Anwendung schließen)" => break,
            _ => break
        }
    }
}