// SPDX-License-Identifier: MIT

pragma solidity 0.8.19;

import { AggregatorV3Interface } from "@chainlink/contracts/src/v0.8/interfaces/AggregatorV3Interface.sol";

error NotOwner();

contract Price {
    uint minimumAmountInUSD = 5e18;
    address _owner;

    constructor() {
        _owner = msg.sender;
    }

    modifier onlyOwner() {
        // require(msg.sender == _owner, "Caller must be owner");
        if (msg.sender != _owner) {
            revert NotOwner();
        }
        _;
    }

    function sendETH() public payable {
        require(getConverstionRate(msg.sender) >= minimumAmountInUSD, "You sent less amount of eth");
    }

    function withdrawETH() external onlyOwner {
        (bool success, ) = payable(_owner).call{value: address(this).balance}("");
        require(success, "Sending ETH failed");
    }

    function getPrice() view public returns (uint256) {
        AggregatorV3Interface priceFeed = AggregatorV3Interface(0x694AA1769357215DE4FAC081bf1f309aDC325306);
        (,int256 price,,,) = priceFeed.latestRoundData;
        return uint256(price) * 1e10;
    }

    function getConverstionRate(uint256 ethAmount) public view returns(uint256) {
        return (ethAmount * getPrice()) / 1e18;
    }

    function getOwner() public view returns(address) {
        return _owner;
    }

    fallback() external payable {}

    receive() external payable {
        sendETH();
    }
}