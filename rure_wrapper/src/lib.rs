extern crate libc;
extern crate rure;

// First attempt, results in symbols missing
//#[no_mangle]
//pub use rure::*;

// Second attempt, also symbols missing
//use rure::rure_error_new as rure_error_new;

// Third attempt, all structs resulted in null pointers in Python
//#[no_mangle]
//pub extern fn rure_error_new() -> *mut rure::Error {
//    rure::rure_error_new()
//}
//
//#[no_mangle]
//pub extern fn rure_error_message(err: *mut rure::Error) -> *const libc::c_char {
//    rure::rure_error_message(err)
//}
//
//#[no_mangle]
//pub extern fn rure_error_free(err: *mut rure::Error) {
//    rure::rure_error_free(err)
//}
//
//#[no_mangle]
//pub extern fn rure_options_new() -> *mut rure::Options {
//    rure::rure_options_new()
//}
//
//#[no_mangle]
//pub extern fn rure_options_free(options: *mut rure::Options) {
//    rure::rure_options_free(options)
//}
//
//#[no_mangle]
//pub extern fn rure_options_dfa_size_limit(options: *mut rure::Options, limit: libc::size_t) {
//    rure::rure_options_dfa_size_limit(options, limit)
//}
//
//#[no_mangle]
//pub extern fn rure_options_size_limit(options: *mut rure::Options, limit: libc::size_t) {
//    rure::rure_options_size_limit(options, limit)
//}
//
//#[no_mangle]
//pub extern fn rure_compile(
//    pattern: *const u8,
//    length: libc::size_t,
//    flags: u32,
//    options: *const rure::Options,
//    error: *mut rure::Error,
//) -> *const rure::Regex {
//    rure::rure_compile(pattern, length, flags, options, error)
//}
//
//#[no_mangle]
//pub extern fn rure_free(re: *const rure::Regex) {
//    rure::rure_free(re)
//}
//
//#[no_mangle]
//pub extern fn rure_is_match(
//    re: *const rure::Regex,
//    haystack: *const u8,
//    len: libc::size_t,
//    start: libc::size_t,
//) -> bool {
//    rure::rure_is_match(re, haystack, len, start)
//}
//
//#[no_mangle]
//pub extern fn rure_find(
//    re: *const rure::Regex,
//    haystack: *const u8,
//    len: libc::size_t,
//    start: libc::size_t,
//    match_info: *mut rure::rure_match,
//) -> bool {
//    rure::rure_find(re, haystack, len, start, match_info)
//}
