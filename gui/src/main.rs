mod network;

use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::input::mouse;
use ggez::{timer, Context, ContextBuilder, GameResult};
use hansing_chess::color::Color;
use hansing_chess::game::Game;
use hansing_chess::movegen;
use hansing_chess::square::Square;
use hansing_chess::title::Title;

use std::env;
use std::path::PathBuf;
use std::{thread, time};

const IP_ADDRESS: (u8, u8, u8, u8) = (192, 168, 157, 102);
const PORT: u16 = 17571;
const CHECK_TIMER: std::time::Duration = time::Duration::from_millis(100);

fn main() {
    // Make path for images
    let path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("images");
        //path.push("assets");
        path
    } else {
        PathBuf::from("./images")
    };
    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) = ContextBuilder::new("game_name", "author_name")
        .add_resource_path(path)
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

#[derive(PartialEq)]
enum Hosting {
    Host,
    Client,
    Offline,
    None,
}

struct MyGame {
    //dt: std::time::Duration, //dead code
    sprites: Vec<graphics::Image>,
    game: Game,
    piece_holding: [i32; 2],
    status: Hosting,
    networker: Option<network::Networker>,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        // Load/create resources here: images, fonts, sounds, etc.
        let mut v: Vec<graphics::Image> = Vec::new();
        v.push(graphics::Image::new(ctx, "/white_pawn.png").unwrap());
        v.push(graphics::Image::new(ctx, "/white_rook.png").unwrap());
        v.push(graphics::Image::new(ctx, "/white_knight.png").unwrap());
        v.push(graphics::Image::new(ctx, "/white_bishop.png").unwrap());
        v.push(graphics::Image::new(ctx, "/white_queen.png").unwrap());
        v.push(graphics::Image::new(ctx, "/white_king.png").unwrap());

        v.push(graphics::Image::new(ctx, "/black_pawn.png").unwrap());
        v.push(graphics::Image::new(ctx, "/black_rook.png").unwrap());
        v.push(graphics::Image::new(ctx, "/black_knight.png").unwrap());
        v.push(graphics::Image::new(ctx, "/black_bishop.png").unwrap());
        v.push(graphics::Image::new(ctx, "/black_queen.png").unwrap());
        v.push(graphics::Image::new(ctx, "/black_king.png").unwrap());

        MyGame {
            //dt: std::time::Duration::new(0, 0),//dead code
            sprites: v,
            game: Game::new(),
            piece_holding: [-1, -1],
            status: Hosting::None,
            networker: None,
        }
    }

    pub fn draw_board(&mut self, ctx: &mut Context) -> GameResult<()> {
        let tile_white = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 60.0, 60.0),
            graphics::Color::from_rgb(140, 140, 140),
        )
        .unwrap();

        let tile_black = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 60.0, 60.0),
            graphics::Color::from_rgb(62, 62, 62),
        )
        .unwrap();

        let dst = graphics::DrawParam::default();
        for i in 0..8 {
            for j in 0..8 {
                if (j + i) % 2 == 0 {
                    graphics::draw(
                        ctx,
                        &tile_white,
                        dst.dest([160.0 + (j as f32 * 60.0), 60.0 + (i as f32 * 60.0)]),
                    )?;
                } else {
                    graphics::draw(
                        ctx,
                        &tile_black,
                        dst.dest([160.0 + (j as f32 * 60.0), 60.0 + (i as f32 * 60.0)]),
                    )?;
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
                            .scale([0.05859375 * 17.0, 0.05859375 * 17.0]); // Tweek these values if pieces appear to large or to small
                        if self.game.board.pieces[&s].color == Color::White {
                            if self.game.board.pieces[&s].title == Title::Pawn {
                                graphics::draw(ctx, &self.sprites[0], a).ok();
                            } else if self.game.board.pieces[&s].title == Title::Rook {
                                graphics::draw(ctx, &self.sprites[1], a).ok();
                            } else if self.game.board.pieces[&s].title == Title::Knight {
                                graphics::draw(ctx, &self.sprites[2], a).ok();
                            } else if self.game.board.pieces[&s].title == Title::Bishop {
                                graphics::draw(ctx, &self.sprites[3], a).ok();
                            } else if self.game.board.pieces[&s].title == Title::Queen {
                                graphics::draw(ctx, &self.sprites[4], a).ok();
                            } else if self.game.board.pieces[&s].title == Title::King {
                                graphics::draw(ctx, &self.sprites[5], a).ok();
                            }
                        } else {
                            if self.game.board.pieces[&s].title == Title::Pawn {
                                graphics::draw(ctx, &self.sprites[6], a).ok();
                            } else if self.game.board.pieces[&s].title == Title::Rook {
                                graphics::draw(ctx, &self.sprites[7], a).ok();
                            } else if self.game.board.pieces[&s].title == Title::Knight {
                                graphics::draw(ctx, &self.sprites[8], a).ok();
                            } else if self.game.board.pieces[&s].title == Title::Bishop {
                                graphics::draw(ctx, &self.sprites[9], a).ok();
                            } else if self.game.board.pieces[&s].title == Title::Queen {
                                graphics::draw(ctx, &self.sprites[10], a).ok();
                            } else if self.game.board.pieces[&s].title == Title::King {
                                graphics::draw(ctx, &self.sprites[11], a).ok();
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn decide_online(&mut self, ctx: &mut Context) {
        graphics::clear(ctx, graphics::Color::from_rgb(15, 15, 20));

        let host_x = 160.0;
        let host_y = 0.0;
        let host_size = 60.0;
        let online_x = 160.0;
        let online_y = 60.0;
        let online_size = 60.0;
        let offline_x = 160.0;
        let offline_y = 120.0;
        let offline_size = 60.0;

        let host_button = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 60.0, 60.0),
            graphics::Color::from_rgb(0, 200, 140),
        )
        .unwrap();

        let online_button = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 60.0, 60.0),
            graphics::Color::from_rgb(140, 60, 140),
        )
        .unwrap();

        let offline_button = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0, 0.0, 60.0, 60.0),
            graphics::Color::from_rgb(200, 200, 140),
        )
        .unwrap();

        let dst = graphics::DrawParam::default();

        graphics::draw(ctx, &host_button, dst.dest([host_x, host_y]));
        graphics::draw(ctx, &online_button, dst.dest([online_x, online_y]));
        graphics::draw(ctx, &offline_button, dst.dest([offline_x, offline_y]));

        graphics::present(ctx);

        if mouse::button_pressed(ctx, mouse::MouseButton::Left) {
            let coord = mouse::position(ctx);
            if coord.x > online_x
                && coord.x < online_x + online_size
                && coord.y > online_y
                && coord.y < online_y + online_size
            {
                self.status = Hosting::Client;
                self.networker = Some(network::Networker::connect(IP_ADDRESS, PORT));
            } else if coord.x > offline_x
                && coord.x < offline_x + offline_size
                && coord.y > offline_y
                && coord.y < offline_y + offline_size
            {
                self.status = Hosting::Offline;
            } else if coord.x > host_x
                && coord.x < host_x + host_size
                && coord.y > host_y
                && coord.y < host_y + host_size
            {
                self.status = Hosting::Host;
                self.networker = Some(network::Networker::new(PORT));
                let mut wait = true;
                while wait {
                    if self.networker.as_mut().unwrap().check_connected() {
                        wait = false;
                    } else {
                        println!("Waiting for connection");
                        thread::sleep(CHECK_TIMER);
                    }
                }
            }
            return ();
        }
        self.status = Hosting::None;
    }

    fn local_move(&mut self, ctx: &mut Context) -> (i32, i32, i32, i32) {
        let coord = mouse::position(ctx);
        let coord_x: i32 = ((coord.x - 160.0) / 60.0) as i32;
        let coord_y: i32 = ((coord.y - 60.0) / 60.0) as i32;
        let mut ret = (i32::MAX, i32::MAX, i32::MAX, i32::MAX);

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
                        == Square::new(self.piece_holding[1], self.piece_holding[0]).unwrap()
                        && curr_move.to == Square::new(coord_y, coord_x).unwrap()
                    {
                        self.game.make_move(*curr_move);
                        ret.0 = self.piece_holding[0];
                        ret.1 = self.piece_holding[1];
                        ret.2 = coord_x;
                        ret.3 = coord_y;
                        self.piece_holding = [-1, -1];
                        break;
                    }
                }
            }
            self.piece_holding = [-1, -1];
        } else if !mouse::button_pressed(ctx, mouse::MouseButton::Left)
            && self.piece_holding != [-1, -1]
        {
            self.piece_holding = [-1, -1];
        }

        ret
    }

    fn online_move(&mut self, ctx: &mut Context, clr: Color) {
        if self.game.board.to_act == clr {
            let coords = self.local_move(ctx);
            if coords == (i32::MAX, i32::MAX, i32::MAX, i32::MAX) {
                return ();
            }
            let from_x = coords.0 as u64;
            let from_y = coords.1 as u64;
            let to_x = coords.2 as u64;
            let to_y = coords.3 as u64;
            let mut origin: u8 = 0;
            origin += from_x as u8;
            origin += (from_y as u8) << 3;
            let mut target: u8 = 0;
            target += to_x as u8;
            target += (from_y as u8) << 3;
            self.networker.as_mut().unwrap().write(&[1, origin, target]);
        } else {
            let mut data: u8 = u8::MAX;
            match self.networker.as_mut().unwrap().read() {
                Ok(byte) => data = byte,
                Err(e) => return (), //Fix so it waits properly for data
            }
            if data == 0 {
                panic!("Disagreement");
            } else if data == 1 {
                let mut move_type: u8 = u8::MAX;
                match self.networker.as_mut().unwrap().read() {
                    Ok(byte) => move_type = byte,
                    Err(e) => panic!(e),
                }

                if move_type == 0 || move_type == 1 || move_type == 2 {
                    let mut from: u8 = u8::MAX;
                    match self.networker.as_mut().unwrap().read() {
                        Ok(byte) => from = byte,
                        Err(e) => panic!(e),
                    }
                    let mut from_x: u8 = 0;
                    from_x += from & (1 << 0);
                    from_x += from & (1 << 1);
                    from_x += from & (1 << 2);
                    let mut from_y: u8 = 0;
                    from_y += from & (1 << 3);
                    from_y += from & (1 << 4);
                    from_y += from & (1 << 5);
                    from_y /= 8;
                    let mut to: u8 = u8::MAX;
                    match self.networker.as_mut().unwrap().read() {
                        Ok(byte) => to = byte,
                        Err(e) => panic!(e),
                    }
                    let mut to_x: u8 = 0;
                    to_x += to & (1 << 0);
                    to_x += to & (1 << 1);
                    to_x += to & (1 << 2);
                    let mut to_y: u8 = 0;
                    to_y += to & (1 << 3);
                    to_y += to & (1 << 4);
                    to_y += to & (1 << 5);
                    to_y /= 8;
                    let all_moves = movegen::generate_action_space(self.game.board.clone());
                    for curr_move in all_moves.iter() {
                        if curr_move.from == Square::new(from_y as i32, from_x as i32).unwrap()
                            && curr_move.to == Square::new(to_y as i32, to_x as i32).unwrap()
                        {
                            self.game.make_move(*curr_move);
                            break;
                        }
                    }
                    if move_type == 2 {
                        let mut promotion_piece: u8 = u8::MAX;
                        match self.networker.as_mut().unwrap().read() {
                            Ok(byte) => promotion_piece = byte,
                            Err(e) => panic!(e),
                        }
                    }

                    if self.game.action_space.len() == 0 {
                        if self.game.board.in_check {
                            self.networker.as_mut().unwrap().write(&[4]);
                        } else {
                            self.networker.as_mut().unwrap().write(&[5]);
                        }
                    }
                } else if move_type == 3 {
                    let all_moves = movegen::generate_action_space(self.game.board.clone());
                    for curr_move in all_moves.iter() {
                        if curr_move.from == Square::new(7, 4).unwrap()
                            && curr_move.to == Square::new(7, 6).unwrap()
                        {
                            self.game.make_move(*curr_move);
                            break;
                        }
                    }
                } else if move_type == 4 {
                    let all_moves = movegen::generate_action_space(self.game.board.clone());
                    for curr_move in all_moves.iter() {
                        if curr_move.from == Square::new(7, 4).unwrap()
                            && curr_move.to == Square::new(7, 2).unwrap()
                        {
                            self.game.make_move(*curr_move);
                            break;
                        }
                    }
                } else {
                    panic!("Unknown move");
                }
            } else if data == 2 {
                //TODO implement undo
                self.networker.as_mut().unwrap().write(&[0]);
            } else if data == 3 {
                //TODO implement Accept (draw and undo)
            } else if data == 4 {
                if self.game.action_space.len() == 0 {
                    if self.game.board.in_check {
                        //Checkmate
                    } else {
                        self.networker.as_mut().unwrap().write(&[0]);
                    }
                }
            } else if data == 5 {
                if self.game.action_space.len() == 0 {
                    if self.game.board.in_check {
                        self.networker.as_mut().unwrap().write(&[0]);
                    } else {
                        // The game is drawn
                    }
                } else {
                    // TODO implement draw request
                    self.networker.as_mut().unwrap().write(&[0]);
                }
            } else if data == 6 {
                // Game over
            }
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.status == Hosting::None {
            self.decide_online(ctx);
        } else if self.status == Hosting::Offline {
            self.local_move(ctx);
        } else if self.status == Hosting::Host {
            self.online_move(ctx, Color::White);
        } else if self.status == Hosting::Client {
            self.online_move(ctx, Color::Black);
        }
        timer::yield_now();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.status != Hosting::None {
            graphics::clear(ctx, graphics::Color::from_rgb(15, 15, 20));
            self.draw_board(ctx).ok();
            self.draw_pieces(ctx).ok();
            graphics::present(ctx)?;
        }
        Ok(())
    }
}
