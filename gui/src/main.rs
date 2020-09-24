use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::{timer, Context, ContextBuilder, GameResult};
use hansing_chess::board::Board;
use hansing_chess::square::Square;

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
        Err(e) => println!("Error occurred: {}", e),
    }
}

struct MyGame {
    dt: std::time::Duration,
    sprites: Vec<graphics::Image>,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        // Load/create resources here: images, fonts, sounds, etc.
        let mut v: Vec<graphics::Image> = Vec::new();
        let image_dir = std::path::Path::new("/images");
        v.push(graphics::Image::new(ctx, image_dir.join("white_pawn.png")).unwrap());
        v.push(graphics::Image::new(ctx, image_dir.join("white_rook.png")).unwrap());
        v.push(graphics::Image::new(ctx, image_dir.join("white_knight.png")).unwrap());
        v.push(graphics::Image::new(ctx, image_dir.join("white_bishop.png")).unwrap());
        v.push(graphics::Image::new(ctx, image_dir.join("white_queen.png")).unwrap());
        v.push(graphics::Image::new(ctx, image_dir.join("white_king.png")).unwrap());

        v.push(graphics::Image::new(ctx, image_dir.join("black_pawn.png")).unwrap());
        v.push(graphics::Image::new(ctx, image_dir.join("black_rook.png")).unwrap());
        v.push(graphics::Image::new(ctx, image_dir.join("black_knight.png")).unwrap());
        v.push(graphics::Image::new(ctx, image_dir.join("black_bishop.png")).unwrap());
        v.push(graphics::Image::new(ctx, image_dir.join("black_queen.png")).unwrap());
        v.push(graphics::Image::new(ctx, image_dir.join("black_king.png")).unwrap());

        MyGame {
            dt: std::time::Duration::new(0, 0),
            sprites: v,
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

    pub fn draw_pieces(&mut self, ctx: &mut Context, board: Board) -> GameResult<()> {
        for rank in (0..8).rev() {
            for file in 0..8 {
                if let Some(s) = Square::new(rank, file) {
                    if board.pieces.contains_key(&s) {
                        print!("{} ", board.pieces[&s].to_char())
                    } else {
                        if (rank + file) % 2 == 0 {
                            print!(", ");
                        } else {
                            print!(". ");
                        }
                    }
                }
            }
            println!();
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
