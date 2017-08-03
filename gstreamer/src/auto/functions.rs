// This file was generated by gir (ede90a4) from gir-files (???)
// DO NOT EDIT

use Bin;
use ClockTime;
use Element;
use Error;
use FlowReturn;
use Plugin;
use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use std::mem;
use std::ptr;


#[cfg(feature = "v1_12")]
pub fn calculate_linear_regression(xy: ClockTime, temp: ClockTime, n: u32) -> Option<(ClockTime, ClockTime, ClockTime, ClockTime, f64)> {
    assert_initialized_main_thread!();
    unsafe {
        let mut m_num = mem::uninitialized();
        let mut m_denom = mem::uninitialized();
        let mut b = mem::uninitialized();
        let mut xbase = mem::uninitialized();
        let mut r_squared = mem::uninitialized();
        let ret = from_glib(ffi::gst_calculate_linear_regression(xy, temp, n, &mut m_num, &mut m_denom, &mut b, &mut xbase, &mut r_squared));
        if ret { Some((m_num, m_denom, b, xbase, r_squared)) } else { None }
    }
}

//pub fn debug_add_log_function<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(func: /*Unknown conversion*//*Unimplemented*/LogFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
//    unsafe { TODO: call ffi::gst_debug_add_log_function() }
//}

//pub fn debug_bin_to_dot_data<P: IsA<Bin>>(bin: &P, details: /*Ignored*/DebugGraphDetails) -> Option<String> {
//    unsafe { TODO: call ffi::gst_debug_bin_to_dot_data() }
//}

//pub fn debug_bin_to_dot_file<P: IsA<Bin>>(bin: &P, details: /*Ignored*/DebugGraphDetails, file_name: &str) {
//    unsafe { TODO: call ffi::gst_debug_bin_to_dot_file() }
//}

//pub fn debug_bin_to_dot_file_with_ts<P: IsA<Bin>>(bin: &P, details: /*Ignored*/DebugGraphDetails, file_name: &str) {
//    unsafe { TODO: call ffi::gst_debug_bin_to_dot_file_with_ts() }
//}

pub fn debug_construct_term_color(colorinfo: u32) -> Option<String> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gst_debug_construct_term_color(colorinfo))
    }
}

pub fn debug_construct_win_color(colorinfo: u32) -> i32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_debug_construct_win_color(colorinfo)
    }
}

//pub fn debug_get_all_categories() -> /*Ignored*/Vec<DebugCategory> {
//    unsafe { TODO: call ffi::gst_debug_get_all_categories() }
//}

//pub fn debug_get_color_mode() -> /*Ignored*/DebugColorMode {
//    unsafe { TODO: call ffi::gst_debug_get_color_mode() }
//}

//pub fn debug_get_default_threshold() -> /*Ignored*/DebugLevel {
//    unsafe { TODO: call ffi::gst_debug_get_default_threshold() }
//}

//#[cfg(feature = "v1_12")]
//pub fn debug_get_stack_trace(flags: /*Ignored*/StackTraceFlags) -> Option<String> {
//    unsafe { TODO: call ffi::gst_debug_get_stack_trace() }
//}

pub fn debug_is_active() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gst_debug_is_active())
    }
}

pub fn debug_is_colored() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gst_debug_is_colored())
    }
}

//pub fn debug_log<'a, P: IsA<glib::Object> + 'a, Q: Into<Option<&'a P>>>(category: /*Ignored*/&mut DebugCategory, level: /*Ignored*/DebugLevel, file: &str, function: &str, line: i32, object: Q, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi::gst_debug_log() }
//}

//pub fn debug_log_default<'a, P: IsA<glib::Object> + 'a, Q: Into<Option<&'a P>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(category: /*Ignored*/&mut DebugCategory, level: /*Ignored*/DebugLevel, file: &str, function: &str, line: i32, object: Q, message: /*Ignored*/&mut DebugMessage, user_data: R) {
//    unsafe { TODO: call ffi::gst_debug_log_default() }
//}

//pub fn debug_log_valist<'a, P: IsA<glib::Object> + 'a, Q: Into<Option<&'a P>>>(category: /*Ignored*/&mut DebugCategory, level: /*Ignored*/DebugLevel, file: &str, function: &str, line: i32, object: Q, format: &str, args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
//    unsafe { TODO: call ffi::gst_debug_log_valist() }
//}

