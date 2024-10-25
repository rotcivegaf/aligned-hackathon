// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {ERC721} from "solmate/src/tokens/ERC721.sol";

contract LeaderBoardVerifierContract is ERC721 {
    struct Score {
        uint256 score;
        bool win;
        uint256 endFrame;
        bytes inputs;
    }

    address public alignedServiceManager = 0x58F280BeBE9B34c9939C3C39e0890C81f163B623;
    address public paymentServiceAddr = 0x815aeCA64a974297942D2Bbf034ABEe22a38A003;

    bytes32 public elfCommitment = 0xd9cd413ee8f1e0e50f40e4a213a5d548700f8d568d61d404e6e63db3e6f5f3e5;
        
    error InvalidElf(bytes32 submittedElf); // c6d95066

    // map to check if proof has already been submitted
    mapping(bytes32 => bool) public mintedProofs;

    mapping(uint256 => Score) public leaderboard;

    constructor() ERC721("Space Aligners", "SA") {}

    function verifyBatchInclusion(
        bytes32 proofCommitment,
        bytes32 pubInputCommitment,
        bytes32 provingSystemAuxDataCommitment,
        bytes20 proofGeneratorAddr,
        bytes32 batchMerkleRoot,
        bytes memory merkleProof,
        uint256 verificationDataBatchIndex,
        // pubInputs
        uint256 score,
        bool win,
        uint256 endFrame,
        bytes memory inputs
    ) external returns (uint256) {
        if (elfCommitment != provingSystemAuxDataCommitment) {
            revert InvalidElf(provingSystemAuxDataCommitment);
        }
        
        require(
            pubInputCommitment == keccak256(abi.encodePacked(score, win, endFrame, inputs)),
            "public inputs don't match"
        );

        require(
            address(proofGeneratorAddr) == msg.sender,
            "proofGeneratorAddr does not match"
        );

        bytes32 fullHash = keccak256(
            abi.encodePacked(
                proofCommitment,
                pubInputCommitment,
                provingSystemAuxDataCommitment,
                proofGeneratorAddr
            )
        );
        require(!mintedProofs[fullHash], "proof already minted");

        (
            bool callWasSuccessfull,
            bytes memory proofIsIncluded
        ) = alignedServiceManager.staticcall(
                abi.encodeWithSignature(
                    "verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256,address)",
                    proofCommitment,
                    pubInputCommitment,
                    provingSystemAuxDataCommitment,
                    proofGeneratorAddr,
                    batchMerkleRoot,
                    merkleProof,
                    verificationDataBatchIndex,
                    paymentServiceAddr
                )
            );

        require(callWasSuccessfull, "static_call failed");

        bool proofIsIncludedBool = abi.decode(proofIsIncluded, (bool));
        require(proofIsIncludedBool, "proof not included in batch");

        mintedProofs[fullHash] = true;

        uint256 tokenId = uint256(fullHash);
        _mint(msg.sender, tokenId);
        leaderboard[tokenId] = Score(
            score, 
            win, 
            endFrame,
            inputs 
        );

        return tokenId;
    }

    function tokenURI(
        uint256 tokenId
    ) public view virtual override returns (string memory) {
        return "ipfs://";
    }
}
