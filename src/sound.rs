use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub const IDS: [&str; 8] = [
    "white",
    "pink",
    "brown",
    "blue",
    "violet",
    "green",
    "rain",
    "fireplace",
];

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Sound {
    pub id: String,
    pub level: u8,
}

impl Sound {
    pub fn validate(&self) -> Result<(), String> {
        if !IDS.contains(&self.id.as_str()) {
            return Err(format!(
                "unknown sound '{}'; choose {}",
                self.id,
                IDS.join(", ")
            ));
        }
        if self.level > 100 {
            return Err("sound level must be between 0 and 100".into());
        }
        Ok(())
    }
}

impl FromStr for Sound {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (id, level) = value.split_once(':').unwrap_or((value, "100"));
        if level.is_empty() || !level.bytes().all(|b| b.is_ascii_digit()) {
            return Err("expected SOUND[:LEVEL], with an integer level from 0 to 100".into());
        }
        let sound = Self {
            id: id.into(),
            level: level
                .parse()
                .map_err(|_| "level must be between 0 and 100")?,
        };
        sound.validate()?;
        Ok(sound)
    }
}

pub fn validate_blend(sounds: &[Sound]) -> Result<(), String> {
    if sounds.is_empty() {
        return Err("a blend needs at least one sound".into());
    }
    for (i, sound) in sounds.iter().enumerate() {
        sound.validate()?;
        if sounds[..i].iter().any(|s| s.id == sound.id) {
            return Err(format!("duplicate sound '{}'", sound.id));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parsing() {
        assert_eq!("rain".parse::<Sound>().unwrap().level, 100);
        assert_eq!("pink:0".parse::<Sound>().unwrap().level, 0);
        for input in [
            "", "rain:", "rain:101", "rain:-1", "rain:2:3", "rain:1.5", "grey", "rain:+1",
        ] {
            assert!(input.parse::<Sound>().is_err(), "{input}");
        }
        assert!(validate_blend(&[]).is_err());
        assert!(validate_blend(&["rain".parse().unwrap(), "rain:10".parse().unwrap()]).is_err());
    }
}
