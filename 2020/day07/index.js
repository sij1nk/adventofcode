const fs = require("fs");

const dict = {};
fs.readFileSync("input_transformed.txt", "utf-8")
	.split("\n")
	.forEach((line) => {
		const fields = line.split(",");
		if (fields[1] != "") {
			dict[fields[0]] = fields.slice(1);
		}
	});

// Part 1
const get_keys = (values) => {
	let keys = [];
	for (let key of Object.keys(dict)) {
		for (let value of values) {
			if (dict[key].filter((v) => v.match(value)).length) {
				keys.push(key);
			}
		}
	}
	return keys;
};

const part1 = () => {
	let values = ["shinygold"];
	let keys;
	let allKeys = [];

	do {
		keys = get_keys(values);
		for (key of keys) {
			!allKeys.includes(key) && allKeys.push(key);
		}

		values = keys;
	} while (keys.length);

	console.log(allKeys.length);
};
part1();

// Part 2
const get_count = (keys) => {
	let count = 1;

	for (k of keys) {
		[n, key] = k.split("*");
		if (Object.keys(dict).includes(key)) {
			count += Number(n) * get_count(dict[key]);
		} else {
			count += Number(n);
		}
	}

	return count;
};

const part2 = () => {
	let keys = ["1*shinygold"];

	console.log(get_count(keys) - 2);
};
part2();
