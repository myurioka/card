use crate::browser::{self, LoopClosure};
use crate::common::*;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use futures::channel::{
    mpsc::{UnboundedReceiver, unbounded},
    oneshot::channel,
};

use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Mutex};
use wasm_bindgen::{JsCast, JsValue, prelude::Closure};
use web_sys::{CanvasRenderingContext2d, HtmlImageElement, Touch, TouchEvent, TouchList};

#[derive(Clone, Copy, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x: x, y: y }
    }
}
pub enum Font {
    Larger,
    Middle,
    Smaller,
}
impl Font {
    pub fn get(&self) -> String {
        match self {
            Font::Larger => "24px MyFont".to_string(),
            Font::Middle => "18px MyFont".to_string(),
            Font::Smaller => "14px MyFont".to_string(),
            _ => "18px Myfont".to_string(),
        }
    }
}
pub enum Align {
    Center,
    Left,
    Right,
}
impl Align {
    pub fn get(&self) -> String {
        match self {
            Align::Center => "center".to_string(),
            Align::Left => "left".to_string(),
            Align::Right => "right".to_string(),
            _ => "center".to_string(),
        }
    }
}
pub struct Message {}
impl Message {
    pub fn new() -> Self {
        Message {}
    }
    pub fn draw(&self, renderer: &Renderer, msg: &[String]) {
        let _ = renderer.polygon(
            &Point { x: 400.0, y: 275.0 },
            &Point { x: 525.0, y: 150.0 },
            &Point { x: 526.0, y: 750.0 },
            &Point { x: 402.0, y: 625.0 },
            Color::Green,
        );
        let _ = renderer.polygon(
            &Point { x: -75.0, y: 150.0 },
            &Point { x: 50.0, y: 275.0 },
            &Point { x: 50.0, y: 625.0 },
            &Point { x: -75.0, y: 750.0 },
            Color::Green,
        );
        let _ = renderer.polygon(
            &Point { x: 325.0, y: 350.0 },
            &Point { x: 400.0, y: 275.0 },
            &Point { x: 400.0, y: 625.0 },
            &Point { x: 325.0, y: 550.0 },
            Color::Green,
        );

        let _ = renderer.polygon(
            &Point { x: 50.0, y: 275.0 },
            &Point { x: 125.0, y: 350.0 },
            &Point { x: 125.0, y: 550.0 },
            &Point { x: 50.0, y: 625.0 },
            Color::Green,
        );
        let mut _interval = 0.0;
        for _m in msg {
            let _ = renderer.text(
                &Point {
                    x: 50.0,
                    y: 360.0 + _interval,
                },
                _m,
                Align::Left,
                Font::Middle,
                Color::Green,
            );
            _interval += 20.0;
        }
    }
}

pub struct Renderer {
    context: CanvasRenderingContext2d,
}

impl Renderer {
    pub fn clear(&self) {
        self.context.clear_rect(
            0.0,
            0.0,
            browser::canvas().unwrap().client_width().into(),
            browser::canvas().unwrap().client_height().into(),
        );
    }
    pub fn text(&self, point: &Point, text: &str, align: Align, font: Font, color: Color) {
        self.context.set_fill_style_str(&color.get());
        match align {
            Align::Center => self.context.set_text_align(&align.get()),
            Align::Left => self.context.set_text_align(&align.get()),
            Align::Right => self.context.set_text_align(&align.get()),
            _ => self.context.set_text_align(&align.get()),
        }
        self.context.set_font(&font.get());
        let _ = self.context.fill_text(text, point.x as f64, point.y as f64);
    }
    pub fn line(&self, a: &Point, b: &Point, color: Color) {
        self.context.set_stroke_style_str(&color.get());
        self.context.set_fill_style_str(&color.get());
        self.context.begin_path();
        self.context.move_to(a.x as f64, a.y as f64);
        self.context.line_to(b.x as f64, b.y as f64);
        self.context.close_path();
        self.context.stroke();
        self.context.fill();
    }
    pub fn polygon(&self, a: &Point, b: &Point, c: &Point, d: &Point, color: Color) {
        self.context.set_stroke_style_str(&color.get());
        self.context.set_fill_style_str(&color.get());
        self.context.begin_path();
        self.context.move_to(a.x as f64, a.y as f64);
        self.context.line_to(b.x as f64, b.y as f64);
        self.context.line_to(c.x as f64, c.y as f64);
        self.context.line_to(d.x as f64, d.y as f64);
        self.context.line_to(a.x as f64, a.y as f64);
        self.context.close_path();
        self.context.stroke();
        self.context.fill();
    }

