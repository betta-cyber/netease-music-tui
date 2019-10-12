mod util;
extern crate failure;

use std::io;
use termion::raw::IntoRawMode;
use tui::{Frame, Terminal};
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders, Tabs, Text, Paragraph};
use tui::layout::{Layout, Constraint, Direction, Rect};
use tui::style::{Color, Style, Modifier};
use termion::event::Key;
use util::event::{Event, Events};
use tui::backend::Backend;

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

struct App<'a> {
    tabs: TabsState<'a>,
}

fn main() -> Result<(), failure::Error> {

    use std::fs::File;
    use std::io::BufReader;
    use rodio::Source;

    let device = rodio::default_output_device().unwrap();

    let file = File::open("kk.mp3").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    rodio::play_raw(&device, source.convert_samples());

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = termion::input::MouseTerminal::from(stdout);
    let stdout = termion::screen::AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.hide_cursor()?;

    let events = Events::new();
    let mut app = App {
        tabs: TabsState::new(vec!["Tab0", "COOL", "Tab2", "Tab3"]),
    };

    loop {
        terminal.draw(|mut f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(size);
            Tabs::default()
                .block(Block::default().borders(Borders::ALL).title("Tabs"))
                .titles(&app.tabs.titles)
                .select(app.tabs.index)
                .style(Style::default().fg(Color::Cyan))
                .highlight_style(Style::default().fg(Color::Yellow))
                .render(&mut f, chunks[0]);

            match app.tabs.index {
                0 => Block::default()
                    .title("Inner 0")
                    .borders(Borders::ALL)
                    .render(&mut f, chunks[1]),
                1 => draw_first_tab(&mut f, &app, chunks[1]),
                2 => Block::default()
                    .title("Inner 2")
                    .borders(Borders::ALL)
                    .render(&mut f, chunks[1]),
                3 => Block::default()
                    .title("Inner 3")
                    .borders(Borders::ALL)
                    .render(&mut f, chunks[1]),
                _ => {}
            }
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Right => app.tabs.next(),
                Key::Left => app.tabs.previous(),
                _ => {}
            },
            _ => {}
        }
    }
    Ok(())
}


fn draw_first_tab<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(7),
                Constraint::Min(7),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(area);
    draw_text(f, chunks[0]);
    draw_text(f, chunks[1]);
    draw_text(f, chunks[2]);
}

fn draw_text<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let text = [
        Text::raw("This is a paragraph with several lines. You can change style your text the way you want.\n\nFox example: "),
        Text::styled("under", Style::default().fg(Color::Red)),
        Text::raw(" "),
        Text::styled("the", Style::default().fg(Color::Green)),
        Text::raw(" "),
        Text::styled("rainbow", Style::default().fg(Color::Blue)),
        Text::raw(".\nOh and if you didn't "),
        Text::styled("notice", Style::default().modifier(Modifier::ITALIC)),
        Text::raw(" you can "),
        Text::styled("automatically", Style::default().modifier(Modifier::BOLD)),
        Text::raw(" "),
        Text::styled("wrap", Style::default().modifier(Modifier::REVERSED)),
        Text::raw(" your "),
        Text::styled("text", Style::default().modifier(Modifier::UNDERLINED)),
        Text::raw(".\nOne more thing is that it should display unicode characters: 10€")
    ];

    Paragraph::new(text.iter())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("TEXT")
                .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::BOLD)),
        )
        .wrap(true)
        .render(f, area);
}
