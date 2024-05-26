// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {MintRelayer} from "../src/MintRelayer.sol";

contract MintRelayerTest is Test {
    MintRelayer public relayer;

    function setUp() public {
        // relayer = new MintRelayer("sei1", 0x0000000000000000000000000000000000000000);
    }

    // function test_Increment() public {
    //     counter.increment();
    //     assertEq(counter.number(), 1);
    // }

    // function testFuzz_SetNumber(uint256 x) public {
    //     counter.setNumber(x);
    //     assertEq(counter.number(), x);
    // }
}
