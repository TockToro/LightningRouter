// Jackson Coxson

use once_cell::sync::Lazy;
use std::ffi::{CStr, c_char};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, UdpSocket},
    runtime::{self, Runtime},
    sync::oneshot::{self, error::TryRecvError},
};

mod tcp;

static RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()
        .unwrap()
});

pub struct ThreadKiller(oneshot::Sender<()>);

/// # Safety
/// Don't be dumb
#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_simple_udp_proxy(
    address: *const c_char,
    port: u16,
    thread_killer: *mut *mut ThreadKiller,
) -> bool {
    if address.is_null() {
        return false;
    }
    let address = match unsafe { CStr::from_ptr(address) }.to_str() {
        Ok(a) => a,
        Err(_) => {
            return false;
        }
    };
    let (killer, mut killed) = oneshot::channel::<()>();
    let input_listener = match std::net::TcpListener::bind("0.0.0.0:51820") {
        Ok(l) => l,
        Err(e) => {
            eprintln!("no input bind: {e:?}");
            return false;
        }
    };
    input_listener.set_nonblocking(true).unwrap();

    RUNTIME.spawn(async move {
        let input_listener = TcpListener::from_std(input_listener).unwrap();

        while let Err(TryRecvError::Empty) = killed.try_recv() {
            tokio::select! {
                Ok((mut input_socket, _addr)) = input_listener.accept() => {
                    input_socket.set_nodelay(true).unwrap();
                    let target_address = format!("{address}:{port}");
                    RUNTIME.spawn(async move {
                        let target_socket = match UdpSocket::bind("0.0.0.0:0").await {
                            Ok(t) => t,
                            Err(e) => {
                                eprintln!("Failed to bind target socket: {e:?}");
                                return;
                            }
                        };

                        if let Err(e) = target_socket.connect(&target_address).await {
                            eprintln!("Failed to connect to target socket {target_address}: {e:?}");
                            return;
                        }

                        let mut buf0 = [0u8; 2];
                        let mut buf1 = [0u8; u16::MAX as usize];
                        loop {
                            tokio::select! {
                                res = input_socket.read_exact(&mut buf0) => {
                                    match res {
                                        Ok(_) => {
                                            let size = u16::from_le_bytes(buf0);
                                            let mut buf0 = vec![0u8; size as usize];
                                            if let Err(e) = input_socket.read_exact(&mut buf0).await {
                                                eprintln!("Failed to read {size} bytes from input socket: {e:?}");
                                                return;
                                            }
                                            if size == 0 {
                                                println!("Input connection closed");
                                                return;
                                            }
                                            if let Err(e) = target_socket.send(&buf0).await {
                                                eprintln!("Failed to send to target socket: {e:?}");
                                                return;
                                            }
                                        },
                                        Err(e) => {
                                            eprintln!("Failed to read from input socket: {e:?}");
                                            return;
                                        },
                                    }
                                }
                                res = target_socket.recv(&mut buf1) => {
                                    match res {
                                        Ok(size) => {
                                            if size == 0 {
                                                println!("Target connection closed");
                                                return;
                                            }
                                            let buf1 = &buf1[..size];
                                            let size = buf1.len() as u16;

                                            if let Err(e) = input_socket.write_all(&size.to_le_bytes()).await {
                                                eprintln!("Failed to send to input socket: {e:?}");
                                                return;
                                            }
                                            if let Err(e) = input_socket.write_all(buf1).await {
                                                eprintln!("Failed to send to input socket: {e:?}");
                                                return;
                                            }
                                        },
                                        Err(e) => {
                                            eprintln!("Failed to read from target socket: {e:?}");
                                            return;
                                        },
                                    }
                                }
                            }
                        }
                    });
                }
                _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {
                        // re-run the loop to test for thread killer
                    }
            }
        }
    });

    unsafe {
        *thread_killer = Box::into_raw(Box::new(ThreadKiller(killer)));
    }
    true
}

/// # Safety
/// Don't be dumb
#[unsafe(no_mangle)]
pub unsafe extern "C" fn thread_killer_kill(killer: *mut ThreadKiller) {
    if killer.is_null() {
        return;
    }

    let killer = unsafe { Box::from_raw(killer) };
    (*killer).0.send(()).ok();
}
