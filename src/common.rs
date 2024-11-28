use crate::imports::imports_status::*;


pub const FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL: i32 = 0x20000116;
pub const FIGHTER_INSTANCE_WORK_ID_FLAG_FORBID_CATCH_SPECIAL: i32 = 0x20000117;
pub const FIGHTER_INSTANCE_CATCH_ATTACK_COUNT : i32 = 0x100000ED;

pub const FIGHTER_STATUS_CATCH_ATTACK_FLAG_DISABLE_CUT: i32 = 0x2100000B;
pub const FIGHTER_STATUS_CATCH_ATTACK_WORK_FLOAT_CLATTER_OPP: i32 = 0x1000007;

pub const PUMMEL_PENALTY_COUNT_MIN: i32 = 0; //0 removes penalty (b). 999 makes it always max penalty (a)
pub const PUMMEL_JAB_PENALTY_COUNT_MIN: i32 = 0; 
pub const PUMMEL_MAX_PENALTY_FACTOR: f32 = 0.75;

pub const DAMAGE_PENALTY_MIN: f32 = 50.0; //0 removes penalty (b). 999 makes it always max penalty (a)
pub const DAMAGE_MAX_PENALTY_FACTOR: f32 = 0.75;


pub unsafe extern "C" fn lerp_pummel_power(fighter: &mut L2CFighterCommon,a: f32, b: f32) -> f32 {
    let pummels = WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_CATCH_ATTACK_COUNT);
    let penalty_count = if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_CATCH_ATTACK 
    {PUMMEL_PENALTY_COUNT_MIN} else {PUMMEL_JAB_PENALTY_COUNT_MIN};

    if penalty_count >= 999 {
        return a;
    }
    else if penalty_count == 0 {
        return b;
    }
    else {
        let t = ((pummels as f32) / (penalty_count as f32)).min(1.0);
        return lerp(a,b,t);
    }
}
pub unsafe extern "C" fn lerp_clatter_by_damage(damage: f32) -> f32 {
    let a = DAMAGE_MAX_PENALTY_FACTOR;
    let b = 1.0;
    if DAMAGE_PENALTY_MIN >= 999.0 {
        return a;
    }
    else if DAMAGE_PENALTY_MIN == 0.0 {
        return b;
    }
    else {
        let t = (damage / DAMAGE_PENALTY_MIN).min(1.0);
        return lerp(a,b,t);
    }
}

// AGENT
pub unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let status_kind_next = StatusModule::status_kind_next(fighter.module_accessor);
    let is_damaged = is_damage_status_next(fighter.module_accessor);
    
    if (
        ![*FIGHTER_STATUS_KIND_ATTACK,*FIGHTER_STATUS_KIND_ATTACK_100].contains(&status_kind_next)
        && ![*FIGHTER_STATUS_KIND_THROW,*FIGHTER_STATUS_KIND_THROW_KIRBY].contains(&status_kind_next)
        && (status_kind != *FIGHTER_STATUS_KIND_CATCH_ATTACK && status_kind_next != *FIGHTER_STATUS_KIND_CATCH_ATTACK)
    )
    || is_damaged
    {
        WorkModule::off_flag(fighter.module_accessor,FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL);
        
        if !(*FIGHTER_STATUS_KIND_CATCH_WAIT..*FIGHTER_STATUS_KIND_CAPTURE_JUMP).contains(&status_kind_next) {
            WorkModule::off_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FORBID_CATCH_SPECIAL);
            WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_INSTANCE_CATCH_ATTACK_COUNT);
        }
    }

    true.into()
}

pub unsafe extern "C" fn agent_start(fighter: &mut L2CFighterCommon)
{
    fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
}

// STATUS HELPERS
pub unsafe fn catch_cut_opponent(opponent: *mut BattleObjectModuleAccessor) {
    let situation = StatusModule::situation_kind(opponent);
    let sit_air = situation == *SITUATION_KIND_AIR;
    let flag_jump = WorkModule::is_flag(opponent, *FIGHTER_STATUS_CAPTURE_PULLED_WORK_FLAG_JUMP);
    let transition = WorkModule::is_enable_transition_term(opponent, *FIGHTER_STATUS_TRANSITION_TERM_ID_CAPTURE_CUT);
    let touch = GroundModule::is_touch(opponent, *GROUND_TOUCH_FLAG_DOWN as u32);
    
    let pos = *PostureModule::pos(opponent);
    let ground_pos_stage = &mut Vector2f::zero();
    let is_touch_stage =  GroundModule::ray_check(opponent, &Vector2f{ x: pos.x, y: pos.y}, &Vector2f{ x: 0.0, y: -5.0},false) == 1;
    let new_status = if (
        !flag_jump
        && !sit_air
        && transition
    )||(
        is_touch_stage
    )
    {*FIGHTER_STATUS_KIND_CAPTURE_CUT} else {*FIGHTER_STATUS_KIND_CAPTURE_JUMP};
    //println!("A: {sit_air} F: {flag_jump} T: {transition} G: {is_touch_stage}");
    StatusModule::change_status_request(opponent, new_status, false);
}
pub unsafe fn fix_position_opponent(opponent: *mut BattleObjectModuleAccessor) {
    if CaptureModule::is_capture(opponent) {
        CaptureModule::update_node_pos(opponent);
        CaptureModule::capture_to_catch_node_pos_diff(opponent);
    }
}

pub unsafe fn catch_attack_check_special_input(fighter: &mut L2CFighterCommon) -> bool {
    let special_input = //ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
    //|| ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW);
    WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL);
    let can_special = !WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FORBID_CATCH_SPECIAL);
    //println!("Check Special Input: {special_input} Can Special: {can_special}");
    return special_input && can_special;
}

pub unsafe fn catch_attack_check_special_anim_boma(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    let has_anim = MotionModule::is_anim_resource(module_accessor, Hash40::new("catch_special"));
    println!("Special pummel has anim: {has_anim}");
    return has_anim;
}

pub unsafe fn catch_attack_check_special_anim(fighter: &mut L2CFighterCommon) -> bool {
    return catch_attack_check_special_anim_boma(fighter.module_accessor);
}

pub unsafe fn catch_attack_check_special(fighter: &mut L2CFighterCommon) -> bool {
    return catch_attack_check_special_input(fighter) && catch_attack_check_special_anim(fighter);
}

pub unsafe fn cont_check_throw(fighter: &mut L2CFighterCommon, transition_term: i32, pad_flag: i32, throw_stick: bool) -> bool {
    let cat2 = fighter.global_table[CMD_CAT2].get_i32();
    if WorkModule::is_enable_transition_term(fighter.module_accessor, transition_term) 
    && (cat2 & pad_flag != 0 || throw_stick) {
        WorkModule::set_int(fighter.module_accessor, transition_term, *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_LAST_STRANS);
        let status_kind = match pad_flag {
            0x100000 => THROW_F_STATUS_KIND,
            0x200000 => THROW_B_STATUS_KIND,
            0x400000 => THROW_HI_STATUS_KIND,
            _ => THROW_LW_STATUS_KIND
        };
        let status_from_table = fighter.global_table[THROW_LW_STATUS_KIND].get_i32();
        fighter.change_status(status_from_table.into(), true.into());
        return true;
    }
    return false;
}

