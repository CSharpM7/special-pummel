use crate::imports::imports_acmd::*;
use crate::imports::imports_status::*;


extern "C" {
    #[link_name = "\u{1}_ZN3app24FighterSpecializer_Snake21is_constraint_articleERNS_7FighterEiNS_22ArticleOperationTargetE"]
    pub fn is_constraint_article(
        arg1: *mut smash::app::Fighter,
        arg2: i32,
        arg3: smash::app::ArticleOperationTarget,
    ) -> bool;
}
pub const FIGHTER_SNAKE_STATUS_CATCH_FLAG_HAS_C4: i32 = 0x2100000D;

unsafe extern "C" fn game_catchspecial(agent: &mut L2CAgentBase) {
    let object = sv_system::battle_object(agent.lua_state_agent);
    let fighter : *mut Fighter = std::mem::transmute(object);

    frame(agent.lua_state_agent, 5.0);
    {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, false, -1);
    }
    frame(agent.lua_state_agent, 11.0);
    FT_MOTION_RATE_RANGE(agent, 11.0, 24.0, 5.0);
    frame(agent.lua_state_agent, 24.0);
    FT_MOTION_RATE(agent,1.0);
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_STATUS_CATCH_ATTACK_FLAG_DISABLE_CUT);
        if is_constraint_article(fighter, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)) {
            ArticleModule::change_status(agent.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_ESTABLISH_TARGET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        if is_constraint_article(fighter, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)) {
            ArticleModule::shoot(agent.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
    }
    frame(agent.lua_state_agent, 40.0); 
    FT_MOTION_RATE_RANGE(agent, 40.0, 55.0, 20.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, FIGHTER_STATUS_CATCH_ATTACK_FLAG_DISABLE_CUT);
    }
    frame(agent.lua_state_agent, 55.0); 
    FT_MOTION_RATE(agent,1.0);
}

unsafe extern "C" fn effect_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        //macros::EFFECT(agent, Hash40::new("sys_item_get"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}
unsafe extern "C" fn sound_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_snake_escapeair"));
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_snake_gear_05"));
    }
}

unsafe extern "C" fn expression_catchspecial(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}
unsafe extern "C" fn game_catchspecial2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH, false, -1);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, FIGHTER_STATUS_CATCH_ATTACK_FLAG_DISABLE_CUT);
    }
    frame(agent.lua_state_agent, 8.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 22.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        let has_c4 = ArticleModule::is_exist(agent.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4);
        if has_c4 {
            ArticleModule::change_status(agent.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, *WEAPON_SNAKE_C4_STATUS_KIND_EXPLOSION, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, FIGHTER_STATUS_CATCH_ATTACK_FLAG_DISABLE_CUT);
    }
}

unsafe extern "C" fn effect_catchspecial2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_catchspecial2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_snake_special_l04"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_snake_rnd_special_l"));
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_snake_special_l05"));
    }
}

unsafe extern "C" fn expression_catchspecial2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}
/*
STATUS
*/
pub unsafe extern "C" fn catch_attack_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let to_return = catch_attack_main_inner(fighter);

    let has_c4 = ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4);
    WorkModule::set_flag(fighter.module_accessor, has_c4, FIGHTER_SNAKE_STATUS_CATCH_FLAG_HAS_C4);

    if has_c4 && WorkModule::is_flag(fighter.module_accessor,FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        fighter.status_CatchAttack_common(L2CValue::Hash40(Hash40::new("catch_special2")));
        return fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_CatchAttack_Main as *const () as _));
    }
    else if WorkModule::is_flag(fighter.module_accessor,FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL) {
        let capture_boma = get_grabbed_opponent_boma(fighter.module_accessor);
        let mut clatter = ControlModule::get_clatter_time(capture_boma, 0);
        ControlModule::set_clatter_time(capture_boma, clatter*0.375,0);
    }
    
    return to_return;
}

pub unsafe extern "C" fn catch_attack_end_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    let has_c4 = ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4);
    if has_c4 {
        let object = sv_system::battle_object(fighter.lua_state_agent);
        let fighter_obj : *mut Fighter = std::mem::transmute(object);
        if is_constraint_article(fighter_obj, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            /* 
            let handle1 = EffectModule::req_on_joint(fighter.module_accessor, Hash40::new("sys_item_get"), Hash40::new("haver"), &Vector3f::zero(), &Vector3f::zero(), 1.0, &Vector3f::zero(), &Vector3f::zero(), false, 0, 0, 0);
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            */
        }
    } 

    ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SNAKE_GENERATE_ARTICLE_C4_SWITCH, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    fighter.status_end_CatchAttack()
}

pub fn install() {
    smashline::Agent::new("snake")
        .acmd("game_catchspecial", game_catchspecial,Priority::Default)
        .acmd("effect_catchspecial", effect_catchspecial,Priority::Default)
        .acmd("sound_catchspecial", sound_catchspecial,Priority::Default)
        .acmd("expression_catchspecial", expression_catchspecial,Priority::Default)
        
        .acmd("game_catchspecial2", game_catchspecial2,Priority::Default)
        .acmd("effect_catchspecial2", effect_catchspecial2,Priority::Default)
        .acmd("sound_catchspecial2", sound_catchspecial2,Priority::Default)
        .acmd("expression_catchspecial2", expression_catchspecial2,Priority::Default)

        .status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_uniq)
        .status(End, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_end_uniq)
    .install();
}
