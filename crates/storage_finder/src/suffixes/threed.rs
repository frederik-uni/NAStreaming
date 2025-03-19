use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum ThreeD {
    Hsbs,
    Fsbs,
    Htab,
    Ftab,
    Mvc,
}

impl Display for ThreeD {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ThreeD::Hsbs => write!(f, "hsbs"),
            ThreeD::Fsbs => write!(f, "fsbs"),
            ThreeD::Htab => write!(f, "htab"),
            ThreeD::Ftab => write!(f, "ftab"),
            ThreeD::Mvc => write!(f, "mvc"),
        }
    }
}

impl TryFrom<String> for ThreeD {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "hsbs" => Ok(Self::Hsbs),
            "fsbs" => Ok(Self::Fsbs),
            "htab" => Ok(Self::Htab),
            "ftab" => Ok(Self::Ftab),
            "mvc" => Ok(Self::Mvc),
            _ => Err(()),
        }
    }
}

impl ThreeD {
    pub fn arr() -> Vec<Self> {
        vec![Self::Htab, Self::Mvc, Self::Hsbs, Self::Ftab, Self::Fsbs]
    }
}
