/* automatically generated by rust-bindgen 0.56.0 */

pub const SOXR_THIS_VERSION_STR: &'static [u8; 6usize] = b"0.1.3\0";
pub const SOXR_TPDF: u32 = 0;
pub const SOXR_NO_DITHER: u32 = 8;
pub const SOXR_ROLLOFF_SMALL: u32 = 0;
pub const SOXR_ROLLOFF_MEDIUM: u32 = 1;
pub const SOXR_ROLLOFF_NONE: u32 = 2;
pub const SOXR_HI_PREC_CLOCK: u32 = 8;
pub const SOXR_DOUBLE_PRECISION: u32 = 16;
pub const SOXR_VR: u32 = 32;
pub const SOXR_COEF_INTERP_AUTO: u32 = 0;
pub const SOXR_COEF_INTERP_LOW: u32 = 2;
pub const SOXR_COEF_INTERP_HIGH: u32 = 3;
pub const SOXR_QQ: u32 = 0;
pub const SOXR_LQ: u32 = 1;
pub const SOXR_MQ: u32 = 2;
pub const SOXR_16_BITQ: u32 = 3;
pub const SOXR_20_BITQ: u32 = 4;
pub const SOXR_24_BITQ: u32 = 5;
pub const SOXR_28_BITQ: u32 = 6;
pub const SOXR_32_BITQ: u32 = 7;
pub const SOXR_LSR0Q: u32 = 8;
pub const SOXR_LSR1Q: u32 = 9;
pub const SOXR_LSR2Q: u32 = 10;
pub const SOXR_LINEAR_PHASE: u32 = 0;
pub const SOXR_INTERMEDIATE_PHASE: u32 = 16;
pub const SOXR_MINIMUM_PHASE: u32 = 48;
pub const SOXR_STEEP_FILTER: u32 = 64;
pub type wchar_t = ::std::os::raw::c_int;
pub type max_align_t = u128;
pub type soxr_io_spec_t = soxr_io_spec;
pub type soxr_quality_spec_t = soxr_quality_spec;
pub type soxr_runtime_spec_t = soxr_runtime_spec;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct soxr {
    _unused: [u8; 0],
}
pub type soxr_t = *mut soxr;
pub type soxr_error_t = *const ::std::os::raw::c_char;
pub type soxr_buf_t = *mut ::std::os::raw::c_void;
pub type soxr_cbuf_t = *const ::std::os::raw::c_void;
pub type soxr_bufs_t = *const soxr_buf_t;
pub type soxr_cbufs_t = *const soxr_cbuf_t;
pub type soxr_in_t = *const ::std::os::raw::c_void;
pub type soxr_out_t = *mut ::std::os::raw::c_void;
extern "C" {
    pub fn soxr_version() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn soxr_create(
        input_rate: f64,
        output_rate: f64,
        num_channels: ::std::os::raw::c_uint,
        arg1: *mut soxr_error_t,
        arg2: *const soxr_io_spec_t,
        arg3: *const soxr_quality_spec_t,
        arg4: *const soxr_runtime_spec_t,
    ) -> soxr_t;
}
extern "C" {
    pub fn soxr_process(
        resampler: soxr_t,
        in_: soxr_in_t,
        ilen: usize,
        idone: *mut usize,
        out: soxr_out_t,
        olen: usize,
        odone: *mut usize,
    ) -> soxr_error_t;
}
pub type soxr_input_fn_t = ::std::option::Option<
    unsafe extern "C" fn(
        input_fn_state: *mut ::std::os::raw::c_void,
        data: *mut soxr_in_t,
        requested_len: usize,
    ) -> usize,
