use crate::imports::imports_acmd::*;
use crate::imports::imports_status::*;


extern "C" {
    #[link_name = "\u{1}_ZN3app44FighterPikminLinkEventWeaponPikminConstraint13new_l2c_tableEv"]
    fn FighterPikminLinkEventWeaponPikminConstraint__new_l2c_table() -> L2CValue;

    #[link_name = "\u{1}_ZN3app46FighterPikminLinkEventWeaponPikminChangeMotion13new_l2c_tableEv"]
    fn FighterPikminLinkEventWeaponPikminChangeMotion__new_l2c_table() -> L2CValue;

    #[link_name = "\u{1}_ZN3app46FighterPikminLinkEventWeaponPikminChangeStatus13new_l2c_tableEv"]
    fn FighterPikminLinkEventWeaponPikminChangeStatus__new_l2c_table() -> L2CValue;

    #[link_name = "\u{1}_ZN3app40FighterPikminLinkEventWeaponPikminSyncLR13new_l2c_tableEv"]
    fn FighterPikminLinkEventWeaponPikminSyncLR__new_l2c_table() -> L2CValue;

    #[link_name = "\u{1}_ZN3app40FighterPikminLinkEventWeaponPikminSyncPos13new_l2c_tableEv"]
    fn FighterPikminLinkEventWeaponPikminSyncPos__new_l2c_table() -> L2CValue;
    
    #[link_name = "\u{1}_ZN3app40FighterPikminLinkEventWeaponPikminOnFlag13new_l2c_tableEv"]
    fn FighterPikminLinkEventWeaponPikminOnFlag__new_l2c_table() -> L2CValue;

    #[link_name = "\u{1}_ZN3app14LinkEventThrow13new_l2c_tableEv"]
    fn new_event_table() -> L2CValue;

    #[link_name = "\u{1}_ZN3app8lua_bind31LinkEvent__store_l2c_table_implEPKNS_9LinkEventE"]
    fn store_event_table(event: *const app::LinkEvent) -> L2CValue;
}
unsafe extern "C" fn link_event_store_l2c_table(fighter: &mut L2CFighterCommon, link_no: L2CValue, event: L2CValue) -> L2CValue {
    let callable: extern "C" fn() -> *mut app::LinkEvent = std::mem::transmute(event["new_instance_lua_"].get_ptr());
    let link_event = callable();
    lua_bind::LinkEvent::load_from_l2c_table(link_event, &event);
    LinkModule::send_event_parents_struct(fighter.module_accessor, link_no.get_i32(), link_event);
    let ret = store_event_table(link_event);
    let deleter: extern "C" fn(*mut app::LinkEvent) = std::mem::transmute(*((*(link_event as *const u64) + 0x8) as *const u64));
    deleter(link_event);
    ret
}
pub unsafe  fn pikmin_variantion_to_string(variation: i32) -> &'static str {
    /*
    
    WEAPON_PIKMIN_PIKMIN_VARIATION_BLUE: 0x2,
    WEAPON_PIKMIN_PIKMIN_VARIATION_RED: 0x0,
    WEAPON_PIKMIN_PIKMIN_VARIATION_VIOLET: 0x4,
    WEAPON_PIKMIN_PIKMIN_VARIATION_WHITE: 0x3,
    WEAPON_PIKMIN_PIKMIN_VARIATION_YELLOW: 0x1,
     */
    return match variation {
        0 => {"r"}
        1 => {"y"}
        2 => {"b"}
        3 => {"w"}
        4 => {"v"}
        _ => {"?"}
    };
}

pub const FIGHTER_PIKMIN_STATUS_THROW_WORK_INT_CHARGE_COUNTDOWN: i32 = 0x1100000E;
pub const FIGHTER_PIKMIN_STATUS_THROW_WORK_INT_CHARGE_COUNT: i32 = 0x1100000F;
pub const FIGHTER_PIKMIN_STATUS_THROW_WORK_INT_CHARGE_COUNT_MAX: i32 = 0x11000010;
pub const FIGHTER_PIKMIN_INSTANCE_WORK_INT_CHARGE_TARGET_ID: i32 = 0x100000CA; //FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT can be used for throws
pub const FIGHTER_PIKMIN_INSTANCE_WORK_INT_CHARGE_COUNTDOWN: i32 = 0x200000CB;
pub const FIGHTER_PIKMIN_STATUS_CATCH_FLAG_CHARGE: i32 = 0x21000014; //0x2100000F
pub const FIGHTER_PIKMIN_STATUS_CATCH_FLAG_START_THROW: i32 = 0x21000013;
pub const FIGHTER_PIKMIN_STATUS_THROW_FLAG_DISABLE_CLATTER: i32 = 0x21000010;
pub const FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLOAT_CHARGE_TARGET_X: i32 = 0x4E;
pub const FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLOAT_CHARGE_TARGET_Y: i32 = 0x4F;

