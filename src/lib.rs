#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
use ahash::AHashMap;
use memmap2::Mmap;
use memmap2::MmapOptions;
use napi::bindgen_prelude::*;
use napi::Error;
use parser::parser_settings::rm_user_friendly_names;
use parser::parser_settings::Parser;
use parser::parser_settings::ParserInputs;
use parser::parser_thread_settings::create_huffman_lookup_table;
use parser::read_bits::DemoParserError;
use parser::variants::OutputSerdeHelperStruct;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::sync::Arc;

#[napi]
pub fn parse_chat_messages(path: String) -> Result<Value> {
  let file = File::open(path.clone())?;
  let arc_mmap = Arc::new(unsafe { MmapOptions::new().map(&file)? });
  let arc_huf = Arc::new(create_huffman_lookup_table());

  let settings = ParserInputs {
    real_name_to_og_name: AHashMap::default(),
    bytes: arc_mmap.clone(),
    wanted_player_props: vec![],
    wanted_player_props_og_names: vec![],
    wanted_other_props: vec![],
    wanted_other_props_og_names: vec![],
    wanted_event: None,
    parse_ents: true,
    wanted_ticks: vec![],
    parse_projectiles: false,
    only_header: true,
    count_props: false,
    only_convars: false,
    huffman_lookup_table: arc_huf.clone(),
  };
  let mut parser = Parser::new(settings);
  let output = match parser.parse_demo() {
    Ok(output) => output,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };

  let s = match serde_json::to_value(&output.chat_messages) {
    Ok(s) => s,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  Ok(s)
}
#[napi]
pub fn parse_grenades(path: String) -> Result<Value> {
  let file = File::open(path.clone())?;
  let arc_mmap = Arc::new(unsafe { MmapOptions::new().map(&file)? });
  let arc_huf = Arc::new(create_huffman_lookup_table());

  let settings = ParserInputs {
    real_name_to_og_name: AHashMap::default(),
    bytes: arc_mmap.clone(),
    wanted_player_props: vec![],
    wanted_player_props_og_names: vec![],
    wanted_other_props: vec![],
    wanted_other_props_og_names: vec![],
    wanted_event: None,
    parse_ents: true,
    wanted_ticks: vec![],
    parse_projectiles: true,
    only_header: true,
    count_props: false,
    only_convars: false,
    huffman_lookup_table: arc_huf.clone(),
  };
  let mut parser = Parser::new(settings);
  let output = match parser.parse_demo() {
    Ok(output) => output,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };

  let s = match serde_json::to_value(&output.projectiles) {
    Ok(s) => s,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  Ok(s)
}
#[napi]
pub fn parse_events(
  path: String,
  event_name: String,
  extra_player: Option<Vec<String>>,
  extra_other: Option<Vec<String>>,
) -> Result<Value> {
  let player_props = match extra_player {
    Some(p) => p,
    None => vec![],
  };
  let other_props = match extra_other {
    Some(p) => p,
    None => vec![],
  };
  let real_names_player = match rm_user_friendly_names(&player_props) {
    Ok(names) => names,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  let real_names_other = match rm_user_friendly_names(&player_props) {
    Ok(names) => names,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };

  let mut real_name_to_og_name = AHashMap::default();
  for (real_name, user_friendly_name) in real_names_player.iter().zip(&player_props) {
    real_name_to_og_name.insert(real_name.clone(), user_friendly_name.clone());
  }

  let file = File::open(path.clone())?;
  let arc_mmap = Arc::new(unsafe { MmapOptions::new().map(&file)? });
  let arc_huf = Arc::new(create_huffman_lookup_table());

  let settings = ParserInputs {
    real_name_to_og_name: real_name_to_og_name,
    bytes: arc_mmap.clone(),
    wanted_player_props: real_names_player.clone(),
    wanted_player_props_og_names: player_props.clone(),
    wanted_other_props: vec![],
    wanted_other_props_og_names: vec![],
    wanted_event: Some(event_name.clone()),
    parse_ents: true,
    wanted_ticks: vec![],
    parse_projectiles: false,
    only_header: true,
    count_props: false,
    only_convars: false,
    huffman_lookup_table: arc_huf.clone(),
  };
  let mut parser = Parser::new(settings);
  let output = match parser.parse_demo() {
    Ok(output) => output,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  let s = match serde_json::to_value(&output.game_events) {
    Ok(s) => s,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  Ok(s)
}

#[napi]
pub fn parse_ticks(path: String, wanted_props: Vec<String>) -> Result<Value> {
  let mut real_names = match rm_user_friendly_names(&wanted_props) {
    Ok(names) => names,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };

  let file = File::open(path.clone())?;
  let arc_mmap = Arc::new(unsafe { MmapOptions::new().map(&file)? });
  let arc_huf = Arc::new(create_huffman_lookup_table());
  let mut real_name_to_og_name = AHashMap::default();

  for (real_name, user_friendly_name) in real_names.iter().zip(&wanted_props) {
    real_name_to_og_name.insert(real_name.clone(), user_friendly_name.clone());
  }

  let settings = ParserInputs {
    real_name_to_og_name: real_name_to_og_name,
    bytes: arc_mmap.clone(),
    wanted_player_props: real_names.clone(),
    wanted_player_props_og_names: wanted_props.clone(),
    wanted_other_props: vec![],
    wanted_other_props_og_names: vec![],
    wanted_event: None,
    parse_ents: true,
    wanted_ticks: vec![],
    parse_projectiles: false,
    only_header: true,
    count_props: false,
    only_convars: false,
    huffman_lookup_table: arc_huf.clone(),
  };
  let mut parser = Parser::new(settings);
  let output = match parser.parse_demo() {
    Ok(output) => output,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  real_names.push("tick".to_owned());
  real_names.push("steamid".to_owned());
  real_names.push("name".to_owned());

  let mut prop_infos = output.prop_info.prop_infos.clone();
  prop_infos.sort_by_key(|x| x.prop_name.clone());
  real_names.sort();

  let helper = OutputSerdeHelperStruct {
    prop_infos: prop_infos,
    inner: output.df.into(),
  };
  let s = match serde_json::to_value(&helper) {
    Ok(s) => s,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  Ok(s)
}

#[napi]
pub fn parse_player_info(path: String) -> Result<Value> {
  let file = File::open(path.clone())?;
  let arc_mmap = Arc::new(unsafe { MmapOptions::new().map(&file)? });
  let arc_huf = Arc::new(create_huffman_lookup_table());

  let settings = ParserInputs {
    real_name_to_og_name: AHashMap::default(),
    bytes: arc_mmap.clone(),
    wanted_player_props: vec![],
    wanted_player_props_og_names: vec![],
    wanted_other_props: vec![],
    wanted_other_props_og_names: vec![],
    wanted_event: None,
    parse_ents: true,
    wanted_ticks: vec![],
    parse_projectiles: false,
    only_header: true,
    count_props: false,
    only_convars: false,
    huffman_lookup_table: arc_huf.clone(),
  };
  let mut parser = Parser::new(settings);
  let output = match parser.parse_demo() {
    Ok(output) => output,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };

  let s = match serde_json::to_value(&output.player_md) {
    Ok(s) => s,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  Ok(s)
}