>;
extern "C" {
    pub fn soxr_set_input_fn(
        resampler: soxr_t,
        arg1: soxr_input_fn_t,
        input_fn_state: *mut ::std::os::raw::c_void,
        max_ilen: usize,
    ) -> soxr_error_t;
}
extern "C" {
    pub fn soxr_output(resampler: soxr_t, data: soxr_out_t, olen: usize) -> usize;
}
extern "C" {
    pub fn soxr_error(arg1: soxr_t) -> soxr_error_t;
}
extern "C" {
    pub fn soxr_num_clips(arg1: soxr_t) -> *mut usize;
}
extern "C" {
    pub fn soxr_delay(arg1: soxr_t) -> f64;
}
extern "C" {
    pub fn soxr_engine(arg1: soxr_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn soxr_clear(arg1: soxr_t) -> soxr_error_t;
}
extern "C" {
    pub fn soxr_delete(arg1: soxr_t);
}
extern "C" {
    pub fn soxr_oneshot(
        input_rate: f64,
        output_rate: f64,
        num_channels: ::std::os::raw::c_uint,
        in_: soxr_in_t,
        ilen: usize,
        idone: *mut usize,
        out: soxr_out_t,
        olen: usize,
        odone: *mut usize,
        arg1: *const soxr_io_spec_t,
        arg2: *const soxr_quality_spec_t,
        arg3: *const soxr_runtime_spec_t,
    ) -> soxr_error_t;
}
extern "C" {
    pub fn soxr_set_io_ratio(arg1: soxr_t, io_ratio: f64, slew_len: usize) -> soxr_error_t;
}
pub const SOXR_FLOAT32: soxr_datatype_t = 0;
pub const SOXR_FLOAT64: soxr_datatype_t = 1;
pub const SOXR_INT32: soxr_datatype_t = 2;
pub const SOXR_INT16: soxr_datatype_t = 3;
pub const SOXR_SPLIT: soxr_datatype_t = 4;
pub const SOXR_FLOAT32_I: soxr_datatype_t = 0;
pub const SOXR_FLOAT64_I: soxr_datatype_t = 1;
pub const SOXR_INT32_I: soxr_datatype_t = 2;
pub const SOXR_INT16_I: soxr_datatype_t = 3;
pub const SOXR_FLOAT32_S: soxr_datatype_t = 4;
pub const SOXR_FLOAT64_S: soxr_datatype_t = 5;
pub const SOXR_INT32_S: soxr_datatype_t = 6;
pub const SOXR_INT16_S: soxr_datatype_t = 7;
pub type soxr_datatype_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct soxr_io_spec {
    pub itype: soxr_datatype_t,
    pub otype: soxr_datatype_t,
    pub scale: f64,
    pub e: *mut ::std::os::raw::c_void,
    pub flags: ::std::os::raw::c_ulong,
}
#[test]
fn bindgen_test_layout_soxr_io_spec() {
    assert_eq!(
        ::std::mem::size_of::<soxr_io_spec>(),
        32usize,
        concat!("Size of: ", stringify!(soxr_io_spec))
    );
    assert_eq!(
        ::std::mem::align_of::<soxr_io_spec>(),
        8usize,
        concat!("Alignment of ", stringify!(soxr_io_spec))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<soxr_io_spec>())).itype as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(soxr_io_spec),
            "::",
            stringify!(itype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<soxr_io_spec>())).otype as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(soxr_io_spec),
            "::",
            stringify!(otype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<soxr_io_spec>())).scale as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(soxr_io_spec),
            "::",
            stringify!(scale)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<soxr_io_spec>())).e as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(soxr_io_spec),
            "::",
            stringify!(e)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<soxr_io_spec>())).flags as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(soxr_io_spec),
            "::",
            stringify!(flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct soxr_quality_spec {
    pub precision: f64,
    pub phase_response: f64,
    pub passband_end: f64,
    pub stopband_begin: f64,
    pub e: *mut ::std::os::raw::c_void,
    pub flags: ::std::os::raw::c_ulong,
}
#[test]
fn bindgen_test_layout_soxr_quality_spec() {
    assert_eq!(
        ::std::mem::size_of::<soxr_quality_spec>(),
        48usize,
        concat!("Size of: ", stringify!(soxr_quality_spec))
    );
    assert_eq!(
        ::std::mem::align_of::<soxr_quality_spec>(),
        8usize,
        concat!("Alignment of ", stringify!(soxr_quality_spec))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<soxr_quality_spec>())).precision as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(soxr_quality_spec),
            "::",
            stringify!(precision)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<soxr_quality_spec>())).phase_response as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(soxr_quality_spec),
            "::",
            stringify!(phase_response)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<soxr_quality_spec>())).passband_end as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(soxr_quality_spec),
            "::",
            stringify!(passband_end)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<soxr_quality_spec>())).stopband_begin as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(soxr_quality_spec),
            "::",
            stringify!(stopband_begin)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<soxr_quality_spec>())).e as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(soxr_quality_spec),
            "::",
            stringify!(e)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<soxr_quality_spec>())).flags as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(soxr_quality_spec),
            "::",
            stringify!(flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct soxr_runtime_spec {
    pub log2_min_dft_size: ::std::os::raw::c_uint,
    pub log2_large_dft_size: ::std::os::raw::c_uint,
    pub coef_size_kbytes: ::std::os::raw::c_uint,
    pub num_threads: ::std::os::raw::c_uint,
    pub e: *mut ::std::os::raw::c_void,
    pub flags: ::std::os::raw::c_ulong,
}
#[test]
fn bindgen_test_layout_soxr_runtime_spec() {
    assert_eq!(
        ::std::mem::size_of::<soxr_runtime_spec>(),
        32usize,
        concat!("Size of: ", stringify!(soxr_runtime_spec))
    );
    assert_eq!(
        ::std::mem::align_of::<soxr_runtime_spec>(),
        8usize,
        concat!("Alignment of ", stringify!(soxr_runtime_spec))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<soxr_runtime_spec>())).log2_min_dft_size as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(soxr_runtime_spec),
            "::",
            stringify!(log2_min_dft_size)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<soxr_runtime_spec>())).log2_large_dft_size as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(soxr_runtime_spec),
            "::",
            stringify!(log2_large_dft_size)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<soxr_runtime_spec>())).coef_size_kbytes as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(soxr_runtime_spec),
            "::",
            stringify!(coef_size_kbytes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<soxr_runtime_spec>())).num_threads as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(soxr_runtime_spec),
            "::",
            stringify!(num_threads)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<soxr_runtime_spec>())).e as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(soxr_runtime_spec),
            "::",
            stringify!(e)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<soxr_runtime_spec>())).flags as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(soxr_runtime_spec),
            "::",
            stringify!(flags)
        )
    );
}
extern "C" {
    pub fn soxr_quality_spec(
        recipe: ::std::os::raw::c_ulong,
        flags: ::std::os::raw::c_ulong,
    ) -> soxr_quality_spec_t;
}
extern "C" {
    pub fn soxr_runtime_spec(num_threads: ::std::os::raw::c_uint) -> soxr_runtime_spec_t;
}
extern "C" {
    pub fn soxr_io_spec(itype: soxr_datatype_t, otype: soxr_datatype_t) -> soxr_io_spec_t;
}
extern "C" {
    pub fn soxr_set_error(arg1: soxr_t, arg2: soxr_error_t) -> soxr_error_t;
}
extern "C" {
    pub fn soxr_set_num_channels(arg1: soxr_t, arg2: ::std::os::raw::c_uint) -> soxr_error_t;
}