pub const WEAPON_PIKMIN_PIKMIN_MAX_CHARGE_RANGE: f32 = 200.0;
pub const WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_FLAG_DISABLE_CLATTER: i32 = 0x21000010;
pub const WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_DISABLE_CHARGE_CHECK: i32 = 0x2000000E;
pub const WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_TARGET_ANIM: i32 = 0x1000001D;
pub const WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_TARGET_FRAME: i32 = 0x15;
/*
    FIGHTER_STATUS_CATCH_CUT_WORK_INT_SITUATION: 0x11000005,
    FIGHTER_STATUS_CATCH_DASH_WORK_INT_CATCH_TURN_FRAME: 0x11000005,
    FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT: 0x2100000C,
    FIGHTER_STATUS_CATCH_PULL_FLAG_UNNECESSARY_CLEAR_TRANS: 0x2100000C,
    FIGHTER_STATUS_CATCH_PULL_WORK_INT_MOTION_KIND: 0x11000005,
    FIGHTER_STATUS_CATCH_WAIT_WORK_INT_IK_LEFT_JOINT_ID: 0x11000007,
    FIGHTER_STATUS_CATCH_WAIT_WORK_INT_IK_RIGHT_JOINT_ID: 0x11000008,
    FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS: 0x11000006,
    FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND: 0x11000005,

FIGHTER_STATUS_THROW_FLAG_INVINCIBLE: 0x2100000D,
    FIGHTER_STATUS_THROW_FLAG_START_AIR: 0x2100000C,
    FIGHTER_STATUS_THROW_FLAG_STOP: 0x2100000E,
    FIGHTER_STATUS_THROW_WORK_FLOAT_MOTION_RATE: 0x1000007,
    FIGHTER_STATUS_THROW_WORK_INT_STOP_FRAME: 0x1100000D,
    FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP: 0x1100000B,
    FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO: 0x1100000C,
    FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT: 0x1100000A,
    
    WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_FLAG_MOTION_STARTED: 0x21000000,
    WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLOAT_THROW_POWER_UP_DEFAULT: 0x14,
    WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_ACTION_COMP_FRAME: 0x1000000D,
    WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_CATCH_TARGET_BATTLE_OBJECT_ID: 0x1000001C,
    WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_DROWN_COUNT: 0x1000000F,
    WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_DROWN_DEAD_FRAME: 0x1000001B,
    WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_HOLD_INDEX: 0x10000018,
    WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_HOLD_NUM: 0x10000019,
    WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_HP: 0x1000000B,
    WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_JUMP_COUNT: 0x1000000E,
    WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_OWNER_CONDITION_CURRENT: 0x10000011,
    WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_OWNER_CONDITION_FOLLOW: 0x10000012,
    WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_OWNER_OPTION_FLAG_FOLLOW: 0x10000013,
    WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_SPECIAL_S_ITEM_OBJECT_ID: 0x10000017,
    WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION: 0x1000000A,
    
    FIGHTER_PIKMIN_INSTANCE_ATTACK_AIR_WORK_FLAG_FALL_SPECIAL: 0x200000E7,
    FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLAG_CATCH_CUT: 0x200000E8,
    FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLAG_CHANGE_CATCH_MOTION_RATE: 0x200000EA,
    FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLAG_IS_SPYCLOAK: 0x200000EE,
    FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_AIR: 0x200000E1,
    FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_GROUND: 0x200000E2,
    FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLAG_MTRANS_SMPL_MOTION_END: 0x200000E3,
    FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLAG_PIKMIN_CATCH_DASH_STATUS: 0x200000EB,
    FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLOAT_CATCH_MOTION_RATE: 0x4D,
    FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_FRAME: 0x4C,
    FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM: 0x100000C5,
    FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0: 0x100000C6,
    FIGHTER_PIKMIN_INSTANCE_WORK_INT_THROW_PIKMIN_VARIATION: 0x100000C9,
    FIGHTER_PIKMIN_INSTANCE_WORK_INT_WING_PIKMIN_END_EFFECT_HANDLE: 0x100000C1,
    FIGHTER_PIKMIN_INSTANCE_WORK_INT_WING_PIKMIN_END_FRAME_COUNT: 0x100000C2,
    
    FIGHTER_PIKMIN_STATUS_CATCH_FLAG_CATCH_DASH_CANCEL_TURN: 0x21000012,
    FIGHTER_PIKMIN_STATUS_CATCH_FLAG_NO_CHANGE_PIKMIN_STATUS: 0x2100000F,
    FIGHTER_PIKMIN_STATUS_CATCH_FLAG_ONE_MAN_SHOW: 0x2100000E,
    FIGHTER_PIKMIN_STATUS_CATCH_FLAG_WEAPON_PIKMIN_IS_CATCH_RETURN_END: 0x21000010,
*/
/*
ACMD
 */
unsafe extern "C" fn game_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0); 
    FT_MOTION_RATE_RANGE(agent, 1.0, 18.0, 16.0);
    frame(agent.lua_state_agent, 18.0); 
    FT_MOTION_RATE(agent,1.0);
    frame(agent.lua_state_agent, 20.0); 
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
    }
    frame(agent.lua_state_agent, 21.0); 
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_PIKMIN_STATUS_CATCH_FLAG_START_THROW);
    }
    frame(agent.lua_state_agent, 23.0); 
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_STATUS_CATCH_ATTACK_FLAG_DISABLE_CUT);
    }
    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
}

unsafe extern "C" fn effect_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("pikmin_order"), Hash40::new("s_antenna4"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {        
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2.75, 0, 0.0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        //macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        EffectModule::kill_kind(agent.module_accessor, Hash40::new("sys_direction"), false, false);
    }
}

unsafe extern "C" fn sound_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_pikmin_001"));
    }
}

unsafe extern "C" fn expression_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_awaken"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn pikmin_game_catchspecial(agent: &mut L2CAgentBase) {
    let mut variation = 0;
    let mut variation_as_str = "r";
    let mut is_sub = false;
    if macros::is_excute(agent) {
        variation = WorkModule::get_int(agent.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
        variation_as_str = pikmin_variantion_to_string(variation);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 30, 0, 10, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        //macros::ATTACK_ABS(agent, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 361, 0, 10, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        is_sub = WorkModule::get_int(agent.module_accessor,*WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_MOTION_START_DELAY_FRAME) != 0;
        //println!("{variation_as_str} is Locked and loaded: {}",!is_sub);
        //WorkModule::on_flag(agent.module_accessor, WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_FLAG_DISABLE_CLATTER);
    }
    frame(agent.lua_state_agent, 3.0);
    FT_MOTION_RATE_RANGE(agent,3.0,19.0,15.0);
    frame(agent.lua_state_agent, 19.0);
    FT_MOTION_RATE(agent,1.0);
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        if !is_sub {
            let target = WorkModule::get_int64(agent.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK);
            let target_group = WorkModule::get_int64(agent.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
            let target_no = WorkModule::get_int64(agent.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO);

            macros::ATK_HIT_ABS(agent, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        }
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        if !is_sub {
            let owner = get_owner_boma(agent);
            WorkModule::on_flag(owner, FIGHTER_PIKMIN_STATUS_CATCH_FLAG_CHARGE);
            WorkModule::set_int64(owner, OBJECT_ID_NULL as i64,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        }
    }
}

unsafe extern "C" fn pikmin_sound_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pikmin_special_l02"));
    }
}
unsafe extern "C" fn pikmin_game_spscharge_v(agent: &mut L2CAgentBase) {   
    if macros::is_excute(agent) {
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 7.0, 0.0, 3.0, 0.0, None, None, None,*COLLISION_KIND_MASK_HSR, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_ALL, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 4.5, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FI, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
        //macros::ATTACK(agent, 2, 0, Hash40::new("top"), 10.4, 361, 8, 0, 4, 4.5, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5.2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
        //set above id to 2?
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_SPECIAL_S_WORK_FLAG_LANDING);
    }
    frame(agent.lua_state_agent, 120.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}
