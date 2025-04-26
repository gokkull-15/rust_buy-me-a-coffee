// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

contract CoffeeDonation {
    struct Donation {
        address donor;
        uint256 amount;
        string message;
        uint256 timestamp;
    }

    address payable public owner;
    Donation[] public donations;

    event Donated(address indexed donor, uint256 amount, string message, uint256 timestamp);

    constructor() {
        owner = payable(msg.sender);
    }

    function donate(string memory message) public payable {
        require(msg.value > 0, "Donation must be greater than 0");
        donations.push(Donation({
            donor: msg.sender,
            amount: msg.value,
            message: message,
            timestamp: block.timestamp
        }));
        emit Donated(msg.sender, msg.value, message, block.timestamp);
    }

    function getDonations() public view returns (Donation[] memory) {
        return donations;
    }

    function withdraw() public {
        require(msg.sender == owner, "Only owner can withdraw");
        owner.transfer(address(this).balance);
    }
}