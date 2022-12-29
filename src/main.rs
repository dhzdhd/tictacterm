use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};

#[derive(PartialEq, Eq, Copy, Clone)]
enum Choice {
    X,
    O,
}

#[derive(PartialEq, Eq)]
enum GameRes {
    Win(Choice),
    Draw,
    Neutral,
}

struct State {
    count: u8,
    choice: Choice,
    arr: [char; 9],
    result: GameRes,
}

impl State {
    fn new() -> Self {
        Self {
            count: 0,
            choice: Choice::X,
            arr: [' '; 9],
            result: GameRes::Neutral,
        }
    }

    fn reset(&mut self) {
        self.count = 0;
        self.choice = Choice::X;
        self.arr = [' '; 9];
        self.result = GameRes::Neutral;
    }

    fn check_result(&self) -> GameRes {
        let get_choice_from_char = |chr| {
            if chr == 'x' {
                Choice::X
            } else {
                Choice::O
            }
        };

        let get_char_from_choice = |choice| match choice {
            Choice::O => 'O',
            Choice::X => 'X',
        };

        let chr = get_char_from_choice(self.choice);

        if (self.arr[0] == chr && self.arr[0] == self.arr[1] && self.arr[1] == self.arr[2])
            || (self.arr[3] == chr && self.arr[3] == self.arr[4] && self.arr[4] == self.arr[5])
            || (self.arr[6] == chr && self.arr[6] == self.arr[7] && self.arr[7] == self.arr[8])
            || (self.arr[0] == chr && self.arr[0] == self.arr[3] && self.arr[3] == self.arr[6])
            || (self.arr[1] == chr && self.arr[1] == self.arr[4] && self.arr[4] == self.arr[7])
            || (self.arr[2] == chr && self.arr[2] == self.arr[5] && self.arr[5] == self.arr[8])
            || (self.arr[0] == chr && self.arr[0] == self.arr[4] && self.arr[4] == self.arr[8])
            || (self.arr[2] == chr && self.arr[2] == self.arr[4] && self.arr[4] == self.arr[6])
        {
            GameRes::Win(get_choice_from_char(chr))
        } else {
            if self.count == 9 {
                GameRes::Draw
            } else {
                GameRes::Neutral
            }
        }
    }

    fn turn(&mut self, index: u32) {
        if self.arr[(index - 1) as usize] != ' ' {
            return;
        };

        self.count += 1;
        self.choice = if self.choice == Choice::X {
            Choice::O
        } else {
            Choice::X
        };
        self.arr[(index - 1) as usize] = match self.choice {
            Choice::X => 'X',
            Choice::O => 'O',
        };

        self.result = self.check_result();
    }
}

fn main() -> Result<(), io::Error> {
    // Setup
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // App
    let mut state: State = State::new();
    run_app(&mut terminal, &mut state)?;

    // Restore
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, state: &mut State) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, state))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char(i) if i >= '1' && i <= '9' && state.result == GameRes::Neutral => {
                    state.turn(i.to_digit(10).unwrap())
                }
                KeyCode::Enter if state.result != GameRes::Neutral => state.reset(),
                _ => continue,
            }
        }
    }
}

fn ui<B: Backend>(frame: &mut Frame<B>, state: &mut State) {
    let size = frame.size();

    let block = Block::default()
        .borders(Borders::ALL)
        .title("TicTacToe | Starting with X")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(Color::Cyan));
    // .style(Style::default().bg(Color::Blue));
    frame.render_widget(block, size);

    let get_size = |i: u16| {
        let width = size.width / 9;
        let height = size.height / 9;
        let init_x = size.width / 2 - (3 * width / 2);
        let init_y = size.height / 2 - height;
        Rect::new(
            init_x + width * (i % 3),
            init_y + height * (i / 3),
            width,
            height,
        )
    };

    let popup_block = |content| {
        Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title_alignment(Alignment::Center)
                    .title("Result")
                    .style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Cyan))
    };
    let popup_area = Rect {
        x: size.width / 2 - (3 * size.width / 18),
        y: size.height / 2 - size.height / 9,
        width: size.width / 3,
        height: 5,
    };
    match state.result {
        GameRes::Win(c) => frame.render_widget(
            popup_block(format!(
                "{} Wins!\nPress Enter to reset.",
                match c {
                    Choice::X => "X",
                    Choice::O => "O",
                }
            )),
            popup_area,
        ),
        GameRes::Draw => frame.render_widget(
            popup_block("Draw!\nPress Enter to reset.".to_string()),
            popup_area,
        ),
        GameRes::Neutral => {
            let _ = state
                .arr
                .iter()
                .enumerate()
                .map(|(index, e)| {
                    let para = Paragraph::new(e.to_string())
                        .style(
                            Style::default()
                                .fg(Color::Green)
                                .add_modifier(Modifier::BOLD),
                        )
                        .alignment(Alignment::Center);
                    frame.render_widget(para, get_size(index as u16));
                })
                .collect::<()>();
        }
    }

    ()
}
