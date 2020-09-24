use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::{timer, Context, ContextBuilder, GameResult};

fn main() {
    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) = ContextBuilder::new("game_name", "author_name")
        .build()
        .unwrap();

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let mut my_game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct MyGame {
    dt: std::time::Duration,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources here: images, fonts, sounds, etc.
        MyGame {
            dt: std::time::Duration::new(0, 0),
        }
    }

    pub fn draw_board(&mut self, ctx: &mut Context) -> GameResult<()> {
        for i in 0..8 {
            for j in 0..8 {
                if (j + i) % 2 == 0 {
                    let tile = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(
                            160.0 + (i as f32 * 60.0),
                            60.0 + (j as f32 * 60.0),
                            60.0,
                            60.0,
                        ),
                        graphics::WHITE,
                    );
                    graphics::draw(ctx, &tile.unwrap(), graphics::DrawParam::default())?;
                } else {
                    let tile = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(
                            160.0 + (i as f32 * 60.0),
                            60.0 + (j as f32 * 60.0),
                            60.0,
                            60.0,
                        ),
                        graphics::BLACK,
                    );
                    graphics::draw(ctx, &tile.unwrap(), graphics::DrawParam::default())?;
                }
            }
        }

        Ok(())
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);
        // FPS limit to 144fps
        if std::time::Duration::from_nanos(6944444) > self.dt {
            std::thread::sleep(std::time::Duration::from_nanos(6944444) - self.dt);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::new(0.2, 0.2, 0.2, 1.0));
        self.draw_board(ctx);
        graphics::present(ctx)?;
        Ok(())
    }
}
