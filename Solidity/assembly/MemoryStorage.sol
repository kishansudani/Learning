// SPDX-License-Identifier: MIT
pragma solidity 0.8.20;

contract MemoryAssembly {
    function memoryStorage() external pure returns (uint256 result) {
        assembly {
            let memPtr := mload(0x40) // Load free memory pointer
            mstore(memPtr, 42) // Store 42 at the free memory pointer
            result := mload(memPtr) // Load the value at the free memory pointer
        }
    }
}

contract StorageAssembly {
    uint256 data;
    function setStorage() external {
        assembly {
            sstore(data.slot, 42) // Store newData in the slot of `data`
        }
    }

    function getStorage() external view returns (uint256 result) {
        assembly {
            result := sload(data.slot) // Load the value from the slot of `data`
        }
    }
}
