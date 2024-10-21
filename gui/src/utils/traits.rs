#![allow(dead_code)]

use gtk::{
    pango::ffi::PANGO_SCALE, 
    prelude::{DialogExtManual, GtkWindowExt, ImageExt, StyleContextExt, WidgetExt}, 
    IconSize, ResponseType,
};

use super::Size;

pub trait NdotterWidgetExt {
    fn set_request_size(&self, size: Size);

    fn set_classes(&self, classes: &[&str]);

    fn set_font_family(&self, family: &str);

    fn set_font_size(&self, size: u32);
}

impl<W: WidgetExt> NdotterWidgetExt for W {
    fn set_request_size(&self, size: Size) {
        self.set_size_request(size.width as i32, size.height as i32);
    }
    
    fn set_classes(&self, classes: &[&str]) {
        let ctx = self.style_context();

        for class in classes {
            ctx.add_class(class);
        }
    }

    fn set_font_family(&self, family: &str) {
        let ctx = self.pango_context();

        let mut font = ctx.font_description().unwrap_or_default();
        font.set_family(family);
        ctx.set_font_description(Some(&font));
    }
    
    fn set_font_size(&self, size: u32) {
        let ctx = self.pango_context();

        let mut font = ctx.font_description().unwrap_or_default();
        font.set_size(size.max(1) as i32 * PANGO_SCALE);
        ctx.set_font_description(Some(&font));
    }
}

pub trait KonnWindowExt {
    fn set_size(&self, size: Size);
}

impl<W: GtkWindowExt> KonnWindowExt for W {
    fn set_size(&self, size: Size) {
        self.set_default_size(size.width as i32, size.height as i32);
    }
}

pub trait KonnImageExt {
    fn set_icon(&self, name_size: (&str, IconSize));
}

impl<I: ImageExt> KonnImageExt for I {
    fn set_icon(&self, name_size: (&str, IconSize)) {
        self.set_from_icon_name(Some(name_size.0), name_size.1);
    }
}

pub trait NdotterDialogExt {
    fn set_buttons(&self, buttons: &[(&str, ResponseType)]);
}

impl<D: DialogExtManual> NdotterDialogExt for D {
    fn set_buttons(&self, buttons: &[(&str, ResponseType)]) {
        self.add_buttons(buttons);
    }
}