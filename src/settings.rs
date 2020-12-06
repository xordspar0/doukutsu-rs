use ggez::{Context, GameResult};
use gilrs::GamepadId;
use serde::{Deserialize, Serialize};
use winit::event::VirtualKeyCode;

use crate::input::keyboard_player_controller::KeyboardController;
use crate::input::player_controller::PlayerController;
use crate::player::TargetPlayer;
use crate::input::gilrs_player_controller::GilrsPlayerController;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum ControllerType {
    Keyboard,
    Gamepad(usize),
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub seasonal_textures: bool,
    pub original_textures: bool,
    pub shader_effects: bool,
    pub motion_interpolation: bool,
    pub touch_controls: bool,
    pub player1_key_map: PlayerKeyMap,
    pub player2_key_map: PlayerKeyMap,
    pub player1_controller_type: ControllerType,
    pub player2_controller_type: ControllerType,
    #[serde(skip)]
    pub speed: f64,
    #[serde(skip)]
    pub god_mode: bool,
    #[serde(skip)]
    pub infinite_booster: bool,
    #[serde(skip)]
    pub debug_outlines: bool,
}

fn to_gamepad_id(raw_id: usize) -> GamepadId {
    unsafe {
        std::mem::transmute(raw_id)
    }
}

impl Settings {
    pub fn load(_ctx: &mut Context) -> GameResult<Settings> {
        Ok(Settings::default())
    }

    pub fn create_player1_controller(&self) -> Box<dyn PlayerController> {
        match self.player1_controller_type {
            ControllerType::Keyboard => Box::new(KeyboardController::new(TargetPlayer::Player1)),
            ControllerType::Gamepad(id) => Box::new(GilrsPlayerController::new(to_gamepad_id(id))),
        }
    }

    pub fn create_player2_controller(&self) -> Box<dyn PlayerController> {
        match self.player2_controller_type {
            ControllerType::Keyboard => Box::new(KeyboardController::new(TargetPlayer::Player2)),
            ControllerType::Gamepad(id) => Box::new(GilrsPlayerController::new(to_gamepad_id(id))),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            seasonal_textures: true,
            original_textures: false,
            shader_effects: true,
            motion_interpolation: true,
            touch_controls: cfg!(target_os = "android"),
            player1_key_map: p1_default_keymap(),
            player2_key_map: p2_default_keymap(),
            player1_controller_type: ControllerType::Keyboard,
            player2_controller_type: ControllerType::Keyboard,
            speed: 1.0,
            god_mode: false,
            infinite_booster: false,
            debug_outlines: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct PlayerKeyMap {
    pub left: VirtualKeyCode,
    pub up: VirtualKeyCode,
    pub right: VirtualKeyCode,
    pub down: VirtualKeyCode,
    pub prev_weapon: VirtualKeyCode,
    pub next_weapon: VirtualKeyCode,
    pub jump: VirtualKeyCode,
    pub shoot: VirtualKeyCode,
    pub inventory: VirtualKeyCode,
    pub map: VirtualKeyCode,
}

fn p1_default_keymap() -> PlayerKeyMap {
    PlayerKeyMap {
        left: VirtualKeyCode::Left,
        up: VirtualKeyCode::Up,
        right: VirtualKeyCode::Right,
        down: VirtualKeyCode::Down,
        prev_weapon: VirtualKeyCode::A,
        next_weapon: VirtualKeyCode::S,
        jump: VirtualKeyCode::Z,
        shoot: VirtualKeyCode::X,
        inventory: VirtualKeyCode::Q,
        map: VirtualKeyCode::W,
    }
}

fn p2_default_keymap() -> PlayerKeyMap {
    PlayerKeyMap {
        left: VirtualKeyCode::Comma,
        up: VirtualKeyCode::L,
        right: VirtualKeyCode::Slash,
        down: VirtualKeyCode::Period,
        prev_weapon: VirtualKeyCode::G,
        next_weapon: VirtualKeyCode::H,
        jump: VirtualKeyCode::B,
        shoot: VirtualKeyCode::N,
        inventory: VirtualKeyCode::T,
        map: VirtualKeyCode::Y,
    }
}
