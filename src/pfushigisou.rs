use crate::imports::imports_acmd::*;
use crate::imports::imports_status::*;

unsafe extern "C" fn game_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        //collision_attr_normal_poison 0x1B65EF3F3D 0x1C9A85BCD9
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.2, 361, 100, 30, 0, 5.0, 0.0, 8.0, 13.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal_poison"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
        AttackModule::set_poison_param(agent.module_accessor, /*ID*/ 0, /*Frames*/ 300, /*Rehit*/ 30, /* Damage*/ 1.2, /*Unk*/ false);
        //AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        //macros::EFFECT_FLIP(agent, Hash40::new("sys_run_smoke"), Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        //
        macros::EFFECT_FOLLOW(agent, Hash40::new("pfushigisou_atk_hi4"), Hash40::new("top"), 0, 13, 5, 0, 0, 0, 0.65, true);
        LAST_EFFECT_SET_COLOR(agent, 0.15, 0.0, 1.0);
    }
}

unsafe extern "C" fn sound_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pfushigisou_smash_h02"));
        macros::PLAY_SE(agent, Hash40::new("vc_pfushigisou_attack06"));
    }
}

unsafe extern "C" fn expression_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("weapon") as i64, hash40("weapon_normal") as i64);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

/*
STATUS
*/
pub unsafe extern "C" fn catch_wait_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let captured_boma = &mut *get_grabbed_opponent_boma(fighter.module_accessor);
    let mut waist = *FIGHTER_WAIST_SIZE_M;
    if utility::get_category(captured_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        waist = WorkModule::get_param_int(captured_boma, hash40("param_motion"), hash40("waist_size"));
    }

    let motion = match waist {
        0x2 => Hash40::new("catch_wait_l"),
        0x1 => Hash40::new("catch_wait_m"),
        _ => Hash40::new("catch_wait"),
    };

    let original = fighter.status_CatchWait_common(L2CValue::Hash40(motion)); 
    MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);

    return original;
}
pub unsafe extern "C" fn catch_attack_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if catch_attack_check_special_input(fighter) {
        if catch_attack_check_special_anim(fighter) {
            catch_special_main(fighter);
            
            let captured_boma = &mut *get_grabbed_opponent_boma(fighter.module_accessor);

            let mut clatter = ControlModule::get_clatter_time(captured_boma, 0);
            ControlModule::set_clatter_time(captured_boma, clatter*0.75,0);

            /*
            //Set final...
            LinkModule::send_event_nodes(fighter.module_accessor, *WEAPON_PTRAINER_PTRAINER_LINK_NO_POKEMON, Hash40::new_raw(0x97824a0a0), 0);
            */
            let trainer_id = LinkModule::get_parent_id(fighter.module_accessor, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
            if trainer_id != OBJECT_ID_NULL {
                let trainer = sv_battle_object::module_accessor(trainer_id as u32);
                StatusModule::change_status_request(trainer, *WEAPON_PTRAINER_PTRAINER_STATUS_KIND_REACTION, false);
                let lr = PostureModule::lr(fighter.module_accessor);
                let motion = if lr < 0.0 {Hash40::new("special_r")} else {Hash40::new("special_l")};
                //special_f
                MotionModule::change_motion(trainer, motion, 0.0, 1.0, false, 0.0, false, false);
            }

            let mut waist = *FIGHTER_WAIST_SIZE_M;
            if utility::get_category(captured_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                waist = WorkModule::get_param_int(captured_boma, hash40("param_motion"), hash40("waist_size"));
            }

            let motion = match waist {
                0x2 => Hash40::new("catch_special_l"),
                0x1 => Hash40::new("catch_special_m"),
                _ => Hash40::new("catch_special"),
            };
            fighter.status_CatchAttack_common(L2CValue::Hash40(motion));
            MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);
            return fighter.sub_shift_status_main(L2CValue::Ptr(catch_special_main_loop as *const () as _));
        }
    }
    return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_CATCH_ATTACK)(fighter);
}

pub unsafe extern "C" fn debug(boma: *mut BattleObjectModuleAccessor) {
    if MotionModule::frame(boma) < 1.0 {
        let status = StatusModule::status_kind(boma);
        println!("status: {status}");
    }
}
pub unsafe extern "C" fn ivy_frame(fighter: &mut L2CFighterCommon)  {
    let trainer_id = LinkModule::get_parent_id(fighter.module_accessor, *FIGHTER_POKEMON_LINK_NO_PTRAINER, true) as u32;
    if trainer_id != OBJECT_ID_NULL {
        let trainer = sv_battle_object::module_accessor(trainer_id as u32);
        debug(trainer);
    }
}
pub unsafe extern "C" fn trainer_frame(weapon: &mut L2CWeaponCommon)  {
    debug(weapon.module_accessor);
}

pub fn install() {
    smashline::Agent::new("pfushigisou")
        .acmd("game_catchspecial", game_catchspecial,Priority::Default)
        .acmd("effect_catchspecial", effect_catchspecial,Priority::Default)
        .acmd("sound_catchspecial", sound_catchspecial,Priority::Default)
        .acmd("expression_catchspecial", expression_catchspecial,Priority::Default)
        
        .acmd("game_catchspecialm", game_catchspecial,Priority::Default)
        .acmd("effect_catchspecialm", effect_catchspecial,Priority::Default)
        .acmd("sound_catchspecialm", sound_catchspecial,Priority::Default)
        .acmd("expression_catchspecialm", expression_catchspecial,Priority::Default)

        .acmd("game_catchspeciall", game_catchspecial,Priority::Default)
        .acmd("effect_catchspeciall", effect_catchspecial,Priority::Default)
        .acmd("sound_catchspeciall", sound_catchspecial,Priority::Default)
        .acmd("expression_catchspeciall", expression_catchspecial,Priority::Default)

        .status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_uniq)
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_WAIT, catch_wait_uniq)
    .install();
}
