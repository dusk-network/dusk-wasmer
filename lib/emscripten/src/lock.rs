use crate::EmEnv;
use libc::c_int;
use wasmer::FunctionEnv;

// NOTE: Not implemented by Emscripten
pub fn ___lock(mut _ctx: FunctionEnv<'_, EmEnv>, _what: c_int) {
    debug!("emscripten::___lock {}", _what);
}

// NOTE: Not implemented by Emscripten
pub fn ___unlock(mut _ctx: FunctionEnv<'_, EmEnv>, _what: c_int) {
    debug!("emscripten::___unlock {}", _what);
}

// NOTE: Not implemented by Emscripten
pub fn ___wait(
    mut _ctx: FunctionEnv<'_, EmEnv>,
    _which: u32,
    _varargs: u32,
    _three: u32,
    _four: u32,
) {
    debug!("emscripten::___wait");
}

pub fn _flock(mut _ctx: FunctionEnv<'_, EmEnv>, _fd: u32, _op: u32) -> u32 {
    debug!("emscripten::_flock");
    0
}
