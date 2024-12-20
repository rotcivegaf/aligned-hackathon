PROGRAM_FOLDER = program
SCRIPT_FOLDER = script

CONTRACT_ADDRESS=0xDc6c4cA2638b498676924110B511A1e255BD2FC3
RPC_URL=https://ethereum-holesky-rpc.publicnode.com
KEYSTORE_PATH=~/.foundry/keystores/test2

# Build targets
all: build-elf build-script

.PHONY: all build-elf build-script

build-elf:
	@echo "Building ELF file"
	cd $(PROGRAM_FOLDER) && cargo prove build && aligned get-vk-commitment --verification_key_file elf/riscv32im-succinct-zkvm-elf --proving_system SP1 2> elf/commitment

build-script:
	@echo "Building SCRIPT files"
	cd $(SCRIPT_FOLDER) && cargo build --release

space_aligners:
	@cd $(SCRIPT_FOLDER) && \
		cargo build --release && \
		cargo run -r -- \
		--keystore-path $(KEYSTORE_PATH) \
 		--rpc-url $(RPC_URL) \
		--leaderboard-verifier-contract-address $(CONTRACT_ADDRESS)
