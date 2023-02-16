mod sys;

use sys::FunctionList;

// Cannot be a pointer, shared static variables need to be `Sync`
#[no_mangle]
pub static IDENTIFIER: &i8 = unsafe { &*cstr::cstr!("FooLib").as_ptr() };

static IDENTIFIER_ALT_STR: &str = "FooLib2\0";
#[no_mangle]
pub static IDENTIFIER_ALT: &i8 = unsafe { &*IDENTIFIER_ALT_STR.as_ptr().cast() };

#[no_mangle]
pub static FUNCTION_LIST: FunctionList = FunctionList {
    foo: Some(meaning_of_life),
    bar: Some(plus_two),
};

extern "C" fn meaning_of_life() -> u32 {
    42
}

extern "C" fn plus_two(num: u32) -> u32 {
    num + 2
}