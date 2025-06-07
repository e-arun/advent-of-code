const input = await Bun.stdin.text();
const lines = input.trim().split("\n");

const reg = { a: 7, b: 0, c: 0, d: 0 };
type RegKey = keyof typeof reg;
const isRegKey = (key: string): key is RegKey => {
	return key in reg;
};

for (let i = 0; i < lines.length; i++) {
	const [instr, x, y] = lines[i].split(" ");

	if (instr === "cpy") {
		const val = isRegKey(x) ? reg[x] : Number(x);
		if (isRegKey(y)) {
			reg[y] = val;
		}
	} else if (instr === "inc") {
		if (isRegKey(x)) {
			reg[x] += 1;
		}
	} else if (instr === "dec") {
		if (isRegKey(x)) {
			reg[x] -= 1;
		}
	} else if (instr === "jnz") {
		const val = isRegKey(x) ? reg[x] : Number(x);
		if (val !== 0) {
			i += isRegKey(y) ? reg[y] : Number(y);
			i--;
		}
	} else if (instr === "tgl") {
		const val = isRegKey(x) ? reg[x] : Number(x);
		const tglLine = lines[i + val];
		if (tglLine !== undefined) {
			if (tglLine.startsWith("tgl") || tglLine.startsWith("dec")) {
				lines[i + val] = "inc" + tglLine.slice(3);
			} else if (tglLine.startsWith("inc")) {
				lines[i + val] = "dec" + tglLine.slice(3);
			} else if (tglLine.startsWith("cpy")) {
				lines[i + val] = "jnz" + tglLine.slice(3);
			} else if (tglLine.startsWith("jnz")) {
				lines[i + val] = "cpy" + tglLine.slice(3);
			} else {
				throw lines[i];
			}
		}
	} else {
		throw instr;
	}
}

console.log(reg.a);
