#[derive(Debug)]
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

#[derive(Debug)]
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
        if value.ends_with(".3gp") {
            return Ok(Self::ThreeGP);
        } else if value.ends_with(".asf") || value.ends_with(".wmv") {
            return Ok(Self::Asf);
        } else if value.ends_with(".avi") {
            return Ok(Self::Avi);
        } else if value.ends_with(".flv") {
            return Ok(Self::Flv);
        } else if value.ends_with(".mov") {
            return Ok(Self::Mov);
        } else if value.ends_with(".mp4") {
            return Ok(Self::MP4);
        } else if value.ends_with(".ogm") {
            return Ok(Self::Ogg);
        } else if value.ends_with(".mpg") || value.ends_with(".ts") {
            return Ok(Self::TS);
        } else if value.ends_with(".nsc") {
            return Ok(Self::Nsc);
        } else if value.ends_with(".nsv") {
            return Ok(Self::Nsv);
        } else if value.ends_with(".mkv") {
            return Ok(Self::Mkv);
        } else if value.ends_with(".nut") {
            return Ok(Self::Nut);
        } else if value.ends_with(".rm") || value.ends_with(".rv") || value.ends_with(".rmbv") {
            return Ok(Self::Real);
        } else if value.ends_with(".a52") || value.ends_with(".dv") || value.ends_with(".vid") {
            return Ok(Self::Raw);
        } else if value.ends_with(".ty") {
            return Ok(Self::TyTivo);
        }
        Err(())
    }
}

#[derive(Debug)]
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
        if value.ends_with(".aqt") {
            return Ok(Self::AQTitle);
        } else if value.ends_with(".cvd") {
            return Ok(Self::Cvd);
        } else if value.ends_with(".dks") {
            return Ok(Self::Dks);
        } else if value.ends_with(".jss") {
            return Ok(Self::JACOsub);
        } else if value.ends_with(".sub") {
            return Ok(Self::MicroDVD);
        } else if value.ends_with(".ttxt") {
            return Ok(Self::MPEG4TimedText);
        } else if value.ends_with(".mpl") {
            return Ok(Self::MPL2);
        } else if value.ends_with(".pjs") {
            return Ok(Self::PhoenixSubtitle);
        } else if value.ends_with(".rt") {
            return Ok(Self::RealText);
        } else if value.ends_with(".smi") {
            return Ok(Self::Sami);
        } else if value.ends_with(".ssf") {
            return Ok(Self::StructuredSubtitleFormat);
        } else if value.ends_with(".ssa") || value.ends_with(".ass") {
            return Ok(Self::SubStationAlpha);
        } else if value.ends_with(".svcd") {
            return Ok(Self::SVCDsubtitles);
        } else if value.ends_with(".usf") {
            return Ok(Self::UniversalSubtitle);
        } else if value.ends_with(".sub") || value.ends_with(".idx") {
            return Ok(Self::VobSub);
        } else if value.ends_with(".srt") {
            return Ok(Self::SubRip);
        }
        Err(())
    }
}

#[derive(Debug)]
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
        if value.ends_with(".ra") || value.ends_with(".ram") {
            return Ok(Self::RA);
        } else if value.ends_with(".tta") || value.ends_with(".tac") {
            return Ok(Self::TrueAudioCodec);
        } else if value.ends_with(".xa") {
            return Ok(Self::Xa);
        } else if value.ends_with(".wav") || value.ends_with(".dts") {
            return Ok(Self::Wav);
        } else if value.ends_with(".au") {
            return Ok(Self::AU);
        } else if value.ends_with(".aac") {
            return Ok(Self::Aac);
        } else if value.ends_with(".ogg") {
            return Ok(Self::Ogg);
        } else if value.ends_with(".m4a") {
            return Ok(Self::M4A);
        } else if value.ends_with(".flac") {
            return Ok(Self::Flac);
        } else if value.ends_with(".mp3") {
            return Ok(Self::MP3);
        } else if value.ends_with(".mka") {
            return Ok(Self::Mka);
        }
        Err(())
    }
}
