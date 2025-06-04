const getHash = (key: string): string => {
	const hasher = new Bun.CryptoHasher("md5");
	hasher.update(key);
	return hasher.digest("hex");
};

let input = await Bun.stdin.text();
input = input.trim();

const ans: string[] = [];
let filled = 0;

for (let i = 0; ; i++) {
	const hash = getHash(input + i);

	if (hash.slice(0, 5) === "00000") {
		const idx = Number(hash[5]);
		if (Number.isNaN(idx) || idx < 0 || idx >= 8) {
			continue;
		}
		if (ans[idx] === undefined) {
			ans[idx] = hash[6];
			filled += 1;
		}
	}

	if (filled === 8) {
		break;
	}
}
console.log(ans.join(""));
