use crate::imports::imports_acmd::*;


pub const ANIM_OFFSET: f32 = 13.0;
unsafe extern "C" fn game_catchspecial(agent: &mut L2CAgentBase) {
    let mut can_shoot=false;
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.65);
    frame(agent.lua_state_agent, 1.0+ANIM_OFFSET);
    macros::FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 10.0+ANIM_OFFSET);
    if macros::is_excute(agent) {
        can_shoot = ArticleModule::is_generatable(agent.module_accessor, *FIGHTER_PACKUN_GENERATE_ARTICLE_POISONBREATH);
        if can_shoot {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PACKUN_GENERATE_ARTICLE_POISONBREATH, false, -1);
        }
    }
    frame(agent.lua_state_agent, 21.0+ANIM_OFFSET);
    macros::FT_MOTION_RATE(agent, 0.9);
}

unsafe extern "C" fn effect_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0+ANIM_OFFSET);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("packun_poison_breath"), Hash40::new("mouth"), 3, -0.6, 0, 0, 90, -100, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(agent.lua_state_agent, 29.0+ANIM_OFFSET);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("packun_poison_mouth"), Hash40::new("mouth"), 1, -0.6, 0, 0, 90, -130, 1, true);
    }
}

unsafe extern "C" fn sound_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0+ANIM_OFFSET);
    if macros::is_excute(agent) {
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
    }
    frame(agent.lua_state_agent, 3.0+ANIM_OFFSET);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_packun_special_s03"));
    }
}

unsafe extern "C" fn expression_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 2.0+ANIM_OFFSET);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attacks"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 5.0+ANIM_OFFSET);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attacks"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 8.0+ANIM_OFFSET);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attacks"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub unsafe extern "C" fn poison_shoot_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner = get_owner_boma(weapon);
    let owner_status = StatusModule::status_kind(owner);
    if owner_status != *FIGHTER_STATUS_KIND_CATCH_ATTACK {
        return smashline::original_status(Init, weapon, *WEAPON_PACKUN_POISONBREATH_STATUS_KIND_SHOOT)(weapon);
    }
    return 0.into();
}
pub fn install() {
    smashline::Agent::new("packun")
        .acmd("game_catchspecial", game_catchspecial,Priority::Default)
        .acmd("effect_catchspecial", effect_catchspecial,Priority::Default)
        .acmd("sound_catchspecial", sound_catchspecial,Priority::Default)
        .acmd("expression_catchspecial", expression_catchspecial,Priority::Default)
    .install();
    smashline::Agent::new("packun_poisonbreath")
        .status(Init, *WEAPON_PACKUN_POISONBREATH_STATUS_KIND_SHOOT, poison_shoot_init)
    .install();
}