pub fn debug_print_stack_trace() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_debug_print_stack_trace();
    }
}

//pub fn debug_remove_log_function<'a, P: Into<Option<&'a /*Unimplemented*/LogFunction>>>(func: P) -> u32 {
//    unsafe { TODO: call ffi::gst_debug_remove_log_function() }
//}

//pub fn debug_remove_log_function_by_data<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(data: P) -> u32 {
//    unsafe { TODO: call ffi::gst_debug_remove_log_function_by_data() }
//}

pub fn debug_set_active(active: bool) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_debug_set_active(active.to_glib());
    }
}

//pub fn debug_set_color_mode(mode: /*Ignored*/DebugColorMode) {
//    unsafe { TODO: call ffi::gst_debug_set_color_mode() }
//}

pub fn debug_set_color_mode_from_string(mode: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_debug_set_color_mode_from_string(mode.to_glib_none().0);
    }
}

pub fn debug_set_colored(colored: bool) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_debug_set_colored(colored.to_glib());
    }
}

//pub fn debug_set_default_threshold(level: /*Ignored*/DebugLevel) {
//    unsafe { TODO: call ffi::gst_debug_set_default_threshold() }
//}

//pub fn debug_set_threshold_for_name(name: &str, level: /*Ignored*/DebugLevel) {
//    unsafe { TODO: call ffi::gst_debug_set_threshold_for_name() }
//}

pub fn debug_set_threshold_from_string(list: &str, reset: bool) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_debug_set_threshold_from_string(list.to_glib_none().0, reset.to_glib());
    }
}

pub fn debug_unset_threshold_for_name(name: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_debug_unset_threshold_for_name(name.to_glib_none().0);
    }
}

pub fn deinit() {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_deinit();
    }
}

pub fn dynamic_type_register(plugin: &Plugin, type_: glib::types::Type) -> bool {
    skip_assert_initialized!();
    unsafe {
        from_glib(ffi::gst_dynamic_type_register(plugin.to_glib_none().0, type_.to_glib()))
    }
}

//pub fn error_get_message(domain: /*Ignored*/glib::Quark, code: i32) -> Option<String> {
//    unsafe { TODO: call ffi::gst_error_get_message() }
//}

pub fn filename_to_uri(filename: &str) -> Result<String, Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::gst_filename_to_uri(filename.to_glib_none().0, &mut error);
        if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
    }
}

pub fn flow_get_name(ret: FlowReturn) -> Option<String> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(ffi::gst_flow_get_name(ret.to_glib()))
    }
}

//pub fn flow_to_quark(ret: FlowReturn) -> /*Ignored*/glib::Quark {
//    unsafe { TODO: call ffi::gst_flow_to_quark() }
//}

//pub fn formats_contains(formats: /*Unimplemented*/&CArray TypeId { ns_id: 1, id: 56 }, format: Format) -> bool {
//    unsafe { TODO: call ffi::gst_formats_contains() }
//}

//pub fn info_strdup_printf(format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<String> {
//    unsafe { TODO: call ffi::gst_info_strdup_printf() }
//}

//pub fn info_strdup_vprintf(format: &str, args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Option<String> {
//    unsafe { TODO: call ffi::gst_info_strdup_vprintf() }
//}

//pub fn info_vasprintf(format: &str, args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> (i32, String) {
//    unsafe { TODO: call ffi::gst_info_vasprintf() }
//}

//pub fn init<P: Into<Option<i32>>>(argv: /*Unimplemented*/Vec<String>) {
//    unsafe { TODO: call ffi::gst_init() }
//}

//pub fn init_check<P: Into<Option<i32>>>(argv: /*Unimplemented*/Vec<String>) -> Result<(), Error> {
//    unsafe { TODO: call ffi::gst_init_check() }
//}

//pub fn init_get_option_group() -> /*Ignored*/Option<glib::OptionGroup> {
//    unsafe { TODO: call ffi::gst_init_get_option_group() }
//}

//pub fn is_caps_features<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(obj: P) -> bool {
//    unsafe { TODO: call ffi::gst_is_caps_features() }
//}

pub fn is_initialized() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gst_is_initialized())
    }
}

//pub fn make_element_message_details(name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<Structure> {
//    unsafe { TODO: call ffi::gst_make_element_message_details() }
//}

