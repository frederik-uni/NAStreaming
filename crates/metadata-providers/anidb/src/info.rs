use serde::Deserialize;

use crate::Instance;

impl Instance {
    pub fn lookup(&self, id: &str) {
        let mut url = self.server.clone();
        url.query_pairs_mut().append_pair("aid", id);
        let text = self.client.get(url).send().unwrap().text().unwrap();

        let data: Anime = quick_xml::de::from_str(text.as_str()).unwrap();
        println!("{data:#?}")
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename = "anime")]
pub struct Anime {
    #[serde(rename = "type")]
    anime_type: String,

    #[serde(rename = "episodecount")]
    episodecount: String,

    #[serde(rename = "startdate")]
    startdate: String,

    #[serde(rename = "enddate")]
    enddate: String,

    #[serde(rename = "titles")]
    titles: Titles,

    #[serde(rename = "relatedanime")]
    relatedanime: Relatedanime,

    // #[serde(rename = "url")]
    // url: String,
    #[serde(rename = "creators")]
    creators: Creators,

    #[serde(rename = "description")]
    description: String,

    #[serde(rename = "ratings")]
    ratings: Ratings,
    #[serde(rename = "picture")]
    picture: String,
    #[serde(rename = "resources")]
    resources: Resources,
    #[serde(rename = "tags")]
    tags: Tags,

    #[serde(rename = "characters")]
    characters: Characters,

    #[serde(rename = "episodes")]
    episodes: Episodes,
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@restricted")]
    restricted: String,
}

#[derive(Debug, Deserialize)]
pub struct Characters {
    #[serde(rename = "character")]
    character: Vec<Character>,
}

#[derive(Debug, Deserialize)]
pub struct Character {
    #[serde(rename = "rating")]
    rating: Rating,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "gender")]
    gender: Gender,

    #[serde(rename = "charactertype")]
    charactertype: Charactertype,

    #[serde(rename = "description")]
    description: Option<String>,

    #[serde(rename = "picture")]
    picture: String,

    #[serde(rename = "seiyuu")]
    seiyuu: Option<Seiyuu>,

    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@type")]
    character_type: Type,

    #[serde(rename = "@update")]
    update: String,
}

#[derive(Debug, Deserialize)]
pub struct Charactertype {
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "$text")]
    text: Text,
}

#[derive(Debug, Deserialize)]
pub struct Rating {
    #[serde(rename = "@votes")]
    votes: String,

    #[serde(rename = "$text")]
    text: String,
}

#[derive(Debug, Deserialize)]
pub struct Seiyuu {
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@picture")]
    picture: String,

    #[serde(rename = "$text")]
    text: String,
}

#[derive(Debug, Deserialize)]
pub struct Creators {
    #[serde(rename = "name")]
    name: Vec<Name>,
}

#[derive(Debug, Deserialize)]
pub struct Name {
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@type")]
    name_type: String,

    #[serde(rename = "$text")]
    text: String,
}

#[derive(Debug, Deserialize)]
pub struct Episodes {
    #[serde(rename = "episode")]
    episode: Vec<Episode>,
}

#[derive(Debug, Deserialize)]
pub struct Episode {
    #[serde(rename = "epno")]
    epno: Epno,

    #[serde(rename = "length")]
    length: String,

    #[serde(rename = "airdate")]
    airdate: Option<String>,

    #[serde(rename = "rating")]
    rating: Option<Rating>,

    #[serde(rename = "title")]
    title: TitleUnion,

    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@update")]
    update: String,
}

#[derive(Debug, Deserialize)]
pub struct Epno {
    #[serde(rename = "@type")]
    epno_type: String,

    #[serde(rename = "$text")]
    text: String,
}

#[derive(Debug, Deserialize)]
pub struct TitleTitle {
    // #[serde(rename = "@xml:lang")]
    // xml_lang: String,
    #[serde(rename = "$text")]
    text: String,
}

#[derive(Debug, Deserialize)]
pub struct Ratings {
    #[serde(rename = "permanent")]
    permanent: Option<Permanent>,

