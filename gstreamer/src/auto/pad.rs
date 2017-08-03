// This file was generated by gir (ede90a4) from gir-files (???)
// DO NOT EDIT

use Caps;
use Element;
use Event;
use EventType;
use FlowReturn;
use Format;
use Iterator;
use Object;
use PadDirection;
use PadLinkReturn;
use PadMode;
use PadTemplate;
#[cfg(feature = "v1_10")]
use Stream;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Pad(Object<ffi::GstPad>): Object;

    match fn {
        get_type => || ffi::gst_pad_get_type(),
    }
}

impl Pad {
    pub fn new<'a, P: Into<Option<&'a str>>>(name: P, direction: PadDirection) -> Pad {
        assert_initialized_main_thread!();
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            from_glib_none(ffi::gst_pad_new(name.0, direction.to_glib()))
        }
    }

    //pub fn new_from_static_template(templ: /*Ignored*/&mut StaticPadTemplate, name: &str) -> Pad {
    //    unsafe { TODO: call ffi::gst_pad_new_from_static_template() }
    //}

    pub fn new_from_template<'a, P: Into<Option<&'a str>>>(templ: &PadTemplate, name: P) -> Pad {
        skip_assert_initialized!();
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            from_glib_none(ffi::gst_pad_new_from_template(templ.to_glib_none().0, name.0))
        }
    }

    pub fn link_get_name(ret: PadLinkReturn) -> Option<String> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gst_pad_link_get_name(ret.to_glib()))
        }
    }
}

unsafe impl Send for Pad {}
unsafe impl Sync for Pad {}

pub trait PadExt {
    fn activate_mode(&self, mode: PadMode, active: bool) -> Result<(), glib::error::BoolError>;

    //fn add_probe<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, mask: PadProbeType, callback: /*Unknown conversion*//*Unimplemented*/PadProbeCallback, user_data: P, destroy_data: /*Unknown conversion*//*Unimplemented*/DestroyNotify) -> libc::c_ulong;

    fn can_link<P: IsA<Pad>>(&self, sinkpad: &P) -> bool;

    //fn chain_list(&self, list: /*Ignored*/&mut BufferList) -> FlowReturn;

    fn check_reconfigure(&self) -> bool;

    fn create_stream_id<'a, P: IsA<Element>, Q: Into<Option<&'a str>>>(&self, parent: &P, stream_id: Q) -> Option<String>;

    //fn create_stream_id_printf<'a, P: IsA<Element>, Q: Into<Option<&'a str>>>(&self, parent: &P, stream_id: Q, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<String>;

    //fn create_stream_id_printf_valist<'a, P: IsA<Element>, Q: Into<Option<&'a str>>>(&self, parent: &P, stream_id: Q, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Option<String>;

    //fn forward<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, forward: /*Unknown conversion*//*Unimplemented*/PadForwardFunction, user_data: P) -> bool;

    fn get_allowed_caps(&self) -> Option<Caps>;

    fn get_current_caps(&self) -> Option<Caps>;

    fn get_direction(&self) -> PadDirection;

    //fn get_element_private(&self) -> /*Unimplemented*/Option<Fundamental: Pointer>;

    fn get_last_flow_return(&self) -> FlowReturn;

    fn get_offset(&self) -> i64;

    fn get_pad_template(&self) -> Option<PadTemplate>;

    fn get_pad_template_caps(&self) -> Option<Caps>;

    fn get_parent_element(&self) -> Option<Element>;

    fn get_peer(&self) -> Option<Pad>;

    fn get_sticky_event(&self, event_type: EventType, idx: u32) -> Option<Event>;

    #[cfg(feature = "v1_10")]
    fn get_stream(&self) -> Option<Stream>;

    fn get_stream_id(&self) -> Option<String>;

    //#[cfg(feature = "v1_12")]
    //fn get_task_state(&self) -> /*Ignored*/TaskState;

    fn has_current_caps(&self) -> bool;

    fn is_active(&self) -> bool;

    fn is_blocked(&self) -> bool;

    fn is_blocking(&self) -> bool;

    fn is_linked(&self) -> bool;

    fn iterate_internal_links(&self) -> Option<Iterator>;

    fn iterate_internal_links_default<'a, P: IsA<Object> + 'a, Q: Into<Option<&'a P>>>(&self, parent: Q) -> Option<Iterator>;

    fn link<P: IsA<Pad>>(&self, sinkpad: &P) -> PadLinkReturn;

    //fn link_full<P: IsA<Pad>>(&self, sinkpad: &P, flags: /*Ignored*/PadLinkCheck) -> PadLinkReturn;

