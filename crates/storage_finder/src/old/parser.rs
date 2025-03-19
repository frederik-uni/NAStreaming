use std::fmt::{Display, Formatter};
use std::fs::read_dir;
use std::fs::DirEntry as FsDirEntry;
use std::io;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::episode_guessing::episode_guess;
use regex::Regex;
use serde::{Deserialize, Serialize};
use walkdir::DirEntry;

use crate::reg::{get_ep_regex, get_season_regex, get_sq_bracket_regex, get_year_regex};
use crate::typ::FileType;

#[derive(Debug, Deserialize, Serialize)]
pub struct EpisodeResult {
    pub main_path: PathBuf,
    pub path: PathBuf,
    pub season_name: String,
    pub name: String,
    pub season: Option<i32>,
    pub season_folder: Option<i32>,
    pub episode: Option<Episode>,
    pub ftype: FileType,
    pub resolution: Option<Resolutions>,
    pub brackets: Vec<String>,
    pub kind_folder: Option<Kind>,
    pub kind: Option<Kind>,
    pub three_d: Option<ThreeD>,
    pub extended: Option<Cut>,
    pub fsize: u64,
    pub sure: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MovieResult {
    pub main_path: PathBuf,
    pub path: PathBuf,
    pub name: String,
    pub ftype: FileType,
    pub extra: Option<Kind>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EntryChild {
    Episode(Vec<EpisodeResult>),
    Movie(MovieResult),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    pub name: String,
    pub children: EntryChild,
    pub brackets: Vec<String>,
    pub year: Option<i32>,
}

impl Entry {
    pub fn items(self) -> Vec<EntryChild> {
        todo!()
    }
}

pub fn parse_library(
    p: &Path,
    series: bool,
    rp: &[(String, String)],
    dont_allow_square_brackets_in_names: bool,
) -> io::Result<Vec<Entry>> {
    let year_regex = get_year_regex();
    let sq_regex = get_sq_bracket_regex();
    let ep_regex = get_ep_regex();
    let s_regex = get_season_regex();
    let mut res = vec![];
    let dirs = read_dir(p)?.flatten().collect::<Vec<_>>();

    for entry in dirs {
        if entry.path().is_dir() || !series {
            if let Some(v) = parse_item(
                entry,
                series,
                &sq_regex,
                &year_regex,
                &ep_regex,
                &s_regex,
                rp,
                dont_allow_square_brackets_in_names,
            ) {
                res.push(v);
            }
        }
    }
    Ok(res)
}

fn parse_item(
    entry: FsDirEntry,
    series: bool,
    sq_regex: &Regex,
    year_regex: &Regex,
    ep_regex: &Regex,
    s_regex: &Regex,
    rp: &[(String, String)],
    dont_allow_square_brackets_in_names: bool,
) -> Option<Entry> {
    let name = match entry.file_name().to_str() {
        None => return None,
        Some(v) => handle_name(v, rp),
    };

    let mut e = Entry {
        name: name.clone(),
        children: if series {
            EntryChild::Episode(parse_episodes(
                entry.path().as_path(),
                sq_regex,
                ep_regex,
                s_regex,
                rp,
                dont_allow_square_brackets_in_names,
            ))
        } else {
            EntryChild::Movie(parse_movie(entry.path().as_path()))
        },
        brackets: sq_regex
            .captures_iter(&name)
            .filter_map(|caps| caps.get(1).map(|v| v.as_str().to_string()))
            .collect::<Vec<_>>(),
        year: None,
    };

    if dont_allow_square_brackets_in_names {
        e.name = sq_regex.replace_all(&e.name, "").to_string();
    }

    if let Some(captures) = year_regex.captures(&name) {
        if let Some(year_match) = captures.get(1) {
            if let Some((v, _)) = e.name.split_once(&format!("({})", year_match.as_str())) {
                e.name = v.to_string();
            }
            e.year = Some(year_match.as_str().parse().expect("Checked with regex"));
        }
    }
    Some(e)
}

fn handle_name(s: &str, replace: &[(String, String)]) -> String {
    let mut value = s.replace(['.', '_', '-'], " ").to_lowercase();
    for (r1, r2) in replace {
        value = value.replace(r1, r2)
    }
    value
}

fn get_valid_files(p: &Path) -> Vec<DirEntry> {
    walkdir::WalkDir::new(p)
        .follow_links(false)
        .contents_first(true)
        .into_iter()
        .filter_entry(valid_file)
        .filter_map(|e| e.ok())
        .collect::<Vec<_>>()
}

fn get_lang(s: &str) -> Option<&str> {
    if s.len() < 4 {
        return None;
    }
    let sub = &s[s.len() - 4..];
    sub.strip_prefix("-")
}

fn get_next_episode(mut s: String, float: bool, value: f64) -> (Episode, String) {
    if float {
        let mut list = vec![];
        list.push(value);
        while let Some(v) = match s.strip_prefix(",e") {
            Some(v) => Some(v),
            None => s.strip_prefix(','),
        } {
            let num: Option<f64>;
            let new_s: String;
            (new_s, num) = remove_number(v.to_string(), valid_float);
            if let Some(num) = num {
                s = new_s;
                list.push(num);
            } else {
                break;
            }
        }

        (
            if list.len() <= 1 {
                Episode::Single(value)
            } else {
                Episode::List(list)
            },
            s,
        )
    } else {
        let conv = ["-", "..=", ".."];
        for (index, val) in conv.into_iter().enumerate() {
            for extra in ["e", ""] {
                if let Some(v) = s.strip_prefix(&format!("{}{}", val, extra)) {
                    let num: Option<u32>;
                    let new_s: String;
                    (new_s, num) = remove_number(v.to_string(), valid_int);
                    return (
                        if let Some(mut num) = num {
                            if index == 2 {
                                num -= 1;
                            }
                            Episode::Range(value as u32..=num)
                        } else {
                            Episode::Single(value)
                        },
                        new_s,
                    );
                }
            }
        }
        (Episode::Single(value), s)
    }
}

fn remove_number<T: FromStr>(s: String, check: fn(char) -> bool) -> (String, Option<T>) {
    let mut chars = s.chars();
    let mut num = vec![];
    while let Some(c) = chars.next() {
        if check(c) {
            num.push(c)
        } else {
            return if num.is_empty() {
                (format!("{}{}", c, chars.as_str()), None)
            } else {
                (
                    format!("{}{}", c, chars.as_str()),
                    num.iter().collect::<String>().parse().ok(),
                )
            };
        }
    }
    return if num.is_empty() {
        (chars.as_str().to_string(), None)
    } else {
        (
            chars.as_str().to_string(),
            num.iter().collect::<String>().parse().ok(),
        )
    };
}

fn valid_int(c: char) -> bool {
    c.is_ascii_digit()
}

fn valid_float(c: char) -> bool {
    c.is_ascii_digit() || c == '.'
}

fn parse_episodes(
    p: &Path,
    sq_regex: &Regex,
    ep_regex: &Regex,
    s_regex: &Regex,
    rp: &[(String, String)],
    dont_allow_square_brackets_in_names: bool,
) -> Vec<EpisodeResult> {
    let mut ret = vec![];
    for entry in get_valid_files(p) {
        let rel_path = entry
            .path()
            .strip_prefix(p)
            .expect("Symlinks are disabled so its always a child");
        let file_name = entry
            .file_name()
            .to_str()
            .expect("Files with no filename should be filtered")
            .to_lowercase();
        let mut ftype = FileType::try_from(file_name.as_str()).expect("Filtered before");
        let mut temp = file_name.split('.').collect::<Vec<_>>();
        temp.pop();
        let mut temp = temp.join(".");

        let file_name = temp.clone();
        match ftype {
            FileType::Video(_) => {}
            FileType::Subtitle { .. } | FileType::Audio { .. } => {
                if let Some(v) = get_lang(&temp.clone()) {
                    temp = temp[..temp.len() - 4].to_string();
                    ftype.set_lang(v);
                }
            }
        }

        let mut res = EpisodeResult {
            main_path: p.to_path_buf(),
            path: rel_path.to_path_buf(),
            season_name: "".to_string(),
            name: "".to_string(),
            episode: None,
            season: None,
            ftype,
            kind: None,
            resolution: None,
            season_folder: None,
            kind_folder: None,
            brackets: sq_regex
                .captures_iter(entry.file_name().to_str().unwrap_or_default())
                .filter_map(|caps| caps.get(1).map(|v| v.as_str().to_string()))
                .collect::<Vec<_>>(),
            three_d: None,
            extended: None,
            fsize: entry.path().metadata().unwrap().len(),
            sure: false,
        };

        if let Some(captures) = ep_regex.captures(&file_name.clone()) {
            let before = captures.get(1).map_or("", |m| m.as_str());
            let season_number: i32 = captures[3].parse().unwrap();
            let episode_number: f64 = captures[4].parse().unwrap();
            let after = captures.get(6).map_or("", |m| m.as_str());
            let (episode, after) =
                get_next_episode(after.to_string(), captures[4].contains('.'), episode_number);
            temp = after;
            res.episode = Some(episode);
            res.season = Some(season_number);
            res.sure = true;
            res.season_name = before
                .split(' ')
                .filter(|v| !v.is_empty())
                .collect::<Vec<_>>()
                .join(" ")
                .trim()
                .to_string();
        }
        let mut pro_name = handle_name(&temp, &rp);
        if let Some(Episode::Single(ep)) = res.episode {
            for part in ["cd, dvd, part, pt, disc, disk"] {
                if let Some(v) = pro_name.strip_prefix(&format!(" {} ", part)) {
                    let (v, num) = remove_number::<u32>(v.to_string(), valid_int);
                    if let Some(num) = num {
                        pro_name = v;
                        res.episode = Some(Episode::Part(ep, num))
                    }
                    break;
                }
            }
        }
        let suff;
        (pro_name, suff) = suffixes(pro_name, vec![]);

        for suf in suff {
            if let Ok(v) = ThreeD::try_from(suf.clone()) {
                res.three_d = Some(v);
            } else if let Ok(v) = Cut::try_from(suf.clone()) {
                res.extended = Some(v);
            } else if let Ok(v) = Kind::try_from(suf) {
                res.kind = Some(v);
            }
        }

        if dont_allow_square_brackets_in_names {
            res.name = sq_regex.replace_all(&res.name, "").to_string();
        }

        let comp: Vec<_> = res.path.components().collect();
        if !comp.is_empty() {
            let folder = comp
                .first()
                .expect("Checked to length before")
                .as_os_str()
                .to_str()
                .unwrap_or("")
                .to_lowercase();
            if let Some(captures) = s_regex.captures(&folder) {
                res.season_folder = Some(captures[1].parse().expect("Should be checked by regex"));
            }
            let folder = comp
                .last()
                .expect("Checked to length before")
                .as_os_str()
                .to_str()
                .unwrap_or_default()
                .to_lowercase();
            res.kind_folder = Kind::try_from(folder).ok();
            if comp.len() > 1 {
                //TODO: check if episode folder
            }
        }

        if let Some((resolution, name)) = Resolutions::from_str(&pro_name) {
            pro_name = name;
            res.resolution = Some(resolution)
        }

        res.name = pro_name
            .split(' ')
            .filter(|v| !v.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        if !res.sure {
            let (s, e) = episode_guess(file_name);
            res.episode = e.map(|e| Episode::Single(e as f64));
            res.season = s;
        }
        ret.push(res);
    }
    ret
}
fn parse_movie(p: &Path) -> MovieResult {
    todo!()
}

fn valid_file(d: &DirEntry) -> bool {
    if !d.path().is_file() {
        return false;
    }
    if let Some(v) = d.file_name().to_str() {
        return FileType::try_from(v).is_ok();
    }
    false
}
