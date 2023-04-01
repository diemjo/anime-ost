use std::{fmt::Display, str::FromStr};

use rocket::request::FromParam;
use rusqlite::{types::{FromSql, FromSqlResult, FromSqlError}, ToSql};
use serde::Serialize;

use crate::error::Error;

// #############################################################################################################
// ANIME USER
// #############################################################################################################

#[derive(Debug, Serialize)]
pub(crate) struct AnimeUser {
    user_id: u32,
    user_name: String,
}

impl AnimeUser {
    pub fn new(user_id: u32, user: String) -> Self {
        AnimeUser { user_id, user_name: user }
    }

    pub fn user_id(&self) -> u32 {
        self.user_id
    }

    pub fn user_name(&self) -> &str {
        &self.user_name
    }
}

impl Display for AnimeUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.user_name, self.user_id)
    }
}

// #############################################################################################################
// ANIME
// #############################################################################################################

#[derive(Debug, Serialize)]
pub(crate) struct Anime {
    proxer_id: u32,
    proxer_name: String,
    mal_id: Option<u32>,
    episode_count: u32,
}

impl Anime {
    pub fn new(proxer_id: u32, proxer_name: String, mal_id: Option<u32>, episode_count: u32) -> Self {
        Anime { proxer_id, proxer_name, mal_id, episode_count }
    }

    pub fn proxer_name(&self) -> &str {
        self.proxer_name.as_str()
    }

    pub fn proxer_id(&self) -> u32 {
        self.proxer_id
    }

    pub fn mal_id(&self) -> Option<u32> {
        self.mal_id
    }

    pub fn episode_count(&self) -> u32 {
        self.episode_count
    }
}

impl AsRef<Anime> for Anime {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Display for Anime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (proxer:{}{}), episodes: {}", self.proxer_name, self.proxer_id, match self.mal_id {
            Some(id) => format!("|mal{}", id),
            None => String::new(),
        }, self.episode_count)
    }
}

// #############################################################################################################
// ANIME USER ENTRY
// #############################################################################################################

#[derive(Debug, Serialize)]
pub(crate) struct AnimeUserEntry {
    #[serde(rename = "proxer_id", with = "anime_user_entry_format")] // only serialize key
    anime: Anime,
    #[serde(rename = "user_id", with = "anime_user_format")] // only serialize key
    user: AnimeUser,
    progress: u32,
}

impl AnimeUserEntry {
    pub fn new(anime: Anime, user: AnimeUser, progress: u32) -> Self {
        AnimeUserEntry { anime, user, progress }
    }

    pub fn anime(&self) -> &Anime {
        &self.anime
    }

    pub fn user(&self) -> &AnimeUser {
        &self.user
    }

    pub fn progress(&self) -> u32 {
        self.progress
    }
}

impl AsRef<AnimeUserEntry> for AnimeUserEntry {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Display for AnimeUserEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}, progress: {}", self.user, self.anime, self.progress)
    }
}

// #############################################################################################################
// ANIME OST TYPE
// #############################################################################################################

#[derive(Debug, Serialize, Clone, Copy)]
pub enum OstType {
    Opening,
    Ending,
}

impl FromStr for OstType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "opening" => Ok(OstType::Opening),
            "ending" => Ok(OstType::Ending),
            _ => Err(Error::EnumParseError("OstType".to_string()))
        }
    }
}

impl FromSql for OstType {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> FromSqlResult<Self> {
        match OstType::from_str(value.as_str()?) {
            Ok(ost_type) => Ok(ost_type),
            Err(e) => Err(FromSqlError::Other(Box::new(e)))
        }
    }
}

impl ToSql for OstType {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        match self {
            Self::Opening => Ok(rusqlite::types::ToSqlOutput::Borrowed("opening".into())),
            Self::Ending => Ok(rusqlite::types::ToSqlOutput::Borrowed("ending".into())),
        }
    }
}

impl<'a> FromParam<'a> for OstType {
    type Error = Error;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        OstType::from_str(param)
    }
}

// #############################################################################################################
// ANIME OST
// #############################################################################################################

#[derive(Debug, Serialize)]
pub(crate) struct AnimeOst {
    proxer_id: u32,
    ost_type: OstType,
    number: u32,
    name: Option<String>,
    artist: Option<String>,
    video_url: Option<String>,
}

impl AnimeOst {
    pub fn new(proxer_id: u32, ost_type: OstType, number: u32, name: Option<String>, artist: Option<String>, video_url: Option<String>) -> Self {
        AnimeOst { proxer_id, ost_type, number, name, artist, video_url }
    }

    pub fn proxer_id(&self) -> u32 {
        self.proxer_id
    }

    pub fn ost_type(&self) -> OstType {
        self.ost_type
    }

    pub fn number(&self) -> u32 {
        self.number
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn artist(&self) -> Option<&str> {
        self.artist.as_deref()
    }

    pub fn video_url(&self) -> Option<&str> {
        self.video_url.as_deref()
    }
}

impl AsRef<AnimeOst> for AnimeOst {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Display for AnimeOst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "{}: {:?} {} - {} by {} ({})",
            self.proxer_id(),
            self.ost_type(),
            self.number(),
            self.name().unwrap_or("NAME_UNKNOWN"),
            self.artist().unwrap_or("ARTIST_UNKNOWN"),
            self.video_url().unwrap_or("NO_VIDEO_URL")
        )
    }
}

// #############################################################################################################
// ANIME FORMATTER
// #############################################################################################################

mod anime_user_entry_format {
    use serde::Serializer;

    use super::Anime;

    pub(super) fn serialize<S: Serializer>(anime: &Anime, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u32(anime.proxer_id())
    }
}

mod anime_user_format {
    use serde::Serializer;

    use super::AnimeUser;

    pub(super) fn serialize<S: Serializer>(user: &AnimeUser, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u32(user.user_id())
    }
}