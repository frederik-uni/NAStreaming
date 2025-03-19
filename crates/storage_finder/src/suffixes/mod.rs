pub use cut::Cut;
pub use kind::Kind;
pub use threed::ThreeD;

mod cut;
mod kind;
mod threed;
mod typ;
pub use typ::FileType;

pub fn suffix_folder(folder_name: &str) -> bool {
    let mut other = ThreeD::arr()
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>();
    other.append(&mut Kind::arr().iter().map(|v| v.to_string()).collect());
    other.append(&mut Cut::arr().iter().map(|v| v.to_string()).collect());
    other.contains(&folder_name.trim().to_lowercase())
}

pub fn suffixes(mut s: String, mut suff: Vec<String>) -> (String, Vec<String>) {
    let mut other = ThreeD::arr()
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>();
    other.append(&mut Kind::arr().iter().map(|v| v.to_string()).collect());
    other.append(&mut Cut::arr().iter().map(|v| v.to_string()).collect());
    let mut found = false;
    for other in other {
        s = s.trim().to_string();
        if let Some(v) = s.strip_suffix(&format!("-{other}")) {
            found = true;
            suff.push(other);
            s = v.trim().to_string();
        }
    }
    if found {
        suffixes(s, suff)
    } else {
        (s, suff)
    }
}
