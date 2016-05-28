extern crate sfml;
extern crate xor;

use std::io::{self, SeekFrom};
use std::io::prelude::*;
use std::fs::{self, File, Metadata};
use std::mem;
use std::slice;

use self::sfml::audio::{Music,SoundStatus};
use self::sfml::system::{Time,sleep};

use super::atos;

#[derive(Debug)]
pub struct M30FileHeader {
    signature       : u32,
    version         : u32,
    encryption      : u32,
    samples         : u32,
    data_address    : u32,
    data_size       : u32,
    _padding        : u32
}

const M30_0412_XORMASK: [u8; 4] = [0x30, 0x34, 0x31, 0x32]; // codec 32
const M30_nami_XORMASK: [u8; 4] = [0x6E, 0x61, 0x6D, 0x69]; // codec 16

#[derive(Debug)]
pub struct M30SampleHeader {
    name         : [u8; 32],
    size         : u32,
    side         : u16,
    _unknown_0002: u16,
    _unknown_type: u32,
    id           : u16,
    _padding     : u16,
    _pcm_samples : u32
}

pub fn parse_m30(meta: &Metadata, file: &mut File) {
    let file_size = meta.len();

    file.seek(SeekFrom::Start(0));
    
    let mut header: M30FileHeader = unsafe { mem::zeroed() };
    let result = unsafe {
        let mut hack = slice::from_raw_parts_mut(
            &mut header as *mut M30FileHeader as *mut u8,
            mem::size_of::<M30FileHeader>()
            );
        file.read(&mut hack)
    };

    let mut pack_size   : u64 = header.data_size as u64;
    let mut pack_address: u64 = header.data_address as u64;

    if pack_size > file_size - pack_address {
        println!("[debug] Header reports different payload size.");
        pack_size = file_size - pack_address;
    }

    file.seek(SeekFrom::Start(pack_address));

    for i in 0..header.samples {
        let mut m30head: M30SampleHeader = unsafe { mem::zeroed() };
        let result = unsafe {
            let mut hack = slice::from_raw_parts_mut(
                &mut m30head as *mut M30SampleHeader as *mut u8,
                mem::size_of::<M30SampleHeader>()
                );
            file.read(&mut hack)
        };

        let name: String = atos::array_to_string(&m30head.name);

        let mut m30data: Vec<u8> = vec![];
        let mut handle = file.take(m30head.size as u64);
        handle.read_to_end(&mut m30data);

        let oggdata = match header.encryption {
            16 => self::xor::xor(&m30data, &M30_nami_XORMASK),
            32 => self::xor::xor(&m30data, &M30_0412_XORMASK),
            _  => m30data
        };
        
        // TODO Use SoundBuffer::new_from_memory when rust-sfml finally includes it
        let mut music = Music::new_from_memory(&oggdata).unwrap();

        println!("loaded {:}:{:} - {:} size {:}", m30head.side, m30head.id, name, m30head.size);
        let _ = io::stdout().flush();

        music.play();
        while music.get_status() == SoundStatus::Playing {
            sleep(Time::with_milliseconds(100));
        }
    }
}

pub fn open_ojm(path: &str) {
    let meta = match fs::metadata(path) {
        Ok (meta) => meta,
        Err(..)   => panic!(),
    };
    let mut file = match File::open(path) {
        Ok (file) => file,
        Err(..)   => panic!(),
    };

    // TODO OMC
    parse_m30(&meta, &mut file);
}
