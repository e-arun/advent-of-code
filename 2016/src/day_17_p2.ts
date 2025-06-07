import { md5 } from "./lib";

const input = await Bun.stdin.text().then((x) => x.trim());
type State = { path: string; r: number; c: number };

let curArr: State[] = [{ path: "", r: 0, c: 0 }];
let nextArr: State[] = [];
const vis = new Set<string>();
let ans = 0;

while (curArr.length) {
	for (const state of curArr) {
		const stateHash = JSON.stringify(state);
		if (vis.has(stateHash)) {
			continue;
		}
		vis.add(stateHash);

		if (state.r === 3 && state.c === 3) {
			if (state.path.length > ans) {
				ans = state.path.length;
			}
			continue;
		}

		const hash = md5(input + state.path);

		if (hash[0] >= "b" && state.r !== 0) {
			nextArr.push({ path: state.path + "U", r: state.r - 1, c: state.c });
		}
		if (hash[1] >= "b" && state.r !== 3) {
			nextArr.push({ path: state.path + "D", r: state.r + 1, c: state.c });
		}
		if (hash[2] >= "b" && state.c !== 0) {
			nextArr.push({ path: state.path + "L", r: state.r, c: state.c - 1 });
		}
		if (hash[3] >= "b" && state.c !== 3) {
			nextArr.push({ path: state.path + "R", r: state.r, c: state.c + 1 });
		}
	}
	curArr = nextArr;
	nextArr = [];
}
console.log(ans);
