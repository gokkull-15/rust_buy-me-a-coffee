const hre = require("hardhat");

async function main() {
  const CoffeeDonation = await hre.ethers.getContractFactory("CoffeeDonation");
  const coffeeDonation = await CoffeeDonation.deploy();

  await coffeeDonation.deployed();
  console.log("CoffeeDonation deployed to:", coffeeDonation.address);
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});