// SPDX-License-Identifier: UNKNOWN
pragma solidity 0.8.20;

contract LogicalAssembly {
    function andOperation(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := and(x, y)
        }
    }

    function orOperation(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := or(x, y)
        }
    }

    function xorOperation(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := xor(x, y)
        }
    }

    function notOperation(uint x) public pure returns (uint result) {
        assembly {
            result := not(x)
        }
    }
}
