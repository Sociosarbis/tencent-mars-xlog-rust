/* automatically generated by rust-bindgen 0.59.2 */

pub const uECC_arch_other: u32 = 0;
pub const uECC_x86: u32 = 1;
pub const uECC_x86_64: u32 = 2;
pub const uECC_arm: u32 = 3;
pub const uECC_arm_thumb: u32 = 4;
pub const uECC_arm_thumb2: u32 = 5;
pub const uECC_arm64: u32 = 6;
pub const uECC_avr: u32 = 7;
pub const uECC_OPTIMIZATION_LEVEL: u32 = 2;
pub const uECC_SQUARE_FUNC: u32 = 0;
pub const uECC_VLI_NATIVE_LITTLE_ENDIAN: u32 = 0;
pub const uECC_SUPPORTS_secp160r1: u32 = 1;
pub const uECC_SUPPORTS_secp192r1: u32 = 1;
pub const uECC_SUPPORTS_secp224r1: u32 = 1;
pub const uECC_SUPPORTS_secp256r1: u32 = 1;
pub const uECC_SUPPORTS_secp256k1: u32 = 1;
pub const uECC_SUPPORT_COMPRESSED_POINT: u32 = 1;
pub type int_least64_t = i64;
pub type uint_least64_t = u64;
pub type int_fast64_t = i64;
pub type uint_fast64_t = u64;
pub type int_least32_t = i32;
pub type uint_least32_t = u32;
pub type int_fast32_t = i32;
pub type uint_fast32_t = u32;
pub type int_least16_t = i16;
pub type uint_least16_t = u16;
pub type int_fast16_t = i16;
pub type uint_fast16_t = u16;
pub type int_least8_t = i8;
pub type uint_least8_t = u8;
pub type int_fast8_t = i8;
pub type uint_fast8_t = u8;
pub type intmax_t = ::std::os::raw::c_longlong;
pub type uintmax_t = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uECC_Curve_t {
    _unused: [u8; 0],
}
pub type uECC_Curve = *const uECC_Curve_t;
extern "C" {
    pub fn uECC_secp160r1() -> uECC_Curve;
}
extern "C" {
    pub fn uECC_secp192r1() -> uECC_Curve;
}
extern "C" {
    pub fn uECC_secp224r1() -> uECC_Curve;
}
extern "C" {
    pub fn uECC_secp256r1() -> uECC_Curve;
}
extern "C" {
    pub fn uECC_secp256k1() -> uECC_Curve;
}
pub type uECC_RNG_Function = ::std::option::Option<
    unsafe extern "C" fn(dest: *mut u8, size: ::std::os::raw::c_uint) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn uECC_set_rng(rng_function: uECC_RNG_Function);
}
extern "C" {
    pub fn uECC_get_rng() -> uECC_RNG_Function;
}
extern "C" {
    pub fn uECC_curve_private_key_size(curve: uECC_Curve) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn uECC_curve_public_key_size(curve: uECC_Curve) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn uECC_make_key(
        public_key: *mut u8,
        private_key: *mut u8,
        curve: uECC_Curve,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn uECC_shared_secret(
        public_key: *const u8,
        private_key: *const u8,
        secret: *mut u8,
        curve: uECC_Curve,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn uECC_compress(public_key: *const u8, compressed: *mut u8, curve: uECC_Curve);
}
extern "C" {
    pub fn uECC_decompress(compressed: *const u8, public_key: *mut u8, curve: uECC_Curve);
}
extern "C" {
    pub fn uECC_valid_public_key(public_key: *const u8, curve: uECC_Curve)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn uECC_compute_public_key(
        private_key: *const u8,
        public_key: *mut u8,
        curve: uECC_Curve,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn uECC_sign(
        private_key: *const u8,
        message_hash: *const u8,
        hash_size: ::std::os::raw::c_uint,
        signature: *mut u8,
        curve: uECC_Curve,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uECC_HashContext {
    pub init_hash: ::std::option::Option<unsafe extern "C" fn(context: *const uECC_HashContext)>,
    pub update_hash: ::std::option::Option<
        unsafe extern "C" fn(
            context: *const uECC_HashContext,
            message: *const u8,
            message_size: ::std::os::raw::c_uint,
        ),
    >,
    pub finish_hash: ::std::option::Option<
        unsafe extern "C" fn(context: *const uECC_HashContext, hash_result: *mut u8),
    >,
    pub block_size: ::std::os::raw::c_uint,
    pub result_size: ::std::os::raw::c_uint,
    pub tmp: *mut u8,
}
#[test]
fn bindgen_test_layout_uECC_HashContext() {
    assert_eq!(
        ::std::mem::size_of::<uECC_HashContext>(),
        40usize,
        concat!("Size of: ", stringify!(uECC_HashContext))
    );
    assert_eq!(
        ::std::mem::align_of::<uECC_HashContext>(),
        8usize,
        concat!("Alignment of ", stringify!(uECC_HashContext))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<uECC_HashContext>())).init_hash as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(uECC_HashContext),
            "::",
            stringify!(init_hash)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<uECC_HashContext>())).update_hash as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(uECC_HashContext),
            "::",
            stringify!(update_hash)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<uECC_HashContext>())).finish_hash as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(uECC_HashContext),
            "::",
            stringify!(finish_hash)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<uECC_HashContext>())).block_size as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(uECC_HashContext),
            "::",
            stringify!(block_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<uECC_HashContext>())).result_size as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(uECC_HashContext),
            "::",
            stringify!(result_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<uECC_HashContext>())).tmp as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(uECC_HashContext),
            "::",
            stringify!(tmp)
        )
    );
}
extern "C" {
    pub fn uECC_sign_deterministic(
        private_key: *const u8,
        message_hash: *const u8,
        hash_size: ::std::os::raw::c_uint,
        hash_context: *const uECC_HashContext,
        signature: *mut u8,
        curve: uECC_Curve,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn uECC_verify(
        public_key: *const u8,
        message_hash: *const u8,
        hash_size: ::std::os::raw::c_uint,
        signature: *const u8,
        curve: uECC_Curve,
    ) -> ::std::os::raw::c_int;
}
