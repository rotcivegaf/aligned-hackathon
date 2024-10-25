use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub fn xy(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }
}

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(long)]
    output: String,
}

struct GameState {
    pub dimension: Vec2,
    pub spaceship: Vec2,
    pub spaceship_shots: Vec<Vec2>,
    pub last_shot_frame: usize,
    pub aliens: Vec<Vec2>,
    pub aliens_shots: Vec<Vec2>,
    pub aliens_movement: (i32, bool), //dir, just_down
    pub last_aliens_movement: usize,
    pub last_aliens_shots: usize,
    pub lives: usize,
    pub score: u8,
    pub user_input: Vec<(u16, u8)>
}

#[derive(Serialize, Deserialize)]
struct Output {
    score: u8,
    win: bool,
    end_frame: u16,
    inputs: String,
}

impl GameState {
    pub fn new(dimension: Vec2) -> GameState {
        let mut aliens = Vec::new();
        for y in 3..7 {
            for x in 5..dimension.x - 5 {
                if x % 2 != 0 {
                    aliens.push(Vec2::xy(x, y));
                }
            }
        }
        GameState {
            dimension,
            spaceship: Vec2::xy(dimension.x / 2, dimension.y - 2),
            spaceship_shots: Vec::new(),
            last_shot_frame: 0,
            aliens,
            aliens_shots: Vec::new(),
            aliens_movement: (1, false),
            last_aliens_movement: 0,
            last_aliens_shots: 0,
            lives: 1,
            score: 0,
            user_input: Vec::new(),
        }
    }

    pub fn spaceship_move_x(&mut self, displacement: i32) {
        if displacement < 0 && self.spaceship.x != 0
            || displacement > 0 && self.spaceship.x != self.dimension.x
        {
            self.spaceship.x += displacement;
        }
    }

    pub fn spaceship_shot(&mut self, shot_frame: usize) {
        if self.last_shot_frame + 15 < shot_frame {
            self.spaceship_shots.push(self.spaceship);
            self.last_shot_frame = shot_frame;
        }
    }

