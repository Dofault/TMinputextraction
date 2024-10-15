// Code here is a modified and expanded form of rusty-xinput's XInputState
// (https://github.com/Lokathor/rusty-xinput)

use winapi::um::xinput::XINPUT_STATE;
use winapi::um::xinput::*;
use winapi::shared::minwindef::WORD;

pub enum ButtonState {
  UP,
  DOWN,
}

pub struct MutableXInputState {
    /// The raw value we're wrapping.
    pub ptr: *mut XINPUT_STATE,
  }
  
  impl ::std::cmp::PartialEq for MutableXInputState {
    /// Equality for `MutableXInputState` values is based _only_ on the
    /// `dwPacketNumber` of the wrapped `XINPUT_STATE` value. This is entirely
    /// correct for values obtained from the xinput system, but if you make your
    /// own `MutableXInputState` values for some reason you can confuse it.
    fn eq(&self, other: &MutableXInputState) -> bool {
      unsafe {
        (*self.ptr).dwPacketNumber == (*other.ptr).dwPacketNumber
      }
    }
  }
  
  impl ::std::cmp::Eq for MutableXInputState {}
  
  impl ::std::fmt::Debug for MutableXInputState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      write!(f, "MutableXInputState (_)")
    }
  }
  
  impl MutableXInputState {
    pub fn from_ptr(ptr: *mut XINPUT_STATE) -> Self {
      Self { ptr }
    }

    // Sets the given wButton on the controller up or down
    fn set_button_bit(&self, bit_mask: WORD, state: ButtonState) {
      match state {
        ButtonState::UP => {
          unsafe {
            (*self.ptr).Gamepad.wButtons &= !bit_mask;
          }
        },
        ButtonState::DOWN => {
          unsafe {
            (*self.ptr).Gamepad.wButtons |= bit_mask;
          }
        }
      }

    }

    /// The north button of the action button group.
    ///
    /// * Nintendo: X
    /// * Playstation: Triangle
    /// * XBox: Y
    #[inline]
    pub fn north_button(&self) -> ButtonState {
      unsafe {
        match (*self.ptr).Gamepad.wButtons & XINPUT_GAMEPAD_Y {
          1 => ButtonState::DOWN,
          _ => ButtonState::UP,
        }
      }
    }

    #[inline]
    pub fn set_north_button(&self, button_state: ButtonState) {
      self.set_button_bit(XINPUT_GAMEPAD_Y, button_state);
    }
  
    /// The south button of the action button group.
    ///
    /// * Nintendo: B
    /// * Playstation: X
    /// * XBox: A
    #[inline]
    pub fn south_button(&self) -> ButtonState {
      unsafe {
        match (*self.ptr).Gamepad.wButtons & XINPUT_GAMEPAD_A {
          1 => ButtonState::DOWN,
          _ => ButtonState::UP,
        }
      }
    }

    #[inline]
    pub fn set_south_button(&self, button_state: ButtonState) {
      self.set_button_bit(XINPUT_GAMEPAD_A, button_state);
    }
  
    /// The east button of the action button group.
    ///
    /// * Nintendo: A
    /// * Playstation: Circle
    /// * XBox: B
    #[inline]
    pub fn east_button(&self) -> bool {
      unsafe {
        (*self.ptr).Gamepad.wButtons & XINPUT_GAMEPAD_B != 0
      }
    }

    #[inline]
    pub fn set_east_button(&self, button_state: ButtonState) {
      self.set_button_bit(XINPUT_GAMEPAD_B, button_state);
    }
  
    /// The west button of the action button group.
    ///
    /// * Nintendo: Y
    /// * Playstation: Square
    /// * XBox: X
    #[inline]
    pub fn west_button(&self) -> bool {
      unsafe {
        (*self.ptr).Gamepad.wButtons & XINPUT_GAMEPAD_X != 0
      }
    }

    #[inline]
    pub fn set_west_button(&self, button_state: ButtonState) {
      self.set_button_bit(XINPUT_GAMEPAD_X, button_state);
    }
  
    /// The up button on the directional pad.
    #[inline]
    pub fn arrow_up(&self) -> bool {
      unsafe {
        (*self.ptr).Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_UP != 0
      }
    }

    #[inline]
    pub fn set_arrow_up(&self, button_state: ButtonState) {
      self.set_button_bit(XINPUT_GAMEPAD_DPAD_UP, button_state);
    }
  
    /// The down button on the directional pad.
    #[inline]
    pub fn arrow_down(&self) -> bool {
      unsafe {
        (*self.ptr).Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_DOWN != 0
      }
    }

    #[inline]
    pub fn set_arrow_down(&self, button_state: ButtonState) {
      self.set_button_bit(XINPUT_GAMEPAD_DPAD_DOWN, button_state);
    }
  
    /// The left button on the directional pad.
    #[inline]
    pub fn arrow_left(&self) -> bool {
      unsafe {
        (*self.ptr).Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_LEFT != 0
      }
    }

    #[inline]
    pub fn set_arrow_left(&self, button_state: ButtonState) {
      self.set_button_bit(XINPUT_GAMEPAD_DPAD_LEFT, button_state);
    }
  
    /// The right button on the directional pad.
    #[inline]
    pub fn arrow_right(&self) -> bool {
      unsafe {
        (*self.ptr).Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_RIGHT != 0
      }
    }

    #[inline]
    pub fn set_arrow_right(&self, button_state: ButtonState) {
      self.set_button_bit(XINPUT_GAMEPAD_DPAD_RIGHT, button_state);
    }
  
    /// The "start" button.
    ///
    /// * Nintendo: Start (NES / SNES), '+' (Pro Controller)
    /// * Playstation: Start
    /// * XBox: Start
    #[inline]
    pub fn start_button(&self) -> bool {
      unsafe {
        (*self.ptr).Gamepad.wButtons & XINPUT_GAMEPAD_START != 0
      }
    }

    #[inline]
    pub fn set_start_button(&self, button_state: ButtonState) {
      self.set_button_bit(XINPUT_GAMEPAD_START, button_state);
    }
  
  
    /// The "not start" button.
    ///
    /// * Nintendo: Select (NES / NES), '-' (Pro Controller)
    /// * Playstation: Select
    /// * XBox: Back
    #[inline]
    pub fn select_button(&self) -> bool {
      unsafe {
        (*self.ptr).Gamepad.wButtons & XINPUT_GAMEPAD_BACK != 0
      }
    }

    #[inline]
    pub fn set_select_button(&self, button_state: ButtonState) {
      self.set_button_bit(XINPUT_GAMEPAD_BACK, button_state);
    }
  
    /// The upper left shoulder button.
    ///
    /// * Nintendo: L
    /// * Playstation: L1
    /// * XBox: LB
    #[inline]
    pub fn left_shoulder(&self) -> bool {
      unsafe {
        (*self.ptr).Gamepad.wButtons & XINPUT_GAMEPAD_LEFT_SHOULDER != 0
      }
    }

    #[inline]
    pub fn set_left_shoulder(&self, button_state: ButtonState) {
      self.set_button_bit(XINPUT_GAMEPAD_LEFT_SHOULDER, button_state);
    }
  
    /// The upper right shoulder button.
    ///
    /// * Nintendo: R
    /// * Playstation: R1
    /// * XBox: RB
    #[inline]
    pub fn right_shoulder(&self) -> bool {
      unsafe {
        (*self.ptr).Gamepad.wButtons & XINPUT_GAMEPAD_RIGHT_SHOULDER != 0
      }
    }

    #[inline]
    pub fn set_right_shoulder(&self, button_state: ButtonState) {
      self.set_button_bit(XINPUT_GAMEPAD_RIGHT_SHOULDER, button_state);
    }
  
    /// The default threshold to count a trigger as being "pressed".
    pub const TRIGGER_THRESHOLD: u8 = XINPUT_GAMEPAD_TRIGGER_THRESHOLD;
  
    /// The lower left shoulder trigger. If you want to use this as a simple
    /// boolean it is suggested that you compare it to the `TRIGGER_THRESHOLD`
    /// constant.
    ///
    /// * Nintendo: ZL
    /// * Playstation: L2
    /// * XBox: LT
    #[inline]
    pub fn left_trigger(&self) -> u8 {
      unsafe {
        (*self.ptr).Gamepad.bLeftTrigger
      }
    }

    #[inline]
    pub fn set_left_trigger(&self, value: u8) {
      unsafe {
        (*self.ptr).Gamepad.bLeftTrigger = value;
      }
    }
  
    /// The lower right shoulder trigger. If you want to use this as a simple
    /// boolean it is suggested that you compare it to the `TRIGGER_THRESHOLD`
    /// constant.
    ///
    /// * Nintendo: ZR
    /// * Playstation: R2
    /// * XBox: RT
    #[inline]
    pub fn right_trigger(&self) -> u8 {
      unsafe {
        (*self.ptr).Gamepad.bRightTrigger
      }
    }

    #[inline]
    pub fn set_right_trigger(&self, value: u8) {
      unsafe {
        (*self.ptr).Gamepad.bRightTrigger = value;
      }
    }
  
    /// The lower left shoulder trigger as a bool using the default threshold.
    ///
    /// * Nintendo: ZL
    /// * Playstation: L2
    /// * XBox: LT
    #[inline]
    pub fn left_trigger_bool(&self) -> bool {
      self.left_trigger() >= MutableXInputState::TRIGGER_THRESHOLD
    }
  
    /// The lower right shoulder trigger as a bool using the default threshold.
    ///
    /// * Nintendo: ZR
    /// * Playstation: R2
    /// * XBox: RT
    #[inline]
    pub fn right_trigger_bool(&self) -> bool {
      self.right_trigger() >= MutableXInputState::TRIGGER_THRESHOLD
    }
  
    /// The left thumb stick being pressed inward.
    ///
    /// * Nintendo: (L)
    /// * Playstation: L3
    /// * XBox: (L)
    #[inline]
    pub fn left_thumb_button(&self) -> bool {
      unsafe {
        (*self.ptr).Gamepad.wButtons & XINPUT_GAMEPAD_LEFT_THUMB != 0
      }
    }

    #[inline]
    pub fn set_left_thumb_button(&self, button_state: ButtonState) {
      self.set_button_bit(XINPUT_GAMEPAD_LEFT_THUMB, button_state);
    }
  
    /// The right thumb stick being pressed inward.
    ///
    /// * Nintendo: (R)
    /// * Playstation: R3
    /// * XBox: (R)
    #[inline]
    pub fn right_thumb_button(&self) -> bool {
      unsafe {
        (*self.ptr).Gamepad.wButtons & XINPUT_GAMEPAD_RIGHT_THUMB != 0
      }
    }

    #[inline]
    pub fn set_right_thumb_button(&self, button_state: ButtonState) {
      self.set_button_bit(XINPUT_GAMEPAD_RIGHT_THUMB, button_state);
    }
  
    /// The suggested default deadzone for use with the left thumb stick.
    pub const LEFT_STICK_DEADZONE: i16 = XINPUT_GAMEPAD_LEFT_THUMB_DEADZONE;
  
    /// The suggested default deadzone for use with the right thumb stick.
    pub const RIGHT_STICK_DEADZONE: i16 = XINPUT_GAMEPAD_RIGHT_THUMB_DEADZONE;
  
    /// The left stick raw value.
    ///
    /// Positive values are to the right (X-axis) or up (Y-axis).
    #[inline]
    pub fn left_stick_raw(&self) -> (i16, i16) {
      unsafe {
        ((*self.ptr).Gamepad.sThumbLX, (*self.ptr).Gamepad.sThumbLY)
      }
    }

    #[inline]
    pub fn set_left_stick_raw(&self, values: (i16, i16)) {
      unsafe {
        (*self.ptr).Gamepad.sThumbLX = values.0;
        (*self.ptr).Gamepad.sThumbLY = values.1;
      }
    }
  
    /// The right stick raw value.
    ///
    /// Positive values are to the right (X-axis) or up (Y-axis).
    #[inline]
    pub fn right_stick_raw(&self) -> (i16, i16) {
      unsafe {
        ((*self.ptr).Gamepad.sThumbRX, (*self.ptr).Gamepad.sThumbRY)
      }
    }

    #[inline]
    pub fn set_right_stick_raw(&self, values: (i16, i16)) {
      unsafe {
        (*self.ptr).Gamepad.sThumbRX = values.0;
        (*self.ptr).Gamepad.sThumbRY = values.1;
      }
    }
  
    /// The left stick value normalized with the default dead-zone.
    ///
    /// See `normalize_raw_stick_value` for more.
    #[inline]
    pub fn left_stick_normalized(&self) -> (f32, f32) {
      MutableXInputState::normalize_raw_stick_value(self.left_stick_raw(), MutableXInputState::LEFT_STICK_DEADZONE)
    }
  
    /// The right stick value normalized with the default dead-zone.
    ///
    /// See `normalize_raw_stick_value` for more.
    #[inline]
    pub fn right_stick_normalized(&self) -> (f32, f32) {
      MutableXInputState::normalize_raw_stick_value(
        self.right_stick_raw(),
        MutableXInputState::RIGHT_STICK_DEADZONE,
      )
    }
  
    /// This helper normalizes a raw stick value using the given deadzone.
    ///
    /// If the raw value's 2d length is less than the deadzone the result will be
    /// `(0.0,0.0)`, otherwise the result is normalized across the range from the
    /// deadzone point to the maximum value.
    ///
    /// The `deadzone` value is clamped to the range 0 to 32,766 (inclusive)
    /// before use. Negative inputs or maximum value inputs make the normalization
    /// just work improperly.
    #[inline]
    pub fn normalize_raw_stick_value(raw_stick: (i16, i16), deadzone: i16) -> (f32, f32) {
      let deadzone_float = deadzone.max(0).min(i16::max_value() - 1) as f32;
      let raw_float = (raw_stick.0 as f32, raw_stick.1 as f32);
      let length = (raw_float.0 * raw_float.0 + raw_float.1 * raw_float.1).sqrt();
      let normalized = (raw_float.0 / length, raw_float.1 / length);
      if length > deadzone_float {
        // clip our value to the expected maximum length.
        let length = length.min(32_767.0);
        let scale = (length - deadzone_float) / (32_767.0 - deadzone_float);
        (normalized.0 * scale, normalized.1 * scale)
      } else {
        (0.0, 0.0)
      }
    }
  }