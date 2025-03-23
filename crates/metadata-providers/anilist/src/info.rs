use metadata_provider::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Instance;

const QUERY: &str = "
query ($id: Int) { # Define which variables will be used in the query (id)
  Media (id: $id, type: ANIME) { # Insert our variables into the query arguments (id) (type: ANIME is hard-coded in the query)
    id
    title {
      romaji
      english
      native
    }
  }
}
";

const QUERY2: &str = "
query media($id: Int, $type: MediaType, $isAdult: Boolean) {
  Media(id: $id, type: $type, isAdult: $isAdult) {
    id
    idMal
    title {
      userPreferred
      romaji
      english
      native
    }
    type
    format
    status(version: 2)
    description
    startDate {
      year
      month
      day
    }
    endDate {
      year
      month
      day
    }
    season
    seasonYear
    episodes
    duration
    chapters
    volumes
    countryOfOrigin
    isLicensed
    source(version: 3)
    hashtag
    trailer {
      id
      site
      thumbnail
    }
    updatedAt
    coverImage {
        extraLarge
        large
        medium
        color
    }
    bannerImage
    genres
    synonyms
    averageScore
    meanScore
    popularity
    trending
    favourites
    tags {
      id
      name
      description
      category
      rank
      isMediaSpoiler
      isGeneralSpoiler
      isAdult
      userId
    }
    relations {
      edges {
        id
        relationType(version: 2)
        node {
          id
          title {
            userPreferred
          }
          format
          type
          status(version: 2)
          bannerImage
          coverImage {
            large
          }
        }
      }
    }
    isFavourite
    isAdult
    nextAiringEpisode {
      airingAt
      timeUntilAiring
      episode
    }

    studios {
      edges {
        isMain
        node {
          id
          isAnimationStudio
          siteUrl
          name
        }
      }
    }
    airingSchedule {
        edges {
            node {
                id
                airingAt
                timeUntilAiring
                episode
            }
        }
    }
    externalLinks {
      id
      site
      url
      type
      language
      color
      icon
      notes
      isDisabled
    }
    isAdult
    genres
    popularity
    streamingEpisodes {
      site
      title
      thumbnail
      url
    }
    characters {
        edges {
            role
            name
            voiceActors(language: JAPANESE, sort: [RELEVANCE, ID]) {
              id
              name {
                userPreferred
              }
              language: languageV2
              image {
                large
              }
            }
            node {
                id
                name {
                    first
                    middle
                    last
                    full
                    native
                    alternative
                    alternativeSpoiler
                    userPreferred
                }
                image {
                    large
                }
            }
        }
    }
    reviews {
        edges {
            node {
                id
                userId
                summary
                body
                rating
                ratingAmount
                userRating
                score
                siteUrl
                createdAt
                updatedAt
                user {
                    name
                    avatar {
                      large
                    }
                }
            }
        }
    }
    staff {
        edges {
            role
            node {
                id
                name {
                    first
                    middle
                    last
                    full
                    native
                    alternative
                    userPreferred
                }
                languageV2
                image {
                    large
                }
                description
                primaryOccupations
                gender
                dateOfBirth {
                  year
                  month
                  day
                }
                dateOfDeath {
                  year
                  month
                  day
                }
                yearsActive
                homeTown
                bloodType
                siteUrl
            }
        }
    }
  }
}
";

impl Instance {
    pub async fn lookup(&self, id: &str) -> Result<Root1, Error> {
        let json = json!({"query": QUERY2, "variables": {"id": id.parse::<u32>().map_err(|_|Error::InvalidId)?, "type":"ANIME","isAdult":false}});
        let resp: Root1 = self
            .client
            .post("https://graphql.anilist.co/")
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(json.to_string())
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }
}

