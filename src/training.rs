//I'm so tired of trying this with a level 9 cpu
//Based off of asimon's clatter work on the TMP: https://github.com/jugeeya/UltimateTrainingModpack/blob/main/src/training/clatter.rs
use crate::imports::imports_agent::*;
//use crate::common::consts::*;
//use crate::common::*;
//use training_mod_sync::*;

//RWLOCK
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
/// Gets a copy of a value inside a RwLock and immediately unlocks
///
/// Requires <T: Copy> such as a bool or usize
pub fn read<T: Copy>(rwlock: &RwLock<T>) -> T {
    *rwlock.read().unwrap()
}

/// Gets a clone of a value inside a RwLock and immediately unlocks
///
/// Can be used if <T> is not Copy, such as Vec<u32>
pub fn read_clone<T: Clone>(rwlock: &RwLock<T>) -> T {
    rwlock.read().unwrap().clone()
}

/// Assigns a new value to a RwLock and immediately unlocks
pub fn assign<T>(rwlock: &RwLock<T>, new_val: T) {
    *rwlock.write().unwrap() = new_val
}

/// Locks a RwLock for writing and returns the guard
///
/// Don't forget to drop the guard as soon as you're finished with it
pub fn lock_write<T>(rwlock: &RwLock<T>) -> RwLockWriteGuard<T> {
    rwlock.write().unwrap()
}

/// Locks a RwLock for reading and returns the guard
///
/// Don't forget to drop the guard as soon as you're finished with it
pub fn lock_read<T>(rwlock: &RwLock<T>) -> RwLockReadGuard<T> {
    rwlock.read().unwrap()
}

pub unsafe fn is_in_clatter(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    [
        *FIGHTER_STATUS_KIND_THROW,
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE,
        *FIGHTER_STATUS_KIND_BURY,
        *FIGHTER_STATUS_KIND_BURY_WAIT,
        *FIGHTER_STATUS_KIND_CAPTURE_YOSHI,
        *FIGHTER_STATUS_KIND_YOSHI_EGG,
        *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI,
        *FIGHTER_STATUS_KIND_CAPTURE_WAIT_YOSHI,
        *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE_YOSHI,
        *FIGHTER_STATUS_KIND_SWALLOWED,
        *FIGHTER_STATUS_KIND_SWALLOWED_CAPTURE,
        *FIGHTER_STATUS_KIND_ICE,
    ]
    .contains(&StatusModule::status_kind(module_accessor))
}

static COUNTER: RwLock<u32> = RwLock::new(0);
static CLATTER_STEP: RwLock<f32> = RwLock::new(8.0);

extern "C"{
    #[link_name = "\u{1}_ZN3app9smashball16is_training_modeEv"]
    pub fn is_training_mode() -> bool;
}

pub fn is_operation_cpu(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    unsafe {
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);

        let info = app::lua_bind::FighterManager::get_fighter_information(singletons::FighterManager(), app::FighterEntryID(entry_id as i32));
        return app::lua_bind::FighterInformation::is_operation_cpu(info); 
    }
}

use bitflags::bitflags;
bitflags! {
    #[derive(PartialEq, Copy, Clone)]
    pub struct ClatterFrequency: u32 {
        const NONE = 0;
        const NORMAL = 1;
        const MEDIUM = 2;
        const HIGH = 4;
    }
}

impl ClatterFrequency {
    pub fn into_u32(self) -> u32 {
        match self {
            ClatterFrequency::NONE => u32::MAX,
            ClatterFrequency::NORMAL => 8,
            ClatterFrequency::MEDIUM => 5,
            ClatterFrequency::HIGH => 2,
            _ => panic!("Invalid value in ClatterFrequency::into_u32") //{}", self),
        }
    }
    pub fn shifted(self) -> ClatterFrequency {
        match self {
            ClatterFrequency::NONE => ClatterFrequency::NORMAL,
            ClatterFrequency::NORMAL => ClatterFrequency::MEDIUM,
            ClatterFrequency::MEDIUM => ClatterFrequency::HIGH,
            ClatterFrequency::HIGH => ClatterFrequency::NONE,
            _ => panic!("Invalid value in ClatterFrequency::into_u32") //{}", self),
        }
    }
    pub fn color(self) -> Vector3f {
        match self {
            ClatterFrequency::NONE => Vector3f::new(0.831,0.686,0.216),
            ClatterFrequency::NORMAL => Vector3f::new(0.0,0.0,1.0),
            ClatterFrequency::MEDIUM => Vector3f::new(0.0,1.0,0.0),
            ClatterFrequency::HIGH => Vector3f::new(1.0,0.0,0.0),
            _ => panic!("Invalid value in ClatterFrequency::into_u32") //{}", self),
        }
    }
}

