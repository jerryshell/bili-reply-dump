use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Root {
    pub code: i64,
    pub message: String,
    pub ttl: i64,
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub page: Page,
    pub list: Option<Vec<List>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    pub rpid: Option<i64>,
    pub oid: Option<i64>,
    #[serde(rename = "type")]
    pub list_type: Option<i64>,
    pub mid: Option<i64>,
    pub root: Option<i64>,
    pub parent: Option<i64>,
    pub dialog: Option<i64>,
    pub count: Option<i64>,
    pub rcount: Option<i64>,
    pub floor: Option<i64>,
    pub state: Option<i64>,
    pub fansgrade: Option<i64>,
    pub attr: Option<i64>,
    pub ctime: Option<i64>,
    pub like: Option<i64>,
    pub action: Option<i64>,
    pub member: Option<ListMember>,
    pub content: Option<ListContent>,
    pub replies: Option<serde_json::Value>,
    pub assist: Option<i64>,
    pub up_action: Option<UpAction>,
    pub invisible: Option<bool>,
    pub folder: Option<Folder>,
    pub track_info: Option<String>,
    pub title: Option<String>,
    pub cover_url: Option<String>,
    pub bvid: Option<String>,
    pub reply_control: Option<ReplyControl>,
    pub parent_info: Option<ParentInfo>,
    pub root_info: Option<RootInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListContent {
    pub message: Option<String>,
    pub at_name_to_mid: Option<HashMap<String, i64>>,
    pub at_name_to_mid_str: Option<HashMap<String, String>>,
    pub members: Option<Vec<MemberElement>>,
    pub jump_url: Option<HashMap<String, JumpUrlValue>>,
    pub max_line: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JumpUrlValue {
    pub title: Option<String>,
    pub state: Option<i64>,
    pub prefix_icon: Option<String>,
    pub app_url_schema: Option<String>,
    pub app_name: Option<String>,
    pub app_package_name: Option<String>,
    pub click_report: Option<String>,
    pub is_half_screen: Option<bool>,
    pub exposure_report: Option<String>,
    pub extra: Option<Extra>,
    pub underline: Option<bool>,
    pub match_once: Option<bool>,
    pub pc_url: Option<String>,
    pub icon_position: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Extra {
    pub goods_show_type: Option<i64>,
    pub is_word_search: Option<bool>,
    pub goods_cm_control: Option<i64>,
    pub goods_click_report: Option<String>,
    pub goods_exposure_report: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberElement {
    pub mid: Option<String>,
    pub uname: Option<String>,
    pub sex: Option<String>,
    pub sign: Option<String>,
    pub avatar: Option<String>,
    pub rank: Option<String>,
    pub face_nft_new: Option<i64>,
    pub is_senior_member: Option<i64>,
    pub senior: Option<Senior>,
    pub level_info: Option<LevelInfo>,
    pub pendant: Option<Pendant>,
    pub nameplate: Option<Nameplate>,
    pub official_verify: Option<OfficialVerify>,
    pub vip: Option<Vip>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelInfo {
    pub current_level: Option<i64>,
    pub current_min: Option<i64>,
    pub current_exp: Option<i64>,
    pub next_exp: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Nameplate {
    pub nid: Option<i64>,
    pub name: Option<String>,
    pub image: Option<String>,
    pub image_small: Option<String>,
    pub level: Option<String>,
    pub condition: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfficialVerify {
    #[serde(rename = "type")]
    pub official_verify_type: Option<i64>,
    pub desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pendant {
    pub pid: Option<i64>,
    pub name: Option<String>,
    pub image: Option<String>,
    pub expire: Option<i64>,
    pub image_enhance: Option<String>,
    pub image_enhance_frame: Option<String>,
    pub n_pid: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Senior {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vip {
    pub vip_type: Option<i64>,
    pub vip_due_date: Option<i64>,
    pub due_remark: Option<String>,
    pub access_status: Option<i64>,
    pub vip_status: Option<i64>,
    pub vip_status_warn: Option<String>,
    pub theme_type: Option<i64>,
    pub label: Option<Label>,
    #[serde(rename = "avatar_subscript")]
    pub avatar_subscript: Option<i64>,
    #[serde(rename = "nickname_color")]
    pub nickname_color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    pub path: Option<String>,
    pub text: Option<String>,
    pub label_theme: Option<String>,
    pub text_color: Option<String>,
    pub bg_style: Option<i64>,
    pub bg_color: Option<String>,
    pub border_color: Option<String>,
    pub use_img_label: Option<bool>,
    pub img_label_uri_hans: Option<String>,
    pub img_label_uri_hant: Option<String>,
    pub img_label_uri_hans_static: Option<String>,
    pub img_label_uri_hant_static: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Folder {
    pub has_folded: Option<bool>,
    pub is_folded: Option<bool>,
    pub rule: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListMember {
    pub mid: Option<String>,
    pub uname: Option<String>,
    pub sex: Option<String>,
    pub sign: Option<String>,
    pub avatar: Option<String>,
    pub rank: Option<String>,
    pub face_nft_new: Option<i64>,
    pub is_senior_member: Option<i64>,
    pub senior: Option<Senior>,
    pub level_info: Option<LevelInfo>,
    pub pendant: Option<Pendant>,
    pub nameplate: Option<Nameplate>,
    pub official_verify: Option<OfficialVerify>,
    pub vip: Option<Vip>,
    pub fans_detail: Option<serde_json::Value>,
    pub user_sailing: Option<serde_json::Value>,
    pub is_contractor: Option<bool>,
    pub contract_desc: Option<String>,
    pub nft_interaction: Option<serde_json::Value>,
    pub avatar_item: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentInfo {
    pub rpid: Option<i64>,
    pub oid: Option<i64>,
    #[serde(rename = "type")]
    pub parent_info_type: Option<i64>,
    pub mid: Option<i64>,
    pub root: Option<i64>,
    pub parent: Option<i64>,
    pub dialog: Option<i64>,
    pub count: Option<i64>,
    pub rcount: Option<i64>,
    pub floor: Option<i64>,
    pub state: Option<i64>,
    pub fansgrade: Option<i64>,
    pub attr: Option<i64>,
    pub ctime: Option<i64>,
    pub like: Option<i64>,
    pub action: Option<i64>,
    pub member: Option<ListMember>,
    pub content: Option<ParentInfoContent>,
    pub replies: Option<serde_json::Value>,
    pub assist: Option<i64>,
    pub up_action: Option<UpAction>,
    pub invisible: Option<bool>,
    pub reply_control: Option<Senior>,
    pub folder: Option<Folder>,
    pub track_info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentInfoContent {
    pub message: Option<String>,
    pub at_name_to_mid: Option<HashMap<String, i64>>,
    pub at_name_to_mid_str: Option<HashMap<String, String>>,
    pub members: Option<Vec<MemberElement>>,
    pub jump_url: Option<HashMap<String, JumpUrlValue>>,
    pub max_line: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpAction {
    pub like: Option<bool>,
    pub reply: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplyControl {
    pub up_like: Option<bool>,
    pub is_elec: Option<bool>,
    pub followed: Option<bool>,
    pub is_garbage: Option<bool>,
    pub is_top: Option<bool>,
    pub is_charge_plus: Option<bool>,
    pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootInfo {
    pub rpid: Option<i64>,
    pub oid: Option<i64>,
    #[serde(rename = "type")]
    pub root_info_type: Option<i64>,
    pub mid: Option<i64>,
    pub root: Option<i64>,
    pub parent: Option<i64>,
    pub dialog: Option<i64>,
    pub count: Option<i64>,
    pub rcount: Option<i64>,
    pub floor: Option<i64>,
    pub state: Option<i64>,
    pub fansgrade: Option<i64>,
    pub attr: Option<i64>,
    pub ctime: Option<i64>,
    pub like: Option<i64>,
    pub action: Option<i64>,
    pub member: Option<ListMember>,
    pub content: Option<RootInfoContent>,
    pub replies: Option<serde_json::Value>,
    pub assist: Option<i64>,
    pub up_action: Option<UpAction>,
    pub invisible: Option<bool>,
    pub reply_control: Option<Senior>,
    pub folder: Option<Folder>,
    pub track_info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RootInfoContent {
    pub message: Option<String>,
    pub members: Option<serde_json::Value>,
    pub jump_url: Option<PurpleJumpUrl>,
    pub max_line: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PurpleJumpUrl {
    pub dma: Option<JumpUrlValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub num: i64,
    pub size: i64,
    pub total: i64,
}
