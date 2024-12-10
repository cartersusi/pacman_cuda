use dialoguer::{Select, theme::ColorfulTheme};
use dialoguer::console::{Term, Style, Color};

const CONFIRM: &[&str] = &["Yes", "No"];

pub enum Lvl {
    Warning,
    Success,
    Error,
    Info,
}

#[derive(PartialEq)]
pub enum Confirm {
    Yes,
    No,
}

pub fn log(msg_type: Lvl, msg: &str) {
    let color = match msg_type {
        Lvl::Warning => Color::Yellow,
        Lvl::Success => Color::Green,
        Lvl::Error => Color::Red,
        Lvl::Info => Color::White,
    };
    Term::stdout().write_line(
        &Style::new().fg(color).bold().apply_to(msg).to_string()
    ).unwrap();
}

pub fn get_choice(c1: &str, c2: &str, prompt: &str) -> usize {
    let choices = &[c1, c2];
    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&choices[..])
        .default(0)
        .interact()
        .expect("Failed to get user choice");

    choice
}

pub fn confirm(prompt: &str) -> Confirm {
    let choice = get_choice(CONFIRM[0], CONFIRM[1], prompt);
    match choice {
        0 => Confirm::Yes,
        1 => Confirm::No,
        _ => panic!("Unknown Error Occurred"),
    }
}