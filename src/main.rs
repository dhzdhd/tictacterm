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

#[derive(PartialEq, Eq)]
enum Choice {
    X,
    O,
}

enum GameRes {
    Win(Choice),
    Draw,
    Neutral,
}

struct State {
    count: u32,
    choice: Choice,
    arr: Vec<char>,
    result: GameRes,
}

impl State {
    fn new() -> Self {
        Self {
            count: 0,
            choice: Choice::X,
            arr: Vec::from([' '; 9]),
            result: GameRes::Neutral,
        }
    }

    fn reset(&mut self) {
        self.count = 0;
        self.choice = Choice::X;
        self.arr = Vec::from([' '; 9]);
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

        // * Find a better way to do this 🗿
        if self.arr[0] == self.arr[1] && self.arr[1] == self.arr[2] {
            GameRes::Win(get_choice_from_char(self.arr[0]))
        } else if self.arr[3] == self.arr[4] && self.arr[4] == self.arr[5] {
            GameRes::Win(get_choice_from_char(self.arr[3]))
        } else if self.arr[6] == self.arr[7] && self.arr[7] == self.arr[8] {
            GameRes::Win(get_choice_from_char(self.arr[6]))
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
        }

        // match self.check_result() {
        //     GameRes::Draw => self.reset(),
        //     GameRes::Win(x) => self.reset(),
        //     _ => (),
        // }

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
                KeyCode::Char(i) if i >= '1' && i <= '9' => state.turn(i.to_digit(10).unwrap()),
                _ => continue,
            }
        }
    }
}

fn ui<B: Backend>(frame: &mut Frame<B>, state: &mut State) {
    let size = frame.size();

    // println!("{:?}", size);

    // ! Add win/lose modal

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

    ()
}
