import * as lib from "./lib";

type IP = { hyperNet: string[]; superNet: string[] };

const parseIp = (line: string): IP => {
	const segments = line.split(/\[|\]/);
	const hyperNet = segments.filter((_, i) => i % 2 === 1);
	const superNet = segments.filter((_, i) => i % 2 === 0);
	return { hyperNet, superNet };
};

const hasABBA = (x: string): boolean => {
	for (const window of lib.windows(x, 4)) {
		if (
			window[0] === window[3] &&
			window[1] === window[2] &&
			window[0] !== window[1]
		) {
			return true;
		}
	}
	return false;
};

const isValidIp = (ip: IP): boolean => {
	return ip.superNet.some(hasABBA) && !ip.hyperNet.some(hasABBA);
};

const input = await Bun.stdin.text();
const ans = input.trim().split("\n").map(parseIp).filter(isValidIp).length;
console.log(ans);