unsafe extern "C" fn pikmin_game_spsgrabattack_v(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_PIKMIN);
    }
    loop {
        frame(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_PIKMIN);
        }
        frame(agent.lua_state_agent, 19.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
        
        agent.clear_lua_stack();
        sv_animcmd::wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

unsafe extern "C" fn pikmin_effect_spsgrabattack_v(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("pikmin_attach"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);

        macros::EFFECT(agent, Hash40::new("pikmin_hit_white"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 1.0);
    }
    loop {
        wait(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("pikmin_hit_white"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_COLOR(agent, 1.0, 0.0, 1.0);
        }
        wait(agent.lua_state_agent, 13.0);
    }
}

/*
STATUS
*/

pub unsafe extern "C" fn sync_lr_pikmin(fighter: &mut L2CFighterCommon, p: i32) -> bool {
    let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    let oLRmar = PostureModule::lr(fighter.module_accessor);
    let pikmin_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0+p) as u32;
    if pikmin_id == 0 {
        return false;
    }

    let link_node = *FIGHTER_PIKMIN_LINK_NO_PIKMIN;
    let is_link = LinkModule::link(fighter.module_accessor, link_node, pikmin_id as u32);
    if is_link & 1 != 0  {
        let pikmin_boma = sv_battle_object::module_accessor(pikmin_id as u32);
        let mut link_event = FighterPikminLinkEventWeaponPikminSyncLR__new_l2c_table();
        link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_sync_lr")));
        link_event["lr_"].assign(&L2CValue::F32(oLRmar));
        let object_id = fighter.global_table[OBJECT_ID].get_u32();
        link_event["sender_id_"].assign(&L2CValue::U32(object_id));
        link_event_store_l2c_table(fighter, link_node.into(), link_event);
        //LinkModule::set_attribute(fighter.module_accessor, link_node, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_STOP as u8}, true);
        //LinkModule::set_attribute(fighter.module_accessor, link_node, LinkAttribute{_address: *LINK_ATTRIBUTE_REFERENCE_PARENT_ATTACK_STOP as u8}, true);
        LinkModule::unlink(fighter.module_accessor, link_node);
        return true;
    }
    return false;
}

pub unsafe extern "C" fn sync_lr_pikmin_all(fighter: &mut L2CFighterCommon) -> bool {
    let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    let hold_pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
    if hold_pikmin_num <= 0 {return false;}
    let mut p = 0;
    for p in 0..hold_pikmin_num {
        if !sync_lr_pikmin(fighter,p) {
            break;
        }        
        //FighterSpecializer_Pikmin::hold_pikmin(olima, 3);
        //FighterSpecializer_Pikmin::update_hold_pikmin_param(olima);
    }
    return true;
}

pub unsafe extern "C" fn change_status_pikmin(fighter: &mut L2CFighterCommon, p: i32, status: i32, force: bool) -> bool {
    let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    let oLRmar = PostureModule::lr(fighter.module_accessor);
    let pikmin_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0+p) as u32;
    if pikmin_id == 0 {
        return false;
    }

    let link_node = *FIGHTER_PIKMIN_LINK_NO_PIKMIN;
    let is_link = LinkModule::link(fighter.module_accessor, link_node, pikmin_id as u32);
    if is_link & 1 != 0  {
        let pikmin_boma = sv_battle_object::module_accessor(pikmin_id as u32);
        let variation = WorkModule::get_int(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
        let variation_as_str = pikmin_variantion_to_string(variation);
        #[cfg(feature = "dev")]
        println!("Change {variation_as_str} Pikmin (#{p})");

        WorkModule::off_flag(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_IS_CATCH_FAILURE_GROUND_FOLLOW_FORCE);
        WorkModule::off_flag(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_IS_CATCH_FAILURE_WAIT_END);
        WorkModule::off_flag(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_IS_CATCH_RETURN_END);
         
        let mut link_event = FighterPikminLinkEventWeaponPikminChangeStatus__new_l2c_table();
        link_event["link_event_kind_"].assign(&L2CValue::Hash40(Hash40::new("fighter_pikmin_link_event_weapon_pikmin_change_status")));
        link_event["status_kind_"].assign(&L2CValue::I32(status));
        let object_id = fighter.global_table[OBJECT_ID].get_u32();
        link_event["sender_id_"].assign(&L2CValue::U32(object_id));
        link_event_store_l2c_table(fighter, link_node.into(), link_event);
        
        if force {StatusModule::change_status_force(pikmin_boma, status, false);}

        LinkModule::unlink(fighter.module_accessor, link_node);
        return true;
    }
    return false;
}

pub unsafe extern "C" fn change_status_pikmin_all(fighter: &mut L2CFighterCommon, status: i32, force: bool) -> bool {
    let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    let hold_pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
    if hold_pikmin_num <= 0 {return false;}
    let mut p = 0;
    for p in 0..hold_pikmin_num {
        if !change_status_pikmin(fighter,p,status, force) {
            break;
        }        
        //FighterSpecializer_Pikmin::hold_pikmin(olima, 3);
        //FighterSpecializer_Pikmin::update_hold_pikmin_param(olima);
    }
    return true;
}

pub unsafe extern "C" fn liberate_pikmin_all(fighter: &mut L2CFighterCommon) -> bool {
    let reduce_instead = false;
    let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    let hold_pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
    //println!("Freeing {hold_pikmin_num} pikmin");
    if hold_pikmin_num <= 0 {return false;}
    let mut p = 0;
    for p in 0..hold_pikmin_num {
        //println!("Pik: {p}");
        let pikmin_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0+p) as u32;
        let link_node = *FIGHTER_PIKMIN_LINK_NO_PIKMIN;
        let is_link = LinkModule::link(fighter.module_accessor, link_node, pikmin_id as u32);
        if is_link & 1 != 0  {
            //make em fall?
            LinkModule::unlink(fighter.module_accessor, link_node);
        }
        if reduce_instead {
            FighterSpecializer_Pikmin::reduce_pikmin_all(olima);
        } else {
            FighterSpecializer_Pikmin::liberty_pikmin_all(olima);
        }
    }
    return true;
}

