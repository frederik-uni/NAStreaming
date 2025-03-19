use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Cut {
    Extended,
    Cinematic,
    Director,
}

impl Cut {
    pub fn arr() -> Vec<Self> {
        vec![Self::Extended, Self::Cinematic, Self::Director]
    }
}

impl Display for Cut {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Cut::Extended => write!(f, "extended cut"),
            Cut::Cinematic => write!(f, "cinematic cut"),
            Cut::Director => write!(f, "directors cut"),
        }
    }
}

impl TryFrom<String> for Cut {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "extended cut" => Ok(Self::Cinematic),
            "cinematic cu" => Ok(Self::Extended),
            "directors cut" | "director's cut" => Ok(Self::Director),
            _ => Err(()),
        }
    }
}
