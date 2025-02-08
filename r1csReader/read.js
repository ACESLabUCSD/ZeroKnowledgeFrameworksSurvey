const loadR1cs = require("r1csfile").readR1cs;

loadR1cs("out.r1cs").then((r1cs) => {
	console.log(r1cs);
});