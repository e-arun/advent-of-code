export const sum = (arr: number[]) => {
	let sum = 0;
	for (const item of arr) {
		sum += item;
	}
	return sum;
};

export const batched = <T>(arr: T[], n: number): T[][] => {
	const batchedArr: T[][] = [];
	for (let i = 0; i + n <= arr.length; i += n) {
		batchedArr.push(arr.slice(i, i + n));
	}
	return batchedArr;
};
