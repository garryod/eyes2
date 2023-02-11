use clap::Parser;
use eyes2::gui::GuiCmd;
use eyes2::world;
use eyes2::world::grid::WorldGrid;
use eyes2::{gui::EyesGui, settings::Settings};
use pancurses::endwin;
use std::io;
use std::io::prelude::*;
use std::{sync::mpsc, thread, time};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// run a performance test
    #[arg(short, long)]
    performance: bool,
    // reset settings to defaults
    #[arg(short, long)]
    reset: bool,
}

const SPEED_TICKS: [u64; 10] = [1, 1, 1, 1, 10, 50, 100, 1000, 10000, 100000];
const SPEED_DELAY: [u64; 10] = [1000, 10, 2, 1, 1, 1, 1, 1, 1, 0];

fn main() {
    let args = Args::parse();

    let settings = if args.reset {
        Settings::reset()
    } else {
        Settings::load()
    };

    if args.performance {
        performance_test(settings);
    } else {
        world_loop(settings);
    }
}

fn world_loop(mut settings: Settings) {
    // outer loop continues until user cancels
    'outer: loop {
        let mut world = world::types::World::new(settings);

        world.populate();

        let (tx_grid, rx_grid) = mpsc::channel();
        let (tx_gui_cmd, rx_gui_cmd) = mpsc::channel::<GuiCmd>();

        thread::spawn(move || {
            let mut gui = EyesGui::new();
            gui.gui_loop(rx_grid, tx_gui_cmd).ok()
        });

        // inner loop runs until all creatures die
        loop {
            if (world.grid.ticks % SPEED_TICKS[world.grid.speed as usize - 1]) == 0 {
                // Gui loop sends a command every 100ms, the None command indicates
                // no user input, but ready to receive the next world update
                let next_cmd = rx_gui_cmd.try_recv();
                if next_cmd.is_ok() {
                    let grid = &mut world.grid;
                    if handle_input(next_cmd.unwrap(), grid) {
                        break 'outer;
                    }
                    tx_grid.send(world.grid.clone()).unwrap();
                }

                if world.grid.speed < 10 {
                    thread::sleep(time::Duration::from_millis(
                        SPEED_DELAY[world.grid.speed as usize - 1],
                    ));
                }
            }
            world.grid.ticks += 1;
            world.tick();

            if world.creature_count() == 0 {
                // copy variable config to the next world
                settings.grass_rate = world.grid.grass_rate;
                settings.speed = world.grid.speed;
                break;
            }
        }
    }
    endwin();
}

fn handle_input(cmd: GuiCmd, grid: &mut WorldGrid) -> bool {
    match cmd {
        GuiCmd::Quit => {
            return true;
        }
        GuiCmd::SpeedUp => {
            grid.increment_speed(true);
        }
        GuiCmd::SpeedDown => {
            grid.increment_speed(false);
        }
        GuiCmd::GrassUp => {
            grid.increment_grass_rate(true);
        }
        GuiCmd::GrassDown => {
            grid.increment_grass_rate(false);
        }
        _ => {}
    }
    false
}

fn performance_test(settings: Settings) {
    // for performance testing, we use 1 creature which survives indefinitely
    let test_settings = Settings {
        size: 40,
        grass_count: 1000,
        creature_count: 1,
        grass_rate: 50,
        creature_move_energy: 0,
        creature_idle_energy: 0,
        creature_move_rate: 0.005,
        speed: 10,

        ..settings
    };

    println!("{:#?}", test_settings);
    println!("\nPerformance test with above settings ...");
    println!("\ntypical rate on giles ws1 is 150,000,000 ticks/s \n");

    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();

    world_loop(test_settings);
}
