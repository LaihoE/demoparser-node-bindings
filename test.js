var {parseEvents, parseTicks, parsePlayerInfo, parseGrenades} = require('./index');
const fs = require('fs');
const path = require('path');

try {
  const files = fs.readdirSync("/home/laiho/Documents/demos/cs2/test2/");
  files.forEach(file => {
    const filePath = path.join("/home/laiho/Documents/demos/cs2/test2/", file);
    // let x = parseEvents(filePath, "player_death")
    // let y = parsePlayerInfo(filePath)
    // let y = parseTicks(filePath, ["X"])
    // let y = parseGrenades(filePath)
    console.log(y)
  });
} catch (err) {
  console.error(err);
}