    #[serde(rename = "temporary")]
    temporary: Option<Permanent>,

    #[serde(rename = "review")]
    review: Option<Permanent>,
}

#[derive(Debug, Deserialize)]
pub struct Permanent {
    #[serde(rename = "@count")]
    count: String,

    #[serde(rename = "$text")]
    text: String,
}

#[derive(Debug, Deserialize)]
pub struct Recommendations {
    #[serde(rename = "recommendation")]
    recommendation: Vec<Recommendation>,

    #[serde(rename = "@total")]
    total: String,
}

#[derive(Debug, Deserialize)]
pub struct Recommendation {
    #[serde(rename = "@type")]
    recommendation_type: String,

    #[serde(rename = "@uid")]
    uid: String,

    #[serde(rename = "$text")]
    text: String,
}

#[derive(Debug, Deserialize)]
pub struct Relatedanime {
    #[serde(rename = "anime")]
    anime: Vec<Name>,
}

#[derive(Debug, Deserialize)]
pub struct Resources {
    #[serde(rename = "resource")]
    resource: Vec<Resource>,
}

#[derive(Debug, Deserialize)]
pub struct Resource {
    #[serde(rename = "externalentity")]
    externalentity: Externalentity,

    #[serde(rename = "@type")]
    resource_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Externalentity {
    #[serde(rename = "identifier")]
    identifier: Option<Identifier>,

    #[serde(rename = "url")]
    url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Similaranime {
    #[serde(rename = "anime")]
    anime: Vec<AnimeElement>,
}

#[derive(Debug, Deserialize)]
pub struct AnimeElement {
    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@approval")]
    approval: String,

    #[serde(rename = "@total")]
    total: String,

    #[serde(rename = "$text")]
    text: String,
}

#[derive(Debug, Deserialize)]
pub struct Tags {
    #[serde(rename = "tag")]
    tag: Vec<Tag>,
}

#[derive(Debug, Deserialize)]
pub struct Tag {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "description")]
    description: Option<String>,

    #[serde(rename = "picurl")]
    picurl: Option<String>,

    #[serde(rename = "@id")]
    id: String,

    #[serde(rename = "@parentid")]
    parentid: Option<String>,

    #[serde(rename = "@weight")]
    weight: String,

    #[serde(rename = "@localspoiler")]
    localspoiler: String,

    #[serde(rename = "@globalspoiler")]
    globalspoiler: String,

    #[serde(rename = "@verified")]
    verified: String,

    #[serde(rename = "@update")]
    update: String,

    #[serde(rename = "@infobox")]
    infobox: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Titles {
    #[serde(rename = "title")]
    title: Vec<TitlesTitle>,
}

#[derive(Debug, Deserialize)]
pub struct TitlesTitle {
    // #[serde(rename = "@xml:lang")]
    // xml_lang: String,
    #[serde(rename = "@type")]
    title_type: String,

    #[serde(rename = "$text")]
    text: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum TitleUnion {
    TitleTitle(TitleTitle),

    TitleTitleArray(Vec<TitleTitle>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Identifier {
    String(Identifier2),

    StringArray(Vec<Identifier2>),
}

#[derive(Debug, Deserialize)]
pub struct Identifier2 {
    #[serde(rename = "$text")]
    value: String,
}

#[derive(Debug, Deserialize)]
pub enum Type {
    #[serde(rename = "appears in")]
    Appears,

    #[serde(rename = "main character in")]
    MainCharacter,

    #[serde(rename = "secondary cast in")]
    SecondaryCast,
}

#[derive(Debug, Deserialize)]
pub enum Text {
    #[serde(rename = "Character")]
    Character,

    #[serde(rename = "Organization")]
    Organization,

    #[serde(rename = "Vessel")]
    Vessel,
}

#[derive(Debug, Deserialize)]
pub enum Gender {
    #[serde(rename = "female")]
    Female,

    #[serde(rename = "male")]
    Male,

    #[serde(rename = "none/does not apply")]
    NoneDoesNotApply,

    #[serde(rename = "unknown")]
    Unknown,
}
