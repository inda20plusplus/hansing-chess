use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::input::mouse;
use ggez::{timer, Context, ContextBuilder, GameResult};
use hansing_chess::color::Color;
use hansing_chess::game::Game;
use hansing_chess::movegen;
use hansing_chess::piece::Piece;
use hansing_chess::square::Square;
use hansing_chess::title::Title;

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
    game: Game,
    piece_holding: [i32; 2],
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
            game: Game::new(),
            piece_holding: [-1, -1],
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
                        graphics::Color::from_rgb(140, 140, 140),
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
                        graphics::Color::from_rgb(62, 62, 62),
                    );
                    graphics::draw(ctx, &tile.unwrap(), graphics::DrawParam::default())?;
                }
            }
        }

        Ok(())
    }

    pub fn draw_pieces(&mut self, ctx: &mut Context) -> GameResult<()> {
        for i in 0..8 {
            for j in 0..8 {
                if let Some(s) = Square::new(i, j) {
                    if self.game.board.pieces.contains_key(&s) {
                        let a = graphics::DrawParam::new()
                            .dest([160.0 + (j as f32 * 60.0), 60.0 + (i as f32 * 60.0)])
                            .scale([0.05859375, 0.05859375]);
                        if self.game.board.pieces[&s].color == Color::White {
                            if self.game.board.pieces[&s].title == Title::Pawn {
                                graphics::draw(ctx, &self.sprites[0], a);
                            } else if self.game.board.pieces[&s].title == Title::Rook {
                                graphics::draw(ctx, &self.sprites[1], a);
                            } else if self.game.board.pieces[&s].title == Title::Knight {
                                graphics::draw(ctx, &self.sprites[2], a);
                            } else if self.game.board.pieces[&s].title == Title::Bishop {
                                graphics::draw(ctx, &self.sprites[3], a);
                            } else if self.game.board.pieces[&s].title == Title::Queen {
                                graphics::draw(ctx, &self.sprites[4], a);
                            } else if self.game.board.pieces[&s].title == Title::King {
                                graphics::draw(ctx, &self.sprites[5], a);
                            }
                        } else {
                            if self.game.board.pieces[&s].title == Title::Pawn {
                                graphics::draw(ctx, &self.sprites[6], a);
                            } else if self.game.board.pieces[&s].title == Title::Rook {
                                graphics::draw(ctx, &self.sprites[7], a);
                            } else if self.game.board.pieces[&s].title == Title::Knight {
                                graphics::draw(ctx, &self.sprites[8], a);
                            } else if self.game.board.pieces[&s].title == Title::Bishop {
                                graphics::draw(ctx, &self.sprites[9], a);
                            } else if self.game.board.pieces[&s].title == Title::Queen {
                                graphics::draw(ctx, &self.sprites[10], a);
                            } else if self.game.board.pieces[&s].title == Title::King {
                                graphics::draw(ctx, &self.sprites[11], a);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::delta(ctx);

        let coord = mouse::position(ctx);
        let coord_x: i32 = ((coord.x - 160.0) / 60.0) as i32;
        let coord_y: i32 = ((coord.y - 60.0) / 60.0) as i32;

        if mouse::button_pressed(ctx, mouse::MouseButton::Left) && self.piece_holding == [-1, -1] {
            if coord_x >= 0 && coord_x <= 7 && coord_y >= 0 && coord_y <= 7 {
                self.piece_holding = [coord_x, coord_y];
            }
        } else if !mouse::button_pressed(ctx, mouse::MouseButton::Left)
            && self.piece_holding != [-1, -1]
        {
            if coord_x >= 0 && coord_x <= 7 && coord_y >= 0 && coord_y <= 7 {
                let all_moves = movegen::generate_action_space(self.game.board.clone());
                for curr_move in all_moves.iter() {
                    if curr_move.from
                        == Square::new(self.piece_holding[0], self.piece_holding[1]).unwrap()
                        && curr_move.to == Square::new(coord_x, coord_y).unwrap()
                    {
                        self.game.make_move(*curr_move);
                    }
                }
                self.piece_holding = [coord_x, coord_y];
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::from_rgb(15, 15, 20));
        self.draw_board(ctx);
        self.draw_pieces(ctx);
        graphics::present(ctx)?;
        Ok(())
    }
}
