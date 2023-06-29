var {parseEvents, parseTicks} = require('./index');
const fs = require('fs');
const path = require('path');

try {
  const files = fs.readdirSync("/home/laiho/Documents/demos/cs2/test2/");
  files.forEach(file => {
    const filePath = path.join("/home/laiho/Documents/demos/cs2/test2/", file);
    // let x = parseEvents(filePath, "player_death")
    let y = parseTicks(filePath, ["X"])
    console.log(filePath)

  });
} catch (err) {
  console.error(err);
}

