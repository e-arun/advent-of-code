export const sum = (arr: number[]) => {
	let sum = 0;
	for (const item of arr) {
		sum += item;
	}
	return sum;
};

export function batched(arr: string, n: number): string[];
export function batched<T>(arr: T[], n: number): T[][];

export function batched<T>(arr: string | T[], n: number) {
	const batchedArr = [];
	for (let i = 0; i + n <= arr.length; i += n) {
		batchedArr.push(arr.slice(i, i + n));
	}
	return batchedArr;
}

export const windows: {
	<T>(arr: T[], n: number): T[][];
	(arr: string, n: number): string[];
	// biome-ignore lint/suspicious/noExplicitAny: required for overladed function
} = <T>(arr: T[] | string, n: number): any[] => {
	const windowArr = [];
	for (let i = 0; i + n <= arr.length; i++) {
		windowArr.push(arr.slice(i, i + n));
	}
	return windowArr;
};

export function combinations<T>(arr: T[], n: number): T[][] {
	if (n > arr.length) {
		throw new Error("n > arr.length");
	}

	const combArr: T[][] = [];

	const idxArr = [...Array(n).keys()];
	combArr.push(idxArr.map((idx) => arr[idx]));

	while (true) {
		let i = n - 1;
		for (; i >= 0; i--) {
			if (idxArr[i] !== i + arr.length - n) {
				break;
			}
		}
		if (i < 0) {
			break;
		}
		idxArr[i] += 1;
		for (let j = i + 1; j < idxArr.length; j++) {
			idxArr[j] = idxArr[j - 1] + 1;
		}
		combArr.push(idxArr.map((idx) => arr[idx]));
	}

	return combArr;
}
