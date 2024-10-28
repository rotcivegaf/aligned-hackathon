// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {ERC721} from "solmate/src/tokens/ERC721.sol";

contract LeaderBoardVerifierContract is ERC721 {
    struct Score {
        uint256 timestamp;
        uint256 score;
        bool win;
        uint256 endFrame;
    }

    address public alignedServiceManager = 0x58F280BeBE9B34c9939C3C39e0890C81f163B623;
    address public paymentServiceAddr = 0x815aeCA64a974297942D2Bbf034ABEe22a38A003;

    bytes32 public elfCommitment = 0x7392ef74250ef5d90135ef96573db00bba367fd236184310f519e59ad33e42b9;

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
        bytes memory pubInput
    ) external returns (uint256) {
        if (elfCommitment != provingSystemAuxDataCommitment) {
            revert InvalidElf(provingSystemAuxDataCommitment);
        }

        require(
            pubInputCommitment == keccak256(abi.encodePacked(pubInput)),
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
            bool callWasSuccessful,
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

        require(callWasSuccessful, "static_call failed");

        bool proofIsIncludedBool = abi.decode(proofIsIncluded, (bool));
        require(proofIsIncludedBool, "proof not included in batch");

        mintedProofs[fullHash] = true;

        uint256 tokenId = uint256(fullHash);
        _mint(msg.sender, tokenId);
        (uint256 score, bool win, uint256 endFrame) =
            abi.decode(pubInput, (uint256, bool, uint256));
        leaderboard[tokenId] = Score(
            block.timestamp,
            score,
            win,
            endFrame
        );

        return tokenId;
    }

    function tokenURI(
        uint256 tokenId
    ) public view virtual override returns (string memory) {
        return "ipfs://TODO";
    }
}