    pub fn fill_round_rect_rotate(
        &self,
        cp: &Point, // 中心座標
        rotate: f32,
        width: f32,
        height: f32,
        radius: f32,
        color: Color,
        alpha: f32,
        text: &str,
    ) {
        self.context.save();
        self.context.set_global_alpha(alpha.into());
        self.context.set_stroke_style_str(&color.get());
        self.context.set_fill_style_str(&color.get());
        self.context.begin_path();

        // 1. 回転中心(cp.x, cp.y + 350)に移動
        let _ = self.context.translate(cp.x as f64, (cp.y + 380.0) as f64);

        // 2. 回転
        let _ = self.context.rotate(rotate as f64);

        // 3. 矩形を描画（回転中心から見た相対位置で描画）
        // カードの中心がcpになるように、回転軸から-350だけY方向にオフセット
        let _ = self.context.round_rect_with_f64(
            -(width / 2.0) as f64,
            -(height / 2.0 + 350.0) as f64,
            width as f64,
            height as f64,
            radius as f64,
        );

        let _ = self.context.close_path();
        self.context.fill();

        // 装飾パターンを描画（トランプ風）
        self.draw_card_decoration(width, height);

        // 4. テキストを描画（同じ回転座標系で）
        self.context.set_text_align("center");
        self.context.set_font("24px MyFont");
        self.context.set_fill_style_str("white");
        // カードの中心にテキストを配置（Y座標は-350でカードの中央）
        let _ = self.context.fill_text(&text, 0.0, -380.0);

        self.context.restore();
    }

