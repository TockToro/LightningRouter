// Jackson Coxson

use std::{net::SocketAddr, str::FromStr};

use idevice::usbmuxd::{Connection, UsbmuxdAddr, UsbmuxdDevice};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::UdpSocket,
};

#[tokio::main]
pub async fn main() {
    env_logger::init();

    let addr = UsbmuxdAddr::default();
    let mut conn = addr
        .connect(69)
        .await
        .expect("Failed to connect to usbmuxd");
    let devices = conn.get_devices().await.expect("Failed to get devices");
    let devices: Vec<UsbmuxdDevice> = devices
        .into_iter()
        .filter(|x| x.connection_type == Connection::Usb)
        .collect();
    let dev = devices.first().expect("No devices connected via USB");
    println!("Using {} to connect to", dev.udid);
    let conn = conn
        .connect_to_device(dev.device_id, 51820, "lightning-router")
        .await
        .expect("Failed to connect to LightningRouter on device. Is the app running?");
    println!("Connected!");
    let mut conn = conn.get_socket().unwrap();

    let input_socket = UdpSocket::bind("0.0.0.0:3400")
        .await
        .expect("Failed to bind to UDP port 3400");

    let mut buf0 = [0u8; u16::MAX as usize];
    let mut conn_buf = [0u8; 2];
    let mut input_address = SocketAddr::from_str("127.0.0.1:1").unwrap();
    loop {
        tokio::select! {
            res = input_socket.recv_from(&mut buf0) => {
                match res {
                    Ok((size, addr)) => {
                        input_address = addr;
                        let buf0 = &buf0[..size];
                        let size = buf0.len() as u16;
                        if let Err(e) = conn.write_all(&size.to_le_bytes()).await {
                            eprintln!("Failed to send to device: {e:?}");
                            return;
                        }
                        if let Err(e) = conn.write_all(buf0).await {
                            eprintln!("Failed to send to device: {e:?}");
                            return;
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to read from input socket: {e:?}");
                        return;
                    },
                }
            }
            res = conn.read_exact(&mut conn_buf) => {
                match res {
                    Ok(_) => {
                        let size = u16::from_le_bytes(conn_buf);
                        let mut buf1 = vec![0u8; size as usize];
                        conn.read_exact(&mut buf1).await.expect("Failed to read body");
                        if let Err(e) = input_socket.send_to(&buf1, input_address).await {
                            eprintln!("Failed to send to input socket: {e:?}");
                            return;
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read from device: {e:?}");
                        return;
                    }
                }
            }
        }
    }
}