//pub fn param_spec_array<P: IsA</*Ignored*/glib::ParamSpec>>(name: &str, nick: &str, blurb: &str, element_spec: &P, flags: /*Ignored*/glib::ParamFlags) -> /*Ignored*/Option<glib::ParamSpec> {
//    unsafe { TODO: call ffi::gst_param_spec_array() }
//}

//pub fn param_spec_fraction(name: &str, nick: &str, blurb: &str, min_num: i32, min_denom: i32, max_num: i32, max_denom: i32, default_num: i32, default_denom: i32, flags: /*Ignored*/glib::ParamFlags) -> /*Ignored*/Option<glib::ParamSpec> {
//    unsafe { TODO: call ffi::gst_param_spec_fraction() }
//}

pub fn parent_buffer_meta_api_get_type() -> glib::types::Type {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gst_parent_buffer_meta_api_get_type())
    }
}

pub fn parse_bin_from_description(bin_description: &str, ghost_unlinked_pads: bool) -> Result<Option<Bin>, Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::gst_parse_bin_from_description(bin_description.to_glib_none().0, ghost_unlinked_pads.to_glib(), &mut error);
        if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
    }
}

//pub fn parse_bin_from_description_full<'a, P: Into<Option<&'a /*Ignored*/ParseContext>>>(bin_description: &str, ghost_unlinked_pads: bool, context: P, flags: /*Ignored*/ParseFlags) -> Result<Element, Error> {
//    unsafe { TODO: call ffi::gst_parse_bin_from_description_full() }
//}

pub fn parse_launch(pipeline_description: &str) -> Result<Element, Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::gst_parse_launch(pipeline_description.to_glib_none().0, &mut error);
        if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
    }
}

//pub fn parse_launch_full<'a, P: Into<Option<&'a /*Ignored*/ParseContext>>>(pipeline_description: &str, context: P, flags: /*Ignored*/ParseFlags) -> Result<Element, Error> {
//    unsafe { TODO: call ffi::gst_parse_launch_full() }
//}

pub fn parse_launchv(argv: &[&str]) -> Result<Element, Error> {
    assert_initialized_main_thread!();
    unsafe {
        let mut error = ptr::null_mut();
        let ret = ffi::gst_parse_launchv(argv.to_glib_none().0, &mut error);
        if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
    }
}

//pub fn parse_launchv_full<'a, P: Into<Option<&'a /*Ignored*/ParseContext>>>(argv: &[&str], context: P, flags: /*Ignored*/ParseFlags) -> Result<Element, Error> {
//    unsafe { TODO: call ffi::gst_parse_launchv_full() }
//}

//#[cfg(feature = "v1_12")]
//pub fn print(format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi::gst_print() }
//}

//#[cfg(feature = "v1_12")]
//pub fn printerr(format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi::gst_printerr() }
//}

//#[cfg(feature = "v1_12")]
//pub fn printerrln(format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi::gst_printerrln() }
//}

//#[cfg(feature = "v1_12")]
//pub fn println(format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
//    unsafe { TODO: call ffi::gst_println() }
//}

pub fn protection_meta_api_get_type() -> glib::types::Type {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gst_protection_meta_api_get_type())
    }
}

pub fn segtrap_is_enabled() -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gst_segtrap_is_enabled())
    }
}

pub fn segtrap_set_enabled(enabled: bool) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_segtrap_set_enabled(enabled.to_glib());
    }
}

pub fn static_caps_get_type() -> glib::types::Type {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gst_static_caps_get_type())
    }
}

pub fn static_pad_template_get_type() -> glib::types::Type {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gst_static_pad_template_get_type())
    }
}

pub fn tag_exists(tag: &str) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gst_tag_exists(tag.to_glib_none().0))
    }
}

pub fn tag_get_description(tag: &str) -> Option<String> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(ffi::gst_tag_get_description(tag.to_glib_none().0))
    }
}

//pub fn tag_get_flag(tag: &str) -> /*Ignored*/TagFlag {
//    unsafe { TODO: call ffi::gst_tag_get_flag() }
//}

pub fn tag_get_nick(tag: &str) -> Option<String> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_none(ffi::gst_tag_get_nick(tag.to_glib_none().0))
    }
}

pub fn tag_get_type(tag: &str) -> glib::types::Type {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gst_tag_get_type(tag.to_glib_none().0))
    }
}

