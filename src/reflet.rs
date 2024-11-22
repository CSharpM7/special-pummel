use crate::imports::imports_acmd::*;
use crate::imports::imports_status::*;


pub const FIGHTER_REFLET_STATUS_CATCH_FLAG_HEAL: i32 = 0x2100000D;
pub const FIGHTER_REFLET_STATUS_CATCH_WORK_INT_REHIT: i32 = 0x11000004;
pub const FIGHTER_REFLET_STATUS_CATCH_WORK_INT_HEAL_COUNTDOWN: i32 = 0x11000005;
pub const FIGHTER_REFLET_STATUS_CATCH_FLAG_HEAL_EFFECT: i32 = 0x11000006;

unsafe extern "C" fn game_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    FT_MOTION_RATE_RANGE(agent,1.0,22.0,18.0);
    frame(agent.lua_state_agent, 22.0);
    FT_MOTION_RATE(agent,1.0);
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor,*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE) {
            WorkModule::on_flag(agent.module_accessor, FIGHTER_REFLET_STATUS_CATCH_FLAG_HEAL);
            let mut current_frame = 22.0;
            for f in 0..7 {
                if macros::is_excute(agent) {
                    macros::ATTACK(agent, 1, 1, Hash40::new("throw"), 2.0, 60, 100, 100, 0, 8.0, 0.0, -1.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 8, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
                }
                if macros::is_excute(agent) {
                    current_frame+=6.0;
                    if current_frame >= 53.0 {
                        break;
                    }
                }
                wait(agent.lua_state_agent, 6.0);
            }
            frame(agent.lua_state_agent, 54.0);
        }
    }
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, FIGHTER_REFLET_STATUS_CATCH_FLAG_HEAL);
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_catchspecial(agent: &mut L2CAgentBase) {
    let mut effect_scale = 1.0;
    if macros::is_excute(agent) {
        let captured_boma = get_grabbed_opponent_boma(agent.module_accessor);
        effect_scale = WorkModule::get_param_float(captured_boma, hash40("effect_scale"), 0);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor,*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE) {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("reflet_rizaia"), Hash40::new("top"), 0, 5, 14, 0, 0, 0, effect_scale, true);
        }
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("reflet_rizaia_capture"), Hash40::new("top"), 0, 5, 15, 0, 0, 0, effect_scale, true);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("reflet_rizaia"), -1);
    }
}
unsafe extern "C" fn sound_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor,*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE) {
            PLAY_SE(agent, Hash40::new("se_reflet_special_l01"));
        }
        else {
            PLAY_SEQUENCE(agent, Hash40::new("seq_reflet_rnd_special_empty"));
        }
    }
    wait(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor,*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE) {
            PLAY_SE(agent, Hash40::new("vc_reflet_special_l02"));
        }
    }
}

unsafe extern "C" fn expression_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor,*FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        else {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_blank"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    wait(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_elecattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}
/*
STATUS
*/
pub unsafe extern "C" fn catch_attack_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let to_return = catch_attack_main_inner(fighter);

    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        let object = sv_system::battle_object(fighter.lua_state_agent);
        let robin : *mut Fighter = std::mem::transmute(object);
        FighterSpecializer_Reflet::change_hud_kind(robin,*FIGHTER_REFLET_MAGIC_KIND_RIZAIA);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_REFLET_MAGIC_KIND_RIZAIA, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_LAST_USED_MAGIC_KIND);
        let magic = WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT);
        let can_suck = magic > 0;
        WorkModule::set_flag(fighter.module_accessor, !can_suck, *FIGHTER_REFLET_INSTANCE_WORK_ID_FLAG_SPECIAL_FAILURE);
        println!("Can suck: {can_suck}");
        if can_suck {
            let robin_module = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
            WorkModule::dec_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_SPECIAL_LW_CURRENT_POINT);
            FighterSpecializer_Reflet::change_grimoire(robin_module,*FIGHTER_REFLET_MAGIC_KIND_RIZAIA);
            if magic-1 == 0 {
                FighterSpecializer_Reflet::set_flag_to_table(robin_module,*FIGHTER_REFLET_MAGIC_KIND_RIZAIA,true,*FIGHTER_REFLET_INSTANCE_WORK_ID_INT_THROWAWAY_TABLE);
            }
            let entry_count = lua_bind::FighterManager::entry_count(singletons::FighterManager());
            let rehit_param = format!("special_lw_serial_hit_frame_{}",entry_count);
            let rehit_rate = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), hash40(rehit_param.as_str()));
            WorkModule::set_int(fighter.module_accessor, rehit_rate, FIGHTER_REFLET_STATUS_CATCH_WORK_INT_REHIT);
            println!("REHIT: {rehit_rate} Param: {rehit_param}");

            WorkModule::off_flag(fighter.module_accessor, FIGHTER_REFLET_STATUS_CATCH_FLAG_HEAL);
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_REFLET_STATUS_CATCH_FLAG_HEAL_EFFECT);
            WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_REFLET_STATUS_CATCH_WORK_INT_HEAL_COUNTDOWN);
            
            fighter.status_CatchAttack_common(L2CValue::Hash40(Hash40::new("catch_special")));
            return fighter.sub_shift_status_main(L2CValue::Ptr(catch_attack_loop_uniq as *const () as _));
        }
    }
    
    to_return
}

pub unsafe extern "C" fn catch_attack_loop_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let rehit = WorkModule::get_int(fighter.module_accessor, FIGHTER_REFLET_STATUS_CATCH_WORK_INT_REHIT);
    if AttackModule::is_attack(fighter.module_accessor, 1, false) 
    && rehit > 0 {
        let attack_data = AttackModule::attack_data(fighter.module_accessor, 1, false);
        let attack_power = (*attack_data).power;
        WorkModule::set_float(fighter.module_accessor,attack_power, *FIGHTER_REFLET_STATUS_SPECIAL_LW_CAPTURE_FLOAT_HEAL_DAMAGE);
        AttackModule::set_serial_hit_frame(fighter.module_accessor, 1, rehit as u32);
    }

    return catch_special_main_loop(fighter);
}
pub unsafe extern "C" fn catch_attack_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let original = smashline::original_status(Exec, fighter, *FIGHTER_STATUS_KIND_CATCH_ATTACK)(fighter);
    if !WorkModule::is_flag(fighter.module_accessor,FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        return original;
    }
    let catch_eff = WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_CATCH_EFFECT_HANDLE) as u32;
    if EffectModule::is_exist_effect(fighter.module_accessor, catch_eff) {
        EffectModule::set_alpha(fighter.module_accessor, catch_eff,0.25);
    }

    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_REFLET_STATUS_CATCH_FLAG_HEAL) {
        if WorkModule::count_down_int(fighter.module_accessor, FIGHTER_REFLET_STATUS_CATCH_WORK_INT_HEAL_COUNTDOWN, 0) {
            let rehit = WorkModule::get_int(fighter.module_accessor, FIGHTER_REFLET_STATUS_CATCH_WORK_INT_REHIT);
            WorkModule::set_int(fighter.module_accessor, rehit, FIGHTER_REFLET_STATUS_CATCH_WORK_INT_HEAL_COUNTDOWN);

            let is_one_on_one = smash_rs::app::FighterCutInManager::is_one_on_one_including_thrown(&*(fighter.module_accessor as *const smash_rs::app::BattleObjectModuleAccessor));
            let one_on_one_mul = if !is_one_on_one {1.2} else 
            {1.0};// 0x1af80fd893 at 1? lua_bind::FighterManager::one_on_one_ratio(singletons::FighterManager());
            let damage = WorkModule::get_float(fighter.module_accessor,*FIGHTER_REFLET_STATUS_SPECIAL_LW_CAPTURE_FLOAT_HEAL_DAMAGE);

            let mut heal = 0.0;
            let current_damage = DamageModule::damage(fighter.module_accessor, 0);
            if current_damage > 0.0 {
                let capture_boma = get_grabbed_opponent_boma(fighter.module_accessor);
                let capture_damage = DamageModule::damage(capture_boma, 0);
                
                let heal_base = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_damage_recovery_basic"))*0.01;
                let damage_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_damage_recovery"));
                let heal_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("special_lw_damage_recovery_upper_limit"));

                let difference = (current_damage-capture_damage).max(0.0);
                let different_add = (difference * damage_mul).min(heal_max);
                heal = (damage * heal_base) + different_add;
                println!("{heal} = ({damage} * {heal_base}) + ({difference} * {damage_mul})>{different_add}");
                DamageModule::heal(fighter.module_accessor, -heal, 0);
            }
            let mut effect_handle = WorkModule::get_int(fighter.module_accessor,FIGHTER_REFLET_STATUS_CATCH_FLAG_HEAL_EFFECT) as u32;
            let has_heal_effect = EffectModule::is_exist_effect(fighter.module_accessor, effect_handle);
            if heal > 0.0 && current_damage > 0.0 {        
                notify_event_msc_cmd!(fighter, 
                    Hash40::new_raw(0x148e71ec03),
                    has_heal_effect,
                    Hash40::new_raw(0x15fbd4c54c),
                );
            }
            if !has_heal_effect {
                effect_handle = EffectModule::req_follow(
                    fighter.module_accessor,
                    Hash40::new("sys_recovery"),
                    Hash40::new("top"),
                    &VECTOR_ZERO,
                    &VECTOR_ZERO,
                    1.0,
                    true,
                    *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32,
                    0,
                    -1,
                    *EFFECT_FLIP_NONE,
                    1,
                    false,
                    true
                ) as u32;
                WorkModule::set_int(fighter.module_accessor,effect_handle as i32,FIGHTER_REFLET_STATUS_CATCH_FLAG_HEAL_EFFECT);
            }
            SoundModule::play_se(fighter.module_accessor, Hash40::new("se_reflet_special_l03"), true, false, false, false, enSEType(0));
        }
    }

    original
}

pub unsafe extern "C" fn catch_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let original = smashline::original_status(End, fighter, *FIGHTER_STATUS_KIND_CATCH_ATTACK)(fighter);
    if WorkModule::is_flag(fighter.module_accessor,FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        let catch_eff = WorkModule::get_int(fighter.module_accessor, *FIGHTER_REFLET_INSTANCE_WORK_ID_INT_CATCH_EFFECT_HANDLE) as u32;
        if EffectModule::is_exist_effect(fighter.module_accessor, catch_eff) {
            EffectModule::set_alpha(fighter.module_accessor, catch_eff,1.0);
        }
    }
    return original;
}

pub fn install() {
    smashline::Agent::new("reflet")
        .acmd("game_catchspecial", game_catchspecial,Priority::Default)
        .acmd("effect_catchspecial", effect_catchspecial,Priority::Default)
        .acmd("sound_catchspecial", sound_catchspecial,Priority::Default)
        .acmd("expression_catchspecial", expression_catchspecial,Priority::Default)

        .status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_uniq)
        .status(Exec, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_exec)
        .status(End, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_end)
    .install();
}
