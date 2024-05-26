// SPDX-License-Identifier: MIT
// RelayedMinter Contract v0.0.1
// Creator: Pallet Exchange

pragma solidity ^0.8.13;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import {RelayedMinter} from "./RelayedMinter.sol";
import {IWasmd} from "./precompiles/IWasmd.sol";
import {IJson} from "./precompiles/IJson.sol";
import {IAddr} from "./precompiles/IAddr.sol";

contract MintRelayer is ERC721 {
    address public cwMintContract;

    string public cwMintContractStr;

    RelayedMinter internal _mintingContract;

    IWasmd internal _wasmdPrecompile;

    IJson internal _jsonPrecompile;

    IAddr internal _addrPrecompile;

    string constant private NAME = "Cosmwasm-to-EVM Mint Replayer";

    string constant private SYMBOL = "RLYR";

    address private constant WASMD_PRECOMPILE_ADDRESS = 0x0000000000000000000000000000000000001002;

    address private constant JSON_PRECOMPILE_ADDRESS = 0x0000000000000000000000000000000000001003;

    address private constant ADDR_PRECOMPILE_ADDRESS = 0x0000000000000000000000000000000000001004;

    constructor(string memory cwMintContract_, address evmMintContract_) ERC721(NAME, SYMBOL) {
        _wasmdPrecompile = IWasmd(WASMD_PRECOMPILE_ADDRESS);
        _jsonPrecompile = IJson(JSON_PRECOMPILE_ADDRESS);
        _addrPrecompile = IAddr(ADDR_PRECOMPILE_ADDRESS);

        cwMintContractStr = cwMintContract_;
        cwMintContract = _addrPrecompile.getEvmAddr(cwMintContract_);
        _mintingContract = RelayedMinter(evmMintContract_);
    }

    function approve(address to, uint256 quantity) public override {
        // // TODO: check that `to` is allowed to mint
        // string memory caller = _addrPrecompile.getSeiAddr(tx.origin);
        // string msg = string.concat(
        //     "{\"is_allowed\":{\"minter\":\"",
        //     string.concat(caller, "\"}}")
        // );
        // bytes memory response = _wasmdPrecompile.query(cwMintContractStr, bytes(msg));

        _mintingContract.relayedMint(to, quantity);
    }
}
