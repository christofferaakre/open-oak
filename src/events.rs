//! Module for handling events such as window resizing,
//! keyboard input, etc.

use glium::glutin::{event, event_loop};

fn handle_keyboard_input(input: event::KeyboardInput) {
    match input.virtual_keycode {
        Some(keycode) => match keycode {
            event::VirtualKeyCode::Escape => {
                std::process::exit(0);
            }
            _ => {}
        },
        None => {}
    }
}

/// Handles events from the glium event loop.
/// # Examples
/// ```rust
///        event_loop.run(move |ev, _, control_flow| {
///            handle_events(ev, control_flow);
///            /* --snip-- /*
///    });
/// ```
pub fn handle_events<T>(ev: event::Event<T>, _control_flow: &mut event_loop::ControlFlow) {
    match ev {
        event::Event::WindowEvent { event, .. } => match event {
            event::WindowEvent::CloseRequested => {
                std::process::exit(0);
            }
            event::WindowEvent::KeyboardInput { input, .. } => {
                handle_keyboard_input(input);
            }
            _ => {}
        },
        _ => {}
    };
}
