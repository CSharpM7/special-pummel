use crate::imports::imports_acmd::*;
use crate::imports::imports_status::*;


unsafe extern "C" fn game_catchspecial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CHANGE_PUNCH_R);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        StatusModule::change_status_request_from_script(agent.module_accessor, *FIGHTER_STATUS_KIND_CATCH_WAIT, true);
    }
}
/*
STATUS
*/
pub unsafe extern "C" fn catch_attack_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if catch_attack_check_special(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_CHANGE_PUNCH_R);
        ControlModule::clear_command(fighter.module_accessor, false);
        
        let captured_boma = &mut *get_grabbed_opponent_boma(fighter.module_accessor);
        let mut waist = *FIGHTER_WAIST_SIZE_M;
        if utility::get_category(captured_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
            waist = WorkModule::get_param_int(captured_boma, hash40("param_motion"), hash40("waist_size"));
        }

        let motion = if waist == *FIGHTER_WAIST_SIZE_L {Hash40::new("catch_special_big")} else {Hash40::new("catch_special")};
        fighter.status_CatchAttack_common(L2CValue::Hash40(motion));
        return fighter.sub_shift_status_main(L2CValue::Ptr(catch_special_main_loop as *const () as _));
    }
    return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_CATCH_ATTACK)(fighter);
}
pub fn install() {
    smashline::Agent::new("tantan")
        .acmd("game_catchspecial", game_catchspecial,Priority::Default)
        .acmd("game_catchspecialbig", game_catchspecial,Priority::Default)
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_uniq)
    .install();
}
