mod card;
use crate::common::*;
use crate::engine::{
    Align, Font, Game, KeyState, Message, MouseState, Point, Renderer, TouchState,
};
use anyhow::Result;
use async_trait::async_trait;
use card::card::*;
use web_sys::HtmlImageElement;

/// ゲーム全体の状態を管理するメイン構造体
pub struct GameStage {
    machine: Option<GameStageStateMachine>,
}
impl GameStage {
    /// 新しいGameStageインスタンスを作成
    pub fn new() -> Self {
        GameStage { machine: None }
    }
}
/// ゲームの状態を表すステートマシン
/// Playing: プレイ中、DisplayMessage: メッセージ表示、GameOver: ゲームオーバー、GameClear: ゲームクリア
enum GameStageStateMachine {
    Playing(GameStageState<Playing>),
    DisplayMessage(GameStageState<DisplayMessage>),
    GameOver(GameStageState<GameOver>),
    GameClear(GameStageState<GameClear>),
}
impl GameStageStateMachine {
    /// 新しいステートマシンを作成（初期状態はPlaying）
    fn new(material: Material) -> Self {
        GameStageStateMachine::Playing(GameStageState::new(material))
    }
    /// ステートマシンの状態を更新
    /// 各状態に応じたupdateを呼び出し、状態遷移を行う
    fn update(
        self,
        _keystate: &mut KeyState,
        _touchstate: &mut TouchState,
        _mousestate: &mut MouseState,
    ) -> Self {
        match self {
            GameStageStateMachine::Playing(state) => {
                state.update(_keystate, _touchstate, _mousestate).into()
            }
            GameStageStateMachine::DisplayMessage(state) => {
                state.update(_keystate, _touchstate, _mousestate).into()
            }
            GameStageStateMachine::GameOver(state) => {
                state.update(_keystate, _touchstate, _mousestate).into()
            }
            GameStageStateMachine::GameClear(state) => {
                state.update(_keystate, _touchstate, _mousestate).into()
            }
        }
    }
    /// 現在の状態に応じて描画を行う
    fn draw(&self, renderer: &Renderer) {
        match self {
            GameStageStateMachine::Playing(state) => state.material.draw(renderer),
            GameStageStateMachine::DisplayMessage(state) => state.material.draw(renderer),
            GameStageStateMachine::GameOver(state) => state.material.draw(renderer),
            GameStageStateMachine::GameClear(state) => state.material.draw(renderer),
        };
    }
}
impl From<GameStageState<Playing>> for GameStageStateMachine {
    fn from(state: GameStageState<Playing>) -> Self {
        GameStageStateMachine::Playing(state)
    }
}
impl From<GameStageState<DisplayMessage>> for GameStageStateMachine {
    fn from(state: GameStageState<DisplayMessage>) -> Self {
        GameStageStateMachine::DisplayMessage(state)
    }
}
impl From<GameStageState<GameOver>> for GameStageStateMachine {
    fn from(state: GameStageState<GameOver>) -> Self {
        GameStageStateMachine::GameOver(state)
    }
}
impl From<GameStageState<GameClear>> for GameStageStateMachine {
    fn from(state: GameStageState<GameClear>) -> Self {
        GameStageStateMachine::GameClear(state)
    }
}

struct GameStageState<T> {
    _state: T,
    material: Material,
}

