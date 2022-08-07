use crate::logic::pieces::{PieceType, MoveError};
use crate::{
    FORMAT_OUTPUT_TURN,
    FORMAT_OUTPUT_ERROR_MOVE_FROM,
    FORMAT_OUTPUT_ERROR_MOVE_FULL,
    FORMAT_OUTPUT_TURN_SHORT,
    FORMAT_OUTPUT_CHECKMATE, FORMAT_PROMPT_MOVE, FORMAT_OUTPUT_CIRITCAL_ERROR,
};
use crate::draw::text::OUTPUT_ENTER_MOVE;
use crate::draw::prompt::Prompt;
use crate::draw::terminal::Terminal;
use crate::logic::game::{Game, GameState};
use crate::logic::basic::{
    FieldColor,
    Player,
    Coordinate,
    row_to_name,
    column_to_name,
};
use crate::logic::board::{TileContent, BOARD_SIZE, BOARD_MAX_AXIS};
use crate::logic::intent::{Intent, PartialCoordinate};

use termion::color;
use termion::event::Key;
use std::io::Write;

use super::text::{OUTPUT_HINT_PROMOTE, OUTPUT_ILLEGAL_MOVE, OUTPUT_MOVE_ERROR_CHECK, OUTPUT_STATE_CHECK, OUTPUT_INVALID_COMMAND};


#[derive(Copy, Clone)]
enum BoardHighlight {
    None,
    Primary,
    Secondary,
    Error,
}


impl BoardHighlight {
    fn background_color(&self) -> String {
        match self {
            Self::Primary => color::Bg(color::Green).to_string(),
            Self::Secondary => color::Bg(color::Blue).to_string(),
            Self::Error => color::Bg(color::Red).to_string(),
            Self::None => "".to_string(),
        }
    }

    fn foreground_color(&self) -> String {
        match self {
            Self::Primary => color::Fg(color::Green).to_string(),
            Self::Secondary => color::Fg(color::Blue).to_string(),
            Self::Error => color::Fg(color::Red).to_string(),
            Self::None => "".to_string(),
        }
    }
}


pub struct GameRenderer<'a> {
    game: &'a mut Game,
    terminal: Terminal,
    prompt: Prompt,
    field_size: usize,
    horizontal_scale: usize,
    highlighted_cells: [[BoardHighlight; BOARD_SIZE]; BOARD_SIZE],
    output_text: String,
}


impl<'a> GameRenderer<'a> {
    pub fn new(game: &'a mut Game) -> Self {
        Self {
            game,
            terminal: Terminal::default(),
            prompt: Prompt::default(),
            field_size: 4,
            horizontal_scale: 2,
            highlighted_cells: [[BoardHighlight::None; BOARD_SIZE]; BOARD_SIZE],
            output_text: "".to_string(),
        }
    }

    pub fn run(&mut self) {
        loop {
            self.draw_board();

            if let Some(key) = self.terminal.read_key() {
                match key {
                    Key::Char('\n') => self.on_prompt_enter(),
                    k => self.prompt.consume_key(&k),
                }
            }
        }
    }

    pub fn evaluate_intent(&mut self, intent: &Intent) {
        if let Intent::Move(Some(from), maybe_to) = intent {
            self.highlight_move(from, maybe_to);
        }
    }

    fn highlight_move(&mut self, from: &PartialCoordinate, maybe_to: &Option<PartialCoordinate>) {
        if let Some(coord_from) = from.to_complete() {
            if self.game.can_move_from(&coord_from) {
                self.highlighted_cells[coord_from.yv()][coord_from.xv()] = BoardHighlight::Primary;

                if let Some(to) = maybe_to {
                    if let Some(coord_to) = to.to_complete() {
                        if self.game.can_move(&coord_from, &coord_to) {
                            self.highlighted_cells[coord_to.yv()][coord_to.xv()] = BoardHighlight::Secondary;
                            self.set_output_text(OUTPUT_ENTER_MOVE.to_string());
                        } else {
                            self.highlighted_cells[coord_to.yv()][coord_to.xv()] = BoardHighlight::Error;
                            self.set_output_text(FORMAT_OUTPUT_ERROR_MOVE_FULL!(
                                coord_from.to_field_name(),
                                coord_to.to_field_name()
                            ));
                        }
                    }
                }
            } else {
                self.set_output_text(FORMAT_OUTPUT_ERROR_MOVE_FROM!(coord_from.to_field_name()));
                self.highlighted_cells[coord_from.yv()][coord_from.xv()] = BoardHighlight::Error;
            }
        }
    }

