use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Direction, Layout, Position, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, List, ListItem, Paragraph};
use ratatui::{DefaultTerminal, Frame};

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn load_word_list(filename: &str) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?)
        .lines()
        .collect()
}


fn letter_to_index(c: char) -> usize {
    (c.to_ascii_lowercase() as u8 - b'a') as usize
}


fn count_chars(a: &str, b: &str) -> Vec<i32> {
    let mut counts = vec![0; 26];
    a.chars().filter(|&c| c.is_alphabetic())
        .for_each(|c| counts[letter_to_index(c)] -= 1);
    b.chars().filter(|&c| c.is_alphabetic())
        .for_each(|c| counts[letter_to_index(c)] += 1);
    counts
}


fn main() {
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    let _ = app_result;
}


struct App {
    input1: String,
    input2: String,
    character_index: usize,
    input_mode: InputMode,
    suggestions: Vec<String>,
    letters: Vec<i32>,
    word_list: Vec<String>
}


#[derive(PartialEq)]
enum InputMode {
    Normal,
    Editing1,
    Editing2,
}


impl App {
    const fn new() -> Self {
        Self {
            input1: String::new(),
            input2: String::new(),
            input_mode: InputMode::Normal,
            suggestions: Vec::new(),
            letters: Vec::new(),
            character_index: 0,
            word_list: Vec::new(),
        }
    }


    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }


    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }


    fn enter_char(&mut self, new_char: char) {
        let byte_index = self.byte_index();
        
        let input = match self.input_mode {
            InputMode::Editing1 => &mut self.input1,
            InputMode::Editing2 => &mut self.input2,
            _ => return,
        };

        input.insert(byte_index, new_char);
        self.move_cursor_right();
    }


    fn byte_index(&self) -> usize {
        let input = match self.input_mode {
            InputMode::Editing1 => &self.input1,
            InputMode::Editing2 => &self.input2,
            _ => "",
        };

        input.char_indices()
             .map(|(i, _)| i)
             .nth(self.character_index)
             .unwrap_or(input.len())
    }


    fn remove_char(&mut self, delete_left: bool) {
        let idx = self.character_index;

        let input = match self.input_mode {
            InputMode::Editing1 => &mut self.input1,
            InputMode::Editing2 => &mut self.input2,
            _ => return,
        };

        if (delete_left && idx == 0) || (!delete_left && idx >= input.len()) {
            return;
        }

        let offset = idx - (delete_left as usize);
        let start = input.chars().take(offset);
        let rest = input.chars().skip(offset + 1);

        *input = start.chain(rest).collect();

        if delete_left {
            self.move_cursor_left();
        }
    }


    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        let input = match self.input_mode {
            InputMode::Editing1 => &self.input1,
            InputMode::Editing2 => &self.input2,
            _ => "",
        };
        new_cursor_pos.clamp(0, input.chars().count())
    }


    fn compute_letters(&mut self) {
        self.letters = match self.input_mode {
            InputMode::Editing1 => count_chars(&self.input1, &self.input2),
            _ => count_chars(&self.input2, &self.input1)
        };
    }


    fn compute_suggestions(&mut self) {
        let input = match self.input_mode {
            InputMode::Editing1 => &self.input1,
            InputMode::Editing2 => &self.input2,
            _ => return,
        };

        let last_word = input.split_whitespace().last().unwrap_or("");
        let word_start = if input.ends_with(' ') { "" } else { last_word };

        let is_legal = |word| {
             count_chars(word_start, word)
                .iter()
                .zip(&self.letters)
                .all(|(w_count, l_count)| w_count <= l_count)
        };

        self.suggestions = self.word_list.iter()
            .filter(|word| word.starts_with(word_start) && is_legal(word))
            .cloned()
            .collect();

        self.suggestions.sort_by(|a, b| b.len().cmp(&a.len()));
        self.suggestions.truncate(100);
    }


    fn run(mut self, mut terminal: DefaultTerminal) -> Result<(), Box<dyn Error>> {
        self.letters = vec![0; 26];
        self.word_list = load_word_list("words.txt")?;
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            self.input_mode = InputMode::Editing1;
                            self.character_index = self.clamp_cursor(self.character_index);
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        _ => {}
                    },
                    InputMode::Editing1|InputMode::Editing2 if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Char(to_insert) => self.enter_char(to_insert),
                        KeyCode::Backspace => self.remove_char(true),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Esc => self.input_mode = InputMode::Normal,
                        KeyCode::Delete => self.remove_char(false),
                        KeyCode::Tab => {
                            match self.input_mode {
                                InputMode::Editing1 => self.input_mode = InputMode::Editing2,
                                InputMode::Editing2 => self.input_mode = InputMode::Editing1,
                                _ => {}
                            }
                            self.character_index = self.clamp_cursor(self.character_index);
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
            self.compute_letters();
            self.compute_suggestions();
        }
    }


    fn get_help_msg(&self) -> Paragraph<'static> {
        let (msg, style) = match self.input_mode {
            InputMode::Normal => (
                vec![
                    "Press ".into(),
                    "q".bold(),
                    " to exit, ".into(),
                    "e".bold(),
                    " to start editing".into(),
                ],
                Style::default().add_modifier(Modifier::RAPID_BLINK),
            ),
            InputMode::Editing1 | InputMode::Editing2 => (
                vec![
                    "Press ".into(),
                    "Esc".bold(),
                    " to stop editing, ".into(),
                    "Tab".bold(),
                    " to swap inputs".into(),
                ],
                Style::default(),
            ),
        };

        Paragraph::new(Text::from(Line::from(msg)).patch_style(style))
    }


    fn new_input<'a>(&self, input: &'a str, mode: InputMode, title: &'a str) -> Paragraph<'a> {
        let colour = match (mode == self.input_mode, self.letters.iter().sum::<i32>()) {
            (true, _) => Color::Yellow,
            (_, 0) => Color::Green,
            _ => Color::Reset,
        };

        Paragraph::new(input)
            .style(Style::default().fg(colour))
            .block(Block::bordered().title(title))
    }


    fn get_letters(&self) -> Vec<Span> {
        self.letters.iter().enumerate().map(|(i, &m)| {
            let letter = (b'A' + i as u8) as char;
            let text_color = match m {
                0 => Color::Green,
                n if n > 0 => Color::Blue,
                _ => Color::Red,
            };
            
            let style = Style::default().fg(text_color);
            Span::styled(format!("{}: {}", letter, m), style)
        }).collect()
    }


    fn get_suggestions(&self) -> List{
        let suggestions: Vec<ListItem> = self
            .suggestions
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = Line::from(Span::raw(format!("{i}: {m}")));
                ListItem::new(content)
            })
            .collect();
        List::new(suggestions).block(Block::bordered().title("Suggestions"))
    }


    fn get_cursor_pos(&self, input_area1: Rect, input_area2: Rect) -> Position {
        match self.input_mode {
            InputMode::Editing1 => Position::new( 
                input_area1.x + self.character_index as u16 + 1,
                input_area1.y + 1,
            ),
            InputMode::Editing2 => Position::new(
                input_area2.x + self.character_index as u16 + 1,
                input_area2.y + 1,
            ),
            _ => Position { x: 0, y: 0 }
        }
    }


    fn draw(&self, frame: &mut Frame) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ]);

        let [
            help_area, input_area1, input_area2, 
            letters_area, suggestions_area
        ] = vertical.areas(frame.area());

        let cursor_pos = self.get_cursor_pos(input_area1, input_area2);

        let input1 = self.new_input(&self.input1, InputMode::Editing1, "Input-1");
        let input2 = self.new_input(&self.input2, InputMode::Editing2, "Input-2");

        let letter_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Ratio(1, 26); 26])
            .vertical_margin(1)
            .horizontal_margin(5)
            .split(letters_area);

        let letters = self.get_letters();
        let suggestions = self.get_suggestions();

        frame.set_cursor_position(cursor_pos);
        frame.render_widget(self.get_help_msg(), help_area);
        frame.render_widget(input1, input_area1);
        frame.render_widget(input2, input_area2);

        frame.render_widget(Block::bordered().title("letters"), letters_area);
        for (i, area) in letter_layout.iter().enumerate() {
            frame.render_widget(letters[i].clone(), *area);
        }

        frame.render_widget(suggestions, suggestions_area);

    }

}