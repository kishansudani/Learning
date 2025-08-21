// SPDX-License-Identifier: UNKNOWN
pragma solidity 0.8.20;

contract ConditionalAssembly {
    function ifStatement(uint x, uint y) public pure returns (uint result) {
        assembly {
            if eq(x, y) {
                result := 0
            }

            if gt(x, y) {
                result := x
            }

            if lt(x, y) {
                result := y
            }
        }
    }
}