// STATUS
#[skyline::hook(replace = L2CFighterCommon_CatchCont)]
unsafe extern "C" fn catchcont(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let mut throw_f = false;
        let mut throw_b = false;
        let mut throw_hi = false;
        let mut throw_lw = false;
        let throw_stick = fighter.IsThrowStick();
        if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_STATUS_KIND_CATCH_ATTACK {
            if throw_stick[0x176d32be0u64].get_bool() {
                throw_f = true;
            }
            if throw_stick[0x171beeff9u64].get_bool() {
                throw_b = true;
            }
            if throw_stick[0x2d8932aacu64].get_bool() {
                throw_hi = true;
            }
            if throw_stick[0x246f0d2cbu64].get_bool() {
                throw_lw = true;
            }
        }
        if cont_check_throw(fighter,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_F,*FIGHTER_PAD_CMD_CAT2_FLAG_THROW_F,throw_f)
        || cont_check_throw(fighter,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_B,*FIGHTER_PAD_CMD_CAT2_FLAG_THROW_B,throw_b)
        || cont_check_throw(fighter,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI,*FIGHTER_PAD_CMD_CAT2_FLAG_THROW_HI,throw_hi)
        || cont_check_throw(fighter,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_LW,*FIGHTER_PAD_CMD_CAT2_FLAG_THROW_LW,throw_lw)
        {
            return true.into();
        }
        /* 
        let attack = fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0;
        let special = fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N != 0;
        */
        let attack = fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER != 0;
        let special = fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER != 0;
        let can_special = !WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FORBID_CATCH_SPECIAL);

        if attack || special {
            let catch_attack_distance = WorkModule::get_param_float(fighter.module_accessor, hash40("param_motion"), hash40("catch_attack_distance"));
            let scale = PostureModule::scale(fighter.module_accessor);
            let capture_pos_x_diff = CatchModule::capture_pos_x_diff(fighter.module_accessor);
            let distance_with_scale = catch_attack_distance * scale;
            if distance_with_scale <= 0.0
            || capture_pos_x_diff <= distance_with_scale {
                WorkModule::set_flag(fighter.module_accessor, special && can_special, FIGHTER_INSTANCE_WORK_ID_FLAG_CATCH_SPECIAL);
                println!("Is special: {special} ({can_special})");
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH_ATTACK.into(), true.into());
                return true.into();
            }
        }
    }
    false.into()
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_CatchAttack)]
unsafe fn status_CatchAttack(fighter: &mut L2CFighterCommon) -> L2CValue {
    return catch_attack_main_inner(fighter);
}
pub unsafe extern "C" fn catch_attack_main_new(fighter: &mut L2CFighterCommon, call_original_first: bool) -> L2CValue {
    if call_original_first {
        let original = smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_CATCH_ATTACK)(fighter);
        if catch_attack_check_special_input(fighter) {
            return catch_attack_main_inner(fighter);
        }
        return original;
    }
    else {
        if catch_attack_check_special_input(fighter) {
            return catch_attack_main_inner(fighter);
        }
        return smashline::original_status(Main, fighter, *FIGHTER_STATUS_KIND_CATCH_ATTACK)(fighter);
    }
}
pub unsafe extern "C" fn catch_attack_main_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    if catch_attack_check_special_input(fighter) {
        ControlModule::clear_command(fighter.module_accessor, false);

        if catch_attack_check_special_anim(fighter) {
            catch_special_main(fighter);

            fighter.status_CatchAttack_common(L2CValue::Hash40(Hash40::new("catch_special")));
            return fighter.sub_shift_status_main(L2CValue::Ptr(catch_special_main_loop as *const () as _))
        }
        else {
            let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
            let mut next_status = FIGHTER_STATUS_KIND_WAIT;

            if [*FIGHTER_KIND_KIRBY,*FIGHTER_KIND_PFUSHIGISOU,*FIGHTER_KIND_WOLF]
            .contains(&fighter_kind) {next_status = FIGHTER_STATUS_KIND_SPECIAL_N;}

            else if [*FIGHTER_KIND_GAMEWATCH,*FIGHTER_KIND_ROBOT,*FIGHTER_KIND_JACK]
            .contains(&fighter_kind) {next_status = FIGHTER_STATUS_KIND_SPECIAL_S;}

            //else if [*FIGHTER_KIND_REFLET,*FIGHTER_KIND_DEMON]
            //.contains(&fighter_kind) {next_status = FIGHTER_STATUS_KIND_SPECIAL_LW;}
            
            else if [*FIGHTER_KIND_PIKACHU,*FIGHTER_KIND_PICHU].contains(&fighter_kind) {next_status = FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK;}
            else if [*FIGHTER_KIND_POPO,*FIGHTER_KIND_NANA,].contains(&fighter_kind) {next_status = FIGHTER_STATUS_KIND_ATTACK_LW3;}
            else if *FIGHTER_KIND_GANON == fighter_kind {next_status = FIGHTER_STATUS_KIND_ATTACK_S3;}
            else if *FIGHTER_KIND_PIKMIN == fighter_kind {next_status = FIGHTER_STATUS_KIND_ATTACK_S3;}
            else if [*FIGHTER_KIND_MURABITO,*FIGHTER_KIND_SHIZUE].contains(&fighter_kind) {next_status = FIGHTER_STATUS_KIND_ATTACK_S3;}
            else if *FIGHTER_KIND_ROCKMAN == fighter_kind {next_status = FIGHTER_STATUS_KIND_ATTACK_DASH;}
            else if *FIGHTER_KIND_WIIFIT == fighter_kind {next_status = FIGHTER_STATUS_KIND_ATTACK_DASH;}
            else if *FIGHTER_KIND_PACMAN == fighter_kind {next_status = FIGHTER_STATUS_KIND_ATTACK_DASH;}
            else if *FIGHTER_KIND_RYU == fighter_kind {next_status = FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1;}
            else if *FIGHTER_KIND_KEN == fighter_kind {next_status = FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1;}
            else if *FIGHTER_KIND_CLOUD == fighter_kind {
                next_status = if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CLOUD_INSTANCE_WORK_ID_FLAG_LIMIT_BREAK_SPECIAL) 
                {FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3} else {FIGHTER_CLOUD_STATUS_KIND_SPECIAL_S3};
            }
            else if *FIGHTER_KIND_DOLLY == fighter_kind {next_status = FIGHTER_STATUS_KIND_ATTACK_HI3;}
            else if *FIGHTER_KIND_PICKEL == fighter_kind {next_status = FIGHTER_STATUS_KIND_ATTACK_LW3;}
            else if *FIGHTER_KIND_DEMON == fighter_kind {next_status = FIGHTER_DEMON_STATUS_KIND_FLASH_PUNCH;}
            else {
                let attack100_type = WorkModule::get_param_int(fighter.module_accessor, hash40("attack100_type"), 0);
                let attack_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_max"), 0);
                if attack100_type != *FIGHTER_ATTACK100_TYPE_NONE {
                    next_status = FIGHTER_STATUS_KIND_ATTACK_100;
                }
                else {
                    next_status = FIGHTER_STATUS_KIND_ATTACK;
                }
            }            
            fighter.change_status(next_status.into(), false.into());
            return 1.into()
        }
    }
    catch_attack_main_default(fighter)
}

