const decompressLen = (input: string): number => {
	let ans = 0;

	for (let i = 0; i < input.length; ) {
		if (input[i] !== "(") {
			ans += 1;
			i += 1;
		} else {
			const nextI = input.indexOf(")", i);
			if (nextI === -1) {
				throw new Error("Failed to find closing parenthesis");
			}
			const [a, b] = input
				.slice(i + 1, nextI)
				.split("x")
				.map(Number);

			if (i + a >= input.length) {
				throw new Error("Not enough string to decompress");
			}
			ans += decompressLen(input.slice(nextI + 1, nextI + a + 1)) * b;
			i = nextI + a + 1;
		}
	}

	return ans;
};

let input = await Bun.stdin.text();
input = input.trim();
console.log(decompressLen(input));
