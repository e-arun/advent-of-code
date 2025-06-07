import { windows } from "./lib";

const getHash = (key: string): string => {
	const hasher = new Bun.CryptoHasher("md5");
	hasher.update(key);
	return hasher.digest("hex");
};

const getRepeating = (hash: string, n: number): string | null => {
	mainLoop: for (const window of windows(hash, n)) {
		for (let i = 1; i < window.length; i++) {
			if (window[0] !== window[i]) {
				continue mainLoop;
			}
		}
		return window[0];
	}
	return null;
};

let input = await Bun.stdin.text();
input = input.trim();

const keyArr: number[] = [];
const repMap = new Map<string, number[]>();

let i = -1;

while (keyArr.length < 64 || keyArr[63] + 1000 > i) {
	i += 1;

	const hash = getHash(input + i);
	const rep = getRepeating(hash, 3);
	if (!rep) {
		continue;
	}

	const repArr = repMap.get(rep) ?? [];
	repMap.set(rep, repArr);
	repArr.push(i);

	const rep5 = getRepeating(hash, 5);
	if (!rep5) {
		continue;
	}

	const oldRepArr = repMap.get(rep5) ?? [];
	for (let j = oldRepArr.length - 1; oldRepArr[j] >= i - 1000; j--) {
		if (oldRepArr[j] === i) {
			continue;
		}
		if (!keyArr.includes(oldRepArr[j])) {
			keyArr.push(oldRepArr[j]);
		}
	}
	keyArr.sort((a, b) => a - b);
}

console.log(keyArr[63]);
