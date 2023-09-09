use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ConfigLiteral {
    trigger: TriggerLiteral,
    executor: Executor,
}

#[derive(Debug)]
pub struct Config {
    trigger: Trigger,
    executor: Executor,
}

#[derive(Debug, Serialize, Deserialize)]
struct Executor {
    #[serde(rename = "bash")]
    bash_exec_string: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TriggerLiteral {
    record_interval: String,
    grace_period: String,
    threshold: String,
}

#[derive(Debug)]
struct Trigger {
    record_interval: std::time::Duration,
    grace_period: std::time::Duration,
    threshold: f64,
}

impl Config {
    pub fn from_toml(config_path: &str) -> Self {
        let contents = std::fs::read_to_string(config_path)
            .unwrap_or_else(|_| panic!("Failed to read {}", config_path));
        let config = toml::from_str::<ConfigLiteral>(&contents).unwrap();
        let trigger = Trigger {
            record_interval: parse_duration::parse(&config.trigger.record_interval).unwrap(),
            grace_period: parse_duration::parse(&config.trigger.grace_period).unwrap(),
            threshold: config.trigger.threshold.parse::<f64>().unwrap(),
        };
        let executor = Executor {
            bash_exec_string: config.executor.bash_exec_string,
        };
        Config { trigger, executor }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_read_config() {
        let config_path = "config/config.toml";
        let config = Config::from_toml(config_path);
        dbg!(&config);
    }
}