static mut CLATTER_SETTING: ClatterFrequency = ClatterFrequency::NORMAL;

unsafe fn do_clatter_input(module_accessor: &mut BattleObjectModuleAccessor) {
    let clatter_step = read(&CLATTER_STEP);
    ControlModule::add_clatter_time(module_accessor, -1.0 * clatter_step, 0);
    let zeros = Vector3f {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    EffectModule::req_on_joint(
        module_accessor,
        Hash40::new("sys_clatter"),
        Hash40::new("hip"),
        &zeros,
        &zeros,
        1.0,
        &zeros,
        &zeros,
        true,
        *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32
            | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32
            | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32,
        0,
        0,
    );
}

pub unsafe fn handle_clatter(module_accessor: &mut BattleObjectModuleAccessor) {
    // TODO: handle swallowed and cargo carry statuses.
    // Look at set_dec_time/set_dec_time_recovery functions
    if !is_training_mode() || !is_operation_cpu(module_accessor) {
        return;
    }
    if ControlModule::get_clatter_time(module_accessor,0) <= 0.0 {
        // Don't do clatter inputs if we're not in clatter
        return;
    }
    let repeat = CLATTER_SETTING.into_u32(); //read(&MENU).clatter_strength.into_u32();

    let mut counter_lock = lock_write(&COUNTER);
    *counter_lock = ((*counter_lock) + 1) % repeat;
    if *counter_lock == repeat - 1 {
        do_clatter_input(module_accessor);
    }
}

#[skyline::hook(replace = ControlModule::start_clatter)]
pub unsafe fn hook_start_clatter(
    module_accessor: &mut BattleObjectModuleAccessor,
    initial_clatter_time: f32,
    auto_recovery_rate: f32,
    manual_recovery_rate: f32,
    arg5: i8,
    arg6: i32,
    arg7: bool,
    arg8: bool,
) -> u64 {
    // This function is called at the beginning of every clatter situation
    // Grab the manual recovery rate and set that as the amount to reduce
    // the clatter time during each simulated input.
    //
    // Most of the time this is 8 frames, but could be less depending on
    // the status (e.g. freeze is 4 frames / input)
    if is_training_mode() && is_operation_cpu(module_accessor) {
        assign(&CLATTER_STEP, manual_recovery_rate);
    }
    original!()(
        module_accessor,
        initial_clatter_time,
        auto_recovery_rate,
        manual_recovery_rate,
        arg5,
        arg6,
        arg7,
        arg8,
    )
}

unsafe extern "C" fn appeal_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let original = fighter.status_Appeal();
    if !is_training_mode() || is_operation_cpu(&mut *fighter.module_accessor) { return original; }

    CLATTER_SETTING = CLATTER_SETTING.shifted();

    let lr = PostureModule::lr(fighter.module_accessor);
    let flash_y_offset = WorkModule::get_param_float(fighter.module_accessor, hash40("height"), 0);

    let flash_handle = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_flash"), Hash40::new("top"), &Vector3f::new(-5.0, flash_y_offset, 2.0), &Vector3f::zero(), 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
    let clatter_color = CLATTER_SETTING.color();
    EffectModule::set_rgb(fighter.module_accessor, flash_handle, clatter_color.x,clatter_color.y,clatter_color.z);	

    original
}

unsafe extern "C" fn training_frame(fighter: &mut L2CFighterCommon) {
    let boma = (&mut *fighter.module_accessor);
    let status = StatusModule::status_kind(boma);
    if is_operation_cpu(boma) &&
    ![*FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_START,*FIGHTER_STATUS_KIND_SHOULDERED_DONKEY,*FIGHTER_STATUS_KIND_SHOULDERED_DONKEY_THROWN].contains(&status){
        handle_clatter(boma);
    }
}

pub fn install() {
    #[cfg(not(feature = "dev"))]
    skyline::install_hooks!(hook_start_clatter);

    let common = &mut Agent::new("fighter");
    common.status(Main, *FIGHTER_STATUS_KIND_APPEAL, appeal_main);
    common.on_line(Main, training_frame);
    common.install();

}

