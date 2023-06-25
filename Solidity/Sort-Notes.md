1. Calldata in solidity will let you modify the varibale
2. Memory will let you modify the varibale
3. {transfer, send, call}
    - transfer 
        - payable(msg.sender).transfer(address(this).balance)
        - throw error if it fails
    - send
        - bool success = payable(msg.sender).send(address(this).balance)
        - return boolean even if it fails
    - call
        - (bool sucess, bytes memory dataReturns) = payable(msg.sender).call{value: address(this).balance}("")
        - return boolean even if it fails
        - data returns will always return array
4. constant vs immutable (Both used for gas optimizations)
    - at compile time use constant
    - at a runtime use immutable
5. for gas optimizations use if with revert & error code then using require and store custom error