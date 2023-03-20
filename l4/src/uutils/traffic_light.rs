use anyhow::{anyhow, Result};
use std::fmt::{self, Display, Formatter};
use std::{str::FromStr, time::Duration};

#[derive(Debug, PartialEq)]
pub enum TrafficLight {
    Red,
    Green,
    Yellow,
}

impl Default for TrafficLight {
    fn default() -> Self {
        Self::Red
    }
}

pub trait LightOn {
    fn light_on(&self) -> Duration;
}

impl LightOn for TrafficLight {
    fn light_on(&self) -> Duration {
        match &self {
            Self::Red => Duration::from_secs(11),
            Self::Green => Duration::from_secs(13),
            Self::Yellow => Duration::from_secs(2),
        }
    }
}

impl FromStr for TrafficLight {
    type Err = anyhow::Error;
    fn from_str(str: &str) -> Result<Self> {
        match str {
            "red" => Ok(Self::Red),
            "green" => Ok(Self::Green),
            "yellow" => Ok(Self::Yellow),
            _ => Err(anyhow!("wrong light")),
        }
    }
}

impl Display for TrafficLight {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::{LightOn, TrafficLight};

    #[test]
    fn test_light_on() {
        let tname = nameof::name_of_type!(TrafficLight);

        let mut durs;
        let mut light = TrafficLight::Red;
        durs = light.light_on();
        println!("{}-{} {:?}", tname, light, durs);

        light = "green".parse().unwrap();
        durs = light.light_on();
        println!("{}-{} {:?}", tname, light, durs);

        assert_eq!(light, TrafficLight::Green);

        light = TrafficLight::Yellow;
        durs = light.light_on();
        println!("{}-{} {:?}", tname, light, durs);
    }
}
