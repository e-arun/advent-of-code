import { batched, combinations, sum, windows } from "./lib";

type Item =
	| { kind: "microchip"; element: string }
	| { kind: "generator"; element: string };

type State = { pos: number; floors: Item[][] };

const parseLine = (line: string): Item[] => {
	let words = line.replaceAll(".", "").replaceAll(",", "").split(" ");
	if (words[4] === "nothing") {
		return [];
	}

	const irrelevantWords = new Set(["a", "and"]);
	words = words.slice(4).filter((word) => !irrelevantWords.has(word));

	const items: Item[] = [];
	for (const [a, b] of batched(words, 2)) {
		if (b === "generator") {
			items.push({ kind: "generator", element: a });
		} else {
			items.push({ kind: "microchip", element: a.split("-")[0] });
		}
	}
	return items;
};

const floorIncludes = (floor: Item[], item: Item): boolean => {
	for (const floorItem of floor) {
		if (floorItem.element === item.element && floorItem.kind === item.kind) {
			return true;
		}
	}
	return false;
};

const floorHasGenerator = (floor: Item[]): boolean => {
	for (const item of floor) {
		if (item.kind === "generator") {
			return true;
		}
	}
	return false;
};

const hashState = (state: State): string => {
	const stateArr = [];

	for (const floor of state.floors) {
		let genCount = 0;
		let chipCount = 0;
		for (const item of floor) {
			if (item.kind === "generator") {
				genCount += 1;
			} else {
				chipCount += 1;
			}
		}
		stateArr.push(genCount, chipCount, "floor");
	}
	stateArr.push(state.pos);
	return JSON.stringify(stateArr);
};

const input = await Bun.stdin.text();
const floors = input.trim().split("\n").map(parseLine);
floors[0].push(
	{ kind: "generator", element: "elerium" },
	{ kind: "microchip", element: "elerium" },
	{ kind: "generator", element: "dilithium" },
	{ kind: "microchip", element: "dilithium" },
);
const allItemCount = floors.reduce((sum, floor) => sum + floor.length, 0);

let dist = 0;
let curArr: State[] = [{ floors, pos: 0 }];
let nextArr: State[] = [];
const vis = new Set<string>();

mainLoop: while (curArr.length) {
	curLoop: for (const state of curArr) {
		if (state.floors[3].length === allItemCount) {
			console.log("Ans:", dist);
			break mainLoop;
		}

		// check if no microchips are fried
		for (const floor of state.floors) {
			for (const item of floor) {
				if (item.kind === "generator") {
					continue;
				}
				if (
					floorHasGenerator(floor) &&
					!floorIncludes(floor, { kind: "generator", element: item.element })
				) {
					continue curLoop;
				}
			}
		}

		const stateHash = hashState(state);
		if (vis.has(stateHash)) {
			continue;
		}
		vis.add(stateHash);

		const nextPosArr = [];
		if (state.pos !== 0) {
			nextPosArr.push(state.pos - 1);
		}
		if (state.pos !== 3) {
			nextPosArr.push(state.pos + 1);
		}

		for (const nextPos of nextPosArr) {
			const curFloor = state.floors[state.pos];

			// pick 1 item
			for (const item of curFloor) {
				const newFloors = state.floors.map((arr) => [...arr]);
				newFloors[state.pos] = newFloors[state.pos].filter((x) => x !== item);
				newFloors[nextPos].push(item);
				nextArr.push({ floors: newFloors, pos: nextPos });
			}

			if (curFloor.length < 2) {
				continue;
			}

			// pick 2 items
			for (const [a, b] of combinations(curFloor, 2)) {
				const newFloors = state.floors.map((arr) => [...arr]);
				newFloors[state.pos] = newFloors[state.pos].filter(
					(x) => x !== a && x !== b,
				);
				newFloors[nextPos].push(a);
				newFloors[nextPos].push(b);
				nextArr.push({ floors: newFloors, pos: nextPos });
			}
		}
	}
	curArr = nextArr;
	nextArr = [];
	dist += 1;
	console.log(dist, curArr.length);
}
