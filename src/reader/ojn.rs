use std::io::SeekFrom;
use std::io::prelude::*;
use std::fs::File;
use std::mem;
use std::slice;

use super::atos;

#[derive(Debug)]
#[repr(packed)]
pub struct OJNFileHeader {
    song_id         : u32,      // OJN music ID
    signature       : u32,      // OJN file identifier
    encoder_id      : f32,      // OJN encoder version identifier
    genre_code      : u32,      // music genre code
    tempo           : f32,      // starting tempo of the song, in beats per minute
    level           : [u16; 4], // chart difficulty level. 4th level is byte-alignment padding
    event_count     : [u32; 3],
    note_count      : [u32; 3],
    measure_count   : [u32; 3],
    note_set_count  : [u32; 3],
    _encoder_id     : u16,      // old OJN encoder identifier
    _song_id        : u16,      // old OJN music ID
    _genre          : [u8; 20], // old music genre string
    _cover_art_size : u32,      // square BMP cover art data size
    _file_version   : f32,      // OJN file version (for notecharter reference)
    title1          : [u8; 32], // Workaround Rust array limitation
    title2          : [u8; 32], // Workaround Rust array limitation
    artist          : [u8; 32],
    charter         : [u8; 32],
    ojm_filename    : [u8; 32],
    cover_art_size  : u32,      // fullscreen JPG cover art data size
    duration        : [u32; 3], // length of chart, in seconds
    data_address    : [u32; 4]  // three note data addresses and one JPG cover art data address
}

#[repr(packed)]
pub struct OJNNoteSetHeader {
    measure: u32,
    channel: u16,
    event_count: u16
}

#[repr(packed)]
pub struct OJNNote {
    sample_id: u16,
    volpan: u8,
    note_type: u8
}

pub fn open_ojn(path: &str) {
    // Open file and create a cursor to it.
    let mut file = match File::open(path) {
        Ok (file) => file,
        Err(..)   => panic!(),
    };

    let mut ojn_header: OJNFileHeader = unsafe { mem::zeroed() };
    let result = unsafe {
        let mut ptr_hack = slice::from_raw_parts_mut(
            &mut ojn_header as *mut OJNFileHeader as *mut u8,
            mem::size_of::<OJNFileHeader>()
            );
        file.read(&mut ptr_hack)
    };

    println!("{:?}", &ojn_header);

    let title = atos::arrays_to_string(
        Vec::from(&[
            &ojn_header.title1 as &[u8],
            &ojn_header.title2 as &[u8]
            ] as &[_])
        );
    println!("{:}", title);

    file.seek(SeekFrom::Start(ojn_header.data_address[2] as u64));

    for i in 0..ojn_header.note_set_count[2] {
        let mut note_set: OJNNoteSetHeader = unsafe { mem::zeroed() };
        let result = unsafe {
            let mut hack = slice::from_raw_parts_mut(
                &mut note_set as *mut OJNNoteSetHeader as *mut u8,
                mem::size_of::<OJNNoteSetHeader>()
                );
            file.read(&mut hack)
        };

        if note_set.channel == 0 {

        } else if note_set.channel == 1 {
            
        } else {
            for k in 0..note_set.event_count {
                let tick: u16 = k * (192 / note_set.event_count);
                let mut note: OJNNote = unsafe { mem::zeroed() };
                let result = unsafe {
                    let mut ptr_hack = slice::from_raw_parts_mut(
                        &mut note as *mut OJNNote as *mut u8,
                        mem::size_of::<OJNNote>()
                        );
                    file.read(&mut ptr_hack)
                };
                if note.sample_id != 0 {
                    let note_type: &str = match note.note_type {
                        0 => "NORMAL",
                        2 => "LONG_START",
                        3 => "LONG_END",
                        _ => "UNKNOWN"
                    };
                    let mut vol: i8 = ((note.volpan >> 4) & 0xF) as i8;
                    let mut pan: i8 = (note.volpan & 0xF) as i8;

                    vol = if vol == 0 { 16 } else { vol };
                    pan = if pan == 0 { 0 } else { pan - 8 };


                    println!(
                        "Note type {:} at {:}:{:} ch {:} sample {:} mix {:}:{:}",
                        note_type, note_set.measure, tick,
                        note_set.channel, note.sample_id, vol, pan
                        );
                }
            }

        }
    }

}