    #[cfg(feature = "v1_10")]
    fn link_maybe_ghosting<P: IsA<Pad>>(&self, sink: &P) -> Result<(), glib::error::BoolError>;

    //#[cfg(feature = "v1_10")]
    //fn link_maybe_ghosting_full<P: IsA<Pad>>(&self, sink: &P, flags: /*Ignored*/PadLinkCheck) -> bool;

    fn mark_reconfigure(&self);

    fn needs_reconfigure(&self) -> bool;

    fn pause_task(&self) -> Result<(), glib::error::BoolError>;

    fn peer_query_accept_caps(&self, caps: &Caps) -> bool;

    fn peer_query_caps<'a, P: Into<Option<&'a Caps>>>(&self, filter: P) -> Option<Caps>;

    fn peer_query_convert(&self, src_format: Format, src_val: i64, dest_format: Format) -> Option<i64>;

    fn peer_query_duration(&self, format: Format) -> Option<i64>;

    fn peer_query_position(&self, format: Format) -> Option<i64>;

    //fn push_list(&self, list: /*Ignored*/&mut BufferList) -> FlowReturn;

    fn query_accept_caps(&self, caps: &Caps) -> bool;

    fn query_caps<'a, P: Into<Option<&'a Caps>>>(&self, filter: P) -> Option<Caps>;

    fn query_convert(&self, src_format: Format, src_val: i64, dest_format: Format) -> Option<i64>;

    fn query_duration(&self, format: Format) -> Option<i64>;

    fn query_position(&self, format: Format) -> Option<i64>;

    //fn set_activate_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, activate: /*Unknown conversion*//*Unimplemented*/PadActivateFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    //fn set_activatemode_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, activatemode: /*Unknown conversion*//*Unimplemented*/PadActivateModeFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    fn set_active(&self, active: bool) -> Result<(), glib::error::BoolError>;

    //fn set_chain_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, chain: /*Unknown conversion*//*Unimplemented*/PadChainFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    //fn set_chain_list_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, chainlist: /*Unknown conversion*//*Unimplemented*/PadChainListFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    //fn set_element_private<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, priv_: P);

    //fn set_event_full_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, event: /*Unknown conversion*//*Unimplemented*/PadEventFullFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    //fn set_event_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, event: /*Unknown conversion*//*Unimplemented*/PadEventFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    //fn set_getrange_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, get: /*Unknown conversion*//*Unimplemented*/PadGetRangeFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    //fn set_iterate_internal_links_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, iterintlink: /*Unknown conversion*//*Unimplemented*/PadIterIntLinkFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    //fn set_link_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, link: /*Unknown conversion*//*Unimplemented*/PadLinkFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    fn set_offset(&self, offset: i64);

    //fn set_query_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, query: /*Unknown conversion*//*Unimplemented*/PadQueryFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    //fn set_unlink_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, unlink: /*Unknown conversion*//*Unimplemented*/PadUnlinkFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    //fn start_task<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/TaskFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) -> bool;

    //fn sticky_events_foreach<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, foreach_func: /*Unknown conversion*//*Unimplemented*/PadStickyEventsForeachFunction, user_data: P);

    fn stop_task(&self) -> Result<(), glib::error::BoolError>;

    fn store_sticky_event(&self, event: &Event) -> FlowReturn;

    fn unlink<P: IsA<Pad>>(&self, sinkpad: &P) -> Result<(), glib::error::BoolError>;

    fn use_fixed_caps(&self);

    fn connect_linked<F: Fn(&Self, &Pad) + Send + Sync + 'static>(&self, f: F) -> u64;

