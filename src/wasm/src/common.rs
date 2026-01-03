/* <-- CONSTANT VALUE */
pub const FRAME_SIZE: f64 = 1.0 / 60.0 * 10000.0;
pub const SCREEN_WIDTH: f32 = 450.0;
pub const SCREEN_HEIGHT: f32 = 600.0;
//pub const OPENING_MESSAGE_Y: f32 = 250.0;
pub const DISPLAY_MESSAGE: &str = "DISPLAY MESSAGE";
pub const DISPLAY_MESSAGE_Y: f32 = 250.0;
pub const GAMEOVER_MESSAGE: &str = "GAME OVER!";
pub const GAMEOVER_MESSAGE_Y: f32 = 250.0;
pub const GAMECLEAR_MESSAGE: &str = "GAME CLEAR!";
pub const GAMECLEAR_MESSAGE_Y: f32 = 250.0;
pub const FLASH_CARD_NUMBERS: i32 = 5;
pub const FLASH_CARD_WIDTH: f32 = 350.0;
pub const FLASH_CARD_HEIGHT: f32 = 480.0;
pub const FLASH_CARD_CORNER_RADIUS: f32 = 10.0;
pub const FLASH_CARD_ROTATE_SPEED: f32 = 0.1;
pub const FLASH_CARD_REMOVING_POINT_ROTATE: f32 = 0.2;
pub const FLASH_CARD_ERASE_POINT_ROTATE: f32 = 1.05;
pub const SWIPING_JUDEGE_DISTANCE: i32 = 20;
pub const PROGRESS_COUNTER_Y: f32 = 50.0;
pub const PROGRESS_COUNTER_FONT_SIZE: &str = "32px MyFont";
pub const PROGRESS_COUNTER_COLOR: &str = "#72F285";
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
            _ => "#008000ff".to_string(),
        }
    }
}

pub const ITEM_SIZE: usize = 5;
pub const ITEMS: [(&str, &str); ITEM_SIZE] = [
    (
        "ハイキング用の [丈夫な] 靴が必要です。",
        "I need [durable] shoes for hiking",
    ),
    (
        "教職はやりがいのある [職業] です。",
        "Teaching is rewarding [profession].",
    ),
    (
        "あなたはこの奨学金の [資格があり] ます。",
        "You are [eligible] for this shcolarship.",
    ),
    (
        "このホテルでは朝食は [無料] です。",
        "Breakfast is [complimentary] at this hotel",

    ),
    (
        "会社は売上予測を [修正] しました。",
        "The company [revised] its salses forecast",
    ),
];