pub unsafe extern "C" fn catch_attack_init_variables(fighter: &mut L2CFighterCommon) {
    WorkModule::set_int(fighter.module_accessor, OBJECT_ID_NULL as i32, FIGHTER_PIKMIN_INSTANCE_WORK_INT_CHARGE_TARGET_ID);
    WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_PIKMIN_STATUS_THROW_WORK_INT_CHARGE_COUNTDOWN);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_PIKMIN_STATUS_CATCH_FLAG_START_THROW); 

    let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    //FighterSpecializer_Pikmin::hold_pikmin(olima, 3);
    //FighterSpecializer_Pikmin::update_hold_pikmin_param(olima);
    let hold_pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
    //println!("Init Holding: {hold_pikmin_num}");
    WorkModule::set_int(fighter.module_accessor, hold_pikmin_num-1, FIGHTER_PIKMIN_STATUS_THROW_WORK_INT_CHARGE_COUNT);

    if hold_pikmin_num == 0 { return; }  
    let lead_pikmin_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0) as u32;
    if lead_pikmin_id == 0 { return; }

    let link_node = *FIGHTER_PIKMIN_LINK_NO_PIKMIN_THROW;
    let is_link = LinkModule::link(fighter.module_accessor, link_node, lead_pikmin_id as u32);
    if is_link & 1 != 0  {
        let pikmin_boma = sv_battle_object::module_accessor(lead_pikmin_id as u32);
        let capture_id = WorkModule::get_int(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_CATCH_TARGET_BATTLE_OBJECT_ID) as u32;
        if capture_id != OBJECT_ID_NULL {
            let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
            let target_pos = *PostureModule::pos(capture_boma);

            let mut clatter = ControlModule::get_clatter_time(capture_boma, 0);
            ControlModule::set_clatter_time(capture_boma, clatter*0.5,0);
            //println!("Has target {capture_id} at {}. Clatter: {clatter}*0.5",target_pos.x);
            
            WorkModule::set_int(fighter.module_accessor, capture_id as i32, FIGHTER_PIKMIN_INSTANCE_WORK_INT_CHARGE_TARGET_ID);
            WorkModule::set_int64(fighter.module_accessor, capture_id as i64, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
            if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_THROW {
                WorkModule::set_int(fighter.module_accessor, capture_id as i32, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
            }
            
        }
        LinkModule::unlink(fighter.module_accessor, link_node);
    } 
}
pub unsafe extern "C" fn catch_attack_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if catch_attack_check_special(fighter) {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL); 
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FORBID_CATCH_SPECIAL); 

        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM) <= 0 {
            return fighter.status_CatchAttack();
        }
        catch_attack_init_variables(fighter);
        catch_special_main(fighter);


        //Tell pikmin to throw
        let lead_pikmin_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0) as u32;
        
        let hold_pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
        //println!("Holding {hold_pikmin_num} pikmin");
        let mut p = 0;
        for p in 0..hold_pikmin_num {
            let link_node = *FIGHTER_PIKMIN_LINK_NO_PIKMIN_THROW;
            let pikmin_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0+p) as u32;
            let is_link = LinkModule::link(fighter.module_accessor, link_node, pikmin_id as u32);
            if is_link & 1 != 0  {
                let pikmin_boma = sv_battle_object::module_accessor(pikmin_id as u32);
                WorkModule::on_flag(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_STATUS_CATCH_WAIT_WORK_FLAG_THROW_F);

                if p == 0 {
                    let capture_id = WorkModule::get_int(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_CATCH_TARGET_BATTLE_OBJECT_ID) as u32;
                    if capture_id != OBJECT_ID_NULL {
                        let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
                        let capture_anim = MotionModule::motion_kind(capture_boma);
                        let capture_frame = MotionModule::frame(capture_boma);
                        WorkModule::set_int64(pikmin_boma, capture_anim as i64, WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_TARGET_ANIM);
                        WorkModule::set_float(pikmin_boma, capture_frame, WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_TARGET_FRAME);
                    }
                }
                LinkModule::unlink(fighter.module_accessor, link_node);
            } 
        }

        fighter.status_CatchAttack_common(L2CValue::Hash40(Hash40::new("catch_special")));
        //MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_special"), 0.0,1.0, false, 0.0, false, false);
        return fighter.sub_shift_status_main(L2CValue::Ptr(catch_attack_loop_uniq as *const () as _));
    }
    
    return fighter.status_CatchAttack();
}

