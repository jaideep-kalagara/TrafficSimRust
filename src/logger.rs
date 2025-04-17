use colored::Colorize;

pub fn log_info(msg: &str) {
    println!("{} {}", "[INFO]".blue(), msg.blue());
}

pub fn log_warning(msg: &str) {
    println!("{} {}", "[WARNING]".yellow(), msg.yellow());
}

pub fn log_error(msg: &str) {
    println!("{} {}", "[ERROR]".red(), msg.red());
}