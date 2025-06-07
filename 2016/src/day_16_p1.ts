import { batched } from "./lib";

const dragon = (a: string): string => {
	const b = a
		.split("")
		.reverse()
		.map((x) => (x === "0" ? "1" : "0"));
	return `${a}0${b.join("")}`;
};

const checksum = (a: string): string => {
	return batched(a, 2)
		.map(([x, y]) => (x === y ? "1" : "0"))
		.join("");
};

let input = await Bun.stdin.text();
input = input.trim();
const n = 272;

let ans = input;
while (ans.length < n) {
	ans = dragon(ans);
}
ans = ans.slice(0, n);

ans = checksum(ans);
while (ans.length % 2 === 0) {
	ans = checksum(ans);
}
console.log(ans);