    fn draw_card_decoration(&self, width: f32, height: f32) {
        self.context.set_fill_style_str("rgba(255, 255, 255, 0.3)");
        self.context
            .set_stroke_style_str("rgba(255, 255, 255, 0.5)");
        self.context.set_line_width(2.5);

        let card_left = -(width / 2.0) as f64;
        let card_top = -(height / 2.0 + 350.0) as f64;
        let center_x = card_left + (width / 2.0) as f64;
        let center_y = card_top + (height / 2.0) as f64 - 80.0; // テキストの上

        // 背景全体にケルト文様の枠線を描画
        self.context.set_line_width(1.5);
        self.context
            .set_stroke_style_str("rgba(255, 255, 255, 0.2)");

        // カード周囲にケルト風の二重線
        let border_offset = 10.0;
        self.context.begin_path();
        let _ = self.context.round_rect_with_f64(
            card_left + border_offset,
            card_top + border_offset,
            width as f64 - border_offset * 2.0,
            height as f64 - border_offset * 2.0,
            8.0,
        );
        self.context.stroke();

        let border_offset2 = 15.0;
        self.context.begin_path();
        let _ = self.context.round_rect_with_f64(
            card_left + border_offset2,
            card_top + border_offset2,
            width as f64 - border_offset2 * 2.0,
            height as f64 - border_offset2 * 2.0,
            6.0,
        );
        self.context.stroke();

        // 背景に繊細なケルト結び目パターンを散りばめる
        self.context.set_line_width(1.0);
        self.context
            .set_stroke_style_str("rgba(255, 255, 255, 0.15)");

        // 上部と下部にケルト風の連続模様
        let positions = [
            (center_x - 60.0, card_top + 40.0),
            (center_x, card_top + 40.0),
            (center_x + 60.0, card_top + 40.0),
            (center_x - 60.0, card_top + height as f64 - 40.0),
            (center_x, card_top + height as f64 - 40.0),
            (center_x + 60.0, card_top + height as f64 - 40.0),
        ];

        for &(px, py) in positions.iter() {
            // 小さなケルト結び目
            self.context.begin_path();
            let _ = self
                .context
                .arc(px, py, 6.0, 0.0, 2.0 * std::f64::consts::PI);
            self.context.stroke();

            // 交差する曲線
            self.context.begin_path();
            self.context.move_to(px - 8.0, py - 8.0);
            self.context
                .bezier_curve_to(px, py, px, py, px + 8.0, py + 8.0);
            self.context.stroke();

            self.context.begin_path();
            self.context.move_to(px - 8.0, py + 8.0);
            self.context
                .bezier_curve_to(px, py, px, py, px + 8.0, py - 8.0);
            self.context.stroke();
        }

        // 中央部分の装飾を強調
        self.context.set_line_width(2.5);
        self.context
            .set_stroke_style_str("rgba(255, 255, 255, 0.5)");

        // ケルト文様：組紐（インターレース）パターンを描画
        // 中央の三つ編み結び目
        let knot_size = 30.0;

        // 中央の円形ノット
        self.context.begin_path();
        let _ = self.context.arc(
            center_x,
            center_y,
            knot_size / 2.0,
            0.0,
            2.0 * std::f64::consts::PI,
        );
        self.context.stroke();

        // 組紐パターン：3つの曲線で構成
        let curve_offset = knot_size * 0.8;

        // 左上から右下への曲線
        self.context.begin_path();
        self.context
            .move_to(center_x - curve_offset, center_y - curve_offset);
        self.context.bezier_curve_to(
            center_x - curve_offset / 2.0,
            center_y,
            center_x,
            center_y - curve_offset / 2.0,
            center_x + curve_offset,
            center_y + curve_offset,
        );
        self.context.stroke();

        // 右上から左下への曲線
        self.context.begin_path();
        self.context
            .move_to(center_x + curve_offset, center_y - curve_offset);
        self.context.bezier_curve_to(
            center_x + curve_offset / 2.0,
            center_y,
            center_x,
            center_y - curve_offset / 2.0,
            center_x - curve_offset,
            center_y + curve_offset,
        );
        self.context.stroke();

        // 上下の曲線（縦方向の組紐）
        self.context.begin_path();
        self.context
            .move_to(center_x, center_y - curve_offset * 1.2);
        self.context.bezier_curve_to(
            center_x + curve_offset / 3.0,
            center_y - curve_offset / 2.0,
            center_x - curve_offset / 3.0,
            center_y + curve_offset / 2.0,
            center_x,
            center_y + curve_offset * 1.2,
        );
        self.context.stroke();

        // 四隅のケルト三つ巴模様
        let corner_offset = 20.0;
        let spiral_size = 10.0;

        let corners = [
            (card_left + corner_offset, card_top + corner_offset),
            (
                card_left + width as f64 - corner_offset,
                card_top + corner_offset,
            ),
            (
                card_left + corner_offset,
                card_top + height as f64 - corner_offset,
            ),
            (
                card_left + width as f64 - corner_offset,
                card_top + height as f64 - corner_offset,
            ),
        ];

        for &(cx, cy) in corners.iter() {
            // スパイラル状のケルト模様
            self.context.begin_path();
            self.context.move_to(cx, cy);
            self.context.bezier_curve_to(
                cx + spiral_size,
                cy - spiral_size / 2.0,
                cx + spiral_size / 2.0,
                cy + spiral_size,
                cx,
                cy,
            );
            self.context.stroke();

            // 小さな円
            self.context.begin_path();
            let _ = self
                .context
                .arc(cx, cy, 4.0, 0.0, 2.0 * std::f64::consts::PI);
            self.context.fill();
        }

        // 左右の装飾：ケルトノット
        let side_offset = 45.0;
        for offset in [-side_offset, side_offset].iter() {
            let x = center_x + offset;

            // 小さな組紐結び目
            self.context.begin_path();
            self.context.move_to(x - 8.0, center_y - 10.0);
            self.context.bezier_curve_to(
                x,
                center_y - 12.0,
                x,
                center_y + 12.0,
                x - 8.0,
                center_y + 10.0,
            );
            self.context.stroke();

            self.context.begin_path();
            self.context.move_to(x + 8.0, center_y - 10.0);
            self.context.bezier_curve_to(
                x,
                center_y - 12.0,
                x,
                center_y + 12.0,
                x + 8.0,
                center_y + 10.0,
            );
            self.context.stroke();
        }
    }
    pub fn fill_round_rect_rotate_with_flip(
        &self,
        cp: &Point, // 中心座標
        rotate: f32,
        width: f32,
        height: f32,
        radius: f32,
        color: Color,
        alpha: f32,
        text: &str,
        flip_angle: f32, // フリップ角度
    ) {
        self.context.save();
        self.context.set_global_alpha(alpha.into());
        self.context.set_stroke_style_str(&color.get());
        self.context.set_fill_style_str(&color.get());

        // 1. 回転中心(cp.x, cp.y + 350)に移動
        let _ = self.context.translate(cp.x as f64, (cp.y + 380.0) as f64);

        // 2. 回転
        let _ = self.context.rotate(rotate as f64);

        // 3. フリップ効果（Y軸方向のスケール変更）
        let scale_x = (flip_angle.cos()).abs();
        let _ = self.context.scale(scale_x as f64, 1.0);

        // 4. 矩形を描画（回転中心から見た相対位置で描画）
        self.context.begin_path();
        let _ = self.context.round_rect_with_f64(
            -(width / 2.0) as f64,
            -(height / 2.0 + 350.0) as f64,
            width as f64,
            height as f64,
            radius as f64,
        );

        let _ = self.context.close_path();
        self.context.fill();

        // 装飾パターンを描画（トランプ風）
        self.draw_card_decoration(width, height);

        // 5. テキストを描画（同じ回転座標系で）
        self.context.set_text_align("center");
        self.context.set_font("24px MyFont");
        self.context.set_fill_style_str("white");
        // カードの中心にテキストを配置（Y座標は-350でカードの中央）
        let _ = self.context.fill_text(&text, 0.0, -340.0);

        self.context.restore();
    }

    pub fn draw_tarot_background(&self, width: f32, height: f32) {
        self.context.save();

        // 背景を黒で塗りつぶし
        self.context.set_fill_style_str("#000000");
        self.context
            .fill_rect(0.0, 0.0, width as f64, height as f64);

        // タロット風の装飾を描画
        self.context.set_stroke_style_str("rgba(138, 43, 226, 0.3)"); // 紫色
        self.context.set_line_width(1.5);

        let center_x = (width / 2.0) as f64;
        let center_y = (height / 2.0) as f64;

        // 中央に大きな円を描画
        self.context.begin_path();
        let _ = self
            .context
            .arc(center_x, center_y, 200.0, 0.0, 2.0 * std::f64::consts::PI);
        self.context.stroke();

        // 中央の円の内側にもう一つ円を描画
        self.context.begin_path();
        let _ = self
            .context
            .arc(center_x, center_y, 180.0, 0.0, 2.0 * std::f64::consts::PI);
        self.context.stroke();

        // 8方向に放射線を描画
        for i in 0..8 {
            let angle = (i as f64) * std::f64::consts::PI / 4.0;
            let x1 = center_x + 180.0 * angle.cos();
            let y1 = center_y + 180.0 * angle.sin();
            let x2 = center_x + 200.0 * angle.cos();
            let y2 = center_y + 200.0 * angle.sin();

            self.context.begin_path();
            self.context.move_to(x1, y1);
            self.context.line_to(x2, y2);
            self.context.stroke();
        }

        // 四隅に星のような模様を描画
        let corners = [
            (50.0, 50.0),                                // 左上
            (width as f64 - 50.0, 50.0),                 // 右上
            (50.0, height as f64 - 50.0),                // 左下
            (width as f64 - 50.0, height as f64 - 50.0), // 右下
        ];

        self.context.set_stroke_style_str("rgba(255, 215, 0, 0.4)"); // 金色
        self.context.set_line_width(1.0);

        for (cx, cy) in corners.iter() {
            // 5点の星を描画
            self.context.begin_path();
            for i in 0..6 {
                let angle =
                    (i as f64) * std::f64::consts::PI * 2.0 / 5.0 - std::f64::consts::PI / 2.0;
                let radius = if i % 2 == 0 { 15.0 } else { 7.0 };
                let x = cx + radius * angle.cos();
                let y = cy + radius * angle.sin();

                if i == 0 {
                    self.context.move_to(x, y);
                } else {
                    self.context.line_to(x, y);
                }
            }
            self.context.close_path();
            self.context.stroke();
        }

        // 画面上部と下部に装飾的な線を描画
        self.context.set_stroke_style_str("rgba(138, 43, 226, 0.4)");
        self.context.set_line_width(2.0);

        // 上部の装飾線
        self.context.begin_path();
        self.context.move_to(50.0, 100.0);
        self.context.line_to(width as f64 - 50.0, 100.0);
        self.context.stroke();

        // 下部の装飾線
        self.context.begin_path();
        self.context.move_to(50.0, height as f64 - 100.0);
        self.context
            .line_to(width as f64 - 50.0, height as f64 - 100.0);
        self.context.stroke();

        // 中央に六芒星（ダビデの星）を描画
        self.context.set_stroke_style_str("rgba(138, 43, 226, 0.2)");
        self.context.set_line_width(1.5);

        let star_radius = 100.0;

        // 上向き三角形
        self.context.begin_path();
        for i in 0..4 {
            let angle = (i as f64) * std::f64::consts::PI * 2.0 / 3.0 - std::f64::consts::PI / 2.0;
            let x = center_x + star_radius * angle.cos();
            let y = center_y + star_radius * angle.sin();

            if i == 0 {
                self.context.move_to(x, y);
            } else {
                self.context.line_to(x, y);
            }
        }
        self.context.stroke();

        // 下向き三角形
        self.context.begin_path();
        for i in 0..4 {
            let angle = (i as f64) * std::f64::consts::PI * 2.0 / 3.0 + std::f64::consts::PI / 2.0;
            let x = center_x + star_radius * angle.cos();
            let y = center_y + star_radius * angle.sin();

            if i == 0 {
                self.context.move_to(x, y);
            } else {
                self.context.line_to(x, y);
            }
        }
        self.context.stroke();

        self.context.restore();
    }
}

#[async_trait(?Send)]
pub trait Game {
    async fn initialize(&self) -> Result<Box<dyn Game>>;
    fn update(
        &mut self,
        keystate: &mut KeyState,
        touchstate: &mut TouchState,
        mousestate: &mut MouseState,
    );
    fn draw(&self, renderer: &Renderer);
}

pub struct GameLoop {
    last_frame: f64,
    accumulated_delta: f64,
}
type SharedLoopClosure = Rc<RefCell<Option<LoopClosure>>>;

impl GameLoop {
    pub async fn start(game: impl Game + 'static) -> Result<()> {
        let mut keyevent_receiver = prepare_input()?;
        let mut touch_receiver = prepare_touch_input()?;
        let mut mouse_receiver = prepare_mouse_input()?;
        let mut game = game.initialize().await?;
        let mut game_loop = GameLoop {
            last_frame: browser::now()?.into(),
            accumulated_delta: 0.0,
        };

        let renderer = Renderer {
            context: browser::context()?,
        };

        let f: SharedLoopClosure = Rc::new(RefCell::new(None));
        let g = f.clone();

        let mut keystate = KeyState::new();
        let mut touchstate = TouchState::new(
            browser::canvas()?.offset_left(),
            browser::canvas().unwrap().client_width(),
            browser::canvas().unwrap().client_height(),
        );
        let mut mousestate = MouseState::new(browser::canvas()?.offset_left());

        *g.borrow_mut() = Some(browser::create_raf_closure(move |perf: f64| {
            process_input(&mut keystate, &mut keyevent_receiver);
            process_touch_input(&mut touchstate, &mut touch_receiver);
            process_mouse_input(&mut mousestate, &mut mouse_receiver);

            game_loop.accumulated_delta += perf - game_loop.last_frame;
            while game_loop.accumulated_delta > FRAME_SIZE {
                game.update(&mut keystate, &mut touchstate, &mut mousestate);

                // 次のフレームのためにクリック/タップフラグをクリア
                mousestate.clear_just_clicked();
                touchstate.clear_just_tapped();

                game_loop.accumulated_delta -= FRAME_SIZE;
            }
            let _last_frame = browser::now().unwrap();
            game.draw(&renderer);
            game_loop.last_frame = _last_frame;

            let _ = browser::request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        browser::request_animation_frame(
            g.borrow()
                .as_ref()
                .ok_or_else(|| anyhow!("GameLoop: Loop is None"))?,
        )?;
        Ok(())
    }
}

pub struct TouchState {
    x: i32,        // current client_x
    y: i32,        // current client_y
    start_x: i32,  // touch start x position
    start_y: i32,  // touch start y position
    s: bool,       // true: pressed, false: unpressed
    offset_x: i32, // Canvas offset
    screen_width: i32,
    screen_height: i32,
    just_tapped: bool, // タップが検出されたフレームでのみtrue
}
impl TouchState {
    fn new(offset_x: i32, screen_width: i32, screen_height: i32) -> Self {
        return TouchState {
            x: 0,
            y: 0,
            start_x: 0,
            start_y: 0,
            s: false,
            offset_x: offset_x,
            screen_width: screen_width,
            screen_height: screen_height,
            just_tapped: false,
        };
    }
    pub fn is_swiping_left(&self) -> bool {
        if self.s {
            let delta_x = self.x - self.start_x;
            // 左方向に30px以上スワイプしている場合
            return delta_x < -SWIPING_JUDEGE_DISTANCE;
        }
        false
    }
    pub fn is_swiping_right(&self) -> bool {
        if self.s {
            let delta_x = self.x - self.start_x;
            // 右方向に30px以上スワイプしている場合
            return delta_x > SWIPING_JUDEGE_DISTANCE;
        }
        false
    }
    pub fn is_tapped(&self) -> bool {
        // タップが検出されたフレームでのみtrueを返す（エッジトリガー）
        self.just_tapped
    }
    fn set_pressed(&mut self, _x: i32, _y: i32) {
        self.x = _x;
        self.y = _y;
        self.start_x = _x;
        self.start_y = _y;
        self.s = true;
    }
    fn set_moved(&mut self, _x: i32, _y: i32) {
        self.x = _x;
        self.y = _y;
    }
    fn set_released(&mut self) {
        self.s = false;

        // タップ判定（移動距離が小さい場合）
        let delta_x = (self.x - self.start_x).abs();
        let delta_y = (self.y - self.start_y).abs();
        if delta_x < 10 && delta_y < 10 {
            // タップが検出されたのでフラグを立てる
            self.just_tapped = true;
        }
    }
    pub fn clear_just_tapped(&mut self) {
        self.just_tapped = false;
    }

    pub fn clear(&mut self) {
        // delta_x (x - start_x) を0にするため、xとstart_xを同じ値に設定
        self.x = self.start_x;
        self.y = self.start_y;
        self.s = false;
        self.just_tapped = false;
    }
}
enum TouchPress {
    TouchStart(TouchEvent),
    TouchMove(TouchEvent),
    TouchEnd(TouchEvent),
}

fn process_touch_input(state: &mut TouchState, touch_receiver: &mut UnboundedReceiver<TouchPress>) {
    loop {
        match touch_receiver.try_next() {
            Ok(None) => break,
            Err(_err) => break,
            Ok(Some(evt)) => match evt {
                TouchPress::TouchStart(evt) => {
                    let _list: TouchList = evt.touches();
                    let _touch: Touch = _list.item(0).unwrap();
                    state.set_pressed(_touch.client_x(), _touch.client_y());
                }
                TouchPress::TouchMove(evt) => {
                    let _list: TouchList = evt.touches();
                    let _touch: Touch = _list.item(0).unwrap();
                    state.set_moved(_touch.client_x(), _touch.client_y());
                }
                TouchPress::TouchEnd(evt) => {
                    // changedTouchesから座標を取得（canvas外でreleaseされた場合に対応）
                    let _list: TouchList = evt.changed_touches();
                    if let Some(_touch) = _list.item(0) {
                        state.set_moved(_touch.client_x(), _touch.client_y());
                    } else {
                        // 座標が取得できない場合、delta_xを0にするためxをstart_xに設定
                        state.x = state.start_x;
                        state.y = state.start_y;
                    }
                    state.set_released();
                }
            },
        };
    }
}

// For Touch Input
fn prepare_touch_input() -> Result<UnboundedReceiver<TouchPress>> {
    let (touch_start_sender, touch_receiver) = unbounded();
    let touch_start_sender = Rc::new(RefCell::new(touch_start_sender));
    let touch_move_sender = Rc::clone(&touch_start_sender);
    let touch_end_sender = Rc::clone(&touch_start_sender);
    let ontouchstart = browser::closure_wrap(Box::new(move |_touchcode: TouchEvent| {
        _touchcode.prevent_default();
        let _ = touch_start_sender
            .borrow_mut()
            .start_send(TouchPress::TouchStart(_touchcode));
    }) as Box<dyn FnMut(TouchEvent)>);
    let ontouchmove = browser::closure_wrap(Box::new(move |_touchcode: TouchEvent| {
        _touchcode.prevent_default();
        let _ = touch_move_sender
            .borrow_mut()
            .start_send(TouchPress::TouchMove(_touchcode));
    }) as Box<dyn FnMut(TouchEvent)>);
    let ontouchend = browser::closure_wrap(Box::new(move |_touchcode: TouchEvent| {
        _touchcode.prevent_default();
        let _ = touch_end_sender
            .borrow_mut()
            .start_send(TouchPress::TouchEnd(_touchcode));
    }) as Box<dyn FnMut(TouchEvent)>);

    browser::canvas()?.set_ontouchstart(Some(ontouchstart.as_ref().unchecked_ref()));
    browser::canvas()?.set_ontouchmove(Some(ontouchmove.as_ref().unchecked_ref()));
    browser::canvas()?.set_ontouchend(Some(ontouchend.as_ref().unchecked_ref()));
    ontouchstart.forget();
    ontouchmove.forget();
    ontouchend.forget();

    Ok(touch_receiver)
}

pub struct MouseState {
    x: i32,             // current mouse x
    y: i32,             // current mouse y
    start_x: i32,       // mouse down x position
    start_y: i32,       // mouse down y position
    pressed: bool,      // true: mouse button pressed
    offset_x: i32,      // Canvas offset
    just_clicked: bool, // クリックが検出されたフレームでのみtrue
}
impl MouseState {
    fn new(offset_x: i32) -> Self {
        return MouseState {
            x: 0,
            y: 0,
            start_x: 0,
            start_y: 0,
            pressed: false,
            offset_x: offset_x,
            just_clicked: false,
        };
    }
    pub fn is_dragging_left(&self) -> bool {
        if self.pressed {
            let delta_x = self.x - self.start_x;
            // 左方向に30px以上ドラッグしている場合
            return delta_x < -30;
        }
        false
    }
    pub fn is_dragging_right(&self) -> bool {
        if self.pressed {
            let delta_x = self.x - self.start_x;
            // 右方向に30px以上ドラッグしている場合
            return delta_x > 30;
        }
        false
    }
    pub fn is_clicked(&self) -> bool {
        // クリックが検出されたフレームでのみtrueを返す（エッジトリガー）
        self.just_clicked
    }
    fn set_pressed(&mut self, _x: i32, _y: i32) {
        self.x = _x;
        self.y = _y;
        self.start_x = _x;
        self.start_y = _y;
        self.pressed = true;
    }
    fn set_moved(&mut self, _x: i32, _y: i32) {
        self.x = _x;
        self.y = _y;
    }
    fn set_released(&mut self) {
        self.pressed = false;

        // クリック判定（移動距離が小さい場合）
        let delta_x = (self.x - self.start_x).abs();
        let delta_y = (self.y - self.start_y).abs();
        if delta_x < 10 && delta_y < 10 {
            // クリックが検出されたのでフラグを立てる
            self.just_clicked = true;
        }
    }
    pub fn clear_just_clicked(&mut self) {
        self.just_clicked = false;
    }

    pub fn clear(&mut self) {
        // delta_x (x - start_x) を0にするため、xとstart_xを同じ値に設定
        self.x = self.start_x;
        self.y = self.start_y;
        self.pressed = false;
        self.just_clicked = false;
    }
}

pub struct KeyState {
    pressed_keys: HashMap<String, web_sys::KeyboardEvent>,
}

impl KeyState {
    fn new() -> Self {
        return KeyState {
            pressed_keys: HashMap::new(),
        };
    }
    pub fn is_pressed(&self, code: &str) -> bool {
        self.pressed_keys.contains_key(code)
    }

    fn set_pressed(&mut self, code: &str, event: web_sys::KeyboardEvent) {
        self.pressed_keys.insert(code.into(), event);
    }

    fn set_released(&mut self, code: &str) {
        self.pressed_keys.remove(code.into());
    }

    pub fn clear(&mut self) {
        self.pressed_keys.clear();
    }
}

enum KeyPress {
    KeyUp(web_sys::KeyboardEvent),
    KeyDown(web_sys::KeyboardEvent),
}

fn process_input(state: &mut KeyState, keyevent_receiver: &mut UnboundedReceiver<KeyPress>) {
    loop {
        match keyevent_receiver.try_next() {
            Ok(None) => break,
            Err(_err) => break,
            Ok(Some(evt)) => match evt {
                KeyPress::KeyUp(evt) => state.set_released(&evt.code()),
                KeyPress::KeyDown(evt) => state.set_pressed(&evt.code(), evt),
            },
        };
    }
}

// For Mouse Input
enum MousePress {
    MouseDown(web_sys::MouseEvent),
    MouseMove(web_sys::MouseEvent),
    MouseUp(web_sys::MouseEvent),
}

fn process_mouse_input(state: &mut MouseState, mouse_receiver: &mut UnboundedReceiver<MousePress>) {
    loop {
        match mouse_receiver.try_next() {
            Ok(None) => break,
            Err(_err) => break,
            Ok(Some(evt)) => match evt {
                MousePress::MouseDown(evt) => {
                    state.set_pressed(evt.client_x(), evt.client_y());
                }
                MousePress::MouseMove(evt) => {
                    if state.pressed {
                        state.set_moved(evt.client_x(), evt.client_y());
                    }
                }
                MousePress::MouseUp(evt) => {
                    // canvas外でreleaseされた場合にも座標を更新
                    state.set_moved(evt.client_x(), evt.client_y());
                    state.set_released();
                }
            },
        };
    }
}

fn prepare_mouse_input() -> Result<UnboundedReceiver<MousePress>> {
    let (mouse_down_sender, mouse_receiver) = unbounded();
    let mouse_down_sender = Rc::new(RefCell::new(mouse_down_sender));
    let mouse_move_sender = Rc::clone(&mouse_down_sender);
    let mouse_up_sender = Rc::clone(&mouse_down_sender);

    let onmousedown = browser::closure_wrap(Box::new(move |evt: web_sys::MouseEvent| {
        let _ = mouse_down_sender
            .borrow_mut()
            .start_send(MousePress::MouseDown(evt));
    }) as Box<dyn FnMut(web_sys::MouseEvent)>);

    let onmousemove = browser::closure_wrap(Box::new(move |evt: web_sys::MouseEvent| {
        let _ = mouse_move_sender
            .borrow_mut()
            .start_send(MousePress::MouseMove(evt));
    }) as Box<dyn FnMut(web_sys::MouseEvent)>);

    let onmouseup = browser::closure_wrap(Box::new(move |evt: web_sys::MouseEvent| {
        let _ = mouse_up_sender
            .borrow_mut()
            .start_send(MousePress::MouseUp(evt));
    }) as Box<dyn FnMut(web_sys::MouseEvent)>);

    browser::canvas()?.set_onmousedown(Some(onmousedown.as_ref().unchecked_ref()));
    browser::canvas()?.set_onmousemove(Some(onmousemove.as_ref().unchecked_ref()));
    browser::canvas()?.set_onmouseup(Some(onmouseup.as_ref().unchecked_ref()));
    onmousedown.forget();
    onmousemove.forget();
    onmouseup.forget();

    Ok(mouse_receiver)
}

// For Keypress Input
fn prepare_input() -> Result<UnboundedReceiver<KeyPress>> {
    let (keydown_sender, keyevent_receiver) = unbounded();
    let keydown_sender = Rc::new(RefCell::new(keydown_sender));
    let keyup_sender = Rc::clone(&keydown_sender);
    let onkeydown = browser::closure_wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
        let _ = keydown_sender
            .borrow_mut()
            .start_send(KeyPress::KeyDown(keycode));
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

    let onkeyup = browser::closure_wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
        let _ = keyup_sender
            .borrow_mut()
            .start_send(KeyPress::KeyUp(keycode));
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

    browser::canvas()?.set_onkeydown(Some(onkeydown.as_ref().unchecked_ref()));
    browser::canvas()?.set_onkeyup(Some(onkeyup.as_ref().unchecked_ref()));
    onkeydown.forget();
    onkeyup.forget();

    Ok(keyevent_receiver)
}
