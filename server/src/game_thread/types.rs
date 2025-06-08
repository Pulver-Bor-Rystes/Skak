use std::time::Duration;


#[derive(Debug, Clone)]
pub struct TimeFormat {
    name: String,
    description: String,
    initial: Duration,
    increment: Duration,
}

impl TimeFormat {
    pub fn formats() -> Vec<TimeFormat> {
        vec![
            TimeFormat {
                name: "M5s3".to_string(),
                description: "5min game with a 3sec increment each turn".to_string(),
                initial: Duration::from_secs(5 * 60),
                increment: Duration::from_secs(5),
            },
            TimeFormat {
                name: "M10".to_string(),
                description: "10min game".to_string(),
                initial: Duration::from_secs(10 * 60),
                increment: Duration::from_secs(0),
            },
        ]
    }

    pub fn from(_timeformat: &str) -> TimeFormat {
        TimeFormat::default()
    }

    pub fn default() -> TimeFormat {
        // TimeFormat::formats()[0]
        TimeFormat {
            name: "M5s3".to_string(),
            description: "5min game with a 3sec increment each turn".to_string(),
            initial: Duration::from_secs(5 * 60),
            increment: Duration::from_secs(5),
        }
    }
}