pub fn tag_is_fixed(tag: &str) -> bool {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gst_tag_is_fixed(tag.to_glib_none().0))
    }
}

//pub fn tag_merge_strings_with_comma(dest: /*Ignored*/glib::Value, src: /*Ignored*/&glib::Value) {
//    unsafe { TODO: call ffi::gst_tag_merge_strings_with_comma() }
//}

//pub fn tag_merge_use_first(dest: /*Ignored*/glib::Value, src: /*Ignored*/&glib::Value) {
//    unsafe { TODO: call ffi::gst_tag_merge_use_first() }
//}

//pub fn tag_register<'a, P: Into<Option<&'a /*Unimplemented*/TagMergeFunc>>>(name: &str, flag: /*Ignored*/TagFlag, type_: glib::types::Type, nick: &str, blurb: &str, func: P) {
//    unsafe { TODO: call ffi::gst_tag_register() }
//}

//pub fn tag_register_static<'a, P: Into<Option<&'a /*Unimplemented*/TagMergeFunc>>>(name: &str, flag: /*Ignored*/TagFlag, type_: glib::types::Type, nick: &str, blurb: &str, func: P) {
//    unsafe { TODO: call ffi::gst_tag_register_static() }
//}

pub fn type_find_get_type() -> glib::types::Type {
    assert_initialized_main_thread!();
    unsafe {
        from_glib(ffi::gst_type_find_get_type())
    }
}

pub fn update_registry() -> Result<(), glib::error::BoolError> {
    assert_initialized_main_thread!();
    unsafe {
        glib::error::BoolError::from_glib(ffi::gst_update_registry(), "Failed to update registry")
    }
}

//pub fn util_array_binary_search<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(array: P, num_elements: u32, element_size: usize, search_func: /*Unknown conversion*//*Unimplemented*/CompareDataFunc, mode: /*Ignored*/SearchMode, search_data: Q, user_data: R) -> /*Unimplemented*/Option<Fundamental: Pointer> {
//    unsafe { TODO: call ffi::gst_util_array_binary_search() }
//}

pub fn util_double_to_fraction(src: f64) -> (i32, i32) {
    assert_initialized_main_thread!();
    unsafe {
        let mut dest_n = mem::uninitialized();
        let mut dest_d = mem::uninitialized();
        ffi::gst_util_double_to_fraction(src, &mut dest_n, &mut dest_d);
        (dest_n, dest_d)
    }
}

pub fn util_fraction_add(a_n: i32, a_d: i32, b_n: i32, b_d: i32) -> Option<(i32, i32)> {
    assert_initialized_main_thread!();
    unsafe {
        let mut res_n = mem::uninitialized();
        let mut res_d = mem::uninitialized();
        let ret = from_glib(ffi::gst_util_fraction_add(a_n, a_d, b_n, b_d, &mut res_n, &mut res_d));
        if ret { Some((res_n, res_d)) } else { None }
    }
}

pub fn util_fraction_compare(a_n: i32, a_d: i32, b_n: i32, b_d: i32) -> i32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_util_fraction_compare(a_n, a_d, b_n, b_d)
    }
}

pub fn util_fraction_multiply(a_n: i32, a_d: i32, b_n: i32, b_d: i32) -> Option<(i32, i32)> {
    assert_initialized_main_thread!();
    unsafe {
        let mut res_n = mem::uninitialized();
        let mut res_d = mem::uninitialized();
        let ret = from_glib(ffi::gst_util_fraction_multiply(a_n, a_d, b_n, b_d, &mut res_n, &mut res_d));
        if ret { Some((res_n, res_d)) } else { None }
    }
}

pub fn util_fraction_to_double(src_n: i32, src_d: i32) -> f64 {
    assert_initialized_main_thread!();
    unsafe {
        let mut dest = mem::uninitialized();
        ffi::gst_util_fraction_to_double(src_n, src_d, &mut dest);
        dest
    }
}

pub fn util_gdouble_to_guint64(value: f64) -> u64 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_util_gdouble_to_guint64(value)
    }
}

//#[cfg(feature = "v1_12")]
//pub fn util_get_object_array<P: IsA<glib::Object>>(object: &P, name: &str, array: /*Ignored*/glib::ValueArray) -> bool {
//    unsafe { TODO: call ffi::gst_util_get_object_array() }
//}

