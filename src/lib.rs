#![no_std]

macro_rules!ins_arch{($($a:literal)*$m:ident)=>{#[cfg(any($(target_arch=$a),*))]ins_mod!($m as pub);}}

macro_rules!ins_mod{($mv:vis$m:ident as$iv:vis)=>{$mv mod$m;$iv use$m::*;};($mv:vis$m:ident)=>{$mv mod$m;}}

pub mod ins;
pub mod reg;
pub mod tab;
