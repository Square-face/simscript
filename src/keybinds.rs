use bevy::input::{keyboard::KeyCode, mouse::MouseButton, ButtonInput};

#[derive(Debug, Default)]
pub struct Keybind(pub Option<KeybindOptions>);

#[derive(Debug)]
#[allow(dead_code)]
pub enum KeybindOptions {
    Keyboard(KeyCode),
    MouseButton(MouseButton),
}

impl KeybindOptions {
    /// Checks if the Keybind is pressed
    pub fn pressed(&self, kbd: &ButtonInput<KeyCode>, mos: &ButtonInput<MouseButton>) -> bool {
        match self {
            KeybindOptions::Keyboard(exp) => kbd.pressed(*exp),
            KeybindOptions::MouseButton(exp) => mos.pressed(*exp),
        }
    }

    /// Checks if the Keybind is just pressed
    pub fn just_pressed(&self, kbd: &ButtonInput<KeyCode>, mos: &ButtonInput<MouseButton>) -> bool {
        match self {
            KeybindOptions::Keyboard(exp) => kbd.just_pressed(*exp),
            KeybindOptions::MouseButton(exp) => mos.just_pressed(*exp),
        }
    }

    /// Checks if the Keybind is just pressed
    pub fn just_released(
        &self,
        kbd: &ButtonInput<KeyCode>,
        mos: &ButtonInput<MouseButton>,
    ) -> bool {
        match self {
            KeybindOptions::Keyboard(exp) => kbd.just_released(*exp),
            KeybindOptions::MouseButton(exp) => mos.just_released(*exp),
        }
    }
}

impl Keybind {
    /// Checks if the Keybind is pressed or not
    ///
    /// If the keybind isn't set, defaults to false
    pub fn pressed(
        &self,
        keyboard: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
    ) -> bool {
        self.check(keyboard, mouse, KeybindOptions::pressed)
    }

    /// Checks if the Keybind was just pressed
    ///
    /// If the keybind isn't set, defaults to false
    pub fn just_pressed(
        &self,
        keyboard: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
    ) -> bool {
        self.check(keyboard, mouse, KeybindOptions::just_pressed)
    }

    /// Checks if the Keybind was just released
    ///
    /// If the keybind isn't set, defaults to false
    pub fn just_released(
        &self,
        keyboard: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
    ) -> bool {
        self.check(keyboard, mouse, KeybindOptions::just_released)
    }

    fn check(
        &self,
        keyboard: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
        fun: fn(&KeybindOptions, &ButtonInput<KeyCode>, &ButtonInput<MouseButton>) -> bool,
    ) -> bool {
        match &self.0 {
            None => false,
            Some(bind) => fun(bind, keyboard, mouse),
        }
    }
}