pub fn util_get_timestamp() -> ClockTime {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_util_get_timestamp()
    }
}

pub fn util_greatest_common_divisor(a: i32, b: i32) -> i32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_util_greatest_common_divisor(a, b)
    }
}

pub fn util_greatest_common_divisor_int64(a: i64, b: i64) -> i64 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_util_greatest_common_divisor_int64(a, b)
    }
}

pub fn util_group_id_next() -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_util_group_id_next()
    }
}

pub fn util_guint64_to_gdouble(value: u64) -> f64 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_util_guint64_to_gdouble(value)
    }
}

pub fn util_seqnum_compare(s1: u32, s2: u32) -> i32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_util_seqnum_compare(s1, s2)
    }
}

pub fn util_seqnum_next() -> u32 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_util_seqnum_next()
    }
}

pub fn util_set_object_arg<P: IsA<glib::Object>>(object: &P, name: &str, value: &str) {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_util_set_object_arg(object.to_glib_none().0, name.to_glib_none().0, value.to_glib_none().0);
    }
}

//#[cfg(feature = "v1_12")]
//pub fn util_set_object_array<P: IsA<glib::Object>>(object: &P, name: &str, array: /*Ignored*/&glib::ValueArray) -> bool {
//    unsafe { TODO: call ffi::gst_util_set_object_array() }
//}

//pub fn util_set_value_from_string(value: /*Ignored*/glib::Value, value_str: &str) {
//    unsafe { TODO: call ffi::gst_util_set_value_from_string() }
//}

pub fn util_uint64_scale(val: u64, num: u64, denom: u64) -> u64 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_util_uint64_scale(val, num, denom)
    }
}

pub fn util_uint64_scale_ceil(val: u64, num: u64, denom: u64) -> u64 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_util_uint64_scale_ceil(val, num, denom)
    }
}

pub fn util_uint64_scale_int(val: u64, num: i32, denom: i32) -> u64 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_util_uint64_scale_int(val, num, denom)
    }
}

pub fn util_uint64_scale_int_ceil(val: u64, num: i32, denom: i32) -> u64 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_util_uint64_scale_int_ceil(val, num, denom)
    }
}

pub fn util_uint64_scale_int_round(val: u64, num: i32, denom: i32) -> u64 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_util_uint64_scale_int_round(val, num, denom)
    }
}

pub fn util_uint64_scale_round(val: u64, num: u64, denom: u64) -> u64 {
    assert_initialized_main_thread!();
    unsafe {
        ffi::gst_util_uint64_scale_round(val, num, denom)
    }
}

//pub fn value_can_compare(value1: /*Ignored*/&glib::Value, value2: /*Ignored*/&glib::Value) -> bool {
//    unsafe { TODO: call ffi::gst_value_can_compare() }
//}

//pub fn value_can_intersect(value1: /*Ignored*/&glib::Value, value2: /*Ignored*/&glib::Value) -> bool {
//    unsafe { TODO: call ffi::gst_value_can_intersect() }
//}

//pub fn value_can_subtract(minuend: /*Ignored*/&glib::Value, subtrahend: /*Ignored*/&glib::Value) -> bool {
//    unsafe { TODO: call ffi::gst_value_can_subtract() }
//}

//pub fn value_can_union(value1: /*Ignored*/&glib::Value, value2: /*Ignored*/&glib::Value) -> bool {
//    unsafe { TODO: call ffi::gst_value_can_union() }
//}

//pub fn value_compare(value1: /*Ignored*/&glib::Value, value2: /*Ignored*/&glib::Value) -> i32 {
//    unsafe { TODO: call ffi::gst_value_compare() }
//}

//pub fn value_deserialize(dest: /*Ignored*/glib::Value, src: &str) -> bool {
//    unsafe { TODO: call ffi::gst_value_deserialize() }
//}

//pub fn value_fixate(dest: /*Ignored*/&mut glib::Value, src: /*Ignored*/&glib::Value) -> bool {
//    unsafe { TODO: call ffi::gst_value_fixate() }
//}

//pub fn value_fraction_multiply(product: /*Ignored*/&mut glib::Value, factor1: /*Ignored*/&glib::Value, factor2: /*Ignored*/&glib::Value) -> bool {
//    unsafe { TODO: call ffi::gst_value_fraction_multiply() }
//}

