import Image from "next/image";
import CoffeeButton from "../components/CoffeeButton";

export default function Home() {
  return (
    <main className="min-h-screen flex flex-col items-center justify-center bg-gray-100 p-4">
      <div className="max-w-md w-full bg-white rounded-lg shadow-lg p-6 text-center">
        <Image
          src="/coffee.png"
          alt="Coffee"
          width={100}
          height={100}
          className="mx-auto mb-4"
        />
        <h1 className="text-2xl font-bold mb-2">Buy Me a Coffee</h1>
        <p className="text-gray-600 mb-6">
          Support my work with a small ETH donation! ☕
        </p>
        <div className="space-y-4">
          <CoffeeButton amount={0.01} message="One Coffee ☕" />
          <CoffeeButton amount={0.02} message="Two Coffees ☕☕" />
          <CoffeeButton amount={0.05} message="Big Support! ☕☕☕" />
        </div>
      </div>
    </main>
  );
}