use crate::board::Board;
use crate::color::Color;
use crate::movedata::MoveData;
use crate::movegen::*;
use crate::square::Square;
use crate::title::Title;
pub struct Game {
    pub board: Board,
    pub action_space: Vec<MoveData>,
    pub history: Vec<Board>,
    pub result: GameResult,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            board: Board::new_standard(),
            action_space: Vec::new(),
            history: Vec::new(),
            result: GameResult::Ongoing,
        };
        game.action_space = generate_action_space(game.board.clone());
        game
    }
    pub fn set_deafault_promotion(&mut self, title: Title) {
        unsafe {
            //oof
            PROMOTE_TO = title;
        }
    }

    pub fn make_move(&mut self, m: MoveData) {
        if self.action_space.contains(&m) {
            self.board.make_move(m);
            self.action_space = generate_action_space(self.board.clone());
            self.check_for_game_over();
        }
    }

    pub fn make_move_from_notation(&mut self, note: String) {
        for m in self.action_space.clone().iter() {
            if m.get_move_notation() == note {
                self.make_move(*m);
            }
        }
    }

    pub fn check_for_game_over(&mut self) {
        if self.action_space.len() == 0 {
            if self.board.in_check {
                self.result = GameResult::Won(self.board.to_act.inverse());
                println!("{} has won!", self.board.to_act.inverse())
            } else {
                self.result = GameResult::Tied;
                println!("The game has ended in a tie.")
            }
        }
    }
}

pub enum GameResult {
    Won(Color),
    Tied,
    Ongoing,
}

pub fn print_board_state(board: &Board) {
    println!();
    println!("CHESS! {} to play.", board.to_act);
    println!("  _a_b_c_d_e_f_g_h_");

    for rank in (0..8).rev() {
        print!("{}| ", rank + 1);
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
    println!("  -a-b-c-d-e-f-g-h-")
}
