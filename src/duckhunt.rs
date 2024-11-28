use crate::imports::{imports_acmd::*, imports_status::empty_status};

pub const WEAPON_DUCKHUNT_RETICLE_STATUS_KIND_TARGET: i32 = 0x2;
pub const WEAPON_DUCKHUNT_RETICLE_INSTANCE_WORK_ID_COUNTDOWN_SHOOT: i32 = 0x10000004;
pub const WEAPON_DUCKHUNT_RETICLE_INSTANCE_WORK_ID_INT_TARGET: i32 = 0x1000000D; //F?
pub const WEAPON_DUCKHUNT_RETICLE_INSTANCE_WORK_ID_INT_TARGET_STEP: i32 = 0x1000000F; 
pub const TARGET_STEP_WAIT: i32 = 0;
pub const TARGET_STEP_LOCKED: i32 = 1;
pub const TARGET_STEP_SHOOT: i32 = 2;
pub const TARGET_STEP_RETURN: i32 = 3;
pub const TARGET_STEP_SHOOT_MAX_FRAME: i32 = 2;

unsafe extern "C" fn game_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
    }
    ArticleModule::change_status(agent.module_accessor, *FIGHTER_DUCKHUNT_GENERATE_ARTICLE_RETICLE, WEAPON_DUCKHUNT_RETICLE_STATUS_KIND_TARGET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
}

unsafe extern "C" fn effect_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        let color = WorkModule::get_int64(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) as i32;
        EFFECT_FOLLOW_arg11(agent,Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, color);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("duckhunt_feather"), false, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        let color = WorkModule::get_int64(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) as i32;
        EFFECT_FOLLOW_arg11(agent,Hash40::new("duckhunt_feather"), Hash40::new("duckneck"), -4, 0, 0, 0, 0, -90, 1, true, color);
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("duckhunt_feather"), false, true);
    }
}

unsafe extern "C" fn sound_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_duckhunt_appeal_h01"));
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_duckhunt_appeal_h04"));
        //macros::PLAY_SE(agent, Hash40::new("vc_duckhunt_duck_attack02"));
        macros::PLAY_STATUS(agent, Hash40::new("se_duckhunt_appeal_s04"));
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SE(agent, Hash40::new("se_duckhunt_appeal_h05"));
        sound!(agent, *MA_MSC_CMD_SOUND_STOP_SE_STATUS);
    }
}

unsafe extern "C" fn expression_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_openwing") as i64);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 53.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
    }
}

pub unsafe extern "C" fn catch_attack_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let has_special_n = !WorkModule::is_enable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    let has_special_s = !WorkModule::is_enable_transition_term_forbid(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    
    if !has_special_n || !has_special_s {
        fighter.status_CatchAttack_common(L2CValue::Hash40(Hash40::new("catch_attack")));
        return fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CatchAttack_Main as *const () as _));
    }
    
    let to_return = catch_attack_main_inner(fighter);
    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
    }

    return to_return;
}

unsafe extern "C" fn reticle_game_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::set_float(agent.module_accessor, 1.0, *WEAPON_DUCKHUNT_RETICLE_INSTANCE_WORK_ID_FLOAT_SIZE);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.8, 77, 32, 0, 72, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -0.9, 0.0, 0, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
}
unsafe extern "C" fn reticle_effect_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("duckhunt_target_impact"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, true);
    }
}
unsafe extern "C" fn reticle_sound_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_duckhunt_special_l03"));
    }
}

