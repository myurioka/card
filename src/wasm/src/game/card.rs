pub mod card {
    use crate::common::*;
    use crate::game::{Align, Color, Font, Point, Renderer};

    #[derive(Clone, Default)]
    pub struct Card {
        cp: Point,              // center of the card
        width: f32,             // card width
        height: f32,            // card height
        color: Color,           // color of the card (deprecated, use front_color/back_color)
        rotate: f32,            // angle
        front_text: String,     // text on the front of the card
        back_text: String,      // text on the back of the card
        etymology: Vec<String>, // etymologies on the back of the card
        face_state: i32,        // 0: front, 1: back
        alpha: f32,
        auto_rotating: i32, // rotate direction: 0: none, 1: right, -1:left
        flip_angle: f32,    // フリップアニメーション用の角度 (0.0 ~ π)
        is_flipping: bool,  // フリップアニメーション中かどうか
        front_color: Color, // 表面の色
        back_color: Color,  // 裏面の色
    }
    impl Card {
        pub fn new(
            cp: Point,
            width: f32,
            height: f32,
            color: Color,
            front_text: &str,
            back_text: &str,
            etymology: &[&str],
        ) -> Self {
            Card {
                cp: cp,                             // Center of the Card
                width: width,                       // Card Width
                height: height,                     // Card Height
                color: color,                       // Card Color (deprecated)
                rotate: 0.0,                        // Card Rotate
                front_text: front_text.to_string(), // Card Front Text
                back_text: back_text.to_string(),   // Card Back Text
                face_state: 0,
                alpha: 1.0,
                auto_rotating: 0, // 0:non_rotate 1:rotate
                flip_angle: 0.0,
                is_flipping: false,
                front_color: Color::Green,    // 表面は緑色（日本語）
                back_color: Color::RoyalBlue, // 裏面はロイヤルブルー（英語）
                etymology: etymology.iter().map(|s| s.to_string()).collect(),
            }
        }
        pub fn rotate_left(&mut self) {
            self.rotate -= FLASH_CARD_ROTATE_SPEED;

            // 閾値を超えたら自動回転開始
            if self.rotate < -FLASH_CARD_REMOVING_POINT_ROTATE && self.auto_rotating == 0 {
                self.auto_rotating = -1;
            }
        }
        pub fn rotate_right(&mut self) {
            self.rotate += FLASH_CARD_ROTATE_SPEED;

            // 閾値を超えたら自動回転開始
            if self.rotate > FLASH_CARD_REMOVING_POINT_ROTATE && self.auto_rotating == 0 {
                self.auto_rotating = 1;
            }
        }
        pub fn update(&mut self) {
            // フリップアニメーション処理
            if self.is_flipping {
                self.flip_angle += 0.4; // フリップ速度（0.4 より大きい数値は設定不可）

                // 半分（π/2）まで回転したら表裏を切り替え
                if self.flip_angle >= std::f32::consts::PI / 2.0
                    && self.flip_angle - 0.35 < std::f32::consts::PI / 2.0
                {
                    self.face_state = if self.face_state == 0 { 1 } else { 0 };
                }

                // 180度（π）回転したらアニメーション終了
                if self.flip_angle >= std::f32::consts::PI {
                    self.flip_angle = 0.0;
                    self.is_flipping = false;
                }
            }

            // 自動回転中なら継続
            if self.auto_rotating == 1 {
                self.rotate += FLASH_CARD_ROTATE_SPEED;
                // alphaを徐々に減少（フェードアウト効果）
                self.alpha = (self.alpha - 0.1).max(0.0);
            } else if self.auto_rotating == -1 {
                self.rotate -= FLASH_CARD_ROTATE_SPEED;
                // alphaを徐々に減少（フェードアウト効果）
                self.alpha = (self.alpha - 0.1).max(0.0);
            }
        }
        pub fn get_rotate(&self) -> f32 {
            self.rotate
        }
        pub fn should_remove(&self) -> bool {
            self.rotate.abs() > FLASH_CARD_ERASE_POINT_ROTATE
        }
        pub fn is_auto_rotating(&self) -> bool {
            self.auto_rotating != 0
        }
        pub fn get_rotate_direction(&self) -> i32 {
            self.auto_rotating
        }
        pub fn stop_auto_rotating(&mut self) {
            self.auto_rotating = 0;
            self.rotate = 0.0;
            self.alpha = 1.0;
        }
        pub fn reset_card(&mut self) {
            self.auto_rotating = 0;
            self.rotate = 0.0;
            self.alpha = 1.0;
            self.face_state = 0;  // 表面に戻す
            self.flip_angle = 0.0;
            self.is_flipping = false;
        }
        pub fn toggle_face(&mut self) {
            // フリップアニメーション開始
            self.is_flipping = true;
            self.flip_angle = 0.0;
        }

        pub fn is_flipping(&self) -> bool {
            self.is_flipping
        }

        pub fn get_front_text(&self) -> &str {
            &self.front_text
        }

        pub fn get_back_text(&self) -> &str {
            &self.back_text
        }

        pub fn get_face_state(&self) -> i32 {
            self.face_state
        }

        pub fn draw(&self, renderer: &Renderer) {
            // カードの矩形を描画
            let (text, color) = if self.face_state == 0 {
                (&self.front_text, self.front_color)
            } else {
                (&self.back_text, self.back_color)
            };

            // etymology を &str のベクタに変換
            let etymology_refs: Vec<&str> = self.etymology.iter().map(|s| s.as_str()).collect();

            renderer.fill_round_rect_rotate_with_flip(
                &Point {
                    x: self.cp.x, //　Card Center.x
                    y: self.cp.y, //  Card Center.y
                },
                self.rotate,              // Cardの傾き
                self.width,               // Card Width
                self.height,              // Card Height
                FLASH_CARD_CORNER_RADIUS, // Card Conner Radius
                color,                    // 表裏に応じた色
                self.alpha,
                text,
                self.flip_angle, // フリップ角度
                etymology_refs,
            );
        }
    }
}
