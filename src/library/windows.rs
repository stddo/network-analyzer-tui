use std::fmt::Error;
use std::mem::MaybeUninit;
use std::ptr;

use windows::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER;
use windows::Win32::Networking::WinSock::ntohs;
use windows::Win32::NetworkManagement::IpHelper::{MIB_TCP6TABLE_OWNER_PID, MIB_TCPROW_OWNER_PID, MIB_UDP6TABLE_OWNER_PID, MIB_UDPTABLE_OWNER_PID};

use crate::app::LocalProcess;

pub(in crate::library) mod tcp_table;
pub(in crate::library) mod udp_table;

pub trait WinDynStruct {
    type Item;
    type Buffer;

    unsafe fn call(buffer: *mut Self::Buffer, size: &mut u32) -> u32;

    fn map(mut f: impl FnMut(&Self::Item) -> ()) -> Result<(), Error> where Self: Sized {
        unsafe {
            let mut size = 0;
            let error = Self::call(ptr::null_mut(), &mut size);

            return if error == ERROR_INSUFFICIENT_BUFFER.0 {
                let mut buf: Vec<u8> = Vec::with_capacity(TryInto::<usize>::try_into(size).unwrap());
                let table = &mut *(buf.as_mut_ptr() as *mut MaybeUninit<Self>);

                let error = Self::call(table.as_mut_ptr() as *mut Self::Buffer, &mut size);
                if error != 0 {
                    return Err(Error);
                }

                let (num_entries, els) = {
                    let pointer = table.as_mut_ptr() as *mut u32;
                    let num_entries = *pointer.offset(0);
                    let els = &*(pointer.offset(1) as *mut [Self::Item; 1]);
                    (num_entries, els)
                };

                let mut i = 0;
                while i < num_entries {
                    let row = &*(els.as_ptr().offset(i as isize));
                    f(row);
                    i += 1;
                };

                Ok(())
            } else {
                Err(Error)
            };
        }
    }
}

pub fn collect_open_ports_by_app() -> Result<Vec<LocalProcess>, Error> {
    let mut apps = vec![];

    MIB_TCPROW_OWNER_PID::map(|el| {
        unsafe {
            apps.push(LocalProcess {
                local_port: ntohs(el.dwLocalPort as u16),
                pid: el.dwOwningPid,
            });
        }
    })?;

    MIB_TCP6TABLE_OWNER_PID::map(|el| {
        unsafe {
            apps.push(LocalProcess {
                local_port: ntohs(el.dwLocalPort as u16),
                pid: el.dwOwningPid,
            });
        }
    })?;

    MIB_UDPTABLE_OWNER_PID::map(|el| {
        unsafe {
            apps.push(LocalProcess {
                local_port: ntohs(el.dwLocalPort as u16),
                pid: el.dwOwningPid,
            });
        }
    })?;

    MIB_UDP6TABLE_OWNER_PID::map(|el| {
        unsafe {
            apps.push(LocalProcess {
                local_port: ntohs(el.dwLocalPort as u16),
                pid: el.dwOwningPid,
            });
        }
    })?;

    Ok(apps)
}
