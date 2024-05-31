// SPDX-License-Identifier: MIT
// RelayedMinter Contract v0.0.1
// Creator: Pallet Exchange

pragma solidity ^0.8.13;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import {RelayedMinterInterface} from "./RelayedMinterInterface.sol";
import {IWasmd} from "./precompiles/IWasmd.sol";
import {IJson} from "./precompiles/IJson.sol";
import {IAddr} from "./precompiles/IAddr.sol";

contract MintRelayer is ERC721 {
    address public cwMintContract;
    string public cwMintContractStr;

    RelayedMinterInterface internal _mintingContract;
    IWasmd internal _wasmdPrecompile;
    IJson internal _jsonPrecompile;
    IAddr internal _addrPrecompile;

    string constant private NAME = "Cosmwasm-to-EVM Mint Replayer";
    string constant private SYMBOL = "RLYR";
    address private constant WASMD_PRECOMPILE_ADDRESS = 0x0000000000000000000000000000000000001002;
    address private constant JSON_PRECOMPILE_ADDRESS = 0x0000000000000000000000000000000000001003;
    address private constant ADDR_PRECOMPILE_ADDRESS = 0x0000000000000000000000000000000000001004;

    address private admin;
    mapping(uint256 => uint256) private _indexToMintAttemptId;
    mapping(uint256 => uint256) private _mintAttemptIdToIndex;
    mapping(uint256 => uint256) private _firstMintedTokenId;
    mapping(uint256 => uint256) private _quantityMinted;
    uint256 private _successfulMintCount = 0;
    uint256 private _emittedMintCount = 0;

    error Unauthorized(address caller);
    error AlreadyMintedOnAttempt(uint256 mintAttemptId);
    error CouldNotFetchAttemptId(uint256 mintAttemptId);
    error CouldNotVerifyMinter(address minter);
    error InvalidMintQuantity(uint256 quantity);

    constructor(string memory cwMintContract_, address evmMintContract_, address admin_) ERC721(NAME, SYMBOL) {
        admin = admin_;
        _wasmdPrecompile = IWasmd(WASMD_PRECOMPILE_ADDRESS);
        _jsonPrecompile = IJson(JSON_PRECOMPILE_ADDRESS);
        _addrPrecompile = IAddr(ADDR_PRECOMPILE_ADDRESS);
        cwMintContractStr = cwMintContract_;
        cwMintContract = _addrPrecompile.getEvmAddr(cwMintContract_);
        _mintingContract = RelayedMinterInterface(evmMintContract_);
    }

    function approve(address minter, uint256 mintAttemptId) public override {
        // Ensure caller is the CW-Minter
        if (cwMintContract != msg.sender) {
            revert Unauthorized(msg.sender);
        }
        // Make sure we do not double mint on the same attempt
        if (_mintAttemptIdToIndex[mintAttemptId] == 0) {
            revert AlreadyMintedOnAttempt(mintAttemptId);
        }

        // Validate mint attempt details
        (uint256 attemptId, address storedMinter, uint256 quantity, uint256 funds) = _queryMintAttempt(mintAttemptId);
        if (attemptId != mintAttemptId) {
            revert CouldNotFetchAttemptId(mintAttemptId);
        }
        if (minter != storedMinter) {
            revert CouldNotVerifyMinter(minter);
        }
        if (quantity < 1) {
            revert InvalidMintQuantity(quantity);
        }

        uint256 startingTokenId = _mintingContract.nextTokenIdToMint();
        _mintingContract.relayedMint{value: funds * 1000000000000}(minter, quantity);

        _indexToMintAttemptId[_successfulMintCount] = attemptId;
        _mintAttemptIdToIndex[attemptId] = _successfulMintCount;
        _firstMintedTokenId[_successfulMintCount] = startingTokenId;
        _firstMintedTokenId[_successfulMintCount] = quantity;
        _successfulMintCount++;
    }

    function getMintLog(uint256 index) public view virtual returns (
        uint256 attemptId,
        address minter,
        uint256 quantity,
        uint256 funds
    ) {
        require(index < _successfulMintCount, "Mint log for index does not exist");
        (attemptId, minter, quantity, funds) = _queryMintAttempt(
            _indexToMintAttemptId[index]
        );
    }

    function numSuccessfulMints() public view virtual returns (uint256) {
        return _successfulMintCount;
    }

    function numMintsWithoutEvents() public view virtual returns (uint256) {
        return _successfulMintCount - _emittedMintCount;
    }

    function emitTransfersForMintsWithoutEvents(uint256 limit) public virtual {
        // Only relayer admin can call this
        if (msg.sender != admin) {
            revert Unauthorized(msg.sender);
        }
        uint256 curHigh = _successfulMintCount;
        uint256 curLow = _emittedMintCount;
        require(curLow <= curHigh, "invalid cursors: Mints with events > mints without events");

        uint256 numEventsEmitted = 0;
        while (curHigh > curLow && numEventsEmitted < limit) {
            uint256 firstMintedTokenId = _firstMintedTokenId[curLow];
            uint256 endMintedTokenId = firstMintedTokenId + _quantityMinted[curLow];
            for (uint256 tid = firstMintedTokenId; tid < endMintedTokenId; tid++) {
                _mintingContract.emitMintEventForToken(tid);
            }
            numEventsEmitted++;
            curLow++;
        }
        _emittedMintCount = curLow;
    }

    function _queryMintAttempt(uint256 mintAttemptId) internal virtual view returns (
        uint256 attemptId,
        address minter,
        uint256 quantity,
        uint256 funds
    ) {
        string memory query = string.concat(
            "{\"mint_attempt\":{\"attempt_id\":",
            string.concat(Strings.toString(mintAttemptId), "}}")
        );
        bytes memory response = _wasmdPrecompile.query(cwMintContractStr, bytes(query));
        attemptId = _jsonPrecompile.extractAsUint256(response, "id");
        bytes memory storedMinterBytes = _jsonPrecompile.extractAsBytes(response, "minter");
        minter = _addrPrecompile.getEvmAddr(string(storedMinterBytes));
        quantity = _jsonPrecompile.extractAsUint256(response, "quantity");
        funds = _jsonPrecompile.extractAsUint256(response, "funds");
    }
}
