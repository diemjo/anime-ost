use std::fmt::Display;

use serde::Serialize;

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
    proxer_name: String,
    proxer_id: u32,
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
    #[serde(with = "anime_user_entry_format")] // only serialize key
    anime: Anime,
    #[serde(with = "anime_user_format")] // only serialize key
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
// ANIME FORMATTER
// #############################################################################################################

mod anime_user_entry_format {
    use serde::Serializer;

    use super::Anime;

    pub(super) fn serialize<S: Serializer>(anime: &Anime, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&anime.proxer_id().to_string())
    }
}

mod anime_user_format {
    use serde::Serializer;

    use super::AnimeUser;

    pub(super) fn serialize<S: Serializer>(user: &AnimeUser, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&user.user_id().to_string())
    }
}