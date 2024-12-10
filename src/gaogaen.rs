use crate::imports::imports_acmd::*;
use crate::imports::imports_status::*;


pub const REVENGE_DAMAGE: f32 = 5.0;
pub const REVENGE_BASE_FACTOR: f32 = 0.5;
pub const REVENGE_INITIAL_ADD: f32 = 0.75;
/*
Incin uses a modified formula to his revenge, so this move
rewards less meter if not stacking revenge
*/
pub const FIGHTER_GAOGAEN_STATUS_CATCH_FLAG_REVENGE: i32 = 0x2100000E;

unsafe extern "C" fn game_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_STATUS_CATCH_ATTACK_FLAG_DISABLE_CUT);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_GAOGAEN_STATUS_CATCH_FLAG_REVENGE);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.4, 361, 0, 10, 60, 16.0, 0.0, 11.0, -1.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, FIGHTER_STATUS_CATCH_ATTACK_FLAG_DISABLE_CUT);
    }
}

unsafe extern "C" fn effect_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        //macros::EFFECT(agent, Hash40::new("gaogaen_revenge_start"), Hash40::new("top"), -2, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("gaogaen_belt_fire_appeal"), Hash40::new("feeler"), 0, 3, 0, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn sound_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_gaogaen_special_l01"));
        //macros::PLAY_SE(agent, Hash40::new("se_gaogaen_special_l02"));
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_gaogaen_special_l03"));
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_gaogaen_rnd_attackappeal01"));
    }
}

unsafe extern "C" fn expression_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 0);
    }
}

/*
STATUS
*/

pub unsafe fn add_revenge(fighter: &mut L2CFighterCommon) {
    
    let has_revenge = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE);
    let rate = WorkModule::get_float(fighter.module_accessor,*FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLOAT_REVENGE_RATE);

    let base = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_revenge_rate_base_add"))*REVENGE_BASE_FACTOR;
    let dmg_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_revenge_rate_attack_mul_add"));
    let max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_revenge_rate_max"));

    let damage_rate = (base+dmg_mul*REVENGE_DAMAGE);

    let mut new_rate = 0.0;

    if !has_revenge {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_REVENGE);
        new_rate = (damage_rate) + (REVENGE_INITIAL_ADD); //was 1.0
    }
    else {
        new_rate = rate + (damage_rate* (1.0-base*(rate-1.0)));
    }
    new_rate = new_rate.min(max);

    let max_time = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_revenge_time")) * 60.0;
    WorkModule::set_int(fighter.module_accessor, max_time as i32, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_INT_REVENGE_TIMER);
    WorkModule::set_float(fighter.module_accessor,new_rate,*FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLOAT_REVENGE_RATE);

    #[cfg(feature = "devhook")]
    println!("Revenge: {rate} > {new_rate}");
    
    MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT, Hash40::new("effect_speciallwrevenge"), -1);
}

pub unsafe extern "C" fn catch_attack_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let to_return = catch_attack_main_inner(fighter);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        add_revenge(fighter);

        let opponent_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) as u32;
        //WorkModule::set_int64(fighter.module_accessor, opponent_id as i64, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_INT_SPECIAL_PUMMEL_ID);
        let opponent = get_grabbed_opponent_boma(fighter.module_accessor);
        StatusModule::change_status_force(opponent, *FIGHTER_STATUS_KIND_SAVING_DAMAGE, true);
        
        fighter.change_status(FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
        return fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CatchAttack_Main as *const () as _));
    }
    
    return to_return;
}

pub unsafe extern "C" fn catch_attack_exec_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        if WorkModule::is_flag(fighter.module_accessor,FIGHTER_GAOGAEN_STATUS_CATCH_FLAG_REVENGE) {
            add_revenge(fighter);
            WorkModule::off_flag(fighter.module_accessor,FIGHTER_GAOGAEN_STATUS_CATCH_FLAG_REVENGE);
        }
    }
    return 0.into();
}

pub unsafe extern "C" fn special_lw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let original = smashline::original_status(Main, fighter, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_LW_HIT)(fighter);
    if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_CATCH_ATTACK {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_special"), 0.0, 1.0, false, 0.0, false, false);
    }
    return original;
}

pub fn install() {
    smashline::Agent::new("gaogaen")
        .acmd("game_catchspecial", game_catchspecial,Priority::Default)
        .acmd("effect_catchspecial", effect_catchspecial,Priority::Default)
        .acmd("sound_catchspecial", sound_catchspecial,Priority::Default)
        .acmd("expression_catchspecial", expression_catchspecial,Priority::Default)

        .status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_uniq)
        .status(Main, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_LW_HIT, special_lw_main)
    .install();
}
