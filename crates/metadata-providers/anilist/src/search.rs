const QUERY: &str = "query (
  $page: Int = 1
  $id: Int
  $type: MediaType
  $isAdult: Boolean = false
  $search: String
  $format: [MediaFormat]
  $status: MediaStatus
  $countryOfOrigin: CountryCode
  $source: MediaSource
  $season: MediaSeason
  $seasonYear: Int
  $year: String
  $onList: Boolean
  $yearLesser: FuzzyDateInt
  $yearGreater: FuzzyDateInt
  $episodeLesser: Int
  $episodeGreater: Int
  $durationLesser: Int
  $durationGreater: Int
  $chapterLesser: Int
  $chapterGreater: Int
  $volumeLesser: Int
  $volumeGreater: Int
  $licensedBy: [Int]
  $isLicensed: Boolean
  $genres: [String]
  $excludedGenres: [String]
  $tags: [String]
  $excludedTags: [String]
  $minimumTagRank: Int
  $sort: [MediaSort] = [POPULARITY_DESC, SCORE_DESC]
) {
  Page(page: $page, perPage: 50) {
    media(
      id: $id
      type: $type
      season: $season
      format_in: $format
      status: $status
      countryOfOrigin: $countryOfOrigin
      source: $source
      search: $search
      onList: $onList
      seasonYear: $seasonYear
      startDate_like: $year
      startDate_lesser: $yearLesser
      startDate_greater: $yearGreater
      episodes_lesser: $episodeLesser
      episodes_greater: $episodeGreater
      duration_lesser: $durationLesser
      duration_greater: $durationGreater
      chapters_lesser: $chapterLesser
      chapters_greater: $chapterGreater
      volumes_lesser: $volumeLesser
      volumes_greater: $volumeGreater
      licensedById_in: $licensedBy
      isLicensed: $isLicensed
      genre_in: $genres
      genre_not_in: $excludedGenres
      tag_in: $tags
      tag_not_in: $excludedTags
      minimumTagRank: $minimumTagRank
      sort: $sort
      isAdult: $isAdult
    ) {
      id
      title {
        userPreferred
      }
      coverImage {
        extraLarge
        large
        color
      }
      startDate {
        year
        month
        day
      }
      description
      format
    }
  }
}
";

use metadata_provider::{
    async_trait,
    search::{Capabilities, SearchProvider, SearchResult},
};
use serde::Deserialize;
use serde_json::json;

use crate::Instance;

#[async_trait]
impl SearchProvider for Instance {
    fn capabilities(&self) -> Vec<Capabilities> {
        vec![
            Capabilities::TitleExact,
            Capabilities::Year,
            Capabilities::Category,
        ]
    }

    async fn search(
        &self,
        query: &str,
        year: Option<u16>,
        series: Option<bool>,
    ) -> Result<Vec<metadata_provider::search::SearchResult>, metadata_provider::Error> {
        let mut variables = json!({"page":1,"type":"ANIME","sort":"SEARCH_MATCH","search": query});
        if let Some(year) = year {
            variables["year"] = json!(format!("{year}%"));
        }
        if let Some(series) = series {
            variables["format"] = match series {
                true => json!(["TV", "TV_SHORT", "SPECIAL", "OVA", "ONA"]),
                false => json!(["MOVIE"]),
            };
        }
        let body = json!({"query": QUERY, "variables": variables});
        let data: Root1 = self
            .client
            .post("https://graphql.anilist.co")
            .json(&body)
            .send()
            .await?
            .json()
            .await?;
        Ok(data
            .data
            .page
            .media
            .into_iter()
            .map(|v| SearchResult {
                id: v.id.to_string(),
                names: vec![v.title.user_preferred],
                overview: v.description,
                cover: v.cover_image.large,
                kind: v.format,
                first_aired: match v.start_date.year {
                    Some(year) => Some(format!(
                        "{year}-{}-{}",
                        v.start_date
                            .month
                            .map(|v| v.to_string())
                            .unwrap_or("?".to_owned()),
                        v.start_date
                            .day
                            .map(|v| v.to_string())
                            .unwrap_or("?".to_owned()),
                    )),
                    None => None,
                },
            })
            .collect())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CoverImage1 {
    color: Option<String>,
    extra_large: Option<String>,
    large: Option<String>,
}

#[derive(Deserialize)]
struct StartDate1 {
    day: Option<i64>,
    month: Option<i64>,
    year: Option<i64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Title1 {
    user_preferred: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Media1 {
    cover_image: CoverImage1,
    description: Option<String>,
    format: Option<String>,
    id: i64,
    start_date: StartDate1,
    title: Title1,
}

#[derive(Deserialize)]
struct Page1 {
    media: Vec<Media1>,
}

#[derive(Deserialize)]
struct Data1 {
    #[serde(rename = "Page")]
    page: Page1,
}

#[derive(Deserialize)]
struct Root1 {
    data: Data1,
}
