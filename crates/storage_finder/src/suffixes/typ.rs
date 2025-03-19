use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Hash)]
pub enum FileType {
    Video(VideoExtension),
    Subtitle {
        data: SubtitleExtension,
        lang: Option<String>,
    },
    Audio {
        data: AudioExtension,
        lang: Option<String>,
    },
}

impl FileType {
    pub fn set_lang(&mut self, l: &str) {
        match self {
            FileType::Video(_) => {}
            FileType::Subtitle { lang, data } => {
                lang.replace(l.to_string());
            }
            FileType::Audio { lang, data } => {
                lang.replace(l.to_string());
            }
        }
    }
}

impl<'a> TryFrom<&'a str> for FileType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        let value = value.as_str();
        if let Ok(v) = VideoExtension::try_from(value) {
            return Ok(Self::Video(v));
        } else if let Ok(v) = SubtitleExtension::try_from(value) {
            return Ok(Self::Subtitle {
                data: v,
                lang: None,
            });
        } else if let Ok(v) = AudioExtension::try_from(value) {
            return Ok(Self::Audio {
                data: v,
                lang: None,
            });
        }
        Err(())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Hash)]
pub enum VideoExtension {
    ThreeGP,
    Asf,
    Avi,
    Flv,
    Mov,
    MP4,
    Ogg,
    Mkv,
    /// MPEG-2 / TS
    TS,
    Nsc,
    Nsv,
    Nut,
    Real,
    Raw,
    TyTivo,
}

impl<'a> TryFrom<&'a str> for VideoExtension {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        if value == "3gp" {
            return Ok(Self::ThreeGP);
        } else if value == "asf" || value == "wmv" {
            return Ok(Self::Asf);
        } else if value == "avi" {
            return Ok(Self::Avi);
        } else if value == "flv" {
            return Ok(Self::Flv);
        } else if value == "mov" {
            return Ok(Self::Mov);
        } else if value == "mp4" {
            return Ok(Self::MP4);
        } else if value == "ogm" {
            return Ok(Self::Ogg);
        } else if value == "mpg" || value == "ts" {
            return Ok(Self::TS);
        } else if value == "nsc" {
            return Ok(Self::Nsc);
        } else if value == "nsv" {
            return Ok(Self::Nsv);
        } else if value == "mkv" {
            return Ok(Self::Mkv);
        } else if value == "nut" {
            return Ok(Self::Nut);
        } else if value == "rm" || value == "rv" || value == "rmbv" {
            return Ok(Self::Real);
        } else if value == "a52" || value == "dv" || value == "vid" {
            return Ok(Self::Raw);
        } else if value == "ty" {
            return Ok(Self::TyTivo);
        }
        Err(())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Hash)]
pub enum SubtitleExtension {
    /// SRT
    SubRip,
    AQTitle,
    Cvd,
    Dks,
    JACOsub,
    MicroDVD,
    MPEG4TimedText,
    MPL2,
    PhoenixSubtitle,
    RealText,
    Sami,
    StructuredSubtitleFormat,
    SubStationAlpha,
    SVCDsubtitles,
    UniversalSubtitle,
    VobSub,
}

impl<'a> TryFrom<&'a str> for SubtitleExtension {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        if value == "aqt" {
            return Ok(Self::AQTitle);
        } else if value == "cvd" {
            return Ok(Self::Cvd);
        } else if value == "dks" {
            return Ok(Self::Dks);
        } else if value == "jss" {
            return Ok(Self::JACOsub);
        } else if value == "sub" {
            return Ok(Self::MicroDVD);
        } else if value == "ttxt" {
            return Ok(Self::MPEG4TimedText);
        } else if value == "mpl" {
            return Ok(Self::MPL2);
        } else if value == "pjs" {
            return Ok(Self::PhoenixSubtitle);
        } else if value == "rt" {
            return Ok(Self::RealText);
        } else if value == "smi" {
            return Ok(Self::Sami);
        } else if value == "ssf" {
            return Ok(Self::StructuredSubtitleFormat);
        } else if value == "ssa" || value == "ass" {
            return Ok(Self::SubStationAlpha);
        } else if value == "svcd" {
            return Ok(Self::SVCDsubtitles);
        } else if value == "usf" {
            return Ok(Self::UniversalSubtitle);
        } else if value == "sub" || value == "idx" {
            return Ok(Self::VobSub);
        } else if value == "srt" {
            return Ok(Self::SubRip);
        }
        Err(())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq, Hash)]
pub enum AudioExtension {
    Mka,
    MP3,
    Flac,
    M4A,
    Ogg,
    Aac,
    RA,
    TrueAudioCodec,
    Xa,
    Wav,
    AU,
}

impl<'a> TryFrom<&'a str> for AudioExtension {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        if value == "ra" || value == "ram" {
            return Ok(Self::RA);
        } else if value == "tta" || value == "tac" {
            return Ok(Self::TrueAudioCodec);
        } else if value == "xa" {
            return Ok(Self::Xa);
        } else if value == "wav" || value == "dts" {
            return Ok(Self::Wav);
        } else if value == "au" {
            return Ok(Self::AU);
        } else if value == "aac" {
            return Ok(Self::Aac);
        } else if value == "ogg" {
            return Ok(Self::Ogg);
        } else if value == "m4a" {
            return Ok(Self::M4A);
        } else if value == "flac" {
            return Ok(Self::Flac);
        } else if value == "mp3" {
            return Ok(Self::MP3);
        } else if value == "mka" {
            return Ok(Self::Mka);
        }
        Err(())
    }
}