//pub fn value_fraction_subtract(dest: /*Ignored*/&mut glib::Value, minuend: /*Ignored*/&glib::Value, subtrahend: /*Ignored*/&glib::Value) -> bool {
//    unsafe { TODO: call ffi::gst_value_fraction_subtract() }
//}

//pub fn value_get_bitmask(value: /*Ignored*/&glib::Value) -> u64 {
//    unsafe { TODO: call ffi::gst_value_get_bitmask() }
//}

//pub fn value_get_caps(value: /*Ignored*/&glib::Value) -> Option<Caps> {
//    unsafe { TODO: call ffi::gst_value_get_caps() }
//}

//pub fn value_get_caps_features(value: /*Ignored*/&glib::Value) -> /*Ignored*/Option<CapsFeatures> {
//    unsafe { TODO: call ffi::gst_value_get_caps_features() }
//}

//pub fn value_get_double_range_max(value: /*Ignored*/&glib::Value) -> f64 {
//    unsafe { TODO: call ffi::gst_value_get_double_range_max() }
//}

//pub fn value_get_double_range_min(value: /*Ignored*/&glib::Value) -> f64 {
//    unsafe { TODO: call ffi::gst_value_get_double_range_min() }
//}

//pub fn value_get_flagset_flags(value: /*Ignored*/&glib::Value) -> u32 {
//    unsafe { TODO: call ffi::gst_value_get_flagset_flags() }
//}

//pub fn value_get_flagset_mask(value: /*Ignored*/&glib::Value) -> u32 {
//    unsafe { TODO: call ffi::gst_value_get_flagset_mask() }
//}

//pub fn value_get_fraction_denominator(value: /*Ignored*/&glib::Value) -> i32 {
//    unsafe { TODO: call ffi::gst_value_get_fraction_denominator() }
//}

//pub fn value_get_fraction_numerator(value: /*Ignored*/&glib::Value) -> i32 {
//    unsafe { TODO: call ffi::gst_value_get_fraction_numerator() }
//}

//pub fn value_get_fraction_range_max(value: /*Ignored*/&glib::Value) -> /*Ignored*/Option<glib::Value> {
//    unsafe { TODO: call ffi::gst_value_get_fraction_range_max() }
//}

//pub fn value_get_fraction_range_min(value: /*Ignored*/&glib::Value) -> /*Ignored*/Option<glib::Value> {
//    unsafe { TODO: call ffi::gst_value_get_fraction_range_min() }
//}

//pub fn value_get_int64_range_max(value: /*Ignored*/&glib::Value) -> i64 {
//    unsafe { TODO: call ffi::gst_value_get_int64_range_max() }
//}

//pub fn value_get_int64_range_min(value: /*Ignored*/&glib::Value) -> i64 {
//    unsafe { TODO: call ffi::gst_value_get_int64_range_min() }
//}

//pub fn value_get_int64_range_step(value: /*Ignored*/&glib::Value) -> i64 {
//    unsafe { TODO: call ffi::gst_value_get_int64_range_step() }
//}

//pub fn value_get_int_range_max(value: /*Ignored*/&glib::Value) -> i32 {
//    unsafe { TODO: call ffi::gst_value_get_int_range_max() }
//}

//pub fn value_get_int_range_min(value: /*Ignored*/&glib::Value) -> i32 {
//    unsafe { TODO: call ffi::gst_value_get_int_range_min() }
//}

//pub fn value_get_int_range_step(value: /*Ignored*/&glib::Value) -> i32 {
//    unsafe { TODO: call ffi::gst_value_get_int_range_step() }
//}

//pub fn value_get_structure(value: /*Ignored*/&glib::Value) -> Option<Structure> {
//    unsafe { TODO: call ffi::gst_value_get_structure() }
//}

//pub fn value_init_and_copy(dest: /*Ignored*/glib::Value, src: /*Ignored*/&glib::Value) {
//    unsafe { TODO: call ffi::gst_value_init_and_copy() }
//}

//pub fn value_intersect(dest: /*Ignored*/glib::Value, value1: /*Ignored*/&glib::Value, value2: /*Ignored*/&glib::Value) -> bool {
//    unsafe { TODO: call ffi::gst_value_intersect() }
//}

//pub fn value_is_fixed(value: /*Ignored*/&glib::Value) -> bool {
//    unsafe { TODO: call ffi::gst_value_is_fixed() }
//}

