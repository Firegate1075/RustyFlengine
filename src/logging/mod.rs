use chrono::{DateTime, Local};
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::file::FileAppender;
use log4rs::{Config, Handle};
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log::LevelFilter;

pub struct LoggingConfig {
    uci_enabled: bool,
    console_enabled: bool,
    file_enabled: bool,
    file_timestamp_format: String,
    file_prefix: String,
    log_level: LevelFilter,
}

impl LoggingConfig {
    pub fn new(
        uci_enabled: bool,
        console_enabled: bool,
        file_enabled: bool,
        file_timestamp_format: String,
        file_prefix: String,
        log_level: LevelFilter,
    ) -> Self {
        LoggingConfig{
            uci_enabled,
            console_enabled,
            file_enabled,
            file_timestamp_format,
            file_prefix,
            log_level,
        }
    }

    pub fn uci_enabled(&self) -> bool {
        self.uci_enabled
    }

    pub fn console_enabled(&self) -> bool {
        self.console_enabled
    }

    pub fn file_enabled(&self) -> bool {
        self.file_enabled
    }

    pub fn file_timestamp_format(&self) -> &str {
        &self.file_timestamp_format
    }
    pub fn file_prefix(&self) -> &str {
        &self.file_prefix
    }

    pub fn set_uci_enabled(&mut self, uci_enabled: bool) {
        self.uci_enabled = uci_enabled;
    }

    pub fn set_console_enabled(&mut self, console_enabled: bool) {
        self.console_enabled = console_enabled;
    }

    pub fn set_file_enabled(&mut self, file_enabled: bool) {
        self.file_enabled = file_enabled;
    }

    pub fn set_file_timestamp_format(&mut self, file_timestamp_format: String) {
        self.file_timestamp_format = file_timestamp_format;
    }

    pub fn set_file_prefix(&mut self, file_prefix: String) {
        self.file_prefix = file_prefix;
    }
}

/// method to set up logging from config
pub fn setup_logger(logging_config: &LoggingConfig) -> Handle{
    let config = compute_config(logging_config);

    log4rs::init_config(config).unwrap()
}

pub fn update_logger(logging_config: &LoggingConfig, handle: &Handle) {
    let config = compute_config(logging_config);
    handle.set_config(config);
}

fn compute_config(logging_config: &LoggingConfig) -> Config {
    // compute log file name
    let local_time: DateTime<Local> = Local::now();
    let file_name = format!("{}{}.log",
                            logging_config.file_prefix(),
                            local_time.format(logging_config.file_timestamp_format()),
    );

    // build appenders
    let uci = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "info string {date(%Y-%m-%d %H:%M:%S)} {level:<6} {module} - {message}{n}"
        )))
        .build();

    let console = ConsoleAppender::builder()
        .target(Target::Stderr)
        .encoder(Box::new(PatternEncoder::new(
            "{date(%Y-%m-%d %H:%M:%S)} {level:<6} {module} - {message}{n}"
        )))
        .build();

    let log_file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{date(%Y-%m-%d %H:%M:%S)} {level:<6} {module} - {message}{n}"
        )))
        .build(file_name)
        .unwrap();

    let mut config = Config::builder();

    // add appenders to config depending on the config
    if logging_config.file_enabled {
        config = config.appender(Appender::builder().build("log_file", Box::new(log_file)));
    }
    if logging_config.console_enabled {
        config = config.appender(Appender::builder().build("console", Box::new(console)));
    }
    if logging_config.uci_enabled {
        config = config.appender(Appender::builder().build("uci", Box::new(uci)));
    }

    let mut root = Root::builder();
    // enable appenders depending on the config
    if logging_config.file_enabled {
        root = root.appender("log_file");
    }
    if logging_config.console_enabled {
        root = root.appender("console");
    }
    if logging_config.uci_enabled {
        root = root.appender("uci");
    }


    config.build(root.build(logging_config.log_level)).unwrap()
}