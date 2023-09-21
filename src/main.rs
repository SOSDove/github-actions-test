mod values_yaml;
use std::io::{stdout, Write};
use std::io;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand, Result,
    event,
};
use serde_yaml;
use crate::values_yaml::{ApplicationValues, Choice};

fn main() {
    let data = std::fs::read_to_string("C:\\Users\\edbafjdu\\RustroverProjects\\sos-application-manager\\test_yaml\\test.yaml").unwrap();
    let app_values: ApplicationValues = serde_yaml::from_str(&data).unwrap();
    let apps_map = app_values.to_applications_map();

    println!("Select an App to view more");
    for (index, (app_name, _)) in apps_map.iter() {
        println!("({}) - {}", index, app_name);
    }

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");

    let choice: i32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return; // or handle the error as you see fit
        }
    };

    if let Some((app_name, app)) = apps_map.get(&choice) {
        // Display app name
        execute!(
        stdout(),
        SetForegroundColor(Color::Yellow),
        Print(format!("Details for {}:\n", app_name)),
        ResetColor
    ).expect("Cudgel");

        // Display framework
        execute!(
        stdout(),
        SetForegroundColor(Color::Green),
        Print("Framework: "),
        ResetColor,
        Print(format!("{}\n", app.framework))
    ).expect("Follf");

        // Display routes
        execute!(
        stdout(),
        SetForegroundColor(Color::Green),
        Print("Routes: "),
        ResetColor,
        Print(format!("{:?}\n", app.routes))
    ).expect("oops");

        // Display environment variables
        for (index, env) in app.extraEnv.iter().enumerate() {
            execute!(
            stdout(),
            SetForegroundColor(Color::Magenta),
            Print(format!("[{}] - [Name] {}: ", index + 1, env.name)),
            ResetColor,
            SetForegroundColor(Color::Cyan),
            Print(format!(" - [Value]: {}\n", env.value)),
            ResetColor
        ).expect("Oops");
        }
    }

    display_choices(vec!["(C)hange an env value", "(A)dd a database", "(D)elete app"]);

    let mut action = String::new();
    io::stdin().read_line(&mut action).expect("Failed to read line");
}

fn display_choices(choices: Vec<&str>) {
    let mut choice_structs = vec!();
    for choice_str in choices {
        // Assuming the format is always "(X)..."
        let short = choice_str.chars().nth(1).unwrap_or_default();
        let name = choice_str[1..].to_string();

        let choice = Choice { short, name };
        choice_structs.push(choice);
    }
    // Initial prompt
    execute!(
        stdout(),
        SetForegroundColor(Color::DarkBlue),
        Print("What would you like to do? "),
        ResetColor
    ).expect("Oops");

    let colors = [Color::Cyan, Color::DarkGreen, Color::DarkRed];

    // Display each choice with different styles
    for (index, choice) in choice_structs.iter().enumerate() {
        let color = colors[index % colors.len()];  // Cycle through colors
        execute!(
            stdout(),
            SetForegroundColor(color),
            Print(format!("{}: ", choice)),  // Displaying choice followed by a colon
            ResetColor
        ).expect("Oops");
    }
}