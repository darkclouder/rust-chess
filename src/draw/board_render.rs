use crate::draw::terminal::Terminal;
use crate::logic::board::{Board, Coordinate, FieldColor};

use termion::event::Key;
use termion::color;
use std::io::Write;


pub struct BoardRenderer<'a> {
    board: &'a Board,
    terminal: Terminal,
    field_size: u16,
    horizontal_scale: u16,
}


impl<'a> BoardRenderer<'a> {
    pub fn new(board: &'a Board) -> Self {
        Self {
            board,
            terminal: Terminal::default(),
            field_size: 4,
            horizontal_scale: 2,
        }
    }

    pub fn run(&mut self) {
        loop {
            self.draw_board();

            if let Some(Key::Char(_)) = self.terminal.read_key() {
                write!(self.terminal.screen, "bla").unwrap()
            }
        }
    }

    fn draw_board(&mut self) {
        // TODO: get terminal size and only draw if size is sufficient

        self.terminal.clear_screen();
        self.draw_coordinates(0, 0);
        self.draw_grid(1 * self.horizontal_scale, 1);
        self.draw_prompt(0, self.board.size * self.field_size + 4);
        self.terminal.flush();
    }

    fn draw_coordinates(&mut self, offset_x: u16, offset_y: u16) {
        let v_center = self.field_size / 2;
        let h_center = v_center * self.horizontal_scale;

        // Draw horizontal coordinates
        for y in 0..=1 {
            let pos_y = y * (self.board.size * self.field_size + 2) + offset_y;

            for x in 0..self.board.size {
                let pos_x = x * self.field_size * self.horizontal_scale + h_center + offset_x + 2;
                let coord = Coordinate { x, y: y * (self.board.size - 1) };
                let label = self.board.coordinate_to_fieldname(&coord).horizontal;

                self.terminal.move_cursor(pos_x, pos_y);
                write!(self.terminal.screen, "{}", label).unwrap();
            }
        }

        // Draw vertical coordinates
        for x in 0..=1 {
            let pos_x = x * (self.board.size * self.field_size * self.horizontal_scale + 4) + offset_x;

            for y in 0..self.board.size {
                let pos_y = y * self.field_size + v_center + offset_y + 1;
                let coord = Coordinate { x: x * self.board.size, y };
                let label = self.board.coordinate_to_fieldname(&coord).vertical;

                self.terminal.move_cursor(pos_x, pos_y);
                write!(self.terminal.screen, "{}", label).unwrap();
            }
        }
    }

    fn draw_grid(&mut self, offset_x: u16, offset_y: u16) {
        let h_bar = '-';
        let v_bar = '|';
        let cross = '+';

        for y in 0..=self.board.size {
            let pos_y = y * self.field_size + offset_y;

            for x in 0..=self.board.size {
                let pos_x = x * self.field_size * self.horizontal_scale + offset_x;

                self.terminal.move_cursor(pos_x, pos_y);

                // Top left corss
                write!(self.terminal.screen, "{}", cross).unwrap();

                // Horizontal
                if x < self.board.size {
                    for _ in 1..(self.field_size * self.horizontal_scale) {
                        write!(self.terminal.screen, "{}", h_bar).unwrap();
                    }
                }

                // Vertical
                if y < self.board.size {
                    for w in 1..self.field_size {
                        // TODO: Improve this so the cursor does not have to be moved for every line
                        self.terminal.move_cursor(pos_x, pos_y + w);
                        write!(self.terminal.screen, "{}", v_bar).unwrap();
                    }
                }

                // Background
                if x < self.board.size && y < self.board.size {
                    let field_color = self.board.get_field_color_at(&Coordinate { x, y });
                    let terminal_color = match field_color {
                        FieldColor::Black => color::Bg(color::Black).to_string(),
                        FieldColor::White => color::Bg(color::White).to_string(),
                    };

                    for yi in 0..self.field_size {
                        for xi in 0..self.field_size * self.horizontal_scale {
                            self.terminal.move_cursor(pos_x + xi + 1, pos_y + yi + 1);
                            write!(self.terminal.screen, "{} {}", terminal_color, color::Bg(color::Reset)).unwrap();
                        }
                    }
                }
            }
        }
    }

    fn draw_prompt(&mut self, offset_x: u16, offset_y: u16) {
        self.terminal.move_cursor(offset_x, offset_y);
        write!(self.terminal.screen, "> ").unwrap();
    }
}
