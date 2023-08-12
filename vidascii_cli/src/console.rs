use colored::Colorize;

#[derive(Debug)]
pub enum Level {
    Info,
    Success,
    #[allow(dead_code)]
    Warn,
    Error,
}

pub fn console_log<T: ToString>(msg: T, level: Level) {
    let lvl_str = format!("{level:?}").to_uppercase();
    let msg = format!("[{lvl_str}]: {}", msg.to_string());

    let color_msg = match level {
        Level::Info => msg.blue(),
        Level::Success => msg.green(),
        Level::Warn => msg.yellow().black(),
        Level::Error => msg.red().bold(),
    };
    println!("{color_msg}");
}
