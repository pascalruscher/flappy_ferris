#![no_std]
mod wasm4;

use core::panic::PanicInfo;
use wasm4::*;

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    trace("Panic!");
    loop {}
}

const SPRITE_SIZE: u32 = 32;

// ferris_jump
const FERRIS_JUMP: [u8; 256] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x0f, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0xcc, 0x33, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0xd7, 0xd7, 0x3c, 0x00, 0x00,
    0x00, 0x00, 0x33, 0xd5, 0x57, 0xcc, 0x00, 0x00, 0x00, 0x00, 0x35, 0x55, 0x55, 0x5c, 0x00, 0x00,
    0x00, 0x00, 0xd5, 0x55, 0x55, 0x57, 0x00, 0x00, 0x00, 0x03, 0x55, 0xa5, 0x5a, 0x55, 0xc0, 0x00,
    0x00, 0x0d, 0x56, 0xf5, 0x5f, 0x95, 0x70, 0x00, 0x00, 0x0d, 0xd7, 0xcd, 0x7c, 0xd7, 0x70, 0x00,
    0x00, 0x37, 0x57, 0xfd, 0x7f, 0xd5, 0xdc, 0x00, 0x00, 0x37, 0x55, 0xf5, 0x5f, 0x55, 0xdc, 0x00,
    0x03, 0xd3, 0x55, 0x55, 0x55, 0x55, 0xc7, 0xc0, 0x0c, 0x54, 0xf5, 0x5b, 0xe5, 0x5f, 0x15, 0x30,
    0x31, 0x65, 0xef, 0xd5, 0x57, 0xfb, 0x59, 0x4c, 0x35, 0xb9, 0xec, 0x3f, 0xfc, 0x3b, 0x6e, 0x5c,
    0x36, 0xfc, 0xfc, 0x00, 0x00, 0x3f, 0x3f, 0x9c, 0x36, 0xcf, 0x00, 0x00, 0x00, 0x00, 0xf3, 0x9c,
    0x36, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x03, 0x9c, 0x0d, 0x30, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x70,
    0x03, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x0f, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

// ferris_fall
const FERRIS_FALL: [u8; 256] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x0f, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x03, 0xf0, 0x30, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x03, 0x0c,
    0xcb, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe3, 0xdb, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x3c, 0xe7,
    0xdb, 0x33, 0x00, 0x00, 0x00, 0x00, 0xcc, 0xe7, 0xd6, 0xe7, 0x00, 0xf0, 0x0f, 0x00, 0xdb, 0x97,
    0xd5, 0x97, 0x00, 0xcc, 0x33, 0x00, 0xd6, 0x57, 0x35, 0x57, 0x3c, 0xd7, 0xd7, 0x3c, 0xd5, 0x5c,
    0x0f, 0x5c, 0x33, 0xd5, 0x57, 0xcc, 0x35, 0xf0, 0x00, 0xdc, 0x35, 0x55, 0x55, 0x5c, 0x37, 0x00,
    0x00, 0xdc, 0xd5, 0x55, 0x55, 0x57, 0x37, 0x00, 0x00, 0x37, 0xd5, 0xa5, 0x5a, 0x57, 0xdc, 0x00,
    0x00, 0x35, 0x56, 0xf5, 0x5f, 0x95, 0x5c, 0x00, 0x00, 0x0d, 0x57, 0xfd, 0x7f, 0xd5, 0x70, 0x00,
    0x00, 0x0f, 0x57, 0xcd, 0x7c, 0xd5, 0xf0, 0x00, 0x00, 0x3b, 0x55, 0xf5, 0x5f, 0x55, 0xec, 0x00,
    0x00, 0xc5, 0x55, 0x55, 0x55, 0x55, 0x53, 0x00, 0x00, 0xc6, 0xf5, 0x5b, 0xe5, 0x5f, 0x93, 0x00,
    0x00, 0xc7, 0xaf, 0xd5, 0x57, 0xfa, 0xd3, 0x00, 0x00, 0x37, 0xec, 0x3f, 0xfc, 0x3b, 0xdc, 0x00,
    0x00, 0x37, 0x3c, 0x00, 0x00, 0x3c, 0xdc, 0x00, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

enum State {
    Menu,
    Running,
    GameOver,
}

#[derive(Clone, Copy)]
struct Position {
    x: f32,
    y: f32,
}

struct Obstacle {
    position: Position,
    size: u32,
    width: u32,
}

impl Obstacle {
    const fn new(position: Position, size: u32, width: u32) -> Self {
        Obstacle {
            position,
            size,
            width,
        }
    }

    fn update(&mut self) {
        self.position.x -= 0.4;
    }

    fn render(&self) {
        set_draw_colors(0x43);
        rect(
            self.position.x as i32,
            -1,
            self.width,
            (self.position.y as u32) - self.size / 2,
        );
        rect(
            self.position.x as i32,
            self.position.y as i32 + (self.size / 2) as i32,
            self.width,
            SCREEN_SIZE - (self.position.y as u32 + self.size / 2) + 1,
        );
        set_draw_colors(0x4321);
    }