pub unsafe extern "C" fn catch_attack_loop_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    /*
    let capture_id = WorkModule::get_int(fighter.module_accessor, FIGHTER_PIKMIN_INSTANCE_WORK_INT_CHARGE_TARGET_ID) as u32;
    let disable_clatter = WorkModule::is_flag(fighter.module_accessor, FIGHTER_STATUS_CATCH_ATTACK_FLAG_DISABLE_CUT);
    if capture_id != OBJECT_ID_NULL {
        let opponent = sv_battle_object::module_accessor(capture_id as u32);
        WorkModule::off_flag(opponent,*FIGHTER_STATUS_CAPTURE_PULLED_WORK_FLAG_JUMP);
    
        let mut clatter = ControlModule::get_clatter_time(opponent, 0);
        //println!("Clatter: {clatter}");
        if disable_clatter {
            //clatter = WorkModule::get_float(fighter.module_accessor,FIGHTER_STATUS_CATCH_ATTACK_WORK_FLOAT_CLATTER_OPP);
            if clatter <= 1.0 {
                ControlModule::set_clatter_time(opponent, 1.0,0);
            }
        }
        else {
            WorkModule::set_float(fighter.module_accessor, clatter, FIGHTER_STATUS_CATCH_ATTACK_WORK_FLOAT_CLATTER_OPP);
        }
    } */
    throw_special_main_loop(fighter);

    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_PIKMIN_STATUS_CATCH_FLAG_START_THROW) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_PIKMIN_STATUS_CATCH_FLAG_START_THROW);
        
        let hold_pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
        //println!("Change motion for {hold_pikmin_num} pikmin");
        let mut p = 0;
        for p in 0..hold_pikmin_num {
            sync_lr_pikmin(fighter,p);
            let link_node = *FIGHTER_PIKMIN_LINK_NO_PIKMIN_THROW;
            let pikmin_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_OBJECT_ID_0+p) as u32;
            let is_link = LinkModule::link(fighter.module_accessor, link_node, pikmin_id as u32);
            if is_link & 1 != 0  {
                let pikmin_boma = sv_battle_object::module_accessor(pikmin_id as u32);
                WorkModule::on_flag(pikmin_boma, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_FLAG_MOTION_STARTED);
                LinkModule::unlink(fighter.module_accessor, link_node);
            } 
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_PIKMIN_STATUS_CATCH_FLAG_CHARGE)  {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_PIKMIN_STATUS_CATCH_FLAG_CHARGE);
        /* 
        //WorkModule::set_int(fighter.module_accessor, 3, FIGHTER_PIKMIN_INSTANCE_WORK_INT_CHARGE_COUNTDOWN);
        //WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_PIKMIN_STATUS_THROW_WORK_INT_CHARGE_COUNTDOWN);
        
        let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
        //FighterSpecializer_Pikmin::hold_pikmin(olima, 3);
        FighterSpecializer_Pikmin::update_hold_pikmin_param(olima);
        let hold_pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
        println!("CHARGE: {hold_pikmin_num}");
        change_status_pikmin_all(fighter,*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S,true);
        FighterSpecializer_Pikmin::liberty_pikmin_all(olima);
*/
        fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), false.into());
        return 1.into();
    }
    return 0.into();//fighter.status_CatchAttack_Main();
}

pub unsafe extern "C" fn catch_attack_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_THROW {
        SoundModule::stop_all_sound(fighter.module_accessor);
    }
    0.into()
}
pub unsafe extern "C" fn throw_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        return smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_THROW)(fighter);
    }

    HitModule::set_status_all(fighter.module_accessor,HitStatus(*HIT_STATUS_INVINCIBLE),0);
    0.into()
}
pub unsafe extern "C" fn throw_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_direction"), false, false);
    return 0.into();
}

pub unsafe extern "C" fn throw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let current_frame = MotionModule::frame(fighter.module_accessor);
    let current_rate = MotionModule::rate(fighter.module_accessor);
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) 
    {
        return fighter.status_Throw();
    }
    WorkModule::set_int(fighter.module_accessor, 1, FIGHTER_PIKMIN_STATUS_THROW_WORK_INT_CHARGE_COUNTDOWN);
    
    //let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
    //MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, current_frame, true, true, false);
    //MotionModule::set_rate(fighter.module_accessor,current_rate);
    //MotionModule::change_motion(fighter.module_accessor, Hash40::new("catch_special"), current_frame,current_rate, false, 0.0, false, false);
    //
    //FighterSpecializer_Pikmin::hold_pikmin(olima, 3);
    //FighterSpecializer_Pikmin::update_hold_pikmin_param(olima);
    //liberate_pikmin_all(fighter);
    //let hold_pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
    //println!("Throw hold {hold_pikmin_num}");

    //change_status_pikmin(fighter,0,*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_LW_RESPOND);
    //liberate_pikmin_all(fighter);
    //FighterSpecializer_Pikmin::sort_pikmin_no_change_status(olima);

    return fighter.sub_shift_status_main(L2CValue::Ptr(throw_sp_main as *const () as _));
}

pub unsafe extern "C" fn throw_sp_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        if !fighter.is_grounded() {
            fighter.sub_set_ground_correct_by_situation(false.into());
            fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(),FIGHTER_KINETIC_TYPE_AIR_STOP.into());
            fighter.sub_change_motion_by_situation(Hash40::new("special_n").into(), Hash40::new("special_air_n").into(), true.into());
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if WorkModule::count_down_int(fighter.module_accessor, FIGHTER_PIKMIN_STATUS_THROW_WORK_INT_CHARGE_COUNTDOWN, 0) {
        let olima = fighter.global_table[MODULE_ACCESSOR].get_ptr() as *mut FighterModuleAccessor;
        FighterSpecializer_Pikmin::hold_pikmin(olima, 1);
        FighterSpecializer_Pikmin::update_hold_pikmin_param(olima);

        let p = WorkModule::get_int(fighter.module_accessor,FIGHTER_PIKMIN_STATUS_THROW_WORK_INT_CHARGE_COUNT);
        let could_throw = change_status_pikmin(fighter,0,*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S,false);
        WorkModule::inc_int(fighter.module_accessor,FIGHTER_PIKMIN_STATUS_THROW_WORK_INT_CHARGE_COUNT);
        FighterSpecializer_Pikmin::liberty_pikmin_all(olima);

        if could_throw {
            WorkModule::set_int(fighter.module_accessor,3,FIGHTER_PIKMIN_STATUS_THROW_WORK_INT_CHARGE_COUNTDOWN);
        }
        else {
            //FighterSpecializer_Pikmin::liberty_pikmin_all(olima);
        }
        /* 
        FighterSpecializer_Pikmin::hold_pikmin(olima, 1);
        FighterSpecializer_Pikmin::update_hold_pikmin_param(olima);
        let hold_pikmin_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_PIKMIN_HOLD_PIKMIN_NUM);
        change_status_pikmin(fighter,0,*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S,true);
        println!("One more! Has {hold_pikmin_num}");
        */
    }


    0.into()
}

