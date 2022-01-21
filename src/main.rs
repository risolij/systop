use std::io;
use tui::Terminal;
use tui::backend::{TermionBackend, Backend};
use tui::widgets::{Block, Borders};
use termion::raw::IntoRawMode;
use tui::widgets::{List, ListState, ListItem, Tabs};
use tui::layout::{Rect, Constraint, Layout, Direction};
use tui::text::Spans;
use tui::style::{Color, Style};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    run(&mut terminal);

    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()>{
    terminal.autoresize();
    terminal.clear();
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(15),
                    Constraint::Percentage(75),
                    Constraint::Percentage(10),
                ].as_ref()
            )
            .split(f.size());

        let tabs = header();
        f.render_widget(tabs, chunks[0]);

        let block = generate_block("Block 2", true);
        f.render_widget(block, chunks[1]);


        let block = generate_block("Block 3", true);
        f.render_widget(block, chunks[2]);
    })?;

    Ok(())
}


fn header() -> Tabs<'static> {
    let titles = ["Tab1", "Tab2", "Tab3"]
        .iter()
        .cloned()
        .map(Spans::from)
        .collect();

    Tabs::new(titles)
        .block(Block::default().title("Tabs").borders(Borders::ALL))
        .style(Style::default().fg(Color::Green))
}

fn generate_block(title: &'static str, borders: bool) -> Block {
    if borders {
        return Block::default().title(title).borders(Borders::ALL)
    };

    return Block::default().title(title)
}

