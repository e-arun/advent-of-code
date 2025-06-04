const getHash = (key: string): string => {
	const hasher = new Bun.CryptoHasher("md5");
	hasher.update(key);
	return hasher.digest("hex");
};

let input = await Bun.stdin.text();
input = input.trim();

const ans: string[] = [];

for (let i = 0; ; i++) {
	const hash = getHash(input + i);

	if (hash.slice(0, 5) === "00000") {
		ans.push(hash[5]);
	}

	if (ans.length === 8) {
		break;
	}
}
console.log(ans.join(""));