pub unsafe extern "C" fn pikmin_special_s_main(weapon: &mut L2CWeaponCommon) -> L2CValue {

    let prev_status = StatusModule::prev_status_kind(weapon.module_accessor, 0);
    let owner = get_owner_boma(weapon);
    if !WorkModule::is_flag(owner, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) 
    || ![*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_THROW_F,*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_THROW_F_SUB].contains(&prev_status) {
        return smashline::original_status(Main, weapon, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S)(weapon);
    }
    //AttackModule::set_power_mul(weapon.module_accessor, 0.0);
    //let original = smashline::original_status(Main, weapon, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S)(weapon);

    /*ORIGINAL*/
    let variation = WorkModule::get_int(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let motion = if variation != *WEAPON_PIKMIN_PIKMIN_VARIATION_VIOLET {Hash40::new("sp_s_thrown")} else {Hash40::new("sp_s_charge")};
    MotionModule::change_motion(weapon.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);

    let lr = PostureModule::lr(weapon.module_accessor);
    let pos = *PostureModule::pos(weapon.module_accessor);
    let lr_z = (lr-1.0)*90.0;
    let eff =EffectModule::req(
        weapon.module_accessor,
        Hash40::new("sys_run_smoke"),
        &pos,
        &Vector3f{ x: 0.0, y: lr_z+90.0, z: 0.0 },
        0.75,
        0,
        -1,
        false,
        0
    ) as u32;
    EffectModule::set_rate(weapon.module_accessor, eff, 1.5);
    /* 
    WorkModule::off_flag(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_SPECIAL_S_WORK_FLAG_LANDING);
    let pos_y = PostureModule::pos_y(weapon.module_accessor);
    WorkModule::set_float(weapon.module_accessor, pos_y, *WEAPON_PIKMIN_PIKMIN_STATUS_SPECIAL_S_WORK_FLOAT_GROUND_Y);
    */
    //Do some random thing with power_special_s and setting power up mul

    WorkModule::off_flag(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_TAKE_FROM_POCKET);
    //WorkModule::on_flag(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_AUTONOMY);

    let hold_num = WorkModule::get_int(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_HOLD_INDEX);
    let variation = WorkModule::get_int(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let variation_as_str = pikmin_variantion_to_string(variation);

    //KineticModule::change_kinetic(weapon.module_accessor,*WEAPON_KINETIC_TYPE_NONE);
    KineticModule::clear_speed_all(weapon.module_accessor);
    PostureModule::add_pos(weapon.module_accessor, &Vector3f::new(0.0,0.25,0.0));


    let mut target_x = PostureModule::pos_x(weapon.module_accessor) + PostureModule::lr(weapon.module_accessor)*15.0;
    let mut target_y = PostureModule::pos_y(weapon.module_accessor) + 6.0;
    //let target_x = WorkModule::get_float(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLOAT_TARGET_X);
    //let target_y = WorkModule::get_float(weapon.module_accessor,*WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLOAT_TARGET_Y);
    let x_offset = (hold_num-1) as f32 * 1.0;
    let y_offset = (hold_num-1) as f32 * 1.0;
    //let target_x = WorkModule::get_float(owner, FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLOAT_CHARGE_TARGET_X)+x_offset;
    //let target_y = WorkModule::get_float(owner,FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLOAT_CHARGE_TARGET_Y)+y_offset;

    //let target_id = WorkModule::get_int(owner, FIGHTER_PIKMIN_INSTANCE_WORK_INT_CHARGE_TARGET_ID) as u32;
    let target_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_CATCH_TARGET_BATTLE_OBJECT_ID) as u32;
    if target_id != OBJECT_ID_NULL && false {
        let target_boma = sv_battle_object::module_accessor(target_id);
        let hip_pos = &mut Vector3f{ x: 0.0, y: 0.0, z: 0.0 };
        ModelModule::joint_global_position(target_boma, Hash40::new("hip"), hip_pos, false);
        target_x = hip_pos.x+0.0;
        target_y = (hip_pos.y+0.0).max(target_y);
    } 
    #[cfg(feature = "dev")]
    println!("Homing in on {target_id} at {target_x},{target_y}");
    //println!("{} > {target_y}",pos.y);
    
    let mut direction_full = Vector2f{x:target_x-pos.x, y: (target_y-pos.y)};
    let direction_len = sv_math::vec2_length(direction_full.x,direction_full.y);
    let direction = Vector2f{x:direction_full.x/direction_len,y:direction_full.y/direction_len};
    let speed = (direction_len*0.1).clamp(3.0, 5.0);

    let speed_x = direction.x*speed;
    let speed_y = (direction.y*speed);//.max(1.0);


    #[cfg(feature = "dev")]
    println!("{variation_as_str} Pikmin (#{hold_num}) Speed: {speed_x},{speed_y}");
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x,//*lr,
        speed_y
    );
    
    return weapon.fastshift(L2CValue::Ptr(pikmin_special_s_loop as *const () as _)); 
    //return 0.into();
}

pub unsafe extern "C" fn pikmin_special_s_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {

    if !weapon.is_grounded() {
        if weapon.global_table[STATUS_FRAME].get_i32() > 300 {
            weapon.change_status(WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S_LANDING.into(), false.into());
            return 1.into();
        }
    }
    else {
        if WorkModule::is_flag(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_SPECIAL_S_WORK_FLAG_LANDING) {
            weapon.change_status(WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S_LANDING.into(), false.into());
            return 1.into();
        }
    }
    0.into()
}

