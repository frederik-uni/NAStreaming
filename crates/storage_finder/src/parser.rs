use std::collections::HashSet;
use std::path::{Path, PathBuf};

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::episode_guessing::episode_guess;
use crate::resolution::Resolutions;
use crate::segments::{parse_segments, segements2_to_tuple, Episode, Segment, Segment2};
use crate::suffixes::{suffix_folder, suffixes, Cut, FileType, Kind, ThreeD};

pub fn parse_library(p: &Path, illegal: &Vec<PathBuf>) -> Vec<Entry> {
    let parse = read_dir(p)
        .into_iter()
        .flat_map(|v| parse_top(v, illegal))
        .collect::<Vec<Parsed>>();

    parse.into_iter().map(parsed_to_entry).collect()
}
#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub name: Vec<String>,
    pub ep_name: Vec<String>,
    pub season: Vec<u64>,
    pub episode: Vec<Episode>,
    pub file_type: FileType,
    pub path: PathBuf,
    pub resolutions: Vec<Resolutions>,
    pub three_ds: Vec<ThreeD>,
    pub extended: Vec<Cut>,
    pub kinds: Vec<Kind>,
}

fn parsed_to_entry(mut data: Parsed) -> Entry {
    let mut three_ds = vec![];
    let mut extended = vec![];
    let mut kinds = vec![];

    for suf in data.suffixes {
        if let Ok(v) = ThreeD::try_from(suf.clone()) {
            three_ds.push(v);
        } else if let Ok(v) = Cut::try_from(suf.clone()) {
            extended.push(v);
        } else if let Ok(v) = Kind::try_from(suf) {
            kinds.push(v);
        }
    }
    if data.episode.is_empty() {
        let name = data
            .path
            .file_name()
            .and_then(|v| v.to_str())
            .unwrap_or_default();
        let (s, e) = episode_guess(name);
        if let Some(s) = s {
            data.season.push(s as u64);
        }
        if let Some(e) = e {
            data.episode.push(Episode::Single(e.into()));
        }
    }
    Entry {
        name: data.name,
        ep_name: data.ep_name,
        season: data.season,
        episode: data.episode,
        file_type: data.file_type.unwrap(),
        path: data.path,
        resolutions: data.resolutions,
        three_ds,
        extended,
        kinds,
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Parsed {
    name: Vec<String>,
    ep_name: Vec<String>,
    season: Vec<u64>,
    episode: Vec<Episode>,
    br: Vec<Segment>,
    suffixes: Vec<String>,
    file_type: Option<FileType>,
    path: PathBuf,
    resolutions: Vec<Resolutions>,
}

fn unique_vec_ordered<T: Eq + std::hash::Hash + Clone>(vec: Vec<T>) -> Vec<T> {
    let mut seen = HashSet::new();
    vec.into_iter()
        .rev()
        .filter(|x| seen.insert(x.clone()))
        .rev()
        .collect()
}

impl Parsed {
    fn join(&self, other: &Self) -> Self {
        let mut data = self.clone();
        data.name.extend(other.name.clone());
        data.name = unique_vec_ordered(data.name);
        data.ep_name.extend(other.ep_name.clone());
        data.ep_name = unique_vec_ordered(data.ep_name);
        data.season.extend(other.season.clone());
        data.season = unique_vec_ordered(data.season);
        data.episode.extend(other.episode.clone());
        data.episode = unique_vec_ordered(data.episode);
        data.br.extend(other.br.clone());
        data.br = unique_vec_ordered(data.br);
        data.suffixes.extend(other.suffixes.clone());
        data.suffixes = unique_vec_ordered(data.suffixes);
        data.resolutions.extend(other.resolutions.clone());
        data.resolutions = unique_vec_ordered(data.resolutions);
        if let Some(v) = other.file_type.clone() {
            data.file_type = Some(v);
        }
        data
    }
    fn set_path(mut self, path: PathBuf) -> Self {
        self.path = path;
        self
    }
}

fn parse_like_file(file_name: &str) -> Parsed {
    let (strs, br): (Vec<_>, Vec<_>) = parse_segments(file_name)
        .into_iter()
        .partition(|v| v.is_none());
    let name = Segment2::parse(strs);
    let v = segements2_to_tuple(name);
    match v {
        Ok((b, (s, e), a)) => {
            let (a, suff) = suffixes(a, vec![]);
            let (resolution, a) = match Resolutions::from_str(&a) {
                Some(v) => (Some(v.0), v.1),
                None => (None, a),
            };

            Parsed {
                name: vec![b],
                ep_name: vec![a],
                season: vec![s],
                episode: vec![e],
                br,
                suffixes: suff,
                file_type: None,
                path: PathBuf::new(),
                resolutions: match resolution {
                    Some(v) => vec![v],
                    None => vec![],
                },
            }
        }
        Err(s) => {
            let (a, suff) = suffixes(s, vec![]);
            Parsed {
                name: vec![a],
                ep_name: vec![],
                season: vec![],
                resolutions: vec![],
                episode: vec![],
                br,
                suffixes: suff,
                file_type: None,
                path: PathBuf::new(),
            }
        }
    }
}

fn parse_file(path: &PathBuf, illegal: &Vec<PathBuf>) -> Option<Parsed> {
    if illegal.contains(path) {
        return None;
    }
    let file_name = path
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();
    let (file_name, ext) = file_name.rsplit_once(".").unwrap_or_default();
    let extension = FileType::try_from(ext).ok()?;

    let mut data = parse_like_file(file_name);
    data.file_type = Some(extension);
    Some(data)
}

fn read_dir(path: &Path) -> Vec<PathBuf> {
    std::fs::read_dir(path)
        .map(|v| v.flatten().map(|v| v.path()).collect::<Vec<_>>())
        .unwrap_or_default()
}

fn parse_top(path: PathBuf, illegal: &Vec<PathBuf>) -> Vec<Parsed> {
    if path.is_file() {
        match parse_file(&path, illegal) {
            Some(v) => vec![v.set_path(path)],
            None => vec![],
        }
    } else {
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        let data = parse_like_file(file_name);
        read_dir(&path)
            .into_iter()
            .flat_map(|v| parse_2(v, illegal, &data))
            .collect::<Vec<_>>()
    }
}

fn parse_2(path: PathBuf, illegal: &Vec<PathBuf>, data: &Parsed) -> Vec<Parsed> {
    if path.is_file() {
        match parse_file(&path, illegal) {
            Some(v) => vec![data.join(&v).set_path(path)],
            None => vec![],
        }
    } else {
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        let data = if let Some(v) = parse_season(file_name) {
            let mut data = data.clone();
            data.season.push(v);
            data
        } else {
            data.join(&parse_like_file(file_name))
        };
        read_dir(&path)
            .into_iter()
            .flat_map(|v| parse_3(v, illegal, &data))
            .collect::<Vec<_>>()
    }
}

fn parse_season(input: &str) -> Option<u64> {
    let separators = [r"\-_ ", r" "];

    for separator in &separators {
        let re = Regex::new(&format!(r"^Season{}(\d+)", separator)).unwrap();

        if let Some(caps) = re.captures(input) {
            if let Some(num_str) = caps.get(1) {
                if let Ok(season_number) = num_str.as_str().parse::<u64>() {
                    return Some(season_number);
                }
            }
        }
    }
    None
}

fn parse_3(path: PathBuf, illegal: &Vec<PathBuf>, data: &Parsed) -> Vec<Parsed> {
    if path.is_file() {
        match parse_file(&path, illegal) {
            Some(v) => vec![data.join(&v).set_path(path)],
            None => vec![],
        }
    } else {
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        let data = if suffix_folder(file_name) {
            let mut data = data.clone();
            data.suffixes.push(file_name.to_owned());
            data
        } else {
            data.join(&parse_like_file(file_name))
        };
        read_dir(&path)
            .into_iter()
            .flat_map(|v| parse_4(v, illegal, &data))
            .collect::<Vec<_>>()
    }
}

fn parse_4(path: PathBuf, illegal: &Vec<PathBuf>, data: &Parsed) -> Vec<Parsed> {
    if path.is_file() {
        match parse_file(&path, illegal) {
            Some(v) => vec![data.join(&v).set_path(path)],
            None => vec![],
        }
    } else {
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        let data = data.join(&parse_like_file(file_name));
        read_dir(&path)
            .into_iter()
            .flat_map(|v| parse_4(v, illegal, &data))
            .collect::<Vec<_>>()
    }
}
