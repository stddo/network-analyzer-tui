use std::collections::HashMap;
use std::fmt::Error;
use std::mem::MaybeUninit;
use std::ptr;

use windows::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER;
use windows::Win32::Networking::WinSock::{ntohl, ntohs};
use windows::Win32::NetworkManagement::IpHelper::{GetTcpTable2, MIB_TCPROW2, MIB_TCPTABLE2};

use crate::common::app::LocalProcess;

pub fn collect_open_ports_by_app() -> Result<Vec<LocalProcess>, Error> {
    let els = unsafe { MIB_TCPTABLE2::get()? };
    let mut apps = vec![];
    for row in els {
        unsafe {
            apps.push(LocalProcess {
                local_ip: ntohl(row.dwLocalAddr),
                local_port: ntohs(row.dwLocalPort as u16),
                remote_ip: ntohl(row.dwRemoteAddr),
                remote_port: ntohs(row.dwRemotePort as u16),
                pid: row.dwOwningPid,
                name: String::new()
            });
        }
    }
    Ok(apps)
}

trait WinDynStruct {
    type Item;

    unsafe fn get() -> Result<Vec<Self::Item>, Error>;
}

impl WinDynStruct for MIB_TCPTABLE2 {
    type Item = MIB_TCPROW2;

    unsafe fn get() -> Result<Vec<Self::Item>, Error> {
        let mut size = 0;
        let e1 = GetTcpTable2(ptr::null_mut(), &mut size, false);

        return if e1 == ERROR_INSUFFICIENT_BUFFER.0 {
            let mut buf: Vec<u8> = Vec::with_capacity(TryInto::<usize>::try_into(size).unwrap());
            let table = &mut *(buf.as_mut_ptr() as *mut MaybeUninit<MIB_TCPTABLE2>);

            let e = GetTcpTable2(table.as_mut_ptr(), &mut size, false);
            if e != 0 {
                return Err(Error);
            }

            let num_entries = table.assume_init().dwNumEntries;
            let els = {
                &*((table.as_mut_ptr() as *mut u32).offset(1) as *mut [Self::Item; 1])
            };

            let mut result = vec![];
            let mut i = 0;
            while i < num_entries {
                let row = *(els.as_ptr().offset(i as isize));
                result.push(row);
                i += 1;
            };

            Ok(result)
        } else {
            Err(Error)
        }
    }
}
