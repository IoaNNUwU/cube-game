use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Hash, Debug)]
#[derive(Ord, PartialOrd, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum Weather {
    Thunder,
    Rain,
    WeakRain,
    Cloudy,
    #[default]
    Sunny,
}

impl Weather {
    fn better(self) -> Self {
        match self {
            Weather::Thunder => Weather::Rain,
            Weather::Rain => Weather::WeakRain,
            Weather::WeakRain => Weather::Cloudy,
            Weather::Cloudy => Weather::Sunny,
            Weather::Sunny => Weather::Sunny,
        }
    }
    fn worse(self) -> Self {
        match self {
            Weather::Thunder => Weather::Thunder,
            Weather::Rain => Weather::Thunder,
            Weather::WeakRain => Weather::Rain,
            Weather::Cloudy => Weather::WeakRain,
            Weather::Sunny => Weather::Cloudy,
        }
    }
}