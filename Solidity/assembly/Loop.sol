// SPDX-License-Identifier: UNKNOWN
pragma solidity 0.8.20;

contract LoopAssembly {
    function sum(uint n) public pure returns (uint256 result) {
        assembly {
            let i := 0
            for {

            } lt(i, n) {
                i := add(i, 1)
            } {
                result := add(result, i)
            }
        }
    }
}