#[derive(Serialize, Deserialize)]
pub struct AiringSchedule1 {
    edges: Vec<()>,
}
#[derive(Serialize, Deserialize)]
pub struct Image1 {
    large: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name1 {
    alternative: Vec<String>,
    alternative_spoiler: Vec<String>,
    first: Option<String>,
    full: Option<String>,
    last: Option<String>,
    middle: Option<String>,
    native: Option<String>,
    user_preferred: String,
}
#[derive(Serialize, Deserialize)]
pub struct Node1 {
    id: i64,
    image: Image1,
    name: Name1,
}
#[derive(Serialize, Deserialize)]
pub struct Image2 {
    large: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name2 {
    user_preferred: String,
}
#[derive(Serialize, Deserialize)]
pub struct VoiceActors1 {
    id: i64,
    image: Image2,
    language: String,
    name: Name2,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edges1 {
    name: Option<String>,
    node: Node1,
    role: String,
    voice_actors: Vec<VoiceActors1>,
}
#[derive(Serialize, Deserialize)]
pub struct Characters1 {
    edges: Vec<Edges1>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverImage1 {
    color: String,
    extra_large: String,
    large: String,
    medium: String,
}
#[derive(Serialize, Deserialize)]
pub struct Date {
    day: Option<i64>,
    month: Option<i64>,
    year: Option<i64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalLinks1 {
    color: Option<String>,
    icon: Option<String>,
    id: i64,
    is_disabled: bool,
    language: Option<String>,
    notes: (),
    site: String,
    r#type: String,
    url: String,
}
#[derive(Serialize, Deserialize)]
pub struct CoverImage2 {
    large: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title1 {
    user_preferred: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node2 {
    banner_image: Option<String>,
    cover_image: CoverImage2,
    format: String,
    id: i64,
    status: String,
    title: Title1,
    r#type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edges2 {
    id: i64,
    node: Node2,
    relation_type: String,
}
#[derive(Serialize, Deserialize)]
pub struct Relations1 {
    edges: Vec<Edges2>,
}
#[derive(Serialize, Deserialize)]
pub struct Avatar1 {
    large: String,
}
#[derive(Serialize, Deserialize)]
pub struct User1 {
    avatar: Avatar1,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node3 {
    body: String,
    created_at: i64,
    id: i64,
    rating: i64,
    rating_amount: i64,
    score: i64,
    site_url: String,
    summary: String,
    updated_at: i64,
    user: User1,
    user_id: i64,
    user_rating: String,
}
#[derive(Serialize, Deserialize)]
pub struct Edges3 {
    node: Node3,
}
#[derive(Serialize, Deserialize)]
pub struct Reviews1 {
    edges: Vec<Edges3>,
}

#[derive(Serialize, Deserialize)]
pub struct Image3 {
    large: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name3 {
    alternative: Vec<String>,
    first: Option<String>,
    full: Option<String>,
    last: Option<String>,
    middle: Option<String>,
    native: Option<String>,
    user_preferred: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node4 {
    blood_type: Option<String>,
    date_of_birth: Date,
    date_of_death: Date,
    description: Option<String>,
    gender: Option<String>,
    home_town: Option<String>,
    id: i64,
    image: Image3,
    language_v2: String,
    name: Name3,
    primary_occupations: Vec<String>,
    site_url: String,
}
#[derive(Serialize, Deserialize)]
pub struct Edges4 {
    node: Node4,
    role: String,
}
#[derive(Serialize, Deserialize)]
pub struct Staff1 {
    edges: Vec<Edges4>,
}

#[derive(Serialize, Deserialize)]
pub struct StreamingEpisodes1 {
    site: String,
    thumbnail: String,
    title: String,
    url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node5 {
    id: i64,
    is_animation_studio: bool,
    name: String,
    site_url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edges5 {
    is_main: bool,
    node: Node5,
}
#[derive(Serialize, Deserialize)]
pub struct Studios1 {
    edges: Vec<Edges5>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags1 {
    category: String,
    description: String,
    id: i64,
    is_adult: bool,
    is_general_spoiler: bool,
    is_media_spoiler: bool,
    name: String,
    rank: i64,
    user_id: Option<i64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title2 {
    english: String,
    native: String,
    romaji: String,
    user_preferred: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Media1 {
    airing_schedule: AiringSchedule1,
    average_score: i64,
    banner_image: String,
    characters: Characters1,
    country_of_origin: String,
    cover_image: CoverImage1,
    description: String,
    duration: i64,
    end_date: Date,
    episodes: i64,
    external_links: Vec<ExternalLinks1>,
    favourites: i64,
    format: String,
    genres: Vec<String>,
    hashtag: Option<String>,
    id: i64,
    id_mal: i64,
    is_adult: bool,
    is_favourite: bool,
    is_licensed: bool,
    mean_score: i64,
    next_airing_episode: (),
    popularity: i64,
    relations: Relations1,
    reviews: Reviews1,
    season: String,
    season_year: i64,
    source: String,
    staff: Staff1,
    start_date: Date,
    status: String,
    streaming_episodes: Vec<StreamingEpisodes1>,
    studios: Studios1,
    synonyms: Vec<String>,
    tags: Vec<Tags1>,
    title: Title2,
    trailer: Option<Trailer>,
    trending: i64,
    r#type: String,
    updated_at: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Trailer {
    id: String,
    site: String,
    thumbnail: String,
}
#[derive(Serialize, Deserialize)]
pub struct Data1 {
    #[serde(rename = "Media")]
    media: Media1,
}
#[derive(Serialize, Deserialize)]
pub struct Root1 {
    data: Data1,
}
