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

pub fn handle_events<T>(ev: event::Event<T>, control_flow: &mut event_loop::ControlFlow) {
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
