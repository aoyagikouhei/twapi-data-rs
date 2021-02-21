use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Entries {
    hashtags: Option<Vec<Hashtag>>,
    media: Option<Vec<Medium>>,
    urls: Option<Vec<Url>>,
    user_mentions: Option<Vec<UserMentions>>,
    symbols: Option<Vec<Symbol>>,
    polls: Option<Vec<Poll>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Url {
    indices: Vec<u64>,
    url: String,
    expanded_url: String,
    display_url: String,
    unwound: Option<Unwound>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Unwound {
    url: String,
    status: u64,
    title: String,
    description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Hashtag {
    indices: Vec<u64>,
    text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Medium {
    display_url: String,
    expanded_url: String,
    id: u64,
    id_str: String,
    indices: Vec<u64>,
    media_url: String,
    media_url_https: String,   
    sizes: Sizes,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sizes {
    thumb: Size,
    large: Size,
    medium: Size,
    small: Size,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Size {
    h: u64,
    resize: String,
    w: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserMentions {
    name: String,
    indices: Vec<u64>,
    screen_name: String,
    id: u64,
    id_str: String,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Symbol {
    indices: Vec<u64>,
    text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Poll {
    options: Vec<PollOption>,
    end_datetime: String,
    duration_minutes: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PollOption {
    position: u64,
    text: String,
}