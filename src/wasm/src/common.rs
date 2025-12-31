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
pub const FLASH_CARD_WIDTH: f32 = 300.0;
pub const FLASH_CARD_HEIGHT: f32 = 450.0;
pub const FLASH_CARD_CORNER_RADIUS: f32 = 10.0;
pub const FLASH_CARD_ROTATE_SPEED: f32 = 0.1;
pub const FLASH_CARD_REMOVING_POINT_ROTATE: f32 = 0.2;
pub const FLASH_CARD_ERASE_POINT_ROTATE: f32 = 1.05;
pub const SWIPING_JUDEGE_DISTANCE: i32 = 20;
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
pub const FRONT_ITEMS: [&str; ITEM_SIZE] =
    ["durable", "revue", "arragement", "applicant", "inventory"];
pub const BACK_ITEMS: [&str; ITEM_SIZE] = ["丈夫な", "批評", "配置", "申請者,応募者", "在庫"];