pub unsafe extern "C" fn reticle_target_pre(weapon: &mut L2CFighterCommon) -> L2CValue {
    smashline::original_status(Pre, weapon, *WEAPON_DUCKHUNT_RETICLE_STATUS_KIND_MOVE)(weapon)
}
pub unsafe extern "C" fn reticle_target_main(weapon: &mut L2CFighterCommon) -> L2CValue {
    AttackModule::sleep(weapon.module_accessor, true);

    WorkModule::set_int(weapon.module_accessor, 0, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    WorkModule::set_int(weapon.module_accessor, 0, WEAPON_DUCKHUNT_RETICLE_INSTANCE_WORK_ID_COUNTDOWN_SHOOT);
    WorkModule::set_int(weapon.module_accessor, 0, WEAPON_DUCKHUNT_RETICLE_INSTANCE_WORK_ID_INT_TARGET_STEP);

    VisibilityModule::set_whole(weapon.module_accessor, true);
    
    let owner = get_founder_boma(weapon);
    WorkModule::enable_transition_term_forbid(owner, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    WorkModule::enable_transition_term_forbid(owner, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);

    let target = get_grabbed_opponent_boma(owner);
    let target_id = (*target).battle_object_id;
    //println!("Target: {target_id}");
    WorkModule::set_int64(weapon.module_accessor, target_id as i64, WEAPON_DUCKHUNT_RETICLE_INSTANCE_WORK_ID_INT_TARGET);
    
    reticle_target_end(weapon);

    LinkModule::link(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT,target_id);
    LinkModule::set_model_constraint_pos_ort(weapon.module_accessor,*LINK_NO_CONSTRAINT,Hash40::new("top"),Hash40::new("hip"),(*CONSTRAINT_FLAG_POSITION) as u32,true);
    
    let test = EffectModule::req_follow(
        weapon.module_accessor,
        Hash40::new("duckhunt_target"),
        Hash40::new("top"),
        &VECTOR_ZERO,
        &VECTOR_ZERO,
        1.0,
        false,
        *EFFECT_SUB_ATTRIBUTE_NONE as u32,
        0,
        -1,
        *EFFECT_FLIP_NONE,
        1,
        false,
        true
    ) as u32;
    EffectModule::set_rate_last(weapon.module_accessor, 1.0);
    let team_color = FighterUtil::get_team_color(weapon.module_accessor) as i32;
    let mut team_color_as_vector = FighterUtil::get_effect_team_color(EColorKind(team_color as i32), Hash40::new("direction_effect_color"));
    let color_r = team_color_as_vector.value[0];
    let color_g = team_color_as_vector.value[1];
    let color_b = team_color_as_vector.value[2];
    EffectModule::set_rgb(weapon.module_accessor, test, color_r,color_g,color_b);

    SoundModule::play_se(weapon.module_accessor, Hash40::new("se_duckhunt_special_l02"), true, false, false, false, enSEType(0));

    return weapon.fastshift(L2CValue::Ptr(reticle_target_loop as *const () as _)); 
}
pub unsafe extern "C" fn reticle_target_loop(weapon: &mut L2CFighterCommon) -> L2CValue {
    let step = WorkModule::get_int(weapon.module_accessor, WEAPON_DUCKHUNT_RETICLE_INSTANCE_WORK_ID_INT_TARGET_STEP);
    if step == TARGET_STEP_RETURN {
        return 0.into();
    }

    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let target_id = WorkModule::get_int64(weapon.module_accessor, WEAPON_DUCKHUNT_RETICLE_INSTANCE_WORK_ID_INT_TARGET) as u32;
    if !sv_battle_object::is_active(target_id) || !sv_battle_object::is_active(owner_id) {
        WorkModule::set_int(weapon.module_accessor, 1, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    
    if step == TARGET_STEP_SHOOT {
        let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        if life == TARGET_STEP_SHOOT_MAX_FRAME {
            ControlModule::set_rumble(weapon.module_accessor, Hash40::new("rbkind_beams"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
            AttackModule::clear_all(weapon.module_accessor);
        }
        if WorkModule::count_down_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE, 0) {
            EffectModule::kill_kind(weapon.module_accessor, Hash40::new("duckhunt_target"), false, false);
            MotionModule::change_motion(weapon.module_accessor, Hash40::new("dummy"), 0.0, 1.0, false, 0.0, false, false);
            weapon.change_status(WEAPON_DUCKHUNT_RETICLE_STATUS_KIND_MOVE.into(), false.into());
        }
        return 0.into();
    }
    else if step == TARGET_STEP_LOCKED {
        if WorkModule::count_down_int(weapon.module_accessor, WEAPON_DUCKHUNT_RETICLE_INSTANCE_WORK_ID_COUNTDOWN_SHOOT, 0) {
            WorkModule::set_int(weapon.module_accessor, TARGET_STEP_SHOOT, WEAPON_DUCKHUNT_RETICLE_INSTANCE_WORK_ID_INT_TARGET_STEP);

            MotionAnimcmdModule::call_script_single(weapon.module_accessor, *WEAPON_ANIMCMD_GAME, Hash40::new("game_catchspecial"), -1);
            MotionAnimcmdModule::call_script_single(weapon.module_accessor, *WEAPON_ANIMCMD_EFFECT, Hash40::new("effect_catchspecial"), -1);
            MotionAnimcmdModule::call_script_single(weapon.module_accessor, *WEAPON_ANIMCMD_SOUND, Hash40::new("sound_catchspecial"), -1);
            //SoundModule::play_se(weapon.module_accessor, Hash40::new("se_duckhunt_special_l03"), true, false, false, false, enSEType(0));
            WorkModule::set_int(weapon.module_accessor, TARGET_STEP_SHOOT_MAX_FRAME, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        }
        return 0.into();
    }
    else if step == TARGET_STEP_WAIT {
        let target = sv_battle_object::module_accessor(target_id);
        let target_status = StatusModule::status_kind(target);
        let shoot_throw = [*FIGHTER_STATUS_KIND_DAMAGE_AIR,*FIGHTER_STATUS_KIND_DAMAGE_FLY,*FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL].contains(&target_status);
        let shoot_cut = [*FIGHTER_STATUS_KIND_CAPTURE_CUT,*FIGHTER_STATUS_KIND_CAPTURE_JUMP].contains(&target_status);
        let shoot_misc = !CaptureModule::is_capture(target) && target_status != *FIGHTER_STATUS_KIND_THROWN;
        let shoot = shoot_throw || shoot_cut || shoot_misc;
        if shoot 
        && WorkModule::get_int(weapon.module_accessor, WEAPON_DUCKHUNT_RETICLE_INSTANCE_WORK_ID_COUNTDOWN_SHOOT) == 0 {
            WorkModule::set_int(weapon.module_accessor, TARGET_STEP_LOCKED, WEAPON_DUCKHUNT_RETICLE_INSTANCE_WORK_ID_INT_TARGET_STEP);

            let hitstun = WorkModule::get_float(target, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
            let owner = get_founder_boma(weapon);
            PostureModule::set_lr(weapon.module_accessor, PostureModule::lr(owner));

            let delay = if shoot_throw {(hitstun*0.5).floor()} else if shoot_cut {1.0} else {5.0};
            AttackModule::sleep(weapon.module_accessor, false);
            WorkModule::set_int(weapon.module_accessor, delay.max(1.0) as i32, WEAPON_DUCKHUNT_RETICLE_INSTANCE_WORK_ID_COUNTDOWN_SHOOT);
        }
    }

    0.into()
}

pub unsafe extern "C" fn reticle_target_end(weapon: &mut L2CFighterCommon) -> L2CValue {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    if sv_battle_object::is_active(owner_id) {
        let owner = get_founder_boma(weapon);
        WorkModule::unable_transition_term_forbid(owner, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
        WorkModule::unable_transition_term_forbid(owner, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    }

    if LinkModule::is_link(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT) {
        LinkModule::unlink(weapon.module_accessor,*WEAPON_LINK_NO_CONSTRAINT);
    }
    LinkModule::remove_model_constraint(weapon.module_accessor,true);
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("duckhunt_target"), false, false);
    0.into()
}

pub fn install() {
    smashline::Agent::new("duckhunt")
        .acmd("game_catchspecial", game_catchspecial,Priority::Default)
        .acmd("effect_catchspecial", effect_catchspecial,Priority::Default)
        .acmd("sound_catchspecial", sound_catchspecial,Priority::Default)
        .acmd("expression_catchspecial", expression_catchspecial,Priority::Default)
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_uniq)
    .install();
    smashline::Agent::new("duckhunt_reticle")
        .acmd("game_catchspecial", reticle_game_catchspecial,Priority::Default)
        .acmd("effect_catchspecial", reticle_effect_catchspecial,Priority::Default)
        .acmd("sound_catchspecial", reticle_sound_catchspecial,Priority::Default)
        .status(Pre, WEAPON_DUCKHUNT_RETICLE_STATUS_KIND_TARGET, reticle_target_pre)
        .status(Main, WEAPON_DUCKHUNT_RETICLE_STATUS_KIND_TARGET, reticle_target_main)
        .status(End, WEAPON_DUCKHUNT_RETICLE_STATUS_KIND_TARGET, reticle_target_end)
    .install();
}