    fn hit(&self, player: &Player) -> bool {
        // TODO: eliminate magic numbers and simplify types
        (player.position.x + 10.0 < self.position.x + self.width as f32
            && player.position.x as i32 + (SPRITE_SIZE as i32 - 10) > self.position.x as i32)
            && ((player.position.y as i32 + 20) < self.position.y as i32 - (self.size / 2) as i32
                || (player.position.y as i32) + (SPRITE_SIZE as i32 - 20)
                    > self.position.y as i32 + self.position.y as i32 / 2)
    }
}

struct Player {
    position: Position,
    velocity: f32,
}

impl Player {
    const fn new(position: Position) -> Self {
        Player {
            position,
            velocity: 0.0,
        }
    }

    fn update(&mut self) {
        if self.velocity < 1.0 {
            self.velocity += 0.1;
        }

        self.position.y += self.velocity;
        if self.position.y < 0.0 {
            self.position.y = 0.0;
        } else if self.position.y > (SCREEN_SIZE - SPRITE_SIZE) as f32 {
            self.position.y = (SCREEN_SIZE - SPRITE_SIZE) as f32;
        }
    }

    fn render(&self) {
        if self.velocity > 0.0 {
            blit(
                &FERRIS_FALL,
                self.position.x as i32,
                self.position.y as i32,
                SPRITE_SIZE,
                SPRITE_SIZE,
                1,
            );
        } else {
            blit(
                &FERRIS_JUMP,
                self.position.x as i32,
                self.position.y as i32,
                SPRITE_SIZE,
                SPRITE_SIZE,
                1,
            );
        }
    }

    fn jump(&mut self) {
        self.velocity = -2.0;
    }
}

struct Game {
    player: Player,
    obstacle: Obstacle,
    score: i32,
    prev_input: u8,
    state: State,
}

impl Game {
    const fn new() -> Self {
        Game {
            player: Player::new(Position {
                x: 10.0,
                y: (SCREEN_SIZE / 2 - SPRITE_SIZE / 2) as f32,
            }),
            obstacle: Obstacle::new(
                Position {
                    x: (SCREEN_SIZE - 10) as f32,
                    y: (SCREEN_SIZE / 2) as f32,
                },
                100,
                10,
            ),
            score: 0,
            prev_input: 0,
            state: State::Menu,
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(Position {
            x: 10.0,
            y: (SCREEN_SIZE / 2 - SPRITE_SIZE / 2) as f32,
        });
        self.obstacle = Obstacle::new(
            Position {
                x: (SCREEN_SIZE - 10) as f32,
                y: (SCREEN_SIZE / 2) as f32,
            },
            100,
            10,
        );
        self.score = 0;
    }

    fn update(&mut self) {
        self.player.update();
        self.obstacle.update();

        if self.obstacle.hit(&self.player) {
            self.state = State::GameOver;
        }

        if self.obstacle.position.x < 0.0 - self.obstacle.width as f32 {
            self.score += 1;
            self.obstacle = Obstacle::new(
                Position {
                    x: SCREEN_SIZE as f32,
                    y: (SCREEN_SIZE / 2) as f32,
                },
                100,
                10,
            );
        }
    }

    fn input(&mut self) {
        let gamepad: u8 = get_gamepad();
        let just_pressed: u8 = gamepad & (gamepad ^ self.prev_input);

        match self.state {
            State::Running => {
                if just_pressed != 0 && BUTTON_1 != 0 {
                    self.player.jump();
                }
            }
            State::Menu => {
                if just_pressed != 0 && BUTTON_1 != 0 {
                    self.state = State::Running;
                }
                // TODO: implement quit
            }
            State::GameOver => {
                self.restart();
                if just_pressed != 0 && BUTTON_1 != 0 {
                    self.state = State::Running;
                }
                // TODO: implement quit
            }
        }

        self.prev_input = gamepad;
    }

    fn render(&self) {
        self.player.render();
        self.obstacle.render();
    }

    fn render_menu(&self) {
        text("Flappy Ferris", 10, 10);
        text("(X) Start Game", 10, 30);
        text("(Z) Quit Game", 10, 40);
    }

    fn render_game_over(&self) {
        text("Game Over", 10, 10);
        text("(X) Restart Game", 10, 30);
        text("(Z) Quit Game", 10, 40);
    }
}

static mut GAME: Game = Game::new();

#[no_mangle]
fn start() {
    set_palette();
    set_draw_colors(0x4321);
}

#[no_mangle]
fn update() {
    let game = unsafe { &mut GAME };
    match game.state {
        State::Running => {
            game.update();
            game.render();
        }
        State::Menu => game.render_menu(),
        State::GameOver => game.render_game_over(),
    }
    game.input();
}

fn set_palette() {
    unsafe { *PALETTE = [0xf08c48, 0xe95f37, 0xbc593d, 0x805043] };
}

fn set_draw_colors(color: u16) {
    unsafe { *DRAW_COLORS = color };
}

fn get_gamepad() -> u8 {
    unsafe { *GAMEPAD1 }
}
