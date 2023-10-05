#![feature(test)]

extern crate test;

#[macro_use]
extern crate sval_derive;
#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, miniserde::Serialize, Value)]
pub struct Twitter {
    statuses: Vec<Status>,
    search_metadata: SearchMetadata,
}

#[derive(Serialize, Deserialize, miniserde::Serialize, Value)]
pub struct Status {
    metadata: Metadata,
    created_at: String,
    id: u64,
    id_str: String,
    text: String,
    source: String,
    truncated: bool,
    in_reply_to_status_id: Option<u64>,
    in_reply_to_status_id_str: Option<String>,
    in_reply_to_user_id: Option<u32>,
    in_reply_to_user_id_str: Option<String>,
    in_reply_to_screen_name: Option<String>,
    user: User,
    geo: (),
    coordinates: (),
    place: (),
    contributors: (),
    retweeted_status: Option<Box<Status>>,
    retweet_count: u32,
    favorite_count: u32,
    entities: StatusEntities,
    favorited: bool,
    retweeted: bool,
    possibly_sensitive: Option<bool>,
    lang: String,
}

#[derive(Serialize, Deserialize, miniserde::Serialize, Value)]
pub struct Metadata {
    result_type: String,
    iso_language_code: String,
}

#[derive(Serialize, Deserialize, miniserde::Serialize, Value)]
pub struct User {
    id: u32,
    id_str: String,
    name: String,
    screen_name: String,
    location: String,
    description: String,
    url: Option<String>,
    entities: UserEntities,
    protected: bool,
    followers_count: u32,
    friends_count: u32,
    listed_count: u32,
    created_at: String,
    favourites_count: u32,
    utc_offset: Option<i32>,
    time_zone: Option<String>,
    geo_enabled: bool,
    verified: bool,
    statuses_count: u32,
    lang: String,
    contributors_enabled: bool,
    is_translator: bool,
    is_translation_enabled: bool,
    profile_background_color: String,
    profile_background_image_url: String,
    profile_background_image_url_https: String,
    profile_background_tile: bool,
    profile_image_url: String,
    profile_image_url_https: String,
    profile_banner_url: Option<String>,
    profile_link_color: String,
    profile_sidebar_border_color: String,
    profile_sidebar_fill_color: String,
    profile_text_color: String,
    profile_use_background_image: bool,
    default_profile: bool,
    default_profile_image: bool,
    following: bool,
    follow_request_sent: bool,
    notifications: bool,
}

#[derive(Serialize, Deserialize, miniserde::Serialize, Value)]
pub struct UserEntities {
    url: Option<UserUrl>,
    description: UserEntitiesDescription,
}

#[derive(Serialize, Deserialize, miniserde::Serialize, Value)]
pub struct UserUrl {
    urls: Vec<Url>,
}

#[derive(Serialize, Deserialize, miniserde::Serialize, Value)]
pub struct Url {
    url: String,
    expanded_url: String,
    display_url: String,
    indices: Indices,
}

#[derive(Serialize, Deserialize, miniserde::Serialize, Value)]
pub struct UserEntitiesDescription {
    urls: Vec<Url>,
}

#[derive(Serialize, Deserialize, miniserde::Serialize, Value)]
pub struct StatusEntities {
    hashtags: Vec<Hashtag>,
    symbols: Vec<()>,
    urls: Vec<Url>,
    user_mentions: Vec<UserMention>,
    media: Option<Vec<Media>>,
}

#[derive(Serialize, Deserialize, miniserde::Serialize, Value)]
pub struct Hashtag {
    text: String,
    indices: Indices,
}

#[derive(Serialize, Deserialize, miniserde::Serialize, Value)]
pub struct UserMention {
    screen_name: String,
    name: String,
    id: u32,
    id_str: String,
    indices: Indices,
}

#[derive(Serialize, Deserialize, miniserde::Serialize, Value)]
pub struct Media {
    id: u64,
    id_str: String,
    indices: Indices,
    media_url: String,
    media_url_https: String,
    url: String,
    display_url: String,
    expanded_url: String,
    #[serde(rename = "type")]
    #[sval(label = "type")]
    media_type: String,
    sizes: Sizes,
    source_status_id: Option<u64>,
    source_status_id_str: Option<String>,
}

