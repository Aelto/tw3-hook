pub const IMAGE_BASE: isize = 0x140000000;
pub const REGISTER_NAME_FN: isize = 0x14029F360;
pub const MEMORY_ALLOCATION_FN: isize = 0x140259780;
pub const CFUNCTION_CONSTRUCTOR_FN: isize = 0x1402BAFB0;
pub const SCRIPTING_CONTEXT_SINGLETON_FN: isize = 0x14025AC70;
pub const REGISTER_SCRIPT_FUNCTION_FN: isize = 0x14029AB70;

/// points to the registering of all string.ws functions
pub const STRING_WS_REGISTER_FN: isize = 0x14102D6F0;

pub const SUB_STR_LOWER_UTF: isize = 0x14102CE10;
pub const SUB_STR_LOWER_UTF_OFFSET: isize = 0x102C410;

pub const SUB_CALC_SEED_OFFSET: isize = 0x103ED90;

pub type VoidPtr = *const ();
