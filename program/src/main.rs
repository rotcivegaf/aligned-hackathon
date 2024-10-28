#![no_main]

sp1_zkvm::entrypoint!(main);
use serde::{Serialize, Deserialize};

use game_prove::{game_prove, GameIO};

#[derive(Serialize, Deserialize)]
struct PubInput {
    pub score: u8, // Score of the use(5 for each defeat ship)
    pub win: bool, // If the ship survives
    pub end_frame: u16, // The final frame of the game(max 1023)
    pub inputs: String, // All user keyboard inputs
}

//fn main() -> std::io::Result<()> {
fn main() {
    let zkinput = sp1_zkvm::io::read::<String>();
    let game_i: GameIO = serde_json::from_str(&zkinput).unwrap();

    // commit the score
    sp1_zkvm::io::commit::<PubInput>(&PubInput {
        score: game_i.score,
        win: game_i.win,
        end_frame: game_i.end_frame,
        inputs: game_i.inputs.clone(),
    });

    let game_o = game_prove(&game_i.inputs);

    if game_i.score != game_o.score {
        eprintln!("Error: score doesn't match, {}, {}", game_i.score, game_o.score);
        std::process::exit(1);
    }
    if game_i.win != game_o.win {
        eprintln!("Error: win doesn't match, {}, {}", game_i.win, game_o.win);
        std::process::exit(1);
    }
    if game_i.end_frame != game_o.end_frame {
        eprintln!("Error: end_frame doesn't match, {}, {}", game_i.end_frame, game_o.end_frame);
        std::process::exit(1);
    }
    if game_i.inputs != game_o.inputs {
        eprintln!("Error: inputs don't match, {}, {}", game_i.inputs, game_o.inputs);
        std::process::exit(1);
    }
}