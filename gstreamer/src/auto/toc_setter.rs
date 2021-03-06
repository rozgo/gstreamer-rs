// This file was generated by gir (0fe730d) from gir-files (???)
// DO NOT EDIT

use Element;
use Object;
use Toc;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct TocSetter(Object<ffi::GstTocSetter>): Element, Object;

    match fn {
        get_type => || ffi::gst_toc_setter_get_type(),
    }
}

unsafe impl Send for TocSetter {}
unsafe impl Sync for TocSetter {}

pub trait TocSetterExt {
    fn get_toc(&self) -> Option<Toc>;

    fn reset(&self);

    fn set_toc<'a, P: Into<Option<&'a Toc>>>(&self, toc: P);
}

impl<O: IsA<TocSetter>> TocSetterExt for O {
    fn get_toc(&self) -> Option<Toc> {
        unsafe {
            from_glib_full(ffi::gst_toc_setter_get_toc(self.to_glib_none().0))
        }
    }

    fn reset(&self) {
        unsafe {
            ffi::gst_toc_setter_reset(self.to_glib_none().0);
        }
    }

    fn set_toc<'a, P: Into<Option<&'a Toc>>>(&self, toc: P) {
        let toc = toc.into();
        let toc = toc.to_glib_none();
        unsafe {
            ffi::gst_toc_setter_set_toc(self.to_glib_none().0, toc.0);
        }
    }
}
