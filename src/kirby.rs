use crate::imports::imports_acmd::*;
use crate::imports::imports_status::*;

pub const FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_SPECIAL_PUMMEL_ID: i32 = 0x10000101;

pub unsafe extern "C" fn catch_attack_uniq(fighter: &mut L2CFighterCommon) -> L2CValue {
    if catch_attack_check_special_input(fighter) {
        ControlModule::clear_command(fighter.module_accessor, false);

        let capture_id = LinkModule::get_node_object_id(fighter.module_accessor, *LINK_NO_CAPTURE) as u32;
        WorkModule::set_int64(fighter.module_accessor, capture_id as i64, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_SPECIAL_PUMMEL_ID);

        let motion = Hash40::new("catch_attack");
        fighter.status_CatchAttack_common(L2CValue::Hash40(motion));
		fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_N.into(), false.into());
		return 1.into();
    }
    return catch_attack_main_default(fighter);
}

pub unsafe extern "C" fn special_n_init(fighter: &mut L2CFighterCommon) -> L2CValue {
	if StatusModule::prev_status_kind(fighter.module_accessor, 0) != *FIGHTER_STATUS_KIND_CATCH_ATTACK {
        WorkModule::set_int64(fighter.module_accessor, OBJECT_ID_NULL as i64, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_SPECIAL_PUMMEL_ID);
	}
	else {
		let capture_id = WorkModule::get_int64(fighter.module_accessor,FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_SPECIAL_PUMMEL_ID) as u64;
		if capture_id != OBJECT_ID_NULL as u64 {
			let capture_boma = sv_battle_object::module_accessor(capture_id as u32);
			StatusModule::change_status_force(capture_boma, *FIGHTER_STATUS_KIND_SLIP,true);
			KineticModule::clear_speed_all(capture_boma);
			let lr = PostureModule::lr(fighter.module_accessor);
			let wind = Vector3f{ x: lr*-1.0,y: 0.0, z: 0.0};
			KineticModule::add_speed_outside(capture_boma, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &wind);
			WorkModule::set_int(capture_boma, 1,*FIGHTER_INSTANCE_WORK_ID_INT_INVALID_CAPTURE_FRAME);
			WorkModule::set_int64(fighter.module_accessor, OBJECT_ID_NULL as i64, FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_SPECIAL_PUMMEL_ID);
		}
	}
    return smashline::original_status(Init, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter);
}

pub fn install() {
    smashline::Agent::new("kirby")
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_ATTACK, catch_attack_uniq)
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_N, special_n_init)
    .install();
}
