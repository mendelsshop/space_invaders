use std::{
    io::{self, stdout},
    process::exit,
    time::Duration,
};

use crossterm::{
    cursor::{Hide, MoveTo},
    event::{poll, read, Event, KeyCode},
    execute,
    style::Print,
    terminal::{self, enable_raw_mode, Clear},
};
use rand::{rngs::ThreadRng, Rng};

// TODO: make movement/speed relative to terminal

#[allow(unused_must_use)]
fn main() {
    let mut game = Game::new().unwrap();
    enable_raw_mode();
    execute!(stdout(), Hide);
    execute!(stdout(), Clear(terminal::ClearType::All));
    // we only change the player positionon key input so we just print player before gameloop and when we move player
    execute!(stdout(), MoveTo(game.player.position, game.size.1));
    execute!(stdout(), Print("^"));
    loop {
        game.enemies.sort();
        if poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key) = read().unwrap() {
                match key.code {
                    KeyCode::Left => {
                        execute!(stdout(), MoveTo(game.player.position, game.size.1));
                        execute!(stdout(), Print(" "));
                        // moving should be relative to the termina
                        game.player.position -= 3;
                        execute!(stdout(), MoveTo(game.player.position, game.size.1));
                        execute!(stdout(), Print("^"));
                    }
                    KeyCode::Right => {
                        execute!(stdout(), MoveTo(game.player.position, game.size.1));
                        execute!(stdout(), Print(" "));
                        game.player.position += 3;
                        execute!(stdout(), MoveTo(game.player.position, game.size.1));
                        execute!(stdout(), Print("^"));
                    }
                    KeyCode::Char('q') => exit(0),
                    KeyCode::Char(' ') => {
                        let bullet_pos = game.player.position;
                        if let Some(enemy) = game
                            .enemies
                            .iter()
                            .position(|enemy| enemy.position.0 == bullet_pos)
                        {
                            let enemy = game.enemies.remove(enemy);
                            execute!(stdout(), MoveTo(enemy.position.0, enemy.position.1));
                            execute!(stdout(), Print(" "));
                        }
                        for i in (0..game.size.1 - 1).step_by(3).rev() {
                            execute!(stdout(), MoveTo(bullet_pos, i));
                            execute!(stdout(), Print("|"));
                            std::thread::sleep(Duration::from_millis(5));
                            execute!(stdout(), MoveTo(bullet_pos, i));
                            execute!(stdout(), Print(" "));
                        }
                    }
                    _ => {}
                }
            }
        }
        for enemy in &mut game.enemies {
            execute!(stdout(), MoveTo(enemy.position.0, enemy.position.1,));
            execute!(stdout(), Print(" "));

            if enemy.position.0 < 6 {
                enemy.position.1 += game.random.gen_range(1..3);
                enemy.position.0 = game.size.0;
            } else {
                enemy.position.0 -= game.random.gen_range(1..6);
            }
            execute!(stdout(), MoveTo(enemy.position.0, enemy.position.1,));
            execute!(stdout(), Print("*"));
        }
    }
    // TODO: rest terminal
}

#[derive(Clone, Copy, Debug)]
struct Player {
    // a player dosnt go up or down (so it just needs an x-coordinate)
    position: u16,
}

impl Player {
    fn new(position: u16) -> Self {
        Self { position }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord)]
struct EnemySpaceShip {
    // x, y
    position: (u16, u16),
}

impl PartialOrd for EnemySpaceShip {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // sort by higher y but lower x
        Some(
            other
                .position
                .1
                .cmp(&self.position.1)
                .then(self.position.0.cmp(&other.position.0)),
        )
    }
}

impl EnemySpaceShip {
    fn new(position: (u16, u16)) -> Self {
        Self { position }
    }
}
#[derive(Debug)]
struct Game {
    // x, y
    pub(crate) size: (u16, u16),
    pub(crate) player: Player,
    pub(crate) enemies: Vec<EnemySpaceShip>,
    pub(crate) random: ThreadRng,
    pub(crate) game_result: GameResult,
}

impl Game {
    /// creates a game with a few enemies
    /// a size a little smaller than the terminal size
    /// and the player at the middle of the bottom
    pub fn new() -> Result<Self, io::Error> {
        let window_size = terminal::window_size()?;
        let size = (window_size.columns, window_size.rows);
        let mut random = rand::thread_rng();
        Ok(Self {
            size,
            player: Player::new(size.0 - (size.0 / 2)),
            enemies: {
                // there should be enough enemies to fit the top three rows
                let number_or_enemies = random.gen_range(0..size.0 * (size.1 - (size.1 - 3)));
                (0..number_or_enemies)
                    .into_iter()
                    .map(|_| {
                        EnemySpaceShip::new((random.gen_range(0..size.0), random.gen_range(0..3)))
                    })
                    .collect()
            },
            random,
            game_result: GameResult::InProgress,
        })
    }

    fn game_result(&self) -> GameResult {
        self.game_result
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GameResult {
    InProgress,
}