    fn clear_highlight(&mut self) {
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                self.highlighted_cells[y][x] = BoardHighlight::None;
            }
        }
    }

    fn draw_prompt(&mut self, offset_x: usize, offset_y: usize, line: &String, intent: &Intent) {
        let formatted_line = self.format_prompt(line, intent);
        let turn = self.game.board.turn.to_label();

        self.terminal.move_cursor(offset_x, offset_y);
        write!(self.terminal.screen, "{}> {}", turn, formatted_line).unwrap();
    }

    fn format_prompt(&mut self, line: &String, intent: &Intent) -> String {
        match intent {
            Intent::Invalid => format!(
                "{}{}{}",
                color::Fg(color::Red),
                line,
                color::Fg(color::Reset),
            ),
            Intent::Move(Some(a), maybe_b) => {
                if let Some(coord_a) = a.to_complete() {
                    let highlight = if self.game.can_move_from(&coord_a) {
                        BoardHighlight::Primary
                    } else {
                        BoardHighlight::Error
                    };

                    let field_name_a = coord_a.to_field_name();
                    let highlighted_a = format!(
                        "{}{}{}",
                        highlight.foreground_color(),
                        field_name_a,
                        color::Fg(color::Reset),
                    );

                    let remaining = &line[field_name_a.len()..];

                    let highlighted_b = if let Some(b) = maybe_b {
                        if let Some(coord_b) = b.to_complete() {
                            let highlight = if self.game.can_move(&coord_a, &coord_b) {
                                BoardHighlight::Secondary
                            } else {
                                BoardHighlight::Error
                            };

                            format!(
                                "{}{}{}",
                                highlight.foreground_color(),
                                coord_b.to_field_name(),
                                color::Fg(color::Reset),
                            )
                        } else {
                            remaining.to_string()
                        }
                    } else {
                        remaining.to_string()
                    };

                    FORMAT_PROMPT_MOVE!(
                        format!("{}{}", highlighted_a, color::Fg(color::LightBlack)),
                        format!("{}{}", color::Fg(color::Reset), highlighted_b)
                    )
                } else {
                    line.clone()
                }
            },
            _ => line.clone()
        }
    }

    fn on_prompt_enter(&mut self) {
        let line = self.prompt.get_line();
        self.prompt.clear();

        if line.is_empty() {
            self.set_output_text("".to_string());
            return;
        }

        let intent = Intent::from_partial_command(&self.game.state, &line);

        self.execute_intent(&intent)
            .unwrap_or_else(
                |m| self.set_output_text(format!(
                    "{}{}{}",
                    color::Fg(color::Red),
                    m,
                    color::Fg(color::Reset),
                ))
            );
    }

    pub fn execute_intent(&mut self, intent: &Intent) -> Result<(), String> {
        match intent {
            Intent::Move(Some(a), Some(b)) => self.execute_move(a, b),
            Intent::SelectPromotionType(piece_type) => self.execute_promotion(piece_type),
            _ => Err(OUTPUT_INVALID_COMMAND.to_string()),
        }
    }

    fn execute_promotion(&mut self, piece_type: &PieceType) -> Result<(), String> {
        match &self.game.state {
            GameState::SelectPromotionType(from, to) => {
                let from = from.clone();
                let to = to.clone();
                match self.game.move_piece_with_promotion(&from, &to, piece_type) {
                    Ok(_) => {
                        self.set_output_text("".to_string());
                        Ok(())
                    },
                    Err(err) => Err(FORMAT_OUTPUT_CIRITCAL_ERROR!(err)),
                }
            },
            _ => Err("Not in promotion state".to_string())
        }
    }

    fn execute_move(&mut self, a: &PartialCoordinate, b: &PartialCoordinate) -> Result<(), String> {
        if let (Some(from), Some(to)) = (a.to_complete(), b.to_complete()) {
            match self.game.move_piece(&from, &to) {
                Ok(_) => {
                    self.set_output_text("".to_string());
                    Ok(())
                },
                Err(MoveError::IsCheck) => Err(OUTPUT_MOVE_ERROR_CHECK.to_string()),
                Err(_) => Err(OUTPUT_ILLEGAL_MOVE.to_string()),
            }
        } else {
            Err(OUTPUT_INVALID_COMMAND.to_string())
        }
    }

    fn draw_board(&mut self) {
        // TODO: get terminal size and only draw if size is sufficient

        let line = self.prompt.get_line();
        let intent = Intent::from_partial_command(&self.game.state, &line);

        self.clear_highlight();
        self.evaluate_intent(&intent);

        self.terminal.clear_screen();
        self.draw_coordinates(0, 0);
        self.draw_grid(1 * self.horizontal_scale, 1);
        self.draw_pieces(1 * self.horizontal_scale, 1);
        self.draw_output(0, BOARD_SIZE * self.field_size + 5);
        self.draw_prompt(0, BOARD_SIZE * self.field_size + 4, &line, &intent);
        self.terminal.flush();
    }

    fn set_output_text(&mut self, new_text: String) {
        self.output_text = new_text;
    }

    fn draw_output(&mut self, offset_x: usize, offset_y: usize) {
        self.terminal.move_cursor(offset_x, offset_y);

        let output_text = if 
        self.output_text.is_empty() {
            match self.game.state {
                GameState::SelectPromotionType(..) => OUTPUT_HINT_PROMOTE.to_string(),
                GameState::WaitMove(true) => format!(
                    "{}{}{} {}",
                    color::Fg(color::Yellow),
                    OUTPUT_STATE_CHECK,
                    color::Fg(color::Reset),
                    FORMAT_OUTPUT_TURN_SHORT!(self.game.board.turn.to_label()),
                ),
                GameState::CheckMate => FORMAT_OUTPUT_CHECKMATE!(self.game.board.turn.to_label()),
                _ => FORMAT_OUTPUT_TURN!(self.game.board.turn.to_label()),
            }
        } else {
            self.output_text.clone()
        };

        write!(self.terminal.screen, "{}", output_text).unwrap();
    }

    fn draw_coordinates(&mut self, offset_x: usize, offset_y: usize) {
        let v_center = self.field_size / 2;
        let h_center = v_center * self.horizontal_scale;

        // Draw horizontal coordinates
        for y in 0..=1 {
            let pos_y = y * (BOARD_SIZE * self.field_size + 2) + offset_y;

            for x in 0..BOARD_SIZE {
                let pos_x = x * self.field_size * self.horizontal_scale + h_center + offset_x + 2;
                let coord = Coordinate::try_new(x, y * BOARD_MAX_AXIS).unwrap();
                let label = column_to_name(coord.xv());

                self.terminal.move_cursor(pos_x, pos_y);
                write!(self.terminal.screen, "{}", label).unwrap();
            }
        }

        // Draw vertical coordinates
        for x in 0..=1 {
            let pos_x = x * (BOARD_SIZE * self.field_size * self.horizontal_scale + 4) + offset_x;

            for y in 0..BOARD_SIZE {
                let pos_y = y * self.field_size + v_center + offset_y + 1;
                let coord = Coordinate::try_new(x * BOARD_MAX_AXIS, y).unwrap();
                let label = row_to_name(coord.yv());

                self.terminal.move_cursor(pos_x, pos_y);
                write!(self.terminal.screen, "{}", label).unwrap();
            }
        }
    }

    fn draw_grid(&mut self, offset_x: usize, offset_y: usize) {
        let h_bar = '-';
        let v_bar = '|';
        let cross = '+';

        for y in 0..=BOARD_SIZE {
            let pos_y = y * self.field_size + offset_y;

            for x in 0..=BOARD_SIZE {
                let pos_x = x * self.field_size * self.horizontal_scale + offset_x;

                self.terminal.move_cursor(pos_x, pos_y);

                // Top left corss
                write!(self.terminal.screen, "{}", cross).unwrap();

                // Horizontal
                if x < BOARD_SIZE {
                    for _ in 1..(self.field_size * self.horizontal_scale) {
                        write!(self.terminal.screen, "{}", h_bar).unwrap();
                    }
                }

                // Vertical
                if y < BOARD_SIZE {
                    for w in 1..self.field_size {
                        // TODO: Improve this so the cursor does not have to be moved for every line
                        self.terminal.move_cursor(pos_x, pos_y + w);
                        write!(self.terminal.screen, "{}", v_bar).unwrap();
                    }
                }

                // Background
                if x < BOARD_SIZE && y < BOARD_SIZE {
                    let background_color = self.get_background_color_at(&Coordinate::try_new(x, y).unwrap());
                    let board_highlight = &self.highlighted_cells[y as usize][x as usize];
                    let is_highlighted = !matches!(board_highlight, BoardHighlight::None);

                    for yi in 0..self.field_size {
                        for xi in 0..self.field_size * self.horizontal_scale {
                            self.terminal.move_cursor(pos_x + xi + 1, pos_y + yi + 1);

                            if is_highlighted && (xi + yi) % 2 == 0 {
                                write!(
                                    self.terminal.screen,
                                    "{}*{}",
                                    board_highlight.background_color(),
                                    color::Bg(color::Reset),
                                ).unwrap();
                            } else {
                                write!(
                                    self.terminal.screen,
                                    "{} {}",
                                    background_color,
                                    color::Bg(color::Reset),
                                ).unwrap();
                            };
                        }
                    }
                }
            }
        }
    }

    fn draw_pieces(&mut self, offset_x: usize, offset_y: usize) {
        let v_center = self.field_size / 2;
        let h_center = v_center * self.horizontal_scale;

        for y in 0..BOARD_SIZE {
            let pos_y = y * self.field_size + offset_y;

            for x in 0..BOARD_SIZE {
                let pos_x = x * self.field_size * self.horizontal_scale + offset_x;

                let coordinate = Coordinate::try_new(x, y).unwrap();
                let tile = self.game.board.get_tile(&coordinate);

                if let TileContent::Piece(piece) = tile {
                    let symbol = piece.get_symbol();

                    let label = match piece.player {
                        Player::White => symbol.to_ascii_uppercase(),
                        Player::Black => symbol.to_ascii_lowercase(),
                    };
                    let background_color = self.get_background_color_at(&coordinate);

                    self.terminal.move_cursor(pos_x + h_center, pos_y + v_center);
                    write!(
                        self.terminal.screen,
                        "{}{}{}{}",
                        background_color,
                        label,
                        color::Bg(color::Reset),
                        color::Fg(color::Reset),
                    ).unwrap();
                }
            }
        }
    }

    fn get_background_color_at(& self, coordinate: &Coordinate) -> String {
        match coordinate.get_field_color() {
            FieldColor::White => color::Bg(color::White).to_string(),
            FieldColor::Black => color::Bg(color::Black).to_string(),
        }
    }
}
