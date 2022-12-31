//! Module for handling events such as window resizing,
//! keyboard input, etc.

use glium::glutin::event::{self, ElementState, VirtualKeyCode};
use std::collections::HashSet;

/// Handles events from the glium event loop:
/// * Responds to window events such as resizing by reiszing the window.
/// * Returns the keyboard event if there was one
/// * Updates the HashSet storing the pressed keys
pub fn handle_events<T>(
    ev: event::Event<T>,
    pressed_keys: &mut HashSet<VirtualKeyCode>,
) -> Option<event::KeyboardInput> {
    match ev {
        event::Event::WindowEvent { event, .. } => match event {
            event::WindowEvent::CloseRequested => {
                std::process::exit(0);
            }
            event::WindowEvent::KeyboardInput { input, .. } => {
                if input.virtual_keycode == None {
                    panic!("Keyboard input {:?} did not have a valid key code", input);
                }
                update_pressed_keys(input, pressed_keys);
                Some(input)
            }
            _ => None,
        },
        _ => None,
    }
}

fn update_pressed_keys(input: event::KeyboardInput, pressed_keys: &mut HashSet<VirtualKeyCode>) {
    let keycode = input
        .virtual_keycode
        .unwrap_or_else(|| panic!("Keyboard input {:?} did not have a valid keycode", input));

    // key pressed and wasn't pressed before
    if input.state == ElementState::Pressed && !pressed_keys.contains(&keycode) {
        pressed_keys.insert(keycode);
    }
    // key was pressed before and was released
    if input.state == ElementState::Released && pressed_keys.contains(&keycode) {
        pressed_keys.remove(&keycode);
    }
}
