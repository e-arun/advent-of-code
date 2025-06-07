const getNextRow = (row: string): string => {
	const newRow: string[] = [];

	for (let i = 0; i < row.length; i++) {
		const a = row[i - 1] ?? ".";
		const b = row[i];
		const c = row[i + 1] ?? ".";
		const section = [a, b, c].join("");

		if (["^^.", ".^^", "^..", "..^"].includes(section)) {
			newRow.push("^");
		} else {
			newRow.push(".");
		}
	}

	return newRow.join("");
};

const input = await Bun.stdin.text().then((x) => x.trim());
const rows = 400000;

const mat = [input];
for (let i = 1; i < rows; i++) {
	mat.push(getNextRow(mat[i - 1]));
}

let ans = 0;
for (const row of mat) {
	for (const chr of row) {
		if (chr === ".") {
			ans += 1;
		}
	}
}
console.log(ans);