// Display Message
struct DisplayMessage;
impl GameStageState<DisplayMessage> {
    fn start_running(self) -> GameStageState<Playing> {
        GameStageState {
            _state: Playing,
            material: self.material,
        }
    }
    fn update(
        self,
        _keystate: &mut KeyState,
        _touchstate: &mut TouchState,
        _mousestate: &mut MouseState,
    ) -> DisplayMessageEndState {
        if _keystate.is_pressed("Space") {
            return DisplayMessageEndState::Complete(self.start_running());
        }
        DisplayMessageEndState::Continue(self)
    }
}
enum DisplayMessageEndState {
    Complete(GameStageState<Playing>),
    Continue(GameStageState<DisplayMessage>),
}
impl From<DisplayMessageEndState> for GameStageStateMachine {
    fn from(state: DisplayMessageEndState) -> Self {
        match state {
            DisplayMessageEndState::Complete(running) => running.into(),
            DisplayMessageEndState::Continue(focus) => focus.into(),
        }
    }
}
/// プレイ中状態
struct Playing;
impl GameStageState<Playing> {
    /// 新しいPlaying状態を作成
    fn new(material: Material) -> GameStageState<Playing> {
        GameStageState {
            _state: Playing,
            material,
        }
    }
    /// プレイ中の状態を更新
    /// カードの回転、削除、状態遷移を処理
    fn update(
        mut self,
        _keystate: &mut KeyState,
        _touchstate: &mut TouchState,
        _mousestate: &mut MouseState,
    ) -> PlayingEndState {
        self.material.frame += 1;

        // カードの自動回転を更新 閾値を超えた場合）
        if let Some(card) = self.material.cards.first_mut() {
            card.update();
        }

        // 削除中のカードも自動回転を継続
        if let Some(removing_card) = &mut self.material.removing_card {
            removing_card.update();
        }

        // ユーザー入力によるカード操作（自動回転中でない場合のみ）
        if let Some(card) = self.material.cards.first() {
            if !card.is_auto_rotating() {
                // タップ/クリックでカードの表裏を切り替え
                if _touchstate.is_tapped() || _mousestate.is_clicked() {
                    if let Some(card) = self.material.cards.first_mut() {
                        card.toggle_face();
                    }
                }

                // 左方向への回転
                if _keystate.is_pressed("ArrowLeft")
                    || _touchstate.is_swiping_left()
                    || _mousestate.is_dragging_left()
                {
                    if let Some(card) = self.material.cards.first_mut() {
                        card.rotate_left();
                    }
                }
                // 右方向への回転
                if _keystate.is_pressed("ArrowRight")
                    || _touchstate.is_swiping_right()
                    || _mousestate.is_dragging_right()
                {
                    if let Some(card) = self.material.cards.first_mut() {
                        card.rotate_right();
                    }
                }
            }
        }

        // 自動回転が開始されたら（閾値0.78超え）次のカードを準備
        if let Some(card) = self.material.cards.first() {
            if card.is_auto_rotating() && !self.material.next_card_ready {
                self.material.next_card_ready = true;
            }
        }

        // カードの回転が閾値(FLASH_CARD_ERASE_POINT_ROTATE)を超えたら削除
        if let Some(card) = self.material.cards.first() {
            if card.should_remove() {
                // 入力状態をクリア
                _keystate.clear();
                _touchstate.clear();
                _mousestate.clear();

                // 削除中のカードとして保存
                self.material.removing_card = Some(self.material.cards.remove(0));
                self.material.next_card_ready = false;

                // プログレスカウンターを更新
                self.material.current_card_index += 1;

                // 次のカードの自動回転を停止
                if let Some(next_card) = self.material.cards.first_mut() {
                    next_card.stop_auto_rotating();
                }

                // 全てのカードがなくなったらゲームクリア
                if self.material.cards.is_empty() {
                    // 少し待ってから削除中のカードもクリア
                    self.material.removing_card = None;
                    return PlayingEndState::GameClear(GameStageState {
                        _state: GameClear,
                        material: self.material,
                    });
                }
            }
        }

        // 削除中のカードがある場合、完全に画面外に出たらクリア
        if let Some(removing_card) = &self.material.removing_card {
            if removing_card.get_rotate().abs() > 1.5 {
                self.material.removing_card = None;
            }
        }

        if _keystate.is_pressed("Space") {
            return PlayingEndState::Message(GameStageState {
                _state: DisplayMessage,
                material: self.material,
            });
        }
        PlayingEndState::Continue(self)
    }
}
impl From<PlayingEndState> for GameStageStateMachine {
    fn from(state: PlayingEndState) -> Self {
        match state {
            PlayingEndState::Continue(running) => running.into(),
            PlayingEndState::Message(message) => message.into(),
            PlayingEndState::GameOver(gameover) => gameover.into(),
            PlayingEndState::GameClear(gameclear) => gameclear.into(),
        }
    }
}
enum PlayingEndState {
    Continue(GameStageState<Playing>),
    Message(GameStageState<DisplayMessage>),
    GameOver(GameStageState<GameOver>),
    GameClear(GameStageState<GameClear>),
}
struct GameOver;
impl GameStageState<GameOver> {
    fn update(
        self,
        _keystate: &mut KeyState,
        _touchstate: &mut TouchState,
        _mousestate: &mut MouseState,
    ) -> GameOverEndState {
        if _keystate.is_pressed("Space") {
            GameOverEndState::Complete(self.new_game())
        } else {
            GameOverEndState::Continue(self)
        }
    }
    fn new_game(self) -> GameStageState<Playing> {
        GameStageState {
            _state: Playing,
            material: Material::reset(&self.material),
        }
    }
}
enum GameOverEndState {
    Continue(GameStageState<GameOver>),
    Complete(GameStageState<Playing>),
}
impl From<GameOverEndState> for GameStageStateMachine {
    fn from(state: GameOverEndState) -> Self {
        match state {
            GameOverEndState::Continue(game_over) => game_over.into(),
            GameOverEndState::Complete(playing) => playing.into(),
        }
    }
}

