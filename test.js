var {parseEvent, parseEvents,parseTicks, parsePlayerInfo, parseGrenades, listGameEvents, parseHeader} = require('./index');
const fs = require('fs');
const path = require('path');

try {
  const files = fs.readdirSync("/home/laiho/Documents/demos/cs2/test3/");
  files.forEach(file => {
    const filePath = path.join("/home/laiho/Documents/demos/cs2/test3/", file);
    // let y = parseEvents(filePath, ["player_death", "bomb_planted"])
    // let y = parsePlayerInfo(filePath)
    // let y = parseTicks(filePath, ["X"], [10000, 10001, 10002])
    let y = listGameEvents(filePath)
    // let y = parseHeader(filePath)
    // let y = parseGrenades(filePath)
    console.log(y)
  });
} catch (err) {
  console.error(err);
}

