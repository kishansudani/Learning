// SPDX-License-Identifier: UNKNOWN
pragma solidity 0.8.20;

contract FactorialAssembly {
    function factorial(uint n) public pure returns (uint256) {
        uint256 result = 1;
        assembly {
            for {

            } gt(n, 0) {
                n := sub(n, 1)
            } {
                result := mul(result, n)
            }
        }
        return result;
    }
}
