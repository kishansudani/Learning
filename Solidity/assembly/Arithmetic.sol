// SPDX-License-Identifier: UNKNOWN
pragma solidity 0.8.20;

contract ArithmeticAssembly {
    function addition(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := add(x, y)
        }
    }

    function subtraction(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := sub(x, y)
        }
    }

    function multiplication(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := mul(x, y)
        }
    }

    function division(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := div(x, y)
        }
    }

    function remainder(uint x, uint y) public pure returns (uint result) {
        assembly {
            result := mod(x, y)
        }
    }
}
