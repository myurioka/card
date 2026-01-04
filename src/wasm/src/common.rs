/* <-- CONSTANT VALUE */
pub const FRAME_SIZE: f64 = 1.0 / 60.0 * 10000.0;
pub const SCREEN_WIDTH: f32 = 450.0;
pub const SCREEN_HEIGHT: f32 = 600.0;
//pub const OPENING_MESSAGE_Y: f32 = 250.0;
pub const DISPLAY_MESSAGE: &str = "DISPLAY MESSAGE";
pub const DISPLAY_MESSAGE_Y: f32 = 250.0;
pub const GAMEOVER_MESSAGE: &str = "GAME OVER!";
pub const GAMEOVER_MESSAGE_Y: f32 = 250.0;
pub const GAMECLEAR_MESSAGE: &str = "Great job! You made it!";
pub const GAMECLEAR_MESSAGE_Y: f32 = 250.0;
pub const FLASH_CARD_NUMBERS: i32 = 5;
pub const FLASH_CARD_WIDTH: f32 = 350.0;
pub const FLASH_CARD_HEIGHT: f32 = 480.0;
pub const FLASH_CARD_CORNER_RADIUS: f32 = 10.0;
pub const FLASH_CARD_ROTATE_SPEED: f32 = 0.15;
pub const FLASH_CARD_REMOVING_POINT_ROTATE: f32 = 0.2;
pub const FLASH_CARD_ERASE_POINT_ROTATE: f32 = 1.05;
pub const SWIPING_JUDEGE_DISTANCE: i32 = 20;
pub const PROGRESS_COUNTER_Y: f32 = 50.0;
/* CONSTANT VALUE --> */

#[derive(Clone, Copy, Default)]
pub enum Color {
    Black,
    DarkGreen,
    DeepGreen,
    MiddleGreen,
    #[default]
    Green,
    LightGreen,
    MintGreen,
    PaleGreen,
    White,
    DarkBlue,  // 裏面用の濃い青
    RoyalBlue, // 裏面用のロイヤルブルー
}
impl Color {
    pub fn get(&self) -> String {
        match self {
            Color::White => "#ffffff".to_string(),
            Color::Black => "#000000".to_string(),
            Color::DarkGreen => "#1b271bff".to_string(),
            Color::DeepGreen => "#293d29ff".to_string(),
            Color::Green => "#008000ff".to_string(),
            Color::LightGreen => "#395c39ff".to_string(),
            Color::MintGreen => "#72F285".to_string(),
            Color::DarkBlue => "#1e3a5f".to_string(),
            Color::RoyalBlue => "#4169e1".to_string(),
            _ => "#008000ff".to_string(),
        }
    }
}

pub const ITEM_SIZE: usize = 5;
pub const ITEMS: [(&str, &str, &[&str]); ITEM_SIZE] = [
    (
        "ハイキング用の[丈夫な]靴が必要です",
        "I need [durable] shoes for hiking",
        &[
            "dūrābilis/ラテン語: 長持ちする,耐久性のある",
            "endure（耐える）: en- + dūrus",
        ],
    ),
    (
        "教職はやりがいのある [職業] です",
        "Teaching is rewarding [profession].",
        &[
            "prōfessiō/ラテン語: 公の宣言,告白,誓約",
            "prō-: 前に,公に",
            "fatērī: 認める,告白する,宣言する",
            "profess: 公言する、告白する",
        ],
    ),
    (
        "あなたはこの奨学金の [資格があり] ます",
        "You are [eligible] for this shcolarship.",
        &[
            "eligere/ラテン語: e-/ex-外へ + legere 選ぶ",
            "⇒ 選ばれるに値する、選ばれる資格がある",
            "election: 選挙",
        ],
    ),
    (
        "このホテルでは朝食は [無料] です",
        "Breakfast is [complimentary] at this hotel",
        &[
            "complēreラテン語: 満たす,完成させる",
            "褒める、賛辞の = 元来の意味",
            "無料の,サービスの = 派生的意味",
        ],
    ),
    (
        "会社は売上予測を [修正] しました",
        "The company [revised] its salses forecast",
        &[
            "revidēre/ラテン語: 再び見る,見直す",
            "もう一度見る → 見直す → 改訂する",
            "revision: 改訂、復習",
        ],
    ),
];
