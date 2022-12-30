# open-oak
`open-oak` is a 2D game engine based on OpenGL written in Rust. It uses
[glium](https://docs.rs/glium/latest/glium/#macros) for openGL bindings.
This project is in very early stages.

# Features
* Rendering textured objects on the screen
* Handling window, mouse, and keyboard events
* Collission detection

## Example
Below is a simple example that renders a textured block to the screen.
 ```rust

 use glium::Surface;

 use open_oak::events::handle_events;
 use open_oak::init::{init, Game};
 use open_oak::rectangle::Rectangle;
 use open_oak::resource_manager::ResourceManager;
 use open_oak::traits::{Renderable, Shaders, Texture};

 use cgmath::Vector2;

 struct Block {
     rect: Rectangle,
 }

 impl Block {
     fn new(position: Vector2<f32>, size: Vector2<f32>, color: imaopen_oak::Rgba<f32>) -> Block {
         let rect = Rectangle::new(position, size, color);
         Block { rect }
     }
 }

 fn main() {
     // init game and destructure
     let game = init();

     // destructure fields off Game
     let Game {
         display,
         event_loop,
         mut resource_manager,
         ..
     } = game;

     // define block
     let mut block = Block::new(
         Vector2::new(0.5, 0.5),
         Vector2::new(0.3, 0.3),
         imaopen_oak::Rgba::from([1.0, 0.0, 0.0, 1.0]),
     );

     // init rectangle
     Rectangle::init(&mut resource_manager, &display);

     // load block texture
     let texture_name = String::from("block");
     let texture = ResourceManager::load_texture(&display, "textures/block.png");
     resource_manager.add_texture(&texture_name, texture);

     // set block texture
     block.rect.set_texture(texture_name.clone());

     // game loop
     event_loop.run(move |ev, _, control_flow| {
         // handle events, keyboard input, etc.
         handle_events(ev, control_flow);

         let mut frame = display.draw();
         frame.clear_color(0.2, 0.3, 0.3, 1.0);

         // DRAW START
         block.rect.draw(&mut frame, &resource_manager).unwrap();

         frame.finish().unwrap();
         // DRAW END
     });
 }
 ```

## Development
I am very happy for any contributions!

To contribute, fork the repository, then edit the source code.
You can create a binary crate and put some example code in
there, so you can verify that the game engine is still working, and that your new
feature/bug fix/whatever is working as intended.
In the `Cargo.toml` of your binary crate, you can link to your local version of `ge`:
```toml
# --snip --
[dependencies]
`ge = { path = "/path/to/ge" }`
# --snip --
```
When you are done making changes, submit a pull request!

