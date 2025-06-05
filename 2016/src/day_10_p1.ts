type Instr =
	| { kind: "value"; value: number; bot: number }
	| { kind: "bot"; bot: number; low: number; high: number };

const parseLine = (line: string): Instr => {
	const words = line.split(" ");
	if (words[0] === "value") {
		return { kind: "value", value: Number(words[1]), bot: Number(words[5]) };
	}

	let low = Number(words[6]);
	if (words[5] === "output") {
		low = -low;
	}

	let high = Number(words[11]);
	if (words[10] === "output") {
		high = -high;
	}

	return {
		kind: "bot",
		bot: Number(words[1]),
		low,
		high,
	};
};

const input = await Bun.stdin.text();
const instrArr = input.trim().split("\n").map(parseLine);
const botChips = new Map<number, number[]>();

const addChips = (bot: number, chip: number) => {
	const curChips = botChips.get(bot) ?? [];
	if (curChips.length > 1) {
		throw "Too many chips to hold";
	}
	curChips.push(chip);
	botChips.set(bot, curChips);
};

for (const instr of instrArr) {
	if (instr.kind === "value") {
		addChips(instr.bot, instr.value);
	}
}

mainLoop: while (true) {
	for (const instr of instrArr) {
		if (instr.kind === "value") {
			continue;
		}
		const curChips = botChips.get(instr.bot) ?? [];
		if (curChips.length !== 2) {
			continue;
		}
		curChips.sort((a, b) => a - b);
		if (curChips[0] === 17 && curChips[1] === 61) {
			console.log(instr.bot);
			break mainLoop;
		}

		if (curChips[0] > curChips[1]) {
			throw curChips;
		}
		addChips(instr.low, curChips[0]);
		addChips(instr.high, curChips[1]);
		botChips.set(instr.bot, []);
	}
}
