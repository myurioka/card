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
pub const GAMECLEAR_MESSAGE_Y: f32 = 300.0; // SCREEN_HEIGHT / 2.0 と同じ（カードと同じ位置）
// 表紙カードのタイトル（\n で改行、カード枚数にはカウントしない）
pub const COVER_TITLE: &str = "第二種\n電気工事士";
pub const FLASH_CARD_NUMBERS: usize = 8;
pub const FLASH_CARD_WIDTH: f32 = 350.0;
pub const FLASH_CARD_HEIGHT: f32 = 480.0;
pub const FLASH_CARD_CORNER_RADIUS: f32 = 10.0;
pub const FLASH_CARD_ROTATE_SPEED: f32 = 0.18;
pub const FLASH_CARD_REMOVING_POINT_ROTATE: f32 = 0.2;
pub const FLASH_CARD_ERASE_POINT_ROTATE: f32 = 1.05;
pub const SWIPING_JUDEGE_DISTANCE: i32 = 20;
pub const PROGRESS_COUNTER_Y: f32 = 50.0;
// リセットボタン（カード束アイコン）の定数
pub const RESET_BUTTON_X: f32 = 400.0; // 右上の位置
pub const RESET_BUTTON_Y: f32 = 50.0;
pub const RESET_BUTTON_WIDTH: f32 = 40.0;
pub const RESET_BUTTON_HEIGHT: f32 = 50.0;
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

pub const ITEMS: [(&str, &str); FLASH_CARD_NUMBERS] = [
    (
        "導体にかかる電磁力の計算式は？",
        "characters/force-formula.svg",
    ),
    (
        "電磁誘導起電力の計算式は？",
        "characters/emf-formula.svg",
    ),
    (
        "電力 (消費電力)の公式",
        "characters/power-formula.svg",
    ),
    (
        "三相3線式 デルタ結線の電流 I の公式",
        "characters/delta-current-formula.svg",
    ),
    (
        "三相3線式 スター結線の電流 I の公式",
        "characters/star-current-formula.svg",
    ),
    (
        "単相3線式 配電方式",
        "characters/single-phase-3wire.svg",
    ),
    (
        "配線図上の表記 E は何を表す？",
        "characters/wiring-symbol-e.svg",
    ),
    (
        "三相かご形誘導機",
        "characters/sync-speed-formula.svg",
    ),
];
