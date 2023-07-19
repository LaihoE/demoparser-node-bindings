var {parseEvents, parseTicks, parsePlayerInfo, parseGrenades, listGameEvents} = require('./index');
const fs = require('fs');
const path = require('path');

try {
  const files = fs.readdirSync("/home/laiho/Documents/demos/cs2/test2/");
  files.forEach(file => {
    const filePath = path.join("/home/laiho/Documents/demos/cs2/test2/", file);
    // let y = parseEvents(filePath, "player_death")
    // let y = parsePlayerInfo(filePath)
    let y = parseTicks(filePath, ["X"])
    // let y = listGameEvents(filePath)
    console.log(y)
  });
} catch (err) {
  console.error(err);
}

