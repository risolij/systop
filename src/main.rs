use std::io;
use termion::raw::IntoRawMode;
use tui::backend::{Backend, TermionBackend};
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::Spans;
use tui::widgets::Tabs;
use tui::widgets::{Block, Borders};
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    run(&mut terminal).unwrap();

    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    terminal.autoresize().unwrap();
    terminal.clear().unwrap();
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(15),
                    Constraint::Percentage(75),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
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
        return Block::default().title(title).borders(Borders::ALL);
    };

    return Block::default().title(title);
}

trait State {}

struct ReadFile {
    state: Option<Box<dyn State>>,
    contents: String,
}

impl ReadFile {
    fn new(file: &'static str) -> Self {
	    match std::fs::read_to_string(file.to_string()) {
	        Ok(f) => {
	    	    Self {
	    	        state: Some(Box::new(Read {})),
	    	        contents: f,
	    	    }
	        },
            Err(_) => {
                Self {
                    state: Some(Box::new(NotRead {})),
                    contents: "failure".to_string(),
                }
            }
	    }
    }
}

struct NotRead {}
impl State for NotRead {}

struct Read {}
impl State for Read {}



  

#[derive(Debug)]
struct CpuInfo {
    model_name: String,
    vendor_id: String,
    cores: usize,
    processors: usize,
}

impl Default for CpuInfo {
    fn default() -> Self {
        let file = ReadFile::new("/proc/cpuinfo");

        let model_name = file
            .contents
            .lines()
            .filter(|f| f.contains("model name"))
            .map(|f| f.to_string())
            .take(1)
            .collect::<String>();

        let vendor_id = file
            .contents
            .lines()
            .filter(|f| f.contains("vendor_id"))
            .map(|f| f.to_string())
            .take(1)
            .collect::<String>();

        let processors = file.contents.lines().filter(|f| f.contains("processor")).count();
        let cores = file.contents.lines().filter(|f| f.contains("cpu cores")).count();

        Self {
            model_name,
            vendor_id,
            cores,
            processors,
        }
    }
}
