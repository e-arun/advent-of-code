const input = await Bun.stdin.text().then((x) => Number(x.trim()));

let cur = input;
const arr: number[] = [1];

while (cur !== 1) {
	if (cur % 2 === 1) {
		arr.push(1);
	} else {
		arr.push(0);
	}
	cur = cur >> 1;
}

let ans = 0;
for (const val of arr.reverse()) {
	ans = ans * 2 + val;
}

console.log(ans);
