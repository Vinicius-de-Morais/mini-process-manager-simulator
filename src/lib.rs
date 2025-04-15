use std::sync::atomic::AtomicBool;

pub mod objects;

pub static DESLIGAR_SISTEMA: AtomicBool = AtomicBool::new(false);