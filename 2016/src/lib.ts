export const sum = (arr: number[]) => {
	let sum = 0;
	for (const item of arr) {
		sum += item;
	}
	return sum;
};
