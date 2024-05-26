// SPDX-License-Identifier: MIT
// RelayedMinter Contract v0.0.1
// Creator: Pallet Exchange

pragma solidity ^0.8.13;

/**
 * @dev Interface of a RelayedMinter compliant contract.
 */
interface RelayedMinter {
    /**
     * @dev Mints and transfers a token to `_recipient`.
     *
     * Requirements:
     *
     * - The `_recipient` must be approved to mint.
     *
     */
    function relayedMint(address recipient, uint256 quantity) external payable;
}
