import readline from "node:readline/promises";

(async () => {
  const secretNum = Math.floor(Math.random() * 100);

  while (true) {
    const rl = readline.createInterface({
      input: process.stdin as ReadableStream,
      out: process.stdout as WritableStream,
    });

    const guessStr = await rl.question("Please input your guess.");
    const guess = parseInt(guessStr.trim());

    if (!Number.isInteger(guess)) {
      continue;
    }

    console.log(`You guessed ${guess}`);

    if (guess < secretNum) {
      console.log("Too small!");
    }

    if (guess > secretNum) {
      console.log("Too big!");
    }

    console.log("You win!");
    break;
  }
})();
