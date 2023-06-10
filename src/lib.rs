#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
use napi::bindgen_prelude::*;
use napi::Error;
use parser::parser_settings::Parser;
use parser::parser_settings::ParserInputs;
use parser::read_bits::DemoParserError;
use parser::variants::OutputSerdeHelperStruct;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;

#[napi]
pub fn parse_chat_messages(file: String) -> Result<Value> {
  let bytes = fs::read(file)?;
  let settings = ParserInputs {
    bytes: &bytes,
    wanted_player_props: vec![],
    wanted_player_props_og_names: vec![],
    wanted_other_props: vec![],
    wanted_other_props_og_names: vec![],
    wanted_event: Some("-".to_owned()),
    parse_ents: false,
    wanted_ticks: vec![],
    parse_projectiles: false,
    only_header: true,
    count_props: false,
    only_convars: false,
  };
  let mut parser = match Parser::new(settings) {
    Ok(parser) => parser,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  match parser.start() {
    Ok(_) => {}
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  let mut messages = vec![];
  for i in 0..parser.chat_messages.param1.len() {
    let mut hm: HashMap<String, Option<String>> = HashMap::default();
    hm.insert(
      "entid".to_string(),
      Some(parser.chat_messages.entity_idx[i].unwrap_or(0).to_string()),
    );
    hm.insert(
      "player_name".to_string(),
      parser.chat_messages.param1[i].clone(),
    );
    hm.insert(
      "message".to_string(),
      parser.chat_messages.param2[i].clone(),
    );
    hm.insert(
      "location".to_string(),
      parser.chat_messages.param3[i].clone(),
    );
    hm.insert("param4".to_string(), parser.chat_messages.param4[i].clone());
    messages.push(hm);
  }
  let s = match serde_json::to_value(&messages) {
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
  let bytes = fs::read(path)?;

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
  let settings = ParserInputs {
    bytes: &bytes,
    wanted_player_props: real_names_player.clone(),
    wanted_player_props_og_names: player_props,
    wanted_other_props: real_names_other,
    wanted_other_props_og_names: other_props,
    wanted_event: Some(event_name),
    parse_ents: true,
    wanted_ticks: vec![],
    parse_projectiles: false,
    only_header: false,
    count_props: false,
    only_convars: false,
  };
  let mut parser = match Parser::new(settings) {
    Ok(parser) => parser,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  match parser.start() {
    Ok(_) => {}
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  let s = match serde_json::to_value(&parser.game_events) {
    Ok(s) => s,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  Ok(s)
}

#[napi]
pub fn parse_ticks(path: String, wanted_props: Vec<String>) -> Result<Value> {
  let bytes = fs::read(path)?;
  let real_names = match rm_user_friendly_names(&wanted_props) {
    Ok(names) => names,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };

  let settings = ParserInputs {
    bytes: &bytes,
    wanted_player_props: real_names,
    wanted_player_props_og_names: wanted_props,
    wanted_other_props: vec![],
    wanted_other_props_og_names: vec![],
    wanted_event: Some("".to_string()),
    parse_ents: true,
    wanted_ticks: vec![],
    parse_projectiles: false,
    only_header: false,
    count_props: false,
    only_convars: false,
  };
  let mut parser = match Parser::new(settings) {
    Ok(parser) => parser,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  match parser.start() {
    Ok(_) => {}
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  let helper = OutputSerdeHelperStruct {
    inner: parser.output,
  };
  let s = match serde_json::to_value(&helper) {
    Ok(s) => s,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  Ok(s)
}

#[napi]
pub fn parse_player_info(path: String) -> Result<Value> {
  let bytes = fs::read(path)?;

  let settings = ParserInputs {
    bytes: &bytes,
    wanted_player_props: vec![],
    wanted_player_props_og_names: vec![],
    wanted_other_props: vec![],
    wanted_other_props_og_names: vec![],
    wanted_event: Some("-".to_owned()),
    parse_ents: false,
    wanted_ticks: vec![],
    parse_projectiles: false,
    only_header: true,
    count_props: false,
    only_convars: false,
  };

  let mut parser = match Parser::new(settings) {
    Ok(parser) => parser,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  match parser.start() {
    Ok(_) => {}
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  let mut messages = vec![];
  for i in 0..parser.skins.ent_idx.len() {
    let mut hm: HashMap<String, Option<String>> = HashMap::default();
    let sid = match parser.player_end_data.steamid[i] {
      Some(sid) => Some(sid.to_string()),
      None => None,
    };
    hm.insert("steamid".to_string(), sid);
    let tm = match parser.player_end_data.team_number[i] {
      Some(t) => Some(t.to_string()),
      None => None,
    };
    hm.insert("team_number".to_string(), tm);
    hm.insert("name".to_string(), parser.player_end_data.name[i].clone());
    messages.push(hm)
  }
  let s = match serde_json::to_value(&messages) {
    Ok(s) => s,
    Err(e) => return Err(Error::new(Status::InvalidArg, format!("{}", e).to_owned())),
  };
  Ok(s)
}
pub fn rm_user_friendly_names(names: &Vec<String>) -> Result<Vec<String>, DemoParserError> {
  let mut real_names = vec![];
  for name in names {
    match FRIENDLY_NAMES_MAPPING.get(name) {
      Some(real_name) => real_names.push(real_name.to_string()),
      None => return Err(DemoParserError::UnknownPropName(name.clone())),
    }
  }
  Ok(real_names)
}

pub static FRIENDLY_NAMES_MAPPING: phf::Map<&'static str, &'static str> = phf_map! {
  "team_surrendered" => "CCSTeam.m_bSurrendered",
  "team_rounds_total" => "CCSTeam.m_iScore",
  "team_name" => "CCSTeam.m_szTeamname",
  "team_score_overtime" => "CCSTeam.m_scoreOvertime",
  "team_match_stat"=>"CCSTeam.m_szTeamMatchStat",
  "team_num_map_victories"=>"CCSTeam.m_numMapVictories",
  "team_score_first_half"=>"CCSTeam.m_scoreFirstHalf",
  "team_score_second_half"=>"CCSTeam.m_scoreSecondHalf",
  "team_clan_name" =>"CCSTeam.m_szClanTeamname",
  "is_freeze_period"=>"CCSGameRulesProxy.CCSGameRules.m_bFreezePeriod",
  "is_warmup_period"=>"CCSGameRulesProxy.CCSGameRules.m_bWarmupPeriod" ,
  "warmup_period_end"=>"CCSGameRulesProxy.CCSGameRules.m_fWarmupPeriodEnd" ,
  "warmup_period_start"=>"CCSGameRulesProxy.CCSGameRules.m_fWarmupPeriodStart" ,
  "is_terrorist_timeout"=>"CCSGameRulesProxy.CCSGameRules.m_bTerroristTimeOutActive" ,
  "is_ct_timeout"=>"CCSGameRulesProxy.CCSGameRules.m_bCTTimeOutActive" ,
  "terrorist_timeout_remaining"=>"CCSGameRulesProxy.CCSGameRules.m_flTerroristTimeOutRemaining" ,
  "ct_timeout_remaining"=>"CCSGameRulesProxy.CCSGameRules.m_flCTTimeOutRemaining" ,
  "num_terrorist_timeouts"=>"CCSGameRulesProxy.CCSGameRules.m_nTerroristTimeOuts" ,
  "num_ct_timeouts"=>"CCSGameRulesProxy.CCSGameRules.m_nCTTimeOuts" ,
  "is_technical_timeout"=>"CCSGameRulesProxy.CCSGameRules.m_bTechnicalTimeOut" ,
  "is_waiting_for_resume"=>"CCSGameRulesProxy.CCSGameRules.m_bMatchWaitingForResume" ,
  "match_start_time"=>"CCSGameRulesProxy.CCSGameRules.m_fMatchStartTime" ,
  "round_start_time"=>"CCSGameRulesProxy.CCSGameRules.m_fRoundStartTime" ,
  "restart_round_time"=>"CCSGameRulesProxy.CCSGameRules.m_flRestartRoundTime" ,
  "is_game_restart?"=>"CCSGameRulesProxy.CCSGameRules.m_bGameRestart" ,
  "game_start_time"=>"CCSGameRulesProxy.CCSGameRules.m_flGameStartTime" ,
  "time_until_next_phase_start"=>"CCSGameRulesProxy.CCSGameRules.m_timeUntilNextPhaseStarts" ,
  "game_phase"=>"CCSGameRulesProxy.CCSGameRules.m_gamePhase" ,
  "total_rounds_played"=>"CCSGameRulesProxy.CCSGameRules.m_totalRoundsPlayed" ,
  "rounds_played_this_phase"=>"CCSGameRulesProxy.CCSGameRules.m_nRoundsPlayedThisPhase" ,
  "hostages_remaining"=>"CCSGameRulesProxy.CCSGameRules.m_iHostagesRemaining" ,
  "any_hostages_reached"=>"CCSGameRulesProxy.CCSGameRules.m_bAnyHostageReached" ,
  "has_bombites"=>"CCSGameRulesProxy.CCSGameRules.m_bMapHasBombTarget" ,
  "has_rescue_zone"=>"CCSGameRulesProxy.CCSGameRules.m_bMapHasRescueZone" ,
  "has_buy_zone"=>"CCSGameRulesProxy.CCSGameRules.m_bMapHasBuyZone" ,
  "is_matchmaking"=>"CCSGameRulesProxy.CCSGameRules.m_bIsQueuedMatchmaking" ,
  "match_making_mode"=>"CCSGameRulesProxy.CCSGameRules.m_nQueuedMatchmakingMode" ,
  "is_valve_dedicated_server"=>"CCSGameRulesProxy.CCSGameRules.m_bIsValveDS" ,
  "gungame_prog_weap_ct"=>"CCSGameRulesProxy.CCSGameRules.m_iNumGunGameProgressiveWeaponsCT" ,
  "gungame_prog_weap_t"=>"CCSGameRulesProxy.CCSGameRules.m_iNumGunGameProgressiveWeaponsT" ,
  "spectator_slot_count"=>"CCSGameRulesProxy.CCSGameRules.m_iSpectatorSlotCount" ,
  "is_match_started"=>"CCSGameRulesProxy.CCSGameRules.m_bHasMatchStarted" ,
  "n_best_of_maps"=>"CCSGameRulesProxy.CCSGameRules.m_numBestOfMaps" ,
  "is_bomb_dropped"=>"CCSGameRulesProxy.CCSGameRules.m_bBombDropped" ,
  "is_bomb_planed"=>"CCSGameRulesProxy.CCSGameRules.m_bBombPlanted" ,
  "round_win_status"=>"CCSGameRulesProxy.CCSGameRules.m_iRoundWinStatus" ,
  "round_win_reason"=>"CCSGameRulesProxy.CCSGameRules.m_eRoundWinReason" ,
  "terrorist_cant_buy"=>"CCSGameRulesProxy.CCSGameRules.m_bTCantBuy" ,
  "ct_cant_buy"=>"CCSGameRulesProxy.CCSGameRules.m_bCTCantBuy" ,
  "num_player_alive_ct"=>"CCSGameRulesProxy.CCSGameRules.m_iMatchStats_PlayersAlive_CT" ,
  "num_player_alive_t"=>"CCSGameRulesProxy.CCSGameRules.m_iMatchStats_PlayersAlive_T" ,
  "ct_losing_streak"=>"CCSGameRulesProxy.CCSGameRules.m_iNumConsecutiveCTLoses" ,
  "t_losing_streak"=>"CCSGameRulesProxy.CCSGameRules.m_iNumConsecutiveTerroristLoses" ,
  "survival_start_time"=>"CCSGameRulesProxy.CCSGameRules.m_flSurvivalStartTime" ,
  "round_in_progress"=>"CCSGameRulesProxy.CCSGameRules.m_bRoundInProgress" ,
  "i_bomb_site?"=>"CCSGameRulesProxy.CCSGameRules.m_iBombSite" ,
  "is_auto_muted"=>"CCSPlayerController.m_bHasCommunicationAbuseMute",
  "crosshair_code"=>"CCSPlayerController.m_szCrosshairCodes",
  "pending_team_num"=>"CCSPlayerController.m_iPendingTeamNum",
  "player_color"=>"CCSPlayerController.m_iCompTeammateColor",
  "ever_played_on_team"=>"CCSPlayerController.m_bEverPlayedOnTeam",
  "clan_name"=>"CCSPlayerController.m_szClan",
  "is_coach_team"=>"CCSPlayerController.m_iCoachingTeam",
  "comp_rank"=>"CCSPlayerController.m_iCompetitiveRanking",
  "comp_wins"=>"CCSPlayerController.m_iCompetitiveWins",
  "comp_rank_type"=>"CCSPlayerController.m_iCompetitiveRankType",
  "is_controlling_bot"=>"CCSPlayerController.m_bControllingBot",
  "has_controlled_bot_this_round"=>"CCSPlayerController.m_bHasControlledBotThisRound",
  "can_control_bot"=>"CCSPlayerController.m_bCanControlObservedBot",
  "is_alive"=>"CCSPlayerController.m_bPawnIsAlive",
  "armor"=>"CCSPlayerController.m_iPawnArmor",
  "has_defuser"=>"CCSPlayerController.m_bPawnHasDefuser",
  "has_helmet"=>"CCSPlayerController.m_bPawnHasHelmet",
  "spawn_time"=>"CCSPlayerController.m_iPawnLifetimeStart",
  "death_time"=>"CCSPlayerController.m_iPawnLifetimeEnd",
  "score"=>"CCSPlayerController.m_iScore",
  "game_time"=>"CCSPlayerController.m_flSimulationTime",
  "is_connected"=>"CCSPlayerController.m_iConnected",
  "player_name"=>"CCSPlayerController.m_iszPlayerName",
  "player_steamid"=>"CCSPlayerController.m_steamID",
  "fov"=>"CCSPlayerController.m_iDesiredFOV",
  "balance"=>"CCSPlayerController.CCSPlayerController_InGameMoneyServices.m_iAccount",
  "start_balance"=>"CCSPlayerController.CCSPlayerController_InGameMoneyServices.m_iStartAccount",
  "total_cash_spent"=>"CCSPlayerController.CCSPlayerController_InGameMoneyServices.m_iTotalCashSpent",
  "cash_spent_this_round"=>"CCSPlayerController.CCSPlayerController_InGameMoneyServices.m_iCashSpentThisRound",
  "music_kit_id"=>"CCSPlayerController.CCSPlayerController_InventoryServices.m_unMusicID",
  "leader_honors"=>"CCSPlayerController.CCSPlayerController_InventoryServices.m_nPersonaDataPublicCommendsLeader",
  "teacher_honors"=>"CCSPlayerController.CCSPlayerController_InventoryServices.m_nPersonaDataPublicCommendsTeacher",
  "friendly_honors"=>"CCSPlayerController.CCSPlayerController_InventoryServices.m_nPersonaDataPublicCommendsFriendly",
  "kills_this_round"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iKills",
  "deaths_this_round"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iDeaths",
  "assists_this_round"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iAssists",
  "alive_time_this_round"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iLiveTime",
  "headshot_kills_this_round"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iHeadShotKills",
  "damage_this_round"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iDamage",
  "objective_this_round"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iObjective",
  "utility_damage_this_round"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iUtilityDamage",
  "enemies_flashed_this_round"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iEnemiesFlashed",
  "equipment_value_this_round"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iEquipmentValue",
  "money_saved_this_round"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iMoneySaved",
  "kill_reward_this_round"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iKillReward",
  "cash_earned_this_round"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.CSPerRoundStats_t.m_iCashEarned",
  "kills_total"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iKills",
  "deaths_total"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iDeaths",
  "assists_total"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iAssists",
  "alive_time_total"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iLiveTime",
  "headshot_kills_total"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iHeadShotKills",
  "ace_rounds_total"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iEnemy5Ks",
  "4k_rounds_total"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iEnemy4Ks",
  "3k_rounds_total"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iEnemy3Ks",
  "damage_total"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iDamage",
  "objective_total"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iObjective",
  "utility_damage_total"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iUtilityDamage",
  "enemies_flashed_total"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iEnemiesFlashed",
  "equipment_value_total"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iEquipmentValue",
  "money_saved_total"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iMoneySaved",
  "kill_reward_total"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iKillReward",
  "cash_earned_total"=>"CCSPlayerController.CCSPlayerController_ActionTrackingServices.m_iCashEarned",
  "ping"=>"CCSPlayerController.m_iPing",
  "move_collide" => "CCSPlayerPawn.m_MoveCollide",
  "move_type" =>  "CCSPlayerPawn.m_MoveType",
  "team_num" => "CCSPlayerPawn.m_iTeamNum",
  "active_weapon" => "CCSPlayerPawn.CCSPlayer_WeaponServices.m_hActiveWeapon",
  "looking_at_weapon" => "CCSPlayerPawn.CCSPlayer_WeaponServices.m_bIsLookingAtWeapon",
  "holding_look_at_weapon" => "CCSPlayerPawn.CCSPlayer_WeaponServices.m_bIsHoldingLookAtWeapon",
  "next_attack_time" => "CCSPlayerPawn.CCSPlayer_WeaponServices.m_flNextAttack",
  "duck_time_ms" =>"CCSPlayerPawn.CCSPlayer_MovementServices.m_nDuckTimeMsecs",
  "max_speed" => "CCSPlayerPawn.CCSPlayer_MovementServices.m_flMaxspeed",
  "max_fall_velo" => "CCSPlayerPawn.CCSPlayer_MovementServices.m_flMaxFallVelocity",
  "duck_amount" => "CCSPlayerPawn.CCSPlayer_MovementServices.m_flDuckAmount",
  "duck_speed" => "CCSPlayerPawn.CCSPlayer_MovementServices.m_flDuckSpeed",
  "duck_overrdie" => "CCSPlayerPawn.CCSPlayer_MovementServices.m_bDuckOverride",
  "old_jump_pressed" => "CCSPlayerPawn.CCSPlayer_MovementServices.m_bOldJumpPressed",
  "jump_until" => "CCSPlayerPawn.CCSPlayer_MovementServices.m_flJumpUntil",
  "jump_velo" => "CCSPlayerPawn.CCSPlayer_MovementServices.m_flJumpVel",
  "fall_velo" => "CCSPlayerPawn.CCSPlayer_MovementServices.m_flFallVelocity",
  "in_crouch" => "CCSPlayerPawn.CCSPlayer_MovementServices.m_bInCrouch",
  "crouch_state" => "CCSPlayerPawn.CCSPlayer_MovementServices.m_nCrouchState",
  "ducked" => "CCSPlayerPawn.CCSPlayer_MovementServices.m_bDucked",
  "ducking" => "CCSPlayerPawn.CCSPlayer_MovementServices.m_bDucking",
  "in_duck_jump" => "CCSPlayerPawn.CCSPlayer_MovementServices.m_bInDuckJump",
  "allow_auto_movement" => "CCSPlayerPawn.CCSPlayer_MovementServices.m_bAllowAutoMovement",
  "jump_time_ms" => "CCSPlayerPawn.CCSPlayer_MovementServices.m_nJumpTimeMsecs",
  "last_duck_time" => "CCSPlayerPawn.CCSPlayer_MovementServices.m_flLastDuckTime",
  "is_rescuing" => "CCSPlayerPawn.CCSPlayer_ActionTrackingServices.m_bIsRescuing",
  "weapon_purchases_this_match" => "CCSPlayerPawn.CCSPlayer_ActionTrackingServices.m_iWeaponPurchasesThisMatch",
  "weapon_purchases_this_round" => "CCSPlayerPawn.CCSPlayer_ActionTrackingServices.m_iWeaponPurchasesThisRound",
  "spotted" => "CCSPlayerPawn.m_bSpotted",
  "spotted_mask" => "CCSPlayerPawn.m_bSpottedByMask",
  "time_last_injury" => "CCSPlayerPawn.m_flTimeOfLastInjury",
  "direction_last_injury" => "CCSPlayerPawn.m_nRelativeDirectionOfLastInjury",
  "player_state" => "CCSPlayerPawn.m_iPlayerState",
  "passive_items" => "CCSPlayerPawn.m_passiveItems",
  "is_scoped" => "CCSPlayerPawn.m_bIsScoped",
  "is_walking" => "CCSPlayerPawn.m_bIsWalking",
  "resume_zoom" => "CCSPlayerPawn.m_bResumeZoom",
  "is_defusing" =>"CCSPlayerPawn.m_bIsDefusing",
  "is_grabbing_hostage" => "CCSPlayerPawn.m_bIsGrabbingHostage",
  "blocking_use_in_progess" => "CCSPlayerPawn.m_iBlockingUseActionInProgress",
  "molotov_damage_time" => "CCSPlayerPawn.m_fMolotovDamageTime",
  "moved_since_spawn" => "CCSPlayerPawn.m_bHasMovedSinceSpawn",
  "in_bomb_zone" => "CCSPlayerPawn.m_bInBombZone",
  "in_buy_zone" => "CCSPlayerPawn.m_bInBuyZone",
  "in_no_defuse_area" => "CCSPlayerPawn.m_bInNoDefuseArea",
  "killed_by_taser" => "CCSPlayerPawn.m_bKilledByTaser",
  "move_state" => "CCSPlayerPawn.m_iMoveState",
  "which_bomb_zone" => "CCSPlayerPawn.m_nWhichBombZone",
  "in_hostage_rescue_zone" => "CCSPlayerPawn.m_bInHostageRescueZone",
  "stamina" => "CCSPlayerPawn.m_flStamina",
  "direction" => "CCSPlayerPawn.m_iDirection",
  "shots_fired" => "CCSPlayerPawn.m_iShotsFired",
  "armor_value" => "CCSPlayerPawn.m_ArmorValue",
  "velo_modifier" => "CCSPlayerPawn.m_flVelocityModifier",
  "ground_accel_linear_frac_last_time" => "CCSPlayerPawn.m_flGroundAccelLinearFracLastTime",
  "flash_duration" => "CCSPlayerPawn.m_flFlashDuration",
  "flash_max_alpha" => "CCSPlayerPawn.m_flFlashMaxAlpha",
  "wait_for_no_attack" => "CCSPlayerPawn.m_bWaitForNoAttack",
  "last_place_name" => "CCSPlayerPawn.m_szLastPlaceName",
  "is_strafing" => "CCSPlayerPawn.m_bStrafing",
  "round_start_equip_value" => "CCSPlayerPawn.m_unRoundStartEquipmentValue",
  "current_equip_value" => "CCSPlayerPawn.m_unCurrentEquipmentValue",
  "time" => "CCSPlayerPawn.m_flSimulationTime",
  "health" => "CCSPlayerPawn.m_iHealth",
  "life_state" => "CCSPlayerPawn.m_lifeState",
  "X"=> "X",
  "Y"=> "Y",
  "Z"=> "Z",
  "pitch" => "CCSPlayerPawnBase.m_angEyeAngles@0",
  "yaw" => "CCSPlayerPawnBase.m_angEyeAngles@1",
  "active_weapon_name" => "weapon_name",
  "active_weapon_ammo" => "m_iClip1",
  "total_ammo_left" => "m_pReserveAmmo",
  "item_def_idx" => "m_iItemDefinitionIndex",
  "weapon_quality" => "m_iEntityQuality",
  "entity_lvl" => "m_iEntityLevel",
  "item_id_high" => "m_iItemIDHigh",
  "item_id_low" => "m_iItemIDLow",
  "item_account_id" => "m_iAccountID",
  "inventory_position" => "m_iInventoryPosition",
  "is_initialized" => "m_bInitialized",
  "econ_item_attribute_def_idx" => "CEconItemAttribute.m_iAttributeDefinitionIndex",
  "econ_raw_val_32" => "CEconItemAttribute.m_iRawValue32",
  "initial_value" => "CEconItemAttribute.m_flInitialValue",
  "refundable_currency" => "CEconItemAttribute.m_nRefundableCurrency",
  "set_bonus"=> "CEconItemAttribute.m_bSetBonus",
  "custom_name" => "m_szCustomName",
  "orig_owner_xuid_low" => "m_OriginalOwnerXuidLow",
  "orig_owner_xuid_high"=> "m_OriginalOwnerXuidHigh",
  "fall_back_paint_kit" => "m_nFallbackPaintKit",
  "fall_back_seed"=> "m_nFallbackSeed",
  "fall_back_wear"=> "m_flFallbackWear",
  "fall_back_stat_track"=> "m_nFallbackStatTrak",
  "m_iState"=> "m_iState",
  "fire_seq_start_time" => "m_flFireSequenceStartTime",
  "fire_seq_start_time_change" => "m_nFireSequenceStartTimeChange",
  "is_player_fire_event_primary"=>  "m_bPlayerFireEventIsPrimary",
  "weapon_mode"=> "m_weaponMode",
  "accuracy_penalty"=> "m_fAccuracyPenalty",
  "i_recoil_idx"=> "m_iRecoilIndex",
  "fl_recoil_idx"=> "m_flRecoilIndex",
  "is_burst_mode"=> "m_bBurstMode",
  "post_pone_fire_ready_time"=> "m_flPostponeFireReadyTime",
  "is_in_reload"=> "m_bInReload",
  "reload_visually_complete"=> "m_bReloadVisuallyComplete",
  "dropped_at_time"=> "m_flDroppedAtTime",
  "is_hauled_back"=> "m_bIsHauledBack",
  "is_silencer_on"=> "m_bSilencerOn",
  "time_silencer_switch_complete"=> "m_flTimeSilencerSwitchComplete",
  "orig_team_number"=> "m_iOriginalTeamNumber",
  "prev_owner"=> "m_hPrevOwner",
  "last_shot_time"=> "m_fLastShotTime",
  "iron_sight_mode"=> "m_iIronSightMode",
  "num_empty_attacks"=> "m_iNumEmptyAttacks",
  "zoom_lvl"=> "m_zoomLevel",
  "burst_shots_remaining"=> "m_iBurstShotsRemaining",
  "needs_bolt_action"=> "m_bNeedsBoltAction",
  "next_primary_attack_tick"=> "m_nNextPrimaryAttackTick",
  "next_primary_attack_tick_ratio"=> "m_flNextPrimaryAttackTickRatio",
  "next_secondary_attack_tick" => "m_nNextSecondaryAttackTick",
  "next_secondary_attack_tick_ratio"=> "m_flNextSecondaryAttackTickRatio",
};
