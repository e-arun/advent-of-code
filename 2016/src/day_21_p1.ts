const performOp = (pass: string, line: string): string => {
	const words = line.split(" ");
	let passArr = pass.split("");

	if (line.startsWith("swap position")) {
		const x = Number(words[2]);
		const y = Number(words[5]);
		const tmp = passArr[x];
		passArr[x] = passArr[y];
		passArr[y] = tmp;
	} else if (line.startsWith("swap letter")) {
		const x = words[2];
		const y = words[5];
		passArr = passArr.map((ch) => {
			if (ch === x) {
				return y;
			}
			if (ch === y) {
				return x;
			}
			return ch;
		});
	} else if (line.startsWith("rotate left")) {
		const x = Number(words[2]) % pass.length;
		passArr.push(...passArr.slice(0, x));
		passArr = passArr.slice(x);
	} else if (line.startsWith("rotate right")) {
		const x = Number(words[2]) % pass.length;
		passArr.unshift(...passArr.slice(pass.length - x));
		passArr = passArr.slice(0, pass.length);
	} else if (line.startsWith("rotate based")) {
		const ch = words[6];
		let x = passArr.indexOf(ch);
		if (x >= 4) {
			x += 2;
		} else {
			x += 1;
		}
		x = x % pass.length;
		passArr.unshift(...passArr.slice(pass.length - x));
		passArr = passArr.slice(0, pass.length);
	} else if (line.startsWith("reverse positions")) {
		const x = Number(words[2]);
		const y = Number(words[4]);

		const n = y - x + 1;
		const arr = passArr.splice(x, n);
		passArr.splice(x, 0, ...arr.reverse());
	} else if (line.startsWith("move position")) {
		const x = Number(words[2]);
		const y = Number(words[5]);
		const [ch] = passArr.splice(x, 1);
		passArr.splice(y, 0, ch);
	} else {
		throw line;
	}

	return passArr.join("");
};

const input = await Bun.stdin.text();
const lines = input.trim().split("\n");

let pass = "abcdefgh";

for (const line of lines) {
	pass = performOp(pass, line);
}
console.log(pass);
