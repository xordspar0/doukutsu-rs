use ggez::{Context, GameResult};
use gilrs::{Axis, Button, GamepadId};
use num_traits::abs;

use crate::bitfield;
use crate::input::player_controller::PlayerController;
use crate::shared_game_state::SharedGameState;

bitfield! {
  #[derive(Clone, Copy)]
  pub struct KeyState(u16);
  impl Debug;

  pub left, set_left: 0;
  pub right, set_right: 1;
  pub up, set_up: 2;
  pub down, set_down: 3;
  pub map, set_map: 4;
  pub inventory, set_inventory: 5;
  pub jump, set_jump: 6;
  pub shoot, set_shoot: 7;
  pub next_weapon, set_next_weapon: 8;
  pub prev_weapon, set_prev_weapon: 9;
  pub start, set_start: 10;
}

/// An implementation of player controller backed by gilrs library.
#[derive(Clone)]
pub struct GilrsPlayerController {
    gamepad_id: GamepadId,
    left_x: f64,
    left_y: f64,
    right_x: f64,
    right_y: f64,
    state: KeyState,
    old_state: KeyState,
    trigger: KeyState,
}

impl GilrsPlayerController {
    pub fn new(gamepad_id: GamepadId) -> GilrsPlayerController {
        GilrsPlayerController {
            gamepad_id,
            left_x: 0.0,
            left_y: 0.0,
            right_x: 0.0,
            right_y: 0.0,
            state: KeyState(0),
            old_state: KeyState(0),
            trigger: KeyState(0),
        }
    }
}

const THRESHOLD: f64 = 0.3;

impl PlayerController for GilrsPlayerController {
    fn update(&mut self, state: &SharedGameState, _ctx: &mut Context) -> GameResult {
        if let Some(gilrs) = state.gilrs.as_ref() {
            if let Some(gamepad) = gilrs.connected_gamepad(self.gamepad_id) {
                let mut axes = [
                    (&mut self.left_x, Axis::LeftStickX),
                    (&mut self.left_y, Axis::LeftStickY),
                    (&mut self.right_x, Axis::RightStickX),
                    (&mut self.right_y, Axis::RightStickY),
                ];

                for (axis_val, id) in axes.iter_mut() {
                    if let Some(axis) = gamepad.axis_data(*id) {
                        **axis_val = if abs(axis.value()) < 0.12 { 0.0 } else { axis.value() } as f64;
                    }
                }

                self.state.set_up(self.left_y > THRESHOLD);
                self.state.set_left(self.left_x < -THRESHOLD);
                self.state.set_down(self.left_y < -THRESHOLD);
                self.state.set_right(self.left_x > THRESHOLD);
                self.state.set_jump(gamepad.is_pressed(Button::South));
                self.state.set_shoot(gamepad.is_pressed(Button::East));
                self.state.set_prev_weapon(gamepad.is_pressed(Button::LeftTrigger));
                self.state.set_next_weapon(gamepad.is_pressed(Button::RightTrigger));
                self.state.set_start(gamepad.is_pressed(Button::Start));
            }
        }

        Ok(())
    }

    fn update_trigger(&mut self) {
        let mut trigger = self.state.0 ^ self.old_state.0;
        trigger &= self.state.0;
        self.old_state = self.state;
        self.trigger = KeyState(trigger);
    }

    fn move_up(&self) -> bool {
        self.left_y > THRESHOLD
    }

    fn move_left(&self) -> bool {
        self.left_x < -THRESHOLD
    }

    fn move_down(&self) -> bool {
        self.left_y < -THRESHOLD
    }

    fn move_right(&self) -> bool {
        self.left_x > THRESHOLD
    }

    fn prev_weapon(&self) -> bool {
        self.state.prev_weapon()
    }

    fn next_weapon(&self) -> bool {
        self.state.next_weapon()
    }

    fn jump(&self) -> bool {
        self.state.jump()
    }

    fn shoot(&self) -> bool {
        self.state.shoot()
    }

    fn trigger_up(&self) -> bool {
        self.trigger.up()
    }

    fn trigger_left(&self) -> bool {
        self.trigger.left()
    }

    fn trigger_down(&self) -> bool {
        self.trigger.down()
    }

    fn trigger_right(&self) -> bool {
        self.trigger.right()
    }

    fn trigger_prev_weapon(&self) -> bool {
        self.trigger.prev_weapon()
    }

    fn trigger_next_weapon(&self) -> bool {
        self.trigger.next_weapon()
    }

    fn trigger_jump(&self) -> bool {
        self.trigger.jump()
    }

    fn trigger_shoot(&self) -> bool {
        self.trigger.shoot()
    }

    fn trigger_menu_ok(&self) -> bool {
        self.trigger.jump()
    }

    fn trigger_menu_back(&self) -> bool {
        self.trigger.shoot()
    }

    fn trigger_menu_pause(&self) -> bool {
        self.trigger.start()
    }

    fn look_up(&self) -> bool {
        self.left_y > THRESHOLD || self.right_y > THRESHOLD
    }

    fn look_left(&self) -> bool {
        self.left_x < -THRESHOLD || self.right_x < -THRESHOLD
    }

    fn look_down(&self) -> bool {
        self.left_y < -THRESHOLD || self.right_y < -THRESHOLD
    }

    fn look_right(&self) -> bool {
        self.left_x > THRESHOLD || self.right_x > THRESHOLD
    }

    fn move_analog_x(&self) -> f64 {
        self.left_x
    }

    fn move_analog_y(&self) -> f64 {
        self.left_y
    }
}
