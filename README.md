# Space Aligners

## Steps

### Install dependencies

From [ruscii repo:](https://github.com/lemunozm/ruscii?tab=readme-ov-file#linux)

```bash
sudo apt install libx11-dev

export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:$PKG_CONFIG_PATH && export PKG_CONFIG_PATH=/usr/share/pkgconfig:$PKG_CONFIG_PATH
```

### Build

```bash
git clone git@github.com:rotcivegaf/aligned-hackathon.git
cd aligned-hackathon
make all
```

### Play Space Aligners Game

```bash
make play-game
```

Once the game is finished on the console, the output of the game will be emitted, which will then be the input for the prover
```rust
struct Output {
    inputs: String, // All user keyboard inputs
    score: u8, // Score of the use(5 for each defeat ship)
    win: bool, // If the ship survives
    end_frame: u16, // The final frame of the game(max 1023)
}
```

### Run the prover and make the proof

In the prove folder: `cd prove`

With the output of the game we can run the prover:
```bash
cargo build --bin space_aligners_bin --release && \
./target/release/space_aligners_bin --prove --output '<YOUR_OUTPUT_GAME>'
```

WIN game example:
```bash
cargo build --bin space_aligners_bin --release && \
./target/release/space_aligners_bin --prove --output '{"inputs":"0541058105C1078107C10C810CC10D01118111C1268126C12701418141C147C1480162C1630163417FC18001804180819E819EC1A7C1A801A841B141B181B1C1B880B8C0BAC0BB00BB40BB80","score":100,"win":true,"end_frame":849}'
```

LOSE game example:
```bash
cargo build --bin space_aligners_bin --release && \
./target/release/space_aligners_bin --prove --output '{"inputs":"038103C1078107C115011541184118811F001F40","score":25,"win":false,"end_frame":157}'
```

### Submit the proof in Aligned Layer Network

```bash
aligned submit \
    --proving_system SP1 \
    --proof prove/proof.bin \
    --vm_program program/elf/riscv32im-succinct-zkvm-elf \
    --aligned_verification_data_path ./aligned_verification_data \
    --batcher_url wss://batcher.alignedlayer.com \
    --network holesky \
    --rpc_url https://ethereum-holesky-rpc.publicnode.com \
    --keystore_path <YOUR_KEYSTORE_PATH>
```