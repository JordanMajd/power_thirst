"use strict";

const fs = require("fs");

const file = fs.readFileSync("log.json",  { encoding: "utf-8", flag: "r" });
const log = JSON.parse(file).log;

let entries = 0;
let watts = 0;
let balance = 0;

log.forEach((entry) => {
  // only count nonzero values
  if(entry.watts !== 0) {
    watts += parseInt(entry.watts);
    entries++;
  }
  balance += parseFloat(entry.balance);
});

watts = watts / entries;
balance = balance / log.length;

console.log(`Avg watts ${watts}, avg balance ${balance}`);