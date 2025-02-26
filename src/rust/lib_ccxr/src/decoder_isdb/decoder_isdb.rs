extern crate libc;
use std::cmp::PartialEq;
use std::io::Write;
use std::{ffi::{CStr, CString}, time::Duration};

use crate::util::log::{debug, DebugMessageFlag};
use lazy_static::lazy_static;
use nanomsg::{Endpoint, Protocol, Socket};
use prost::Message;
use std::os::raw::{c_char, c_int};
use std::sync::Mutex;
use std::thread::sleep;

// use crate::bindings::{cc_subtitle, ccx_output_format};

pub const CCX_DECODER_608_SCREEN_ROWS: usize = 15;
pub const CCX_DECODER_608_SCREEN_WIDTH: usize = 32;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CcxEia608Format {
    SFormatCcScreen = 0,
    SFormatCcLine = 1,
    SFormatXds = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CcModes {
    ModePopOn = 0,
    ModeRollUp2 = 1,
    ModeRollUp3 = 2,
    ModeRollUp4 = 3,
    ModeText = 4,
    ModePaintOn = 5,
    ModeFakeRollUp1 = 100,
}


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CcxDecoder608ColorCode {
    White = 0,
    Green = 1,
    Blue = 2,
    Cyan = 3,
    Red = 4,
    Yellow = 5,
    Magenta = 6,
    UserDefined = 7,
    Black = 8,
    Transparent = 9,
    Max = 10,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FontBits {
    Normal = 0,
    Italics = 1,
    Underline = 2,
    UnderlineItalics = 3,
}


#[repr(C)]
#[derive(Debug, Clone)]
pub struct Eia608Screen {
    pub format: CcxEia608Format,
    pub characters: [[c_char; CCX_DECODER_608_SCREEN_WIDTH + 1]; CCX_DECODER_608_SCREEN_ROWS],
    pub colors: [[CcxDecoder608ColorCode; CCX_DECODER_608_SCREEN_WIDTH + 1]; CCX_DECODER_608_SCREEN_ROWS],
    pub fonts: [[FontBits; CCX_DECODER_608_SCREEN_WIDTH + 1]; CCX_DECODER_608_SCREEN_ROWS],
    pub row_used: [c_int; CCX_DECODER_608_SCREEN_ROWS],
    pub empty: c_int,
    pub start_time: i64,
    pub end_time: i64,
    pub mode: CcModes,
    pub channel: c_int,
    pub my_field: c_int,
    pub xds_str: *mut c_char,
    pub xds_len: usize,
    pub cur_xds_packet_class: c_int,
}