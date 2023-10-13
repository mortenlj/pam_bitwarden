#[macro_use]
extern crate pamsm;

use std::ffi::CStr;
use std::io::Write;
use std::process::{Command, Output, Stdio};
use std::str;
use std::str::FromStr;

use anyhow::Result;
use log::{error, info};
use pamsm::{Pam, PamError, PamFlags, PamLibExt, PamServiceModule};

struct PamLoadKeys;

impl PamServiceModule for PamLoadKeys {
    fn open_session(pam: Pam, _flags: PamFlags, args: Vec<String>) -> PamError {
        init_logging(&args);

        match pam.get_user(None) {
            Ok(Some(user)) => {
                let output = run_bw(user, "status").unwrap();

                info!("bw status: {}", str::from_utf8(output.stdout.as_slice()).unwrap());

                info!("Hello, {:?}!", user);
            }
            Ok(None) => {
                error!("ERROR: get_user returned None, should not happen!");
                return PamError::SYSTEM_ERR;
            }
            Err(e) => {
                error!("ERROR: get_user returned error: {:?}", e);
                return e;
            }
        }

        let _password = pam.get_authtok(None).unwrap();
        PamError::SUCCESS
    }

    fn close_session(pam: Pam, _flags: PamFlags, args: Vec<String>) -> PamError {
        init_logging(&args);
        let user = pam.get_user(None).unwrap();
        info!("Goodbye, {:?}!", user);
        PamError::SUCCESS
    }
}

fn run_bw(user: &CStr, cmd: &str) -> Result<Output> {
    let child = Command::new("/usr/bin/sudo")
        .arg("--non-interactive")
        .arg("--set-home")
        .arg("--user")
        .arg(user.to_str().unwrap())
        .arg("/snap/bin/bw")
        .arg(cmd)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| anyhow::anyhow!("failed to execute process: {}", e))?;
    let output = child
        .wait_with_output()
        .map_err(|e| anyhow::anyhow!("failed to wait on child: {}", e))?;
    Ok(output)
}

fn init_logging(args: &Vec<String>) {
    let level = if args.len() > 1 {
        args[1].as_str()
    } else {
        "warn"
    };
    env_logger::builder()
        .filter_level(log::LevelFilter::from_str(level).unwrap_or(log::LevelFilter::Warn))
        .format(|buf, record| {
            let ts = buf.timestamp();
            writeln!(buf, "pam_bitwarden: {}: {}: {}", ts, record.level(), record.args())
        })
        .init();
}

pam_module!(PamLoadKeys);
