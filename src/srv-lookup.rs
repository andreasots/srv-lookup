#![crate_id="srv-lookup#0.1"]
#![crate_type="rlib"]

extern crate libc;
use libc::{c_char, c_void};
use std::c_str::CString;
use std::mem::transmute;
use std::rand::Rng;

#[link(name="srv", kind="static")]
#[link(name="resolv")]
extern {
    fn srv_lookup(name: *const c_char, callback: extern fn(*mut c_void, u16, u16, u16, *const c_char), data: *mut c_void);
}

#[deriving(Clone)]
struct SrvRecord {
    priority: u16,
    weight: u16,
    port: u16,
    name: String
}

extern fn append_to_vec(vec: *mut c_void, priority: u16, weight: u16, port: u16, name: *const c_char) {
    let vec: &mut Vec<SrvRecord> = unsafe { transmute(vec) };
    vec.push(SrvRecord {
        priority: priority,
        weight: weight,
        port: port,
        name: unsafe { CString::new(name, false) }.as_str().unwrap().to_string()
    });
}

fn shuffle(data: &mut Vec<SrvRecord>) -> Vec<SrvRecord> {
    std::rand::task_rng().shuffle(data.as_mut_slice());
    let mut ret = Vec::with_capacity(data.len());
    let mut total = 0;
    for e in data.iter() {
        total += e.weight;
    }

    while data.len() > 0 {
        let w = if total != 0 { std::rand::task_rng().gen_range(0, total) } else { 0 };
        let i = {
            let mut running_total = 0;
            let mut index = 0;
            for (i, ref elem) in data.iter().enumerate() {
                running_total += elem.weight;
                if running_total >= w {
                    index = i;
                    break;
                }
            }
            index
        };
        ret.push(data.remove(i).unwrap());
    }

    return ret;
}

pub fn lookup(service: &str, protocol: &str, domain: &str) -> Vec<(String, u16)> {
    let name = format!("_{}._{}.{}", service, protocol, domain);
    let mut records: Vec<SrvRecord> = Vec::new();
    unsafe {
        name.with_c_str(|name_ptr| srv_lookup(name_ptr, append_to_vec, transmute(&mut records)));
    }

    records = shuffle(&mut records);
    records.sort_by(|a, b| a.priority.cmp(&b.priority));
    
    return FromIterator::from_iter(records.iter().map(|srv| (srv.name.clone(), srv.port)));
}
