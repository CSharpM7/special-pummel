#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    unused,
    static_mut_refs
)]
#![deny(
    deprecated
)]

use smash::{
    lib::{
        L2CValue,
        LuaConst,
        lua_const::*
    },
    hash40,
    lua2cpp::*,
    phx::*
};
use smashline::*;

mod imports;
mod duckhunt;
mod edge;
mod gaogaen;
mod gekkouga;
mod iceclimber;
mod koopa;
mod koopajr;
mod metaknight;
mod packun;
mod pfushigisou;
mod pikmin;
mod reflet;
mod rosetta;
mod snake;
mod tantan;
pub mod common;
pub mod vars;

mod training;

#[no_mangle]
pub fn smashline_install() {
    install();
}
#[no_mangle]
pub fn smashline_uninstall() {
    uninstall();
}

pub fn install() {
    println!("Loading spummels");
    common::install();

    #[cfg(feature = "devhook")]
    training::install();
    
    #[cfg(feature = "devhook")]
    return;

    duckhunt::install();
    edge::install();
    gaogaen::install();
    gekkouga::install();
    iceclimber::install();
    koopa::install();
    koopajr::install();
    metaknight::install();
    packun::install();
    pfushigisou::install();
    pikmin::install();
    reflet::install();
    rosetta::install();
    snake::install();
    tantan::install();
}
pub fn uninstall() {
    println!("Uninstalling...");
    common::uninstall();
}

#[skyline::main(name = "special_pummel")]
pub fn main() {
    #[cfg(feature = "devhook")]
    println!("Devhook Loading spummels");

    #[cfg(not(feature = "dev"))]
    install();
    #[cfg(feature = "dev")]
    smashline_install();
}