pub unsafe extern "C" fn pikmin_catch_cut_pre_inner(weapon: &mut L2CWeaponCommon) -> bool {
    let owner = get_owner_boma(weapon);
    if WorkModule::is_flag(owner, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) 
    && StatusModule::status_kind(owner) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        StatusModule::set_status_kind_interrupt(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_AIR_FOLLOW);
        //weapon.change_status(WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_LW_RESPOND.into(),false.into());
        return true;
    }
    return false;
}
pub unsafe extern "C" fn pikmin_catch_cut_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if pikmin_catch_cut_pre_inner(weapon) {return 1.into();}
    let original = smashline::original_status(Pre, weapon, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_CATCH_CUT)(weapon);
    return original;
}



pub unsafe extern "C" fn pikmin_special_lw_respond_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner = get_owner_boma(weapon);
    if WorkModule::is_flag(owner, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("sp_lw_respond"), 0.0, 2.0, false, 0.0, false, false);
        return weapon.fastshift(L2CValue::Ptr(pikmin_special_lw_respond_loop as *const () as _)); 
    }
    return smashline::original_status(Main, weapon, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_LW_RESPOND)(weapon);
}

pub unsafe extern "C" fn pikmin_special_lw_respond_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if MotionModule::is_end(weapon.module_accessor) {
        weapon.change_status(WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S.into(), false.into());
        return 1.into();
    }
    0.into()
}
pub unsafe extern "C" fn pikmin_throw_f_sp(weapon: &mut L2CWeaponCommon,sub: bool) -> L2CValue {
    let hold_num = WorkModule::get_int(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_HOLD_INDEX);
    let variation = WorkModule::get_int(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let variation_as_str = pikmin_variantion_to_string(variation);
    //println!("FThrow: {variation_as_str} Pikmin (#{hold_num}): {} ",!sub);
    WorkModule::set_int(weapon.module_accessor, if sub {1} else {0}, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_MOTION_START_DELAY_FRAME);

    //let motion = Hash40::new("catch_special");
    //MotionModule::change_motion(weapon.module_accessor, Hash40::new("catch_special"), 0.0, 1.0, false, 0.0, false, false);
    
    //AttackModule::set_power_up(weapon.module_accessor,0.0);
    WorkModule::set_int64(weapon.module_accessor, hash40("throw_f") as i64, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_MOTION_KIND);
    WorkModule::off_flag(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_FLAG_MOTION_STARTED);

    if !sub {
        let capture_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_CATCH_TARGET_BATTLE_OBJECT_ID) as u32;
        if capture_id != OBJECT_ID_NULL {
            let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
            let capture_anim = WorkModule::get_int64(weapon.module_accessor, WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_TARGET_ANIM) as u64;

            MotionModule::change_motion(capture_boma, Hash40::new_raw(capture_anim), 0.0, 1.5, false, 0.0, false, false);
        }
    }

    weapon.fastshift(L2CValue::Ptr(pikmin_throw_f_sp_loop as *const () as _))
}

pub unsafe extern "C" fn pikmin_throw_f_sp_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let olimar = get_owner_boma(weapon);
    
    if WorkModule::is_flag(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_FLAG_MOTION_STARTED) {
        if MotionModule::motion_kind(weapon.module_accessor) != hash40("catch_special") {
            MotionModule::change_motion(weapon.module_accessor, Hash40::new("catch_special"), 0.0, 1.0, false, 0.0, false, false);
        }
    }

    let sub = WorkModule::get_int(weapon.module_accessor,*WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_MOTION_START_DELAY_FRAME) == 1;
    //if !StatusModule::is_changing(weapon.module_accessor)
    //&& StatusModule::is_situation_changed(weapon.module_accessor) {
        if !weapon.is_grounded() {
            weapon.change_status(WEAPON_PIKMIN_PIKMIN_STATUS_KIND_FALL.into(), false.into());
        }
    //}
    let status_frame = weapon.global_table[STATUS_FRAME].get_i32();
    let olimar_status = StatusModule::status_kind(olimar);
    let olimar_done = ![*FIGHTER_STATUS_KIND_CATCH_ATTACK,*FIGHTER_STATUS_KIND_THROW].contains(&olimar_status);
    let should_end = olimar_done; //status_frame > 80;//MotionModule::is_end(weapon.module_accessor)
    if should_end
    && WorkModule::is_flag(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_FLAG_MOTION_STARTED) {
        weapon.change_status(WEAPON_PIKMIN_PIKMIN_STATUS_KIND_GROUND_FOLLOW.into(), false.into());
        return 1.into();
    }

    if !sub {
        if StopModule::is_hit(weapon.module_accessor) {
            StopModule::end_stop(weapon.module_accessor);
        }
        let capture_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_CATCH_TARGET_BATTLE_OBJECT_ID) as u32;
        if capture_id != OBJECT_ID_NULL {
            let capture_boma = sv_battle_object::module_accessor(capture_id as u32);

            let mut clatter = ControlModule::get_clatter_time(capture_boma, 0);
            //println!("Throw Clatter: {clatter}");
            if clatter <= 0.0 && !WorkModule::is_flag(weapon.module_accessor, WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_FLAG_DISABLE_CLATTER) {
                let new_status = if !sub {WEAPON_PIKMIN_PIKMIN_STATUS_KIND_CATCH_CUT} else {WEAPON_PIKMIN_PIKMIN_STATUS_KIND_CATCH_CUT_SUB};
                weapon.change_status(new_status.into(),false.into());
                catch_cut_opponent(capture_boma);
                let owner = get_owner_boma(weapon);
                WorkModule::on_flag(owner, *FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLAG_CATCH_CUT);
            }
            else {
                fix_position_opponent(capture_boma);
            }
        }
    }

    0.into()
}