pub unsafe extern "C" fn catch_attack_main_default(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::inc_int(fighter.module_accessor, FIGHTER_INSTANCE_CATCH_ATTACK_COUNT);
    fighter.status_CatchAttack_common(L2CValue::Hash40(Hash40::new("catch_attack")));
    return fighter.sub_shift_status_main(L2CValue::Ptr(catch_attack_main_default_loop as *const () as _));
}
pub unsafe extern "C" fn catch_attack_main_default_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_CatchAttack_Main();
}
pub unsafe extern "C" fn catch_special_main(fighter: &mut L2CFighterCommon) {
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_FORBID_CATCH_SPECIAL); 

    let opponent = get_grabbed_opponent_boma(fighter.module_accessor);
    let opponent_damage = DamageModule::damage(opponent, 0);
    let clatter = ControlModule::get_clatter_time(opponent, 0);
    /*
    let clatter_factor = lerp_pummel_power(fighter,PUMMEL_MAX_PENALTY_FACTOR,1.0);
    let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor,Hash40::new("catch_special"),true);
    let bonus_t = (opponent_damage/75.0).min(1.0);
    let clatter_bonus = lerp(0.0,cancel_frame,bonus_t);
    println!("New clatter: {clatter}*{clatter_factor} + {clatter_bonus}."); */
    let clatter_factor = lerp_clatter_by_damage(opponent_damage);
    let clatter_new = clatter*clatter_factor;
    println!("New clatter: {clatter}*{clatter_factor}: {clatter_new}");
    ControlModule::set_clatter_time(opponent, clatter_new,0);
}

pub unsafe extern "C" fn catch_special_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let opponent = get_grabbed_opponent_boma(fighter.module_accessor);
    WorkModule::off_flag(opponent,*FIGHTER_STATUS_CAPTURE_PULLED_WORK_FLAG_JUMP);

    let mut clatter = ControlModule::get_clatter_time(opponent, 0);
    let disable_clatter = WorkModule::is_flag(fighter.module_accessor, FIGHTER_STATUS_CATCH_ATTACK_FLAG_DISABLE_CUT);
    //println!("Clatter: {clatter} ({disable_clatter})");
    if disable_clatter {
        //clatter = WorkModule::get_float(fighter.module_accessor,FIGHTER_STATUS_CATCH_ATTACK_WORK_FLOAT_CLATTER_OPP);
        if clatter <= 1.0 {
            ControlModule::set_clatter_time(opponent, 1.0,0);
        }
        //ControlModule::set_clatter_time(opponent, clatter,0);
    }
    else {
        WorkModule::set_float(fighter.module_accessor, clatter, FIGHTER_STATUS_CATCH_ATTACK_WORK_FLOAT_CLATTER_OPP);
        if clatter <= 1.0 {
            //ControlModule::end_clatter(opponent, 0);
        }
    }
    return fighter.status_CatchAttack_Main();
}

pub unsafe extern "C" fn throw_special_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let capture_id = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT) as u32;
    if capture_id != OBJECT_ID_NULL {
        return catch_special_main_loop(fighter);
    }
    return fighter.status_CatchAttack_Main();
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_attack_mtrans_post_process)]
unsafe extern "C" fn attack_mtrans_post_process(fighter: &mut L2CFighterCommon) {
    let original = original!()(fighter);
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_CATCH_ATTACK {
        let attack_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_max"), 0);
        let new_motion = match (attack_combo_max) {
            3 => "attack_13",
            2 => "attack_12",
            _ => "attack_11"
        };
        let power = lerp_pummel_power(fighter,1.25,1.5);
        AttackModule::set_power_mul_status(fighter.module_accessor, power);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new(new_motion), 0.0, 1.0, false, 0.0, false, false);
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Attack_Main)]
unsafe extern "C" fn attack_main(fighter: &mut L2CFighterCommon) {
    attack_main_inner(fighter);
}

pub unsafe extern "C" fn attack_main_dev(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Attack();
    return fighter.sub_shift_status_main(L2CValue::Ptr(attack_main_inner as *const () as _));
}

pub unsafe extern "C" fn attack_main_inner(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_CATCH_ATTACK {
        if AttackModule::is_attack(fighter.module_accessor, 0, false)  {
            for i in 0..6 {
                if AttackModule::is_attack(fighter.module_accessor, i, false)  {
                    (*AttackModule::attack_data(fighter.module_accessor, i, false)).stop_frame = 0.5;
                }
                else {break;}
            }
        }
    }
    return fighter.status_Attack_Main_button(CONTROL_PAD_BUTTON_ATTACK.into(),L2CValue::Ptr(L2CFighterCommon_change_status_jump_mini_attack as *const () as _));
}


#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Attack100_Main_uniq_func)]
unsafe extern "C" fn attack_100_main(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    original!()(fighter,param_1); 

    let step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_INT_100_STEP);
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_CATCH_ATTACK {
        if step == *FIGHTER_STATUS_ATTACK_100_STEP_LOOP {
            let pummels = WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_CATCH_ATTACK_COUNT);
            let power_t = ((pummels as f32) / (PUMMEL_JAB_PENALTY_COUNT_MIN as f32)).min(1.0);
            let power = lerp_pummel_power(fighter,1.5,1.75);
            AttackModule::set_power_mul_status(fighter.module_accessor, power);

            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_LOOPED) {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE);
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_100_end"), 0.0, 1.0, false, 0.0, false, false);
                WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ATTACK_WORK_INT_100_HIT_NEAR_COUNT);
                WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_100_STEP_END, *FIGHTER_STATUS_ATTACK_WORK_INT_100_STEP);
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_SUB_KIND_UNIQ, *FIGHTER_INSTANCE_WORK_ID_INT_TRICK_SUB);
            }
        }
        else if step == *FIGHTER_STATUS_ATTACK_100_STEP_END {
            AttackModule::set_power_mul_status(fighter.module_accessor, 0.625);
        }
    }
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        #[cfg(not(feature = "dev"))]{
            println!("Install hooks");
            skyline::install_hooks!(
                catchcont,
                status_CatchAttack,
                    
                attack_mtrans_post_process,
                attack_main,
                attack_100_main,
            );
        }
        #[cfg(feature = "dev")]
        println!("Dev is in nro hook?");
    }
}

pub fn install() {
    #[cfg(not(feature = "dev"))]{
    skyline::nro::add_hook(nro_hook);
    }

    #[cfg(feature = "devhook")]
    return;

    let common = &mut Agent::new("fighter");
    common.on_start(agent_start);
    //common.status(Main, *FIGHTER_STATUS_KIND_ATTACK, attack_main_dev);
    
    common.install();
}

pub fn uninstall() {
}
