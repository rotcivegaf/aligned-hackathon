# Space Aligners

## Team Background

I am Rotcivegaf, I studied at the National Technological University of Buenos Aires(UTN), Argentina. Around 2015, I started studying Bitcoin but didn't fully understand it's technology. By the end of 2017, with Ethereum just emerging, I grasped the technology and began studying Solidity (taking courses like [CryptoZombies](https://cryptozombies.io/es/) and [Ethernaut](https://ethernaut.openzeppelin.com/), among others). With this foundation, I started working at RCN(Ripio Credit Network), an ICO for an Argentine company, Ripio, as HSM (Head of Smart Contracts). Then, at the beginning of the pandemic, I became interested in security and dedicated myself to becoming a Smart Contract Auditor.

Currently, I have two roles: I continue working as a independent Smart Contract Auditor in EVM and Rust (Solana and others), and I also participate in hackathons, studying various technologies, such as ZK, which has always intrigued me but until now, I hadn't had the time or the level of knowledge to develop a product in this area.

## Inspiration

When I was a child (around the year 2000), I played web games where prizes were awarded to the top of the leaderboard. I remember striving to reach the winning score, but the top players surpassed me by an incredibly large margin. It didn't take long for me to realize that the scores were hacked and not legitimate.

Some time ago, I watched a few Speedrun videos, like the one for "Super Mario 64," among others. I also saw documentaries, particularly one that showed a fake world record holder captured after 12 years: [Fake Super Mario 64 World Record Caught After 12 Years](https://www.youtube.com/watch?v=dockhgV__pE)

While Speedrun is a niche, it's also difficult to prove the validity of results in gambling, where there is a large audience. Games like "Chicken Cross" are very popular and impossible to verify, as is everything related to online casinos.

## Description(What I built)

With the help of ZK proofs and the Aligned network, I decided to create a game inspired by the classic "Space Invaders," as a test case for ZK-verified games and aligners.

This opens the door to port any type of game to ZK games, where different users can generate a reliable proof of their score.

Space Aligners is just the beginning; any type of game could be developed, both single-player and multiplayer.

## Achievements(What I have achieved so far)

I have successfully developed the Space Invaders game that runs in the terminal using Rust. For this, I created a script that serves as the entry point for the user, reading the user's wallet, depositing ether in the Aligned Layer network, starting the game, and generating the proof from its outputs. The proof is verified and sent to the Aligned Layer network; finally, on the Ethereum side, a transaction is sent to the verifier contract, where the game result is verified and an NFT is minted with the game score.

To make the proof possible, I had to:
- Set the maximum frame to 1024, which is a u16 (~34 seconds).
- Remove randomness from enemy ship shots.
- The user-controlled ship has only one life.
- Limit the number of enemies to 20.
- The ship fires automatically.
- Make some minor adjustments.

These changes make it possible to process the proof on a standard computer.

## Deployment and execution instructions

### Install dependencies

From [ruscii repo:](https://github.com/lemunozm/ruscii?tab=readme-ov-file#linux)

```bash
sudo apt install libx11-dev

export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig:$PKG_CONFIG_PATH && export PKG_CONFIG_PATH=/usr/share/pkgconfig:$PKG_CONFIG_PATH
```

[SP1 install](https://docs.succinct.xyz/getting-started/install.html) (I use the Option 2: Building from Source)

### Clone repo and Build

```bash
git clone git@github.com:rotcivegaf/aligned-hackathon.git
cd aligned-hackathon
make all
```

### Play Space Aligners Game

Change the [`KEYSTORE_PATH`](https://github.com/rotcivegaf/aligned-hackathon/blob/bdf497f78249c016a8fb915b5b9a0d229254047a/Makefile#L6) of the Makefile on root folder

```bash
make space_aligners
```

## Project structure

```css
aligned-hackathon/
├── contracts/
│   ├── src/
│   │   └── LeaderBoardVerifierContract.sol // The verifier contract
│   └── ...
├── game_prove/
│   ├── src/
│   │   └── game_prove.rs // The base game prover without graphics(used in program and script)
│   └── ...
├── program/
│   ├── elf/
│   │   └── commitment // To cache the commitment hash
│   │   └── riscv32im-succinct-zkvm-elf // ELF file
│   ├── src/
│   │   └── main.rs // Used to create the ELF file to prove the game
│   └── ...
├── script/
│   ├── src/
│   │   ├── game.rs // Game file
│   │   └── main.rs // Used to play the game, create/submit the proof and mint the leaderboard NFT
│   └── ...
└── Makefile // To send the commands build and play
```

## Contracts and transactions

- LeaderBoardVerifierContract: [0xdc6c4ca2638b498676924110b511a1e255bd2fc3](https://holesky.etherscan.io/address/0xdc6c4ca2638b498676924110b511a1e255bd2fc3)

Lose, Score: 20, End Frame: 289, transaction:
- Aligned Layer Submitted Proof.: [0x9547...64cd](https://explorer.alignedlayer.com/batches/0x9547c86df4b89356f4fefad6f23559504364bcd994189a6e6fd39d817a2364cd)
- Mint Leaderboard NTF and Verify Inclusion [0xe0db...1bdc](https://holesky.etherscan.io/tx/0xe0db250b82e3834004b3cb00ce0ac521edc74b2345326c07445e7134bbb81bdc)
- NFT: [11504478815665675982291088304872436013016560641840469854149660576072638918263](https://holesky.etherscan.io/token/0xdc6c4ca2638b498676924110b511a1e255bd2fc3?a=11504478815665675982291088304872436013016560641840469854149660576072638918263)

Win, Score: 100, End Frame: 833, transaction:
- Aligned Layer Submitted Proof.: [0x4f62...45dc](https://explorer.alignedlayer.com/batches/0x4f62d2ec76a27d01edc307c473113a1b3fe772292a94c7d545c1b9db12a345dc)
- Mint Leaderboard NTF and Verify Inclusion [0x48c3...74d4](https://holesky.etherscan.io/tx/0x48c3cdc51432369d314a9c83a4703d789933c652663e5bd9d7d6502c4a5874d4)
- NFT: [41460959426820802061370219630270451824079970532205278481933466754574501234463](https://holesky.etherscan.io/token/0xdc6c4ca2638b498676924110b511a1e255bd2fc3?a=41460959426820802061370219630270451824079970532205278481933466754574501234463)

## An overview of the challenges faced during development and key design considerations

I have faced several challenges; I think the first one was reading the documentation and truly understanding how ZK and Aligned work.

I’m not an expert in Rust, so that was another issue.

Regarding the Aligned documentation, I found it very difficult to install SP1 until I came across the documentation for SP1. Using the method from option 2, I was able to install it; perhaps they should recommend this solution.

I noticed that they recently updated the example repository from [yetanotherco/aligned_layer](https://github.com/yetanotherco/aligned_layer). I didn’t have time to look at the new version, but I found it challenging to understand the quiz example and how to send public inputs to the verifier contract.

## What next(What else needs to be added to make the project production-ready) and project roadmap

This is just the beginning; it’s a good start for an MVP. Here are some things I would like to add to this game in particular:

- Further compact the user movement logs, which would enable:
    - Extending the game time (beyond just 34 seconds).
    - Increasing the number of enemies.
    - Making the ship’s shooting controllable.
- Create a leaderboard by reading events from the verifier contract and publishing them on a frontend.
- Port the game to a frontend.

Once this is accomplished, I aim to port more games to my product, such as Tetris, Pac-Man, gambling games, etc. An interesting example would be porting Doom.
