use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Copy, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum Weather {
    #[default]
    Sunny,
    Cloudy,
    WeakRain,
    Rain,
    Thunder,
}

impl Weather {
    pub fn better(self) -> Self {
        match self {
            Weather::Sunny => Weather::Sunny,
            Weather::Cloudy => Weather::Sunny,
            Weather::WeakRain => Weather::Cloudy,
            Weather::Rain => Weather::WeakRain,
            Weather::Thunder => Weather::Rain,
        }
    }
    pub fn worse(self) -> Self {
        match self {
            Weather::Sunny => Weather::Cloudy,
            Weather::Cloudy => Weather::WeakRain,
            Weather::WeakRain => Weather::Rain,
            Weather::Rain => Weather::Thunder,
            Weather::Thunder => Weather::Thunder,
        }
    }
}