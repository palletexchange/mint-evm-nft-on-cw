// SPDX-License-Identifier: MIT
// RelayedMinter Contract v0.0.1
// Creator: Pallet Exchange

pragma solidity ^0.8.13;

/**
 * @dev Interface of a RelayedMinter contract, for helping Cosmwasm users to
 * mint on EVM-deployed contracts.
 */
interface RelayedMinterInterface {
    /**
     * @dev Configures the allowed relayer address that can call invoke this.
     * contract's functions.
     *
     * Requirements:
     *
     * - Can only be set by an admin.
     *
     */
    function setAllowedRelayer(address relayer) external;

    /**
     * @dev Mints and transfers `quantity` of tokens to `recipient`.
     *
     * Important information to note:
     *
     * 1. Since this function is called by a Cosmwasm minting relayer on Sei,
     *    you should not expect there to be any events emitted. Cosmwasm -> EVM
     *    execution does not generate transaction hashes or emit EVM events.
     * 2. `tx.origin` will be set to the associated EVM address for the
     *    Cosmwasm minting contract that gets called by the user. You will not
     *    be able to access the signing user's wallet address through either
     *    `tx` or `msg` objects.
     *
     * Requirements:
     *
     * - Must only permit allowed relayer to call this function. See `setAllowedRelayer`.
     * - `recipient` must be approved to mint.
     * - `quantity` must be a valid quantity to mint.
     * - `msg.value` must be the correct amount to mint.
     *
     */
    function relayedMint(address recipient, uint256 quantity) external payable;

    /**
     * @dev Emits a mint Transfer event for a specific minted token.
     *
     * We need to do this because minting invocation from Cosmwasm will not
     * emit Transfer events. The initial Transfer event from the null address
     * is required for EVM indexers and marketplaces to track tokens in a
     * collection.
     *
     * As such, to emit a mint Transfer event, the relayer will need to call
     * this function in a separate transaction from the EVM execution layer.
     * This way, events will be emitted.
     *
     * Requirements:
     *
     * - Must only permit allowed relayer to call this function. See `setAllowedRelayer`.
     * - Transfer events emitted must match what would ordinarily be emitted
     *   through any other minting function. This means the `from` must be
     *   marked as `address(0)`.
     *
     * Emits a {Transfer} event.
     */
    function emitMintEventForToken(uint256 tokenId) external;

    /**
     * @dev Returns the next token id to mint. We assume that token ids to mint
     * are incremented sequentially.
     *
     */
    function nextTokenIdToMint() external view returns (uint256);
}
