// SPDX-License-Identifier: GPL-3.0-only

use core::fmt;
use std::ops::Deref;

use gtk4::gdk::RGBA;
use hex::encode;
use palette::{rgb::Srgba, Pixel};

pub fn hex_from_rgba(rgba: &Srgba) -> String {
    let hex = encode::<[u8; 4]>(Srgba::into_raw(rgba.into_format()));
    format!("#{hex}")
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct SRGBA(pub Srgba);

impl SRGBA {
    pub fn into_inner(self) -> Srgba {
        self.0
    }
}

impl From<RGBA> for SRGBA {
    fn from(rgba: RGBA) -> Self {
        Self(Srgba::new(
            rgba.red(),
            rgba.blue(),
            rgba.green(),
            rgba.alpha(),
        ))
    }
}

impl Deref for SRGBA {
    type Target = Srgba;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for SRGBA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex_from_rgba(&*self))
    }
}