//pub fn value_is_subset(value1: /*Ignored*/&glib::Value, value2: /*Ignored*/&glib::Value) -> bool {
//    unsafe { TODO: call ffi::gst_value_is_subset() }
//}

//pub fn value_register(table: /*Ignored*/&ValueTable) {
//    unsafe { TODO: call ffi::gst_value_register() }
//}

//pub fn value_serialize(value: /*Ignored*/&glib::Value) -> Option<String> {
//    unsafe { TODO: call ffi::gst_value_serialize() }
//}

//pub fn value_set_bitmask(value: /*Ignored*/&mut glib::Value, bitmask: u64) {
//    unsafe { TODO: call ffi::gst_value_set_bitmask() }
//}

//pub fn value_set_caps(value: /*Ignored*/&mut glib::Value, caps: &Caps) {
//    unsafe { TODO: call ffi::gst_value_set_caps() }
//}

//pub fn value_set_caps_features(value: /*Ignored*/&mut glib::Value, features: /*Ignored*/&CapsFeatures) {
//    unsafe { TODO: call ffi::gst_value_set_caps_features() }
//}

//pub fn value_set_double_range(value: /*Ignored*/&mut glib::Value, start: f64, end: f64) {
//    unsafe { TODO: call ffi::gst_value_set_double_range() }
//}

//pub fn value_set_flagset(value: /*Ignored*/&mut glib::Value, flags: u32, mask: u32) {
//    unsafe { TODO: call ffi::gst_value_set_flagset() }
//}

//pub fn value_set_fraction(value: /*Ignored*/&mut glib::Value, numerator: i32, denominator: i32) {
//    unsafe { TODO: call ffi::gst_value_set_fraction() }
//}

//pub fn value_set_fraction_range(value: /*Ignored*/&mut glib::Value, start: /*Ignored*/&glib::Value, end: /*Ignored*/&glib::Value) {
//    unsafe { TODO: call ffi::gst_value_set_fraction_range() }
//}

//pub fn value_set_fraction_range_full(value: /*Ignored*/&mut glib::Value, numerator_start: i32, denominator_start: i32, numerator_end: i32, denominator_end: i32) {
//    unsafe { TODO: call ffi::gst_value_set_fraction_range_full() }
//}

//pub fn value_set_int64_range(value: /*Ignored*/&mut glib::Value, start: i64, end: i64) {
//    unsafe { TODO: call ffi::gst_value_set_int64_range() }
//}

//pub fn value_set_int64_range_step(value: /*Ignored*/&mut glib::Value, start: i64, end: i64, step: i64) {
//    unsafe { TODO: call ffi::gst_value_set_int64_range_step() }
//}

//pub fn value_set_int_range(value: /*Ignored*/&mut glib::Value, start: i32, end: i32) {
//    unsafe { TODO: call ffi::gst_value_set_int_range() }
//}

//pub fn value_set_int_range_step(value: /*Ignored*/&mut glib::Value, start: i32, end: i32, step: i32) {
//    unsafe { TODO: call ffi::gst_value_set_int_range_step() }
//}

//pub fn value_set_structure(value: /*Ignored*/&mut glib::Value, structure: &Structure) {
//    unsafe { TODO: call ffi::gst_value_set_structure() }
//}

//pub fn value_subtract(dest: /*Ignored*/glib::Value, minuend: /*Ignored*/&glib::Value, subtrahend: /*Ignored*/&glib::Value) -> bool {
//    unsafe { TODO: call ffi::gst_value_subtract() }
//}

//pub fn value_union(dest: /*Ignored*/glib::Value, value1: /*Ignored*/&glib::Value, value2: /*Ignored*/&glib::Value) -> bool {
//    unsafe { TODO: call ffi::gst_value_union() }
//}

pub fn version() -> (u32, u32, u32, u32) {
    assert_initialized_main_thread!();
    unsafe {
        let mut major = mem::uninitialized();
        let mut minor = mem::uninitialized();
        let mut micro = mem::uninitialized();
        let mut nano = mem::uninitialized();
        ffi::gst_version(&mut major, &mut minor, &mut micro, &mut nano);
        (major, minor, micro, nano)
    }
}

pub fn version_string() -> Option<String> {
    assert_initialized_main_thread!();
    unsafe {
        from_glib_full(ffi::gst_version_string())
    }
}