#[derive(Serialize, Deserialize, miniserde::Serialize, Value)]
pub struct Sizes {
    medium: Size,
    small: Size,
    thumb: Size,
    large: Size,
}

#[derive(Serialize, Deserialize, miniserde::Serialize, Value)]
pub struct Size {
    w: u16,
    h: u16,
    resize: String,
}

pub type Indices = (u8, u8);

#[derive(Serialize, Deserialize, miniserde::Serialize, Value)]
pub struct SearchMetadata {
    completed_in: f32,
    max_id: u64,
    max_id_str: String,
    next_results: String,
    query: String,
    refresh_url: String,
    count: u8,
    since_id: u64,
    since_id_str: String,
}

pub fn input_json() -> String {
    std::fs::read_to_string("./twitter.json").unwrap()
}

pub fn input_struct() -> Twitter {
    let j = input_json();
    serde_json::from_str(&j).unwrap()
}

#[test]
fn compat() {
    let serde = serde_json::to_string(&input_struct()).unwrap();
    let sval = sval_json::stream_to_string(input_struct()).unwrap();

    assert_eq!(serde, sval);
}

#[bench]
fn primitive_miniserde(b: &mut test::Bencher) {
    b.iter(|| miniserde::json::to_string(&42));
}

#[bench]
fn primitive_serde(b: &mut test::Bencher) {
    b.iter(|| serde_json::to_string(&42).unwrap());
}

#[bench]
fn primitive_erased_serde(b: &mut test::Bencher) {
    let s: Box<dyn erased_serde::Serialize> = Box::new(42);

    b.iter(|| serde_json::to_string(&s).unwrap());
}

#[bench]
fn primitive_sval(b: &mut test::Bencher) {
    b.iter(|| sval_json::stream_to_string(&42).unwrap());
}

#[bench]
fn twitter_miniserde(b: &mut test::Bencher) {
    let s = input_struct();
    b.iter(|| miniserde::json::to_string(&s));
}

#[bench]
fn twitter_serde(b: &mut test::Bencher) {
    let s = input_struct();
    b.iter(|| serde_json::to_string(&s).unwrap());
}

#[bench]
fn twitter_erased_serde(b: &mut test::Bencher) {
    let s: Box<dyn erased_serde::Serialize> = Box::new(input_struct());
    b.iter(|| serde_json::to_string(&s).unwrap());
}

#[bench]
fn twitter_sval(b: &mut test::Bencher) {
    let s = input_struct();
    b.iter(|| sval_json::stream_to_string(&s).unwrap());
}

#[bench]
fn twitter_sval_dynamic(b: &mut test::Bencher) {
    let s: Box<dyn sval_dynamic::Value> = Box::new(input_struct());
    b.iter(|| sval_json::stream_to_string(&s).unwrap());
}

#[bench]
fn twitter_sval_to_serde(b: &mut test::Bencher) {
    let s = input_struct();
    b.iter(|| serde_json::to_string(&sval_serde::ToSerialize::new(&s)).unwrap());
}

#[bench]
fn twitter_serde_to_sval(b: &mut test::Bencher) {
    let s = input_struct();
    b.iter(|| sval_json::stream_to_string(sval_serde::ToValue::new(&s)).unwrap());
}

#[bench]
fn twitter_serde_to_sval_to_serde(b: &mut test::Bencher) {
    let s = input_struct();
    b.iter(|| {
        serde_json::to_string(&sval_serde::ToSerialize::new(sval_serde::ToValue::new(&s))).unwrap()
    });
}

#[bench]
fn twitter_sval_collect(b: &mut test::Bencher) {
    let s = input_struct();
    b.iter(|| sval_buffer::Value::collect(&s).unwrap());
}

#[bench]
fn twitter_sval_collect_owned(b: &mut test::Bencher) {
    let s = input_struct();
    b.iter(|| sval_buffer::Value::collect(&s).unwrap().into_owned().unwrap());
}

#[bench]
fn twitter_serde_collect(b: &mut test::Bencher) {
    let s = input_struct();
    b.iter(|| serde_json::to_value(&s).unwrap());
}
