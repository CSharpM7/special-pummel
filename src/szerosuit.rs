use crate::imports::imports_acmd::*;
use crate::imports::imports_status::*;

pub const FIGHTER_SZEROSUIT_INSTANCE_WORK_ID_FLAG_SPUMMEL: i32 = 0x200000E2;

pub unsafe extern "C" fn catch_attack_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if catch_attack_check_special_input(fighter) && !catch_attack_check_special_anim(fighter) {
        ControlModule::clear_command(fighter.module_accessor, false);

		WorkModule::on_flag(fighter.module_accessor, FIGHTER_SZEROSUIT_INSTANCE_WORK_ID_FLAG_SPUMMEL);

        let motion = Hash40::new("catch_attack");
        fighter.status_CatchAttack_common(L2CValue::Hash40(motion));
		fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_LW4.into(), false.into());
		return 1.into();
    }
    return catch_attack_main_default(fighter);
}

pub unsafe extern "C" fn attack_lw4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
	let ret = fighter.status_AttackLw4();
	if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_CATCH_ATTACK {
        MotionModule::set_frame(fighter.module_accessor,15.0,false);
		//AttackModule::set_reaction_mul(fighter.module_accessor, 0.5);
		AttackModule::set_power_mul_status(fighter.module_accessor, 0.25);
	}
	ret
}
pub unsafe extern "C" fn attack_lw4_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
	let ret = fighter.sub_attack_s4_uniq_process_exit();
	AttackModule::set_reaction_mul(fighter.module_accessor, 1.0);
	ret
}

pub unsafe extern "C" fn special_n_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
	if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_CATCH_ATTACK {
        MotionModule::set_rate(fighter.module_accessor,2.0);
	}
	else {
		WorkModule::off_flag(fighter.module_accessor, FIGHTER_SZEROSUIT_INSTANCE_WORK_ID_FLAG_SPUMMEL);
	}
	0.into()
}
pub unsafe extern "C" fn special_n_h_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
	if WorkModule::is_flag(fighter.module_accessor, FIGHTER_SZEROSUIT_INSTANCE_WORK_ID_FLAG_SPUMMEL) {
        MotionModule::set_rate(fighter.module_accessor,2.0);
	}
	0.into()
}

pub fn install() {
    smashline::Agent::new("szerosuit")
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_uniq)
        .status(Main, *FIGHTER_STATUS_KIND_ATTACK_LW4, attack_lw4_main)
        //.status(Exit, *FIGHTER_STATUS_KIND_ATTACK_LW4, attack_lw4_exit)

        .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_exec)
        .status(Exec, *FIGHTER_SZEROSUIT_STATUS_KIND_SPECIAL_N_SHOOT_H, special_n_h_exec)
    .install();
}
