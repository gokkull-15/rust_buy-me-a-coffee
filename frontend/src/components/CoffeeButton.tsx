"use client";

import { useState } from "react";
import { ethers } from "ethers";

interface CoffeeButtonProps {
  amount: number;
  message: string;
}

const CONTRACT_ADDRESS = process.env.NEXT_PUBLIC_CONTRACT_ADDRESS || "";
const ABI = [
  {
    inputs: [{ internalType: "string", name: "message", type: "string" }],
    name: "donate",
    outputs: [],
    stateMutability: "payable",
    type: "function",
  },
];

export default function CoffeeButton({ amount, message }: CoffeeButtonProps) {
  const [loading, setLoading] = useState(false);

  const handleDonate = async () => {
    setLoading(true);
    try {
      if (!window.ethereum) {
        alert("MetaMask is required");
        return;
      }

      const provider = new ethers.BrowserProvider(window.ethereum);
      const signer = await provider.getSigner();
      const contract = new ethers.Contract(CONTRACT_ADDRESS, ABI, signer);

      const tx = await contract.donate(message, {
        value: ethers.parseEther(amount.toString()),
      });
      await tx.wait();

      window.location.href = "/success";
    } catch (error) {
      console.error("Error:", error);
      alert("Donation failed");
    } finally {
      setLoading(false);
    }
  };

  return (
    <button
      onClick={handleDonate}
      disabled={loading}
      className="w-full bg-yellow-500 text-white py-2 px-4 rounded hover:bg-yellow-600 disabled:opacity-50"
    >
      {loading ? "Processing..." : message}
    </button>
  );
}