pub unsafe extern "C" fn pikmin_throw_f_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner = get_owner_boma(weapon);
    if WorkModule::is_flag(owner, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        return pikmin_throw_f_sp(weapon,false);
    }
    let original = smashline::original_status(Main, weapon, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_THROW_F)(weapon);
    return original;
}
pub unsafe extern "C" fn pikmin_throw_f_sub_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner = get_owner_boma(weapon);
    if WorkModule::is_flag(owner, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        return pikmin_throw_f_sp(weapon,true);
    }
    let original = smashline::original_status(Main, weapon, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_THROW_F_SUB)(weapon);
    return original;
}

pub unsafe extern "C" fn olimar_frame(fighter: &mut L2CFighterCommon)  {
    let charge = WorkModule::get_int(fighter.module_accessor, FIGHTER_PIKMIN_INSTANCE_WORK_INT_CHARGE_COUNTDOWN);
    WorkModule::count_down_int(fighter.module_accessor, FIGHTER_PIKMIN_INSTANCE_WORK_INT_CHARGE_COUNTDOWN, 0);
    let spummel = WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL);
    if MotionModule::frame(fighter.module_accessor) < 2.0 {
        //println!("Spummel: {spummel}");
    }
}
pub unsafe extern "C" fn pikmin_frame(weapon: &mut L2CWeaponCommon)  {
    let status = weapon.global_table[STATUS_KIND].get_i32();
    let listen_to_charge =
    (*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_WAIT..*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_TURN_WAIT).contains(&status)
    || (*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_GROUND_FOLLOW..*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_JUMP_AERIAL).contains(&status)
    //|| (*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S_CLING..*WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S_CLING_REMOVE).contains(&status)
    //|| *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_CATCH_CUT == status
    //|| *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_CATCH_ATTACK == status
    //|| *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_CATCH_CUT_SUB == status
    ;
    if listen_to_charge {
        let owner = get_owner_boma(weapon);
        if WorkModule::get_int(owner, FIGHTER_PIKMIN_INSTANCE_WORK_INT_CHARGE_COUNTDOWN) > 0 
        && !WorkModule::is_flag(weapon.module_accessor, WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_DISABLE_CHARGE_CHECK) {
            WorkModule::on_flag(weapon.module_accessor, WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_DISABLE_CHARGE_CHECK);

            let pos = *PostureModule::pos(weapon.module_accessor);
            let target_x = WorkModule::get_float(owner, FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLOAT_CHARGE_TARGET_X);
            let target_y = WorkModule::get_float(owner,FIGHTER_PIKMIN_INSTANCE_WORK_ID_FLOAT_CHARGE_TARGET_Y);
        
            let mut direction_full = Vector2f{x:target_x-pos.x, y: (target_y-pos.y)};
            let direction_len = sv_math::vec2_length(direction_full.x,direction_full.y);

            if direction_len < WEAPON_PIKMIN_PIKMIN_MAX_CHARGE_RANGE {
                WorkModule::on_flag(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_AUTONOMY);
                weapon.change_status(WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_LW_RESPOND.into(),false.into());
            }
        }
    }
    else {
        WorkModule::off_flag(weapon.module_accessor, WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_DISABLE_CHARGE_CHECK);
    }
    //debug(weapon);
}
pub unsafe extern "C" fn debug(weapon: &mut L2CWeaponCommon)  {
    let hold_num = WorkModule::get_int(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_HOLD_INDEX);
    let variation = WorkModule::get_int(weapon.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION);
    let variation_as_str = pikmin_variantion_to_string(variation);
    if variation == 0 {
        if MotionModule::frame(weapon.module_accessor) < 1.0 {
            let status = weapon.global_table[STATUS_KIND].get_i32();
        }
    }
}

pub fn install() {
    smashline::Agent::new("pikmin")
        .acmd("game_catchspecial", game_catchspecial,Priority::Default)
        .acmd("effect_catchspecial", effect_catchspecial,Priority::Default)
        .acmd("sound_catchspecial", sound_catchspecial,Priority::Default)
        .acmd("expression_catchspecial", expression_catchspecial,Priority::Default)
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_uniq)
        .status(Exit, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_exit)
        .status(Init, *FIGHTER_STATUS_KIND_THROW, throw_init)
        .status(Exit, *FIGHTER_STATUS_KIND_THROW, throw_exit)
        .status(Main, *FIGHTER_STATUS_KIND_THROW, throw_main)
        .on_line(Main, olimar_frame)
    .install();
    let agent_pikmin = &mut smashline::Agent::new("pikmin_pikmin");
    agent_pikmin
        .acmd("game_catchspecial", pikmin_game_catchspecial,Priority::Default)
        .acmd("game_catchspecial_y", pikmin_game_catchspecial,Priority::Default)
        .acmd("game_catchspecial_b", pikmin_game_catchspecial,Priority::Default)
        .acmd("game_catchspecial_w", pikmin_game_catchspecial,Priority::Default)
        .acmd("game_catchspecial_v", pikmin_game_catchspecial,Priority::Default)
        .acmd("sound_catchspecial", pikmin_sound_catchspecial,Priority::Default)
        .acmd("sound_catchspecial_y", pikmin_sound_catchspecial,Priority::Default)
        .acmd("sound_catchspecial_b", pikmin_sound_catchspecial,Priority::Default)
        .acmd("sound_catchspecial_w", pikmin_sound_catchspecial,Priority::Default)
        .acmd("sound_catchspecial_v", pikmin_sound_catchspecial,Priority::Default)

        .acmd("game_spscharge_v", pikmin_game_spscharge_v,Priority::Default)
        .acmd("game_spsgrabattack_v", pikmin_game_spsgrabattack_v,Priority::Default)
        .acmd("effect_spsgrabattack_v", pikmin_effect_spsgrabattack_v,Priority::Default)
    
        .status(Main, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_THROW_F, pikmin_throw_f_main)
        .status(Main, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_THROW_F_SUB, pikmin_throw_f_sub_main)

        .status(Main, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_S, pikmin_special_s_main)

        //.status(Pre, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_CATCH_CUT, pikmin_catch_cut_pre)

        //.status(Main, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_SPECIAL_LW_RESPOND, pikmin_special_lw_respond_main)
        //.on_line(Main, pikmin_frame)
    .install();
}
