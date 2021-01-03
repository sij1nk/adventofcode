const fs = require("fs");

const dict = {};
fs.readFileSync("day07_transformed.txt", "utf-8")
	.split("\n")
	.forEach((line) => {
		const fields = line.split(",");
		if (fields[1] != "") {
			dict[fields[0]] = fields.slice(1);
		}
	});

const containers_of = (bags) => {
	let containers = [];

	for (let container of Object.keys(dict)) {
		for (let bag of bags) {
			if (dict[container].filter((v) => v.match(bag)).length) {
				containers.push(container);
				containers = containers.concat(containers_of([container]));
			}
		}
	}

	return containers.filter((c, i, s) => s.indexOf(c) === i);
};

const cost_of = (keys) => {
	let cost = 1;

	for (k of keys) {
		[n, key] = k.split("*");
		if (Object.keys(dict).includes(key)) {
			cost += Number(n) * cost_of(dict[key]);
		} else {
			cost += Number(n);
		}
	}

	return cost;
};

console.log(containers_of(["shinygold"]).length);
console.log(cost_of(["1*shinygold"]) - 2);
