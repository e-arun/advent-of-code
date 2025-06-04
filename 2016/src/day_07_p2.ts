import * as lib from "./lib";

type IP = { hyperNet: string[]; superNet: string[] };

const parseIp = (line: string): IP => {
	const segments = line.split(/\[|\]/);
	const hyperNet = segments.filter((_, i) => i % 2 === 1);
	const superNet = segments.filter((_, i) => i % 2 === 0);
	return { hyperNet, superNet };
};

const isValidIp = (ip: IP): boolean => {
	const babSet = new Set<string>();

	for (const x of ip.superNet) {
		for (const window of lib.windows(x, 3)) {
			if (window[0] !== window[1] && window[0] === window[2]) {
				babSet.add([window[1], window[0], window[1]].join(""));
			}
		}
	}

	for (const x of ip.hyperNet) {
		for (const window of lib.windows(x, 3)) {
			if (babSet.has(window)) {
				return true;
			}
		}
	}
	return false;
};

const input = await Bun.stdin.text();
const ans = input.trim().split("\n").map(parseIp).filter(isValidIp).length;
console.log(ans);