    pub fn update(&mut self, frame: usize) {
        let mut partial_score = 0;
        let aliens = &mut self.aliens;
        self.spaceship_shots.retain(|shot| {
            if shot.y == 1 {
                return false;
            }
            let pre_len = aliens.len();
            aliens.retain(|alien| alien != shot);
            let destroyed = aliens.len() != pre_len;
            if destroyed {
                partial_score += 5;
            }
            !destroyed
        });
        self.score += partial_score;

        self.spaceship_shots.iter_mut().for_each(|shot| shot.y -= 1);

        if self.last_aliens_shots + 5 < frame {
            self.last_aliens_shots = frame;
            for alien in &self.aliens {
                let must_shot = frame % 66 == 0;
                let who_shot = alien.x % 6 == 0 && alien.y % 4 == 0;
                if must_shot && who_shot {
                    self.aliens_shots.push(*alien);
                }
            }

            let bottom_shot_limit = self.dimension.y;
            self.aliens_shots.retain(|shot| shot.y < bottom_shot_limit);
            self.aliens_shots.iter_mut().for_each(|shot| shot.y += 1);
        }

        let mut damage = 0;
        let spaceship = &self.spaceship;
        self.aliens_shots.retain(|shot| {
            if shot.y == spaceship.y
                && (shot.x == spaceship.x || shot.x == spaceship.x + 1 || shot.x == spaceship.x - 1)
            {
                damage += 1;
                return false;
            }
            true
        });

        self.aliens.iter().for_each(|alien| {
            if alien.y == spaceship.y
                && (alien.x == spaceship.x
                    || alien.x == spaceship.x + 1
                    || alien.x == spaceship.x - 1)
            {
                damage = 1000;
            }
        });

        self.lives = if damage >= self.lives {
            0
        } else {
            self.lives - damage
        };

        if self.aliens.len() > 0 {
            let left = self.aliens.iter().min_by_key(|alien| alien.x).unwrap();
            let right = self.aliens.iter().max_by_key(|alien| alien.x).unwrap();
            if self.last_aliens_movement + 20 < frame {
                self.last_aliens_movement = frame;

                if left.x == 0 || right.x == self.dimension.x {
                    if self.aliens_movement.1 {
                        self.aliens_movement.0 = -self.aliens_movement.0;
                        let dir = self.aliens_movement.0;
                        self.aliens
                            .iter_mut()
                            .for_each(|alien| alien.x = alien.x + dir);
                        self.aliens_movement.1 = false;
                    } else {
                        self.aliens.iter_mut().for_each(|alien| alien.y += 1);
                        self.aliens_movement.1 = true;
                    }
                } else {
                    let dir = self.aliens_movement.0;
                    self.aliens
                        .iter_mut()
                        .for_each(|alien| alien.x = alien.x + dir);
                }
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut state = GameState::new(Vec2::xy(60, 22));
    let args = Args::parse();
    let output: Output = serde_json::from_str(&args.output).unwrap();
    
    let mut user_output = hex_string_to_vec(&output.inputs);
    let mut actual_frame: Vec<(u16, u8)> = get_and_remove_first_equal_numbers(&mut user_output);
    
    let mut state_output = Output {
        score: 0,
        win: false,
        end_frame: 0,
        inputs: String::new(),
    };

    let mut frame_count = 0;

    while !state.aliens.is_empty() && state.lives > 0 {
        frame_count += 1;

        if actual_frame[0].0 == frame_count as u16 {
            if actual_frame[0].1 == 0 {
                state.spaceship_move_x(-1);
                state.user_input.push((frame_count as u16, 0));
            } else {
                state.spaceship_move_x(1);
                state.user_input.push((frame_count as u16, 1));
            }
            if actual_frame.len() == 2 {
                if actual_frame[1].1 == 0 {
                    state.spaceship_move_x(-1);
                    state.user_input.push((frame_count as u16, 0));
                } else {
                    state.spaceship_move_x(1);
                    state.user_input.push((frame_count as u16, 1));
                }
            }
            if !user_output.is_empty() {
                actual_frame = get_and_remove_first_equal_numbers(&mut user_output);
            }
        }

        state.spaceship_shot(frame_count);
        state.update(frame_count);
    }

    state_output.inputs = vec_to_hex_string(state.user_input);
    state_output.score = state.score;
    state_output.win = state.lives > 0;

    if output.score != state_output.score {
        eprintln!("Error: score doesn't match, {}, {}", output.score, state_output.score);
        std::process::exit(1);
    }
    if output.win != state_output.win {
        eprintln!("Error: win doesn't match, {}, {}", output.win, state_output.win);
        std::process::exit(1);
    }
    if output.end_frame != (frame_count + 1) as u16 {
        eprintln!("Error: end_frame doesn't match, {}, {}", output.end_frame, frame_count);
        std::process::exit(1);
    }
    if output.inputs != state_output.inputs {
        eprintln!("Error: inputs don't match, {}, {}", output.inputs, state_output.inputs);
        std::process::exit(1);
    }

    Ok(())
}

fn vec_to_hex_string(input: Vec<(u16, u8)>) -> String {
    let mut result = String::new();

    for (num, boolean) in input {
        assert!(num <= 4095);
        assert!(boolean <= 1);

        // Primer byte: los primeros 8 bits del u16
        let byte1 = (num >> 4) as u8;

        // Segundo byte: los últimos 4 bits del u16 y el u8 (booleano)
        let byte2 = ((num & 0x0F) << 4) as u8 | (boolean & 0x01);

        // Convertir los bytes a hexadecimal y añadir al string
        result.push_str(&format!("{:02X}{:02X}", byte1, byte2));
    }

    result
}

fn hex_string_to_vec(hex_string: &str) -> Vec<(u16, u8)> {
    let mut result = Vec::new();

    // Asegurarse de que la longitud de la cadena sea par y que tenga pares de caracteres
    if hex_string.len() % 4 != 0 {
        panic!("La longitud de la cadena debe ser múltiplo de 4");
    }

    // Iterar sobre la cadena en chunks de 4 caracteres (2 bytes)
    for chunk in hex_string.as_bytes().chunks(4) {
        // Convertir los dos primeros caracteres en un byte (byte1)
        let byte1_str = std::str::from_utf8(&chunk[0..2]).unwrap();
        let byte1 = u8::from_str_radix(byte1_str, 16).unwrap();

        // Convertir los dos siguientes caracteres en un byte (byte2)
        let byte2_str = std::str::from_utf8(&chunk[2..4]).unwrap();
        let byte2 = u8::from_str_radix(byte2_str, 16).unwrap();

        // Reconstruir el u16 (12 bits) y el booleano u8
        let num: u16 = ((byte1 as u16) << 4) | ((byte2 as u16) >> 4);
        let boolean: u8 = byte2 & 0x01;

        // Añadir el par (u16, u8) al resultado
        result.push((num, boolean));
    }

    result
}

fn get_and_remove_first_equal_numbers(input: &mut Vec<(u16, u8)>) -> Vec<(u16, u8)> {
    let mut result = Vec::new();

    // Comprobar si el vector está vacío
    if input.is_empty() {
        return result; // Devuelve un vector vacío si no hay elementos
    }

    let first_value = input[0].0; // Obtener el primer valor (u16)

    // Iterar sobre el vector y buscar elementos iguales al primer valor
    while let Some(&(num, boolean)) = input.first() {
        if num == first_value {
            result.push((num, boolean)); // Añadir el valor a los resultados
            input.remove(0); // Eliminar el primer elemento del vector
        } else {
            break; // Salir si el valor cambia
        }
    }

    result
}