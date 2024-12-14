use crate::imports::imports_acmd::*;


unsafe extern "C" fn game_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE_RANGE(agent, 1.0, 11.0, 10.0);
    frame(agent.lua_state_agent, 11.0);
    FT_MOTION_RATE_RANGE(agent, 11.0, 67.0, 47.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.3, 361, 100, 30, 0, 7.0, 0.0, 10.0, 11.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 6, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_STATUS_CATCH_ATTACK_FLAG_DISABLE_CUT);
    }
    frame(agent.lua_state_agent, 62.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, FIGHTER_STATUS_CATCH_ATTACK_FLAG_DISABLE_CUT);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 67.0);
    FT_MOTION_RATE(agent,1.0);
}

unsafe extern "C" fn effect_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::COL_NORMAL(agent);
    }
    frame(agent.lua_state_agent, 11.0);
    for _ in 0..3 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("koopa_breath_m_fire"), Hash40::new("head"), -2, 5, 0, 180, 0, 50, 2.0, true);
        }
        wait(agent.lua_state_agent, 10.0);
    }
}

unsafe extern "C" fn sound_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_koopa_special_n01"));
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_STATUS(agent, Hash40::new("se_koopa_special_n02_win03"));
    }
    frame(agent.lua_state_agent, 62.0);
    if macros::is_excute(agent) {
        SoundModule::stop_status_se(agent.module_accessor);
    }
}

unsafe extern "C" fn expression_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 11.0);
    for i in 0..5 {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        wait(agent.lua_state_agent, 10.0);
    }
    frame(agent.lua_state_agent, 62.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub unsafe extern "C" fn catch_attack_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let to_return = catch_attack_main_inner(fighter);
    return to_return;
}

pub fn install() {
    smashline::Agent::new("koopa")
        .acmd("game_catchspecial", game_catchspecial,Priority::Default)
        .acmd("effect_catchspecial", effect_catchspecial,Priority::Default)
        .acmd("sound_catchspecial", sound_catchspecial,Priority::Default)
        .acmd("expression_catchspecial", expression_catchspecial,Priority::Default)
    .install();
}
