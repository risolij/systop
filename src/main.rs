use std::io;
use tui::Terminal;
use tui::backend::{TermionBackend, Backend};
use tui::widgets::{Block, Borders};
use termion::raw::IntoRawMode;
use tui::widgets::Tabs;
use tui::layout::{Constraint, Layout, Direction};
use tui::text::Spans;
use tui::style::{Color, Style};

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let cpuinfo = CpuInfo::default();
    println!("{:?}", cpuinfo);
    run(&mut terminal).unwrap();

    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()>{
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


#[derive(Debug)]
struct CpuInfo {
    model_name: String,
    vendor_id: String,
    cores: usize,
    processors: usize,
}

impl Default for CpuInfo {
    fn default() -> Self {
        match std::fs::read_to_string("/proc/cpuinfo") {
            Ok(file) => {
                let processors = file.lines().filter(|f| f.contains("processor")).count();
                let vendor_id = file.lines().filter(|f| f.contains("vendor_id")).map(|f| f.to_string()).take(1).collect::<String>();
                let model_name = file.lines().filter(|f| f.contains("model name")).map(|f| f.to_string()).take(1).collect::<String>();
                let cores = file.lines().filter(|f| f.contains("cpu cores")).count();
                let mut v = vec![];
                for line in file.lines() {
                    if line.starts_with("model name") || line.starts_with("vendor_id") || line.starts_with("cpu cores") {
                        v.push(line)
                    }
                };

                Self {
                    model_name, 
                    vendor_id, 
                    cores,
                    processors,
                }
            },
            Err(e) => {
                println!("Uh oh {}", e);
                std::process::exit(1);
            }
        }
    }

}
