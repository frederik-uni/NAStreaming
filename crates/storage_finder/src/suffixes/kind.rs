use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Kind {
    Trailers,
    Intros,
    Outros,
    Soundtracks,
    BehindTheScenes,
    DeletedScenes,
    Interviews,
    Scenes,
    Samples,
    Shorts,
    Featurettes,
    Clips,
}

impl Kind {
    pub fn arr() -> Vec<Self> {
        vec![
            Self::Trailers,
            Self::Intros,
            Self::Outros,
            Self::Soundtracks,
            Self::BehindTheScenes,
            Self::DeletedScenes,
            Self::Interviews,
            Self::Scenes,
            Self::Samples,
            Self::Shorts,
            Self::Featurettes,
            Self::Clips,
        ]
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Trailers => write!(f, "trailer"),
            Kind::Intros => write!(f, "intro"),
            Kind::Outros => write!(f, "outro"),
            Kind::Soundtracks => write!(f, "soundtrack"),
            Kind::BehindTheScenes => write!(f, "behind the scenes"),
            Kind::DeletedScenes => write!(f, "deleted scene"),
            Kind::Interviews => write!(f, "interview"),
            Kind::Scenes => write!(f, "scene"),
            Kind::Samples => write!(f, "sample"),
            Kind::Shorts => write!(f, "short"),
            Kind::Featurettes => write!(f, "featurette"),
            Kind::Clips => write!(f, "clip"),
        }
    }
}

impl TryFrom<String> for Kind {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "trailer" => Ok(Kind::Trailers),
            "intro" => Ok(Kind::Intros),
            "outro" => Ok(Kind::Outros),
            "soundtrack" => Ok(Kind::Soundtracks),
            "behind the scenes" => Ok(Kind::BehindTheScenes),
            "deleted scene" => Ok(Kind::DeletedScenes),
            "interview" => Ok(Kind::Interviews),
            "scene" => Ok(Kind::Scenes),
            "sample" => Ok(Kind::Samples),
            "short" => Ok(Kind::Shorts),
            "featurette" => Ok(Kind::Featurettes),
            "clip" => Ok(Kind::Clips),
            _ => Err(()),
        }
    }
}
