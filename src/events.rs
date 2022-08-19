//! Module for handling events such as window resizing,
//! keyboard input, etc.

use glium::glutin::{event, event_loop};

/// Handles events from the glium event loop.
/// # Examples
/// ```rust
///        fn handle_keyboard_input(glium::glutin::KeyboardInput) {
///            /* --snip -- /*
///        }
///        event_loop.run(move |ev, _, control_flow| {
///            handle_events(ev, control_flow, handle_keyboard_input);
///            /* --snip-- /*
///    });
/// ```
pub fn handle_events<T>(
    ev: event::Event<T>,
    _control_flow: &mut event_loop::ControlFlow,
    handle_keyboard_input: fn(event::KeyboardInput) -> (),
) {
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
