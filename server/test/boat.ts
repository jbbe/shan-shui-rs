import { parse } from "https://deno.land/std/flags/mod.ts";

const parsedArgs = parse(Deno.args);

console.log(parsedArgs, parsedArgs._[0]);
const iterations = parsedArgs._[0];

// const concurrencyMax = 0;

async function getBoat(i: number) {
  const seed = Math.floor((i+1) * new Date().getTime() * Math.random() % 22424023);
  const u = "http://localhost:6767/boat/" + seed;
  console.log("Getting boat ", i, u);
  await fetch(u);
  console.log("Got boat ", i);
}
for (let i = 0; i < iterations; i++) {
  getBoat(i);
}
