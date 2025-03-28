use std::collections::HashSet;
use std::path::{Path, PathBuf};

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::episode_guessing::episode_guess;
use crate::path_tree::PathTree;
use crate::resolution::Resolutions;
use crate::segments::{parse_segments, segements2_to_tuple, Episode, Segment, Segment2};
use crate::suffixes::{suffix_folder, suffixes, Cut, FileType, Kind, ThreeD};
pub async fn parse_library(p: &Path, illegal: &PathTree) -> Vec<Entry> {
    let mut parse = vec![];
    for item in read_dir(p).await {
        parse.extend(parse_top(p, item, illegal).await);
    }

    parse.into_iter().map(|v| parsed_to_entry(p, v)).collect()
}
#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub name: Vec<String>,
    pub ep_name: Vec<String>,
    pub sure: bool,
    pub year: Vec<u16>,
    pub season: Vec<u64>,
    pub episode: Vec<Episode>,
    pub file_type: FileType,
    pub root_path: PathBuf,
    pub path: PathBuf,
    pub resolutions: Vec<Resolutions>,
    pub three_ds: Vec<ThreeD>,
    pub extended: Vec<Cut>,
    pub kinds: Vec<Kind>,
}

fn parsed_to_entry(root: &Path, mut data: Parsed) -> Entry {
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
    let mut sure = true;
    if data.episode.is_empty() {
        sure = false;
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
    let (years, br): (Vec<_>, Vec<_>) = data.br.into_iter().partition(|v| v.is_year());
    Entry {
        sure,
        year: years.into_iter().flat_map(|v| v.as_u16()).collect(),
        name: data.name.into_iter().filter(|v| !v.is_empty()).collect(),
        ep_name: data.ep_name.into_iter().filter(|v| !v.is_empty()).collect(),
        season: data.season,
        episode: data.episode,
        file_type: data.file_type.unwrap(),
        path: data.path,
        resolutions: data.resolutions,
        three_ds,
        extended,
        kinds,
        root_path: root.to_path_buf(),
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

fn parse_file(path: &PathBuf, illegal: &PathTree) -> Option<Parsed> {
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

async fn read_dir(path: &Path) -> Vec<PathBuf> {
    let dir = tokio::fs::read_dir(path).await;
    let mut out = vec![];
    if let Ok(mut v) = dir {
        while let Ok(Some(path)) = v.next_entry().await {
            out.push(path.path());
        }
    }
    out
}

async fn parse_top(root_path: &Path, path: PathBuf, illegal: &PathTree) -> Vec<Parsed> {
    if path.is_file() {
        match parse_file(&path, illegal) {
            Some(v) => {
                vec![v.set_path(path.strip_prefix(root_path).unwrap_or(&path).to_path_buf())]
            }
            None => vec![],
        }
    } else {
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        let data = parse_like_file(file_name);
        let mut out = vec![];
        for item in read_dir(&path).await {
            out.extend(parse_2(root_path, item, illegal, &data).await);
        }
        out
    }
}

async fn parse_2(
    root_path: &Path,
    path: PathBuf,
    illegal: &PathTree,
    data: &Parsed,
) -> Vec<Parsed> {
    if path.is_file() {
        match parse_file(&path, illegal) {
            Some(v) => vec![data
                .join(&v)
                .set_path(path.strip_prefix(root_path).unwrap_or(&path).to_path_buf())],
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
        let mut out = vec![];
        for item in read_dir(&path).await {
            out.extend(parse_3(root_path, item, illegal, &data).await);
        }

        out
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

async fn parse_3(
    root_path: &Path,
    path: PathBuf,
    illegal: &PathTree,
    data: &Parsed,
) -> Vec<Parsed> {
    if path.is_file() {
        match parse_file(&path, illegal) {
            Some(v) => vec![data
                .join(&v)
                .set_path(path.strip_prefix(root_path).unwrap_or(&path).to_path_buf())],
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

        let mut out = vec![];
        for item in read_dir(&path).await {
            out.extend(parse_4(root_path, item, illegal, &data).await);
        }

        out
    }
}

#[async_recursion::async_recursion]
async fn parse_4(
    root_path: &Path,
    path: PathBuf,
    illegal: &PathTree,
    data: &Parsed,
) -> Vec<Parsed> {
    if path.is_file() {
        match parse_file(&path, illegal) {
            Some(v) => vec![data
                .join(&v)
                .set_path(path.strip_prefix(root_path).unwrap_or(&path).to_path_buf())],
            None => vec![],
        }
    } else {
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();
        let data = data.join(&parse_like_file(file_name));
        let mut out = vec![];
        for item in read_dir(&path).await {
            out.extend(parse_4(root_path, item, illegal, &data).await);
        }

        out
    }
}
