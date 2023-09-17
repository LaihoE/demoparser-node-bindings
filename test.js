
var {parseEvent, parseEvents,parseTicks, parsePlayerInfo, parseGrenades, listGameEvents, parseHeader} = require('./index');
const fs = require('fs');
const path = require('path');

try {
  const files = fs.readdirSync("/home/laiho/Documents/demos/cs2/test3/");
  files.forEach(file => {
    const filePath = path.join("/home/laiho/Documents/demos/cs2/test3/", file);
    // let y = parseEvents(filePath, ["player_death", "bomb_planted"])
    let y = parseEvent(filePath, "player_death", [], ["game_time"])
    // let y = parseEvents(filePath, ["bomb_planted"], ["X", "Y"], ["total_rounds_played"])
    // let y = parseHeader(filePath)
    // let y = parsePlayerInfo(filePath)
    // let y = parseTicks(filePath, ["damage_total"], [10000, 10001, 10002])
    // let y = listGameEvents(filePath)
    // let y = parseHeader(filePath)
    // let y = parseGrenades(filePath)
    console.log(y)
    // let ticks = parseTicks(filePath, ["active_weapon_name"], [10000, 10001], true)
    // let uniqueItems = [...new Set(ticks.active_weapon_name)]

  });
} catch (err) {
  console.error(err);
}

var {parseEvent, parseTicks} = require('./index');
// The event includes a field called site, but it gives you a big int like 204 etc.
// This will give you fields like "BombsiteA" and "BombsiteB"
// let events = parseEvent("/home/laiho/Documents/demos/cs2/test3/1.dem", "weapon_fire", ["X", "Y"])

// filtered = events.filter(e => e.weapon == "weapon_smokegrenade")
// let path = "/home/laiho/Documents/demos/cs2/test3/1.dem"
// let detonate_eve = parseEvent("/home/laiho/Documents/demos/cs2/test3/1.dem", "smokegrenade_detonate", ["X", "Y"])
// let d = parseTicks(path, ["active_weapon_name"])
// console.log(d)
