use std::ffi::c_void;
use std::io::stdin;
use std::ptr;

use anyhow::{Error, Result};
use clap::{Parser, Subcommand};
use windows::Win32::Media::Audio::{
    IMMDeviceEnumerator, IMMNotificationClient, MMDeviceEnumerator,
};
use windows::Win32::System::Com::{CoCreateInstance, CoInitialize, CLSCTX_INPROC_SERVER};
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;

use crate::client::ListenClient;

mod client;

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(name = "mmefix")]
#[command(arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Listen for all device updates")]
    Listen = 0,
    #[command(about = "Apply stored settings")]
    Apply = 1,
}

// TODO: Apply and store of named devices
// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#subcommands

fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Listen) => {
            listen_changes().unwrap();
        }
        Some(Commands::Apply) => {
            println!("Apply")
        }
        None => {}
    }

    Ok(())
}

fn listen_changes() -> Result<()> {
    let _ = unsafe { CoInitialize(Some(ptr::null_mut() as *const c_void)) };

    let device_enumerator: IMMDeviceEnumerator =
        unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_INPROC_SERVER)? };

    let client: IMMNotificationClient = ListenClient.into();
    unsafe {
        device_enumerator
            .RegisterEndpointNotificationCallback(&client)
            .expect("Failed to register client");
    }
    println!("Client registered");

    println!("Press enter to quit");
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_n) => {
            println!("Quitting.")
        }
        Err(_error) => {
            println!("Quitting.")
        }
    }

    // Finally unregister
    unsafe {
        device_enumerator
            .UnregisterEndpointNotificationCallback(&client)
            .expect("Failed to register client");
    }

    Ok(())
}

// The device type enum
static CLASS_GUID: &str = "{a45c254e-df1c-4efd-8020-67d146a850e0},24";

#[allow(dead_code)]
fn find_device() -> Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let capture = hklm
        .open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\MMDevices\\Audio\\Capture")?;

    for i in capture.enum_keys().map(|x| x.unwrap()) {
        println!("{}", i);
        let properties = capture.open_subkey(format!("{i}\\Properties")).unwrap();

        // Check if device type matches
        let device_type: String = properties.get_value(CLASS_GUID)?;
        if device_type == "FOCUSRITEUSB" {
            println!("Found our device as {i}!")
        }
    }

    Ok(())
}