    fn connect_unlinked<F: Fn(&Self, &Pad) + Send + Sync + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<Pad> + IsA<glib::object::Object>> PadExt for O {
    fn activate_mode(&self, mode: PadMode, active: bool) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_pad_activate_mode(self.to_glib_none().0, mode.to_glib(), active.to_glib()), "Failed to activate mode pad")
        }
    }

    //fn add_probe<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, mask: PadProbeType, callback: /*Unknown conversion*//*Unimplemented*/PadProbeCallback, user_data: P, destroy_data: /*Unknown conversion*//*Unimplemented*/DestroyNotify) -> libc::c_ulong {
    //    unsafe { TODO: call ffi::gst_pad_add_probe() }
    //}

    fn can_link<P: IsA<Pad>>(&self, sinkpad: &P) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_can_link(self.to_glib_none().0, sinkpad.to_glib_none().0))
        }
    }

    //fn chain_list(&self, list: /*Ignored*/&mut BufferList) -> FlowReturn {
    //    unsafe { TODO: call ffi::gst_pad_chain_list() }
    //}

    fn check_reconfigure(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_check_reconfigure(self.to_glib_none().0))
        }
    }

    fn create_stream_id<'a, P: IsA<Element>, Q: Into<Option<&'a str>>>(&self, parent: &P, stream_id: Q) -> Option<String> {
        let stream_id = stream_id.into();
        let stream_id = stream_id.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_pad_create_stream_id(self.to_glib_none().0, parent.to_glib_none().0, stream_id.0))
        }
    }

    //fn create_stream_id_printf<'a, P: IsA<Element>, Q: Into<Option<&'a str>>>(&self, parent: &P, stream_id: Q, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<String> {
    //    unsafe { TODO: call ffi::gst_pad_create_stream_id_printf() }
    //}

    //fn create_stream_id_printf_valist<'a, P: IsA<Element>, Q: Into<Option<&'a str>>>(&self, parent: &P, stream_id: Q, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Option<String> {
    //    unsafe { TODO: call ffi::gst_pad_create_stream_id_printf_valist() }
    //}

    //fn forward<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, forward: /*Unknown conversion*//*Unimplemented*/PadForwardFunction, user_data: P) -> bool {
    //    unsafe { TODO: call ffi::gst_pad_forward() }
    //}

    fn get_allowed_caps(&self) -> Option<Caps> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_allowed_caps(self.to_glib_none().0))
        }
    }

    fn get_current_caps(&self) -> Option<Caps> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_current_caps(self.to_glib_none().0))
        }
    }

    fn get_direction(&self) -> PadDirection {
        unsafe {
            from_glib(ffi::gst_pad_get_direction(self.to_glib_none().0))
        }
    }

    //fn get_element_private(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi::gst_pad_get_element_private() }
    //}

    fn get_last_flow_return(&self) -> FlowReturn {
        unsafe {
            from_glib(ffi::gst_pad_get_last_flow_return(self.to_glib_none().0))
        }
    }

    fn get_offset(&self) -> i64 {
        unsafe {
            ffi::gst_pad_get_offset(self.to_glib_none().0)
        }
    }

    fn get_pad_template(&self) -> Option<PadTemplate> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_pad_template(self.to_glib_none().0))
        }
    }

    fn get_pad_template_caps(&self) -> Option<Caps> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_pad_template_caps(self.to_glib_none().0))
        }
    }

    fn get_parent_element(&self) -> Option<Element> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_parent_element(self.to_glib_none().0))
        }
    }

    fn get_peer(&self) -> Option<Pad> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_peer(self.to_glib_none().0))
        }
    }

    fn get_sticky_event(&self, event_type: EventType, idx: u32) -> Option<Event> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_sticky_event(self.to_glib_none().0, event_type.to_glib(), idx))
        }
    }

    #[cfg(feature = "v1_10")]
    fn get_stream(&self) -> Option<Stream> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_stream(self.to_glib_none().0))
        }
    }

    fn get_stream_id(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gst_pad_get_stream_id(self.to_glib_none().0))
        }
    }

    //#[cfg(feature = "v1_12")]
    //fn get_task_state(&self) -> /*Ignored*/TaskState {
    //    unsafe { TODO: call ffi::gst_pad_get_task_state() }
    //}

    fn has_current_caps(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_has_current_caps(self.to_glib_none().0))
        }
    }

    fn is_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_is_active(self.to_glib_none().0))
        }
    }

    fn is_blocked(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_is_blocked(self.to_glib_none().0))
        }
    }

    fn is_blocking(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_is_blocking(self.to_glib_none().0))
        }
    }

    fn is_linked(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_is_linked(self.to_glib_none().0))
        }
    }

    fn iterate_internal_links(&self) -> Option<Iterator> {
        unsafe {
            from_glib_full(ffi::gst_pad_iterate_internal_links(self.to_glib_none().0))
        }
    }

    fn iterate_internal_links_default<'a, P: IsA<Object> + 'a, Q: Into<Option<&'a P>>>(&self, parent: Q) -> Option<Iterator> {
        let parent = parent.into();
        let parent = parent.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_pad_iterate_internal_links_default(self.to_glib_none().0, parent.0))
        }
    }

    fn link<P: IsA<Pad>>(&self, sinkpad: &P) -> PadLinkReturn {
        unsafe {
            from_glib(ffi::gst_pad_link(self.to_glib_none().0, sinkpad.to_glib_none().0))
        }
    }

    //fn link_full<P: IsA<Pad>>(&self, sinkpad: &P, flags: /*Ignored*/PadLinkCheck) -> PadLinkReturn {
    //    unsafe { TODO: call ffi::gst_pad_link_full() }
    //}

    #[cfg(feature = "v1_10")]
    fn link_maybe_ghosting<P: IsA<Pad>>(&self, sink: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_pad_link_maybe_ghosting(self.to_glib_none().0, sink.to_glib_none().0), "Failed to link pad, possibly ghosting")
        }
    }

    //#[cfg(feature = "v1_10")]
    //fn link_maybe_ghosting_full<P: IsA<Pad>>(&self, sink: &P, flags: /*Ignored*/PadLinkCheck) -> bool {
    //    unsafe { TODO: call ffi::gst_pad_link_maybe_ghosting_full() }
    //}

    fn mark_reconfigure(&self) {
        unsafe {
            ffi::gst_pad_mark_reconfigure(self.to_glib_none().0);
        }
    }

    fn needs_reconfigure(&self) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_needs_reconfigure(self.to_glib_none().0))
        }
    }

    fn pause_task(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_pad_pause_task(self.to_glib_none().0), "Failed to pause pad task")
        }
    }

    fn peer_query_accept_caps(&self, caps: &Caps) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_peer_query_accept_caps(self.to_glib_none().0, caps.to_glib_none().0))
        }
    }

    fn peer_query_caps<'a, P: Into<Option<&'a Caps>>>(&self, filter: P) -> Option<Caps> {
        let filter = filter.into();
        let filter = filter.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_pad_peer_query_caps(self.to_glib_none().0, filter.0))
        }
    }

    fn peer_query_convert(&self, src_format: Format, src_val: i64, dest_format: Format) -> Option<i64> {
        unsafe {
            let mut dest_val = mem::uninitialized();
            let ret = from_glib(ffi::gst_pad_peer_query_convert(self.to_glib_none().0, src_format.to_glib(), src_val, dest_format.to_glib(), &mut dest_val));
            if ret { Some(dest_val) } else { None }
        }
    }

    fn peer_query_duration(&self, format: Format) -> Option<i64> {
        unsafe {
            let mut duration = mem::uninitialized();
            let ret = from_glib(ffi::gst_pad_peer_query_duration(self.to_glib_none().0, format.to_glib(), &mut duration));
            if ret { Some(duration) } else { None }
        }
    }

    fn peer_query_position(&self, format: Format) -> Option<i64> {
        unsafe {
            let mut cur = mem::uninitialized();
            let ret = from_glib(ffi::gst_pad_peer_query_position(self.to_glib_none().0, format.to_glib(), &mut cur));
            if ret { Some(cur) } else { None }
        }
    }

    //fn push_list(&self, list: /*Ignored*/&mut BufferList) -> FlowReturn {
    //    unsafe { TODO: call ffi::gst_pad_push_list() }
    //}

    fn query_accept_caps(&self, caps: &Caps) -> bool {
        unsafe {
            from_glib(ffi::gst_pad_query_accept_caps(self.to_glib_none().0, caps.to_glib_none().0))
        }
    }

    fn query_caps<'a, P: Into<Option<&'a Caps>>>(&self, filter: P) -> Option<Caps> {
        let filter = filter.into();
        let filter = filter.to_glib_none();
        unsafe {
            from_glib_full(ffi::gst_pad_query_caps(self.to_glib_none().0, filter.0))
        }
    }

    fn query_convert(&self, src_format: Format, src_val: i64, dest_format: Format) -> Option<i64> {
        unsafe {
            let mut dest_val = mem::uninitialized();
            let ret = from_glib(ffi::gst_pad_query_convert(self.to_glib_none().0, src_format.to_glib(), src_val, dest_format.to_glib(), &mut dest_val));
            if ret { Some(dest_val) } else { None }
        }
    }

    fn query_duration(&self, format: Format) -> Option<i64> {
        unsafe {
            let mut duration = mem::uninitialized();
            let ret = from_glib(ffi::gst_pad_query_duration(self.to_glib_none().0, format.to_glib(), &mut duration));
            if ret { Some(duration) } else { None }
        }
    }

    fn query_position(&self, format: Format) -> Option<i64> {
        unsafe {
            let mut cur = mem::uninitialized();
            let ret = from_glib(ffi::gst_pad_query_position(self.to_glib_none().0, format.to_glib(), &mut cur));
            if ret { Some(cur) } else { None }
        }
    }

    //fn set_activate_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, activate: /*Unknown conversion*//*Unimplemented*/PadActivateFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_pad_set_activate_function_full() }
    //}

    //fn set_activatemode_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, activatemode: /*Unknown conversion*//*Unimplemented*/PadActivateModeFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_pad_set_activatemode_function_full() }
    //}

    fn set_active(&self, active: bool) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_pad_set_active(self.to_glib_none().0, active.to_glib()), "Failed to activate pad")
        }
    }

    //fn set_chain_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, chain: /*Unknown conversion*//*Unimplemented*/PadChainFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_pad_set_chain_function_full() }
    //}

    //fn set_chain_list_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, chainlist: /*Unknown conversion*//*Unimplemented*/PadChainListFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_pad_set_chain_list_function_full() }
    //}

    //fn set_element_private<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, priv_: P) {
    //    unsafe { TODO: call ffi::gst_pad_set_element_private() }
    //}

    //fn set_event_full_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, event: /*Unknown conversion*//*Unimplemented*/PadEventFullFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_pad_set_event_full_function_full() }
    //}

    //fn set_event_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, event: /*Unknown conversion*//*Unimplemented*/PadEventFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_pad_set_event_function_full() }
    //}

    //fn set_getrange_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, get: /*Unknown conversion*//*Unimplemented*/PadGetRangeFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_pad_set_getrange_function_full() }
    //}

    //fn set_iterate_internal_links_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, iterintlink: /*Unknown conversion*//*Unimplemented*/PadIterIntLinkFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_pad_set_iterate_internal_links_function_full() }
    //}

    //fn set_link_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, link: /*Unknown conversion*//*Unimplemented*/PadLinkFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_pad_set_link_function_full() }
    //}

    fn set_offset(&self, offset: i64) {
        unsafe {
            ffi::gst_pad_set_offset(self.to_glib_none().0, offset);
        }
    }

    //fn set_query_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, query: /*Unknown conversion*//*Unimplemented*/PadQueryFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_pad_set_query_function_full() }
    //}

    //fn set_unlink_function_full<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, unlink: /*Unknown conversion*//*Unimplemented*/PadUnlinkFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gst_pad_set_unlink_function_full() }
    //}

    //fn start_task<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, func: /*Unknown conversion*//*Unimplemented*/TaskFunction, user_data: P, notify: /*Unknown conversion*//*Unimplemented*/DestroyNotify) -> bool {
    //    unsafe { TODO: call ffi::gst_pad_start_task() }
    //}

    //fn sticky_events_foreach<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, foreach_func: /*Unknown conversion*//*Unimplemented*/PadStickyEventsForeachFunction, user_data: P) {
    //    unsafe { TODO: call ffi::gst_pad_sticky_events_foreach() }
    //}

    fn stop_task(&self) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_pad_stop_task(self.to_glib_none().0), "Failed to stop pad task")
        }
    }

    fn store_sticky_event(&self, event: &Event) -> FlowReturn {
        unsafe {
            from_glib(ffi::gst_pad_store_sticky_event(self.to_glib_none().0, event.to_glib_none().0))
        }
    }

    fn unlink<P: IsA<Pad>>(&self, sinkpad: &P) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib::error::BoolError::from_glib(ffi::gst_pad_unlink(self.to_glib_none().0, sinkpad.to_glib_none().0), "Failed to unlink pad")
        }
    }

    fn use_fixed_caps(&self) {
        unsafe {
            ffi::gst_pad_use_fixed_caps(self.to_glib_none().0);
        }
    }

    fn connect_linked<F: Fn(&Self, &Pad) + Send + Sync + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Pad) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "linked",
                transmute(linked_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_unlinked<F: Fn(&Self, &Pad) + Send + Sync + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Pad) + Send + Sync + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "unlinked",
                transmute(unlinked_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn linked_trampoline<P>(this: *mut ffi::GstPad, peer: *mut ffi::GstPad, f: glib_ffi::gpointer)
where P: IsA<Pad> {
    callback_guard!();
    let f: &&(Fn(&P, &Pad) + Send + Sync + 'static) = transmute(f);
    f(&Pad::from_glib_none(this).downcast_unchecked(), &from_glib_none(peer))
}

unsafe extern "C" fn unlinked_trampoline<P>(this: *mut ffi::GstPad, peer: *mut ffi::GstPad, f: glib_ffi::gpointer)
where P: IsA<Pad> {
    callback_guard!();
    let f: &&(Fn(&P, &Pad) + Send + Sync + 'static) = transmute(f);
    f(&Pad::from_glib_none(this).downcast_unchecked(), &from_glib_none(peer))
}
