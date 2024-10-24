PROGRAM_FOLDER = program
GAME_FOLDER = script
PROVER_DIR = prove

# Build targets
all: build-game build-elf build-prover 

.PHONY: all build-game build-elf build-prover 

build-game:
	@echo "Building end user GAME"
	cd $(GAME_FOLDER) && cargo build --release

play-game:
	@echo "Start user GAME"
	cd $(GAME_FOLDER) && cargo run --release 

build-elf:
	@echo "Building ELF file"
	cd $(PROGRAM_FOLDER) && cargo prove build && aligned get-vk-commitment --verification_key_file elf/riscv32im-succinct-zkvm-elf --proving_system SP1 2> elf/commitment

build-prover:
	@echo "Building PROVER files"
	cd $(PROVER_DIR) && cargo build --bin space_aligners_bin --release

proof:
	@echo "Building PROVE files"
	cd $(PROVER_DIR) && cargo build --bin space_aligners_bin --release && ./target/release/space_aligners_bin --prove --output '{"inputs":"0541058105C1078107C10C810CC10D01118111C1268126C12701418141C147C1480162C1630163417FC18001804180819E819EC1A7C1A801A841B141B181B1C1B880B8C0BAC0BB00BB40BB80","score":100,"win":true,"end_frame":849}'