struct GameClear;
impl GameStageState<GameClear> {
    fn update(
        self,
        _keystate: &mut KeyState,
        _touchstate: &mut TouchState,
        _mousestate: &mut MouseState,
    ) -> GameClearEndState {
        if _keystate.is_pressed("Space") {
            GameClearEndState::Complete(self.new_game())
        } else {
            GameClearEndState::Continue(self)
        }
    }
    fn new_game(self) -> GameStageState<Playing> {
        GameStageState {
            _state: Playing,
            material: Material::reset(&self.material),
        }
    }
}
enum GameClearEndState {
    Continue(GameStageState<GameClear>),
    Complete(GameStageState<Playing>),
}
impl From<GameClearEndState> for GameStageStateMachine {
    fn from(state: GameClearEndState) -> Self {
        match state {
            GameClearEndState::Continue(game_clear) => game_clear.into(),
            GameClearEndState::Complete(play) => play.into(),
        }
    }
}

/// ゲームの素材（カード、フレームカウンタなど）を管理する構造体
pub struct Material {
    frame: i32,                  // フレームカウンタ
    cards: Vec<Card>,            // カードの配列（最大{FLASH_CARD_NUMBERS}枚）
    removing_card: Option<Card>, // 削除中のカード
    next_card_ready: bool,       // 次のカードの準備完了フラグ
    current_card_index: i32,     // 現在のカード番号（1から始まる）
    total_cards: i32,            // 総カード枚数
}
impl Material {
    /// 新しいMaterialインスタンスを作成
    /// {FLASH_CARD_NUMBERS}枚のカード（"Card 1"〜"Card {FLASH_CARD_NUMBERS}"）を初期化
    fn new() -> Self {
        let mut cards = Vec::new();

        // {FLASH_CARD_NUMBERS} 個のカードを作成
        for i in 0..FLASH_CARD_NUMBERS {
            // ITEMSの範囲内でループさせる
            let front_text = ITEMS[(i % ITEM_SIZE as i32) as usize].0;
            let back_text = ITEMS[(i % ITEM_SIZE as i32) as usize].1;
            let etymologies = ITEMS[(i % ITEM_SIZE as i32) as usize].2;
            let card = Card::new(
                Point::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
                FLASH_CARD_WIDTH,
                FLASH_CARD_HEIGHT,
                Color::Green,
                front_text,
                back_text,
                etymologies,
            );
            cards.push(card);
        }

        Material {
            frame: 0,
            cards: cards,
            removing_card: None,
            next_card_ready: false,
            current_card_index: 1,
            total_cards: FLASH_CARD_NUMBERS,
        }
    }
    /// Materialをリセット（新しいインスタンスを作成）
    fn reset(&self) -> Material {
        Material::new()
    }
    /// カードを描画
    /// 削除中のカードがある場合はそれを描画し、準備完了なら次のカードも表示
    fn draw(&self, _renderer: &Renderer) {
        // 次のカードを先に描画（背面）
        if self.next_card_ready {
            if let Some(card) = self.cards.get(0) {
                card.draw(_renderer);
            }
        }

        // 削除中のカードを描画（前面）
        if let Some(removing_card) = &self.removing_card {
            removing_card.draw(_renderer);
        } else if !self.next_card_ready {
            // 削除中でも次カード準備中でもない場合、最初のカードを描画
            if let Some(card) = self.cards.first() {
                card.draw(_renderer);
            }
        }

        // プログレスカウンターを描画（カードの上部・ケルト風）
        // カードの表裏に応じて色を変更
        let counter_color = if let Some(card) = self.cards.first() {
            if card.get_face_state() == 0 {
                Color::MintGreen.get() // 表面: ミントグリーン
            } else {
                Color::RoyalBlue.get() // 裏面: ロイヤルブルー
            }
        } else {
            Color::MintGreen.get() // デフォルト
        };

        let progress_text = format!("{}/{}", self.current_card_index, self.total_cards);
        _renderer.celtic_progress_counter(
            &Point {
                x: SCREEN_WIDTH / 2.0,
                y: PROGRESS_COUNTER_Y,
            },
            &progress_text,
            &counter_color,
        );
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait(?Send)]
impl Game for GameStage {
    /// ゲームの初期化
    /// Materialを作成し、ステートマシンをPlaying状態で開始
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        log!("START");
        match &self.machine {
            _none => {
                let machine = GameStageStateMachine::new(Material::new());
                Ok(Box::new(GameStage {
                    machine: Some(machine),
                }))
            }
        }
    }
    /// ゲーム全体の更新処理
    /// ステートマシンのupdateを呼び出し、入力に応じた状態遷移を行う
    fn update(
        &mut self,
        _keystate: &mut KeyState,
        _touchstate: &mut TouchState,
        _mousestate: &mut MouseState,
    ) {
        if let Some(machine) = self.machine.take() {
            self.machine
                .replace(machine.update(_keystate, _touchstate, _mousestate));
        }
        assert!(self.machine.is_some());
    }
    /// ゲーム全体の描画処理
    /// 画面をクリアし、現在の状態に応じたメッセージとカードを描画
    fn draw(&self, renderer: &Renderer) {
        renderer.clear();
        match &self.machine {
            Some(GameStageStateMachine::Playing(_state)) => {
                // 最初のカードのみ描画
                if let Some(card) = _state.material.cards.first() {
                    card.draw(renderer);
                }
            }
            Some(GameStageStateMachine::DisplayMessage(_state)) => {}
            Some(GameStageStateMachine::GameOver(_state)) => {
                let _ = renderer.text(
                    &Point {
                        x: SCREEN_WIDTH / 2.0,
                        y: GAMEOVER_MESSAGE_Y,
                    },
                    GAMEOVER_MESSAGE,
                    Align::Center,
                    Font::Middle,
                    Color::Green,
                );
            }
            Some(GameStageStateMachine::GameClear(_state)) => {
                // 最初のカードのみ描画
                if let Some(card) = _state.material.cards.first() {
                    card.draw(renderer);
                }
                let _ = renderer.text(
                    &Point {
                        x: SCREEN_WIDTH / 2.0,
                        y: GAMECLEAR_MESSAGE_Y,
                    },
                    GAMECLEAR_MESSAGE,
                    Align::Center,
                    Font::Middle,
                    Color::Green,
                );
            }
            _ => {}
        }
        if let Some(machine) = &self.machine {
            machine.draw(renderer);
        }
    }
}
