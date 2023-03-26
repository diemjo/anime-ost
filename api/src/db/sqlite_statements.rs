pub const DB_INIT_TABLES: &str =
    "CREATE TABLE IF NOT EXISTS anime (
        proxer_id INT NOT NULL,
        proxer_name TEXT NOT NULL,
        mal_id INT,
        episode_count INT NOT NULL,
        PRIMARY KEY(proxer_id)
    );
    CREATE TABLE IF NOT EXISTS user (
        user_id INT NOT NULL,
        user_name TEXT NOT NULL,
        PRIMARY KEY(user_id)
    );
    CREATE TABLE IF NOT EXISTS user_anime (
        user_id INT NOT NULL,
        episode_progress INT NOT NULL,
        proxer_id INT NOT NULL,
        date_added TEXT NOT NULL,
        PRIMARY KEY(proxer_id, user_id),
        FOREIGN KEY(proxer_id) REFERENCES anime(proxer_id),
        FOREIGN KEY(user_id) REFERENCES user(user_id)
    );
    CREATE TABLE IF NOT EXISTS ost (
        proxer_id INT NOT NULL,
        ost_type TEXT NOT NULL,
        number INT NOT NULL,
        name TEXT,
        artist TEXT,
        video_url TEXT,
        PRIMARY KEY(proxer_id, ost_type, number),
        FOREIGN KEY(proxer_id) REFERENCES anime(proxer_id)
    );";

pub const SELECT_ALL_USERS: &str =
    "SELECT user_id, user_name
    FROM user;";

pub const SELECT_ALL_ANIME: &str =
    "SELECT proxer_id, proxer_name, mal_id, episode_count
    FROM anime;";

pub const SELECT_ALL_USER_ANIME: &str =
    "SELECT user.user_id as user_id, user_name, episode_progress, anime.proxer_id as proxer_id, proxer_name, mal_id, episode_count
    FROM user_anime
    JOIN user ON user.user_id = user_anime.user_id
    JOIN anime ON anime.proxer_id = user_anime.proxer_id;";

pub const INSERT_USER: &str =
    "INSERT OR IGNORE INTO user (user_id, user_name) VALUES (:user_id, :user_name);";

pub const INSERT_USER_ANIME: &str =
    "INSERT OR IGNORE INTO user_anime (user_id, episode_progress, proxer_id, date_added) VALUES (:user_id, :episode_progress, :proxer_id, :date_added);";

pub const INSERT_ANIME: &str =
    "INSERT OR IGNORE INTO anime (proxer_id, proxer_name, mal_id, episode_count) VALUES (:proxer_id, :proxer_name, :mal_id, :episode_count);";