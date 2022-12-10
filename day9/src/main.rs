use aoclib::reader::lines;

use std::{error::Error, io::{self, Stdout}, thread, time::Duration, collections::BTreeSet};

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture}};
use tui::{backend::{CrosstermBackend, Backend}, Terminal, Frame, layout::{Layout, Direction, Constraint}, widgets::{canvas::Canvas}};

const RENDER: bool = true;
const DELAY: u64 = 5;

fn main() {
    println!("Part 1 = {}", run("src/input.txt", 2, RENDER, DELAY).unwrap());
    println!("Part 2 = {}", run("src/input.txt", 10, RENDER, DELAY).unwrap());
}

struct App {
    knots: Vec<(i32,i32)>,
    visited: BTreeSet<(i32,i32)>,
    render: bool,
    delay: u64,
}

fn setup_terminal() -> Result<Option<Terminal<CrosstermBackend<Stdout>>>, Box<dyn Error>> {
    if !RENDER {
        return Ok(None);
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    
    Ok(Some(Terminal::new(backend)?))
}

fn cleanup_terminal(terminal: &mut Option<Terminal<CrosstermBackend<Stdout>>>) -> Result<(), Box<dyn Error>> {
    match terminal {
        Some(terminal) => {
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
        
            Ok(terminal.show_cursor()?)
        },
        None => Ok(())
    }
}

pub fn run(input: &str, nodes: u8, render: bool, delay: u64) -> Result<u32, Box<dyn Error>> {
    let mut terminal = setup_terminal()?;

    let mut app = App {
        knots: Vec::new(),
        visited: BTreeSet::new(),
        render: render,
        delay: delay,
    };

    for _ in 0..nodes {
        app.knots.push((0,0));
    }

    runit(input, &mut app, &mut terminal);

    if app.render {
        thread::sleep(Duration::from_secs(5));
    }

    cleanup_terminal(&mut terminal)?;

    Ok(app.visited.len() as u32)
}

fn move_up(app: &mut App) {
    app.knots[0].0 += 1;
    move_tails(app);
}

fn move_down(app: &mut App) {
    app.knots[0].0 -= 1;
    move_tails(app);
}

fn move_right(app: &mut App) {
    app.knots[0].1 += 1;
    move_tails(app);
}

fn move_left(app: &mut App) {
    app.knots[0].1 -= 1;
    move_tails(app);
}

fn move_tails(app: &mut App) {
    for i in 1..app.knots.len() {
        let head = app.knots[i - 1]; // copy
        let tail = &mut app.knots[i];

        let rowdiff = head.0 - tail.0;
        let coldiff = head.1 - tail.1;

        match rowdiff {
            -2 => {
                match coldiff {
                    -2 => {
                        tail.0 -= 1;
                        tail.1 -= 1;
                    },
                    -1 => {
                        tail.0 -= 1;
                        tail.1 -= 1;
                    },
                    0 => {
                        tail.0 -= 1;
                    },
                    1 => {
                        tail.0 -= 1;
                        tail.1 += 1;
                    }
                    2 => {
                        tail.0 -= 1;
                        tail.1 += 1;
                    },
                    _ => panic!("xyz")
                };
            },
            -1 => {
                match coldiff {
                    -2 => {
                        tail.0 -= 1;
                        tail.1 -= 1;
                    },
                    -1 => {
                    },
                    0 => {
                    },
                    1 => {
                    }
                    2 => {
                        tail.0 -= 1;
                        tail.1 += 1;
                    },
                    _ => panic!("xyz")
                };
            },
            0 => {
                match coldiff {
                    -2 => {
                        tail.1 -= 1;
                    },
                    -1 => {
                    },
                    0 => {
                    },
                    1 => {
                    }
                    2 => {
                        tail.1 += 1;
                    },
                    _ => panic!("xyz")
                };
            },
            1 => {
                match coldiff {
                    -2 => {
                        tail.0 += 1;
                        tail.1 -= 1;
                    },
                    -1 => {
                    },
                    0 => {
                    },
                    1 => {
                    }
                    2 => {
                        tail.0 += 1;
                        tail.1 += 1;
                    },
                    _ => panic!("xyz")
                };
            },
            2 => {
                match coldiff {
                    -2 => {
                        tail.0 += 1;
                        tail.1 -= 1;
                    },
                    -1 => {
                        tail.0 += 1;
                        tail.1 -= 1;
                    },
                    0 => {
                        tail.0 += 1;
                    },
                    1 => {
                        tail.0 += 1;
                        tail.1 += 1;
                    }
                    2 => {
                        tail.0 += 1;
                        tail.1 += 1;
                    },
                    _ => panic!("xyz")
                };
            },
            _ => { panic!("abc"); }
        }

    }

    app.visited.insert(app.knots.last().unwrap().clone());
}

fn runit(input: &str, app: &mut App, terminal: &mut Option<Terminal<CrosstermBackend<Stdout>>>) {
    for line in lines(input) {
        let (direction, count) = line.split_once(" ").unwrap();
        let count: i32 = count.parse().unwrap();

        match direction {
            "R" => {
                for _ in 0..count {
                    move_right(app);
                    render(terminal, app);
                }
            }
            "L" => {
                for _ in 0..count {
                    move_left(app);
                    render(terminal, app);
                }
            },
            "U" => {
                for _ in 0..count {
                    move_up(app);
                    render(terminal, app);
                }
            },
            "D" => {
                for _ in 0..count {
                    move_down(app);
                    render(terminal, app);
                }
            },
            &_ => {}
        
        };
    }

}

fn render(terminal: &mut Option<Terminal<CrosstermBackend<Stdout>>>, app: &App) {
    match terminal {
        Some(terminal) => {
            terminal.draw(|f| draw(f, app)).unwrap();
        }
        None => {}
    }
}

fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    if !app.render {
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        //        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let mut w = chunks[0];

    if (w.width % 2) != 0 {
        w.width -= 1;
    }

    if (w.height % 2) != 0 {
        w.height -= 1;
    }

    let canvas_x = [ 0.0 -(w.width / 2) as f64, (w.width / 2) as f64];
    let canvas_y = [ 0.0 -(w.height / 2) as f64, (w.height / 2) as f64];

    let canvas = Canvas::default()
        //.block(Block::default().borders(Borders::ALL).title("Follow"))
        .paint(|ctx| {
            for (row,col) in &app.visited {
                ctx.print(*col as f64, *row as f64, format!("#"));
            }

            for (i,(row, col)) in app.knots.iter().enumerate().rev() {
                ctx.print(*col as f64, *row as f64, format!("{}", i));
            }

            let head = app.knots.first().unwrap();
            ctx.print(head.1 as f64, head.0 as f64, format!("H"));

        })
        .x_bounds(canvas_x)
        .y_bounds(canvas_y);

    f.render_widget(canvas, chunks[0]);

    thread::sleep(Duration::from_millis(app.delay));
}