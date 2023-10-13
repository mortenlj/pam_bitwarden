#[macro_use]
extern crate pamsm;

use std::ffi::CStr;
use std::io::Write;
use std::process::{Command, Output, Stdio};
use std::str;
use std::str::FromStr;

use anyhow::Result;
use log::{debug, error};
use pamsm::{Pam, PamError, PamFlags, PamLibExt, PamServiceModule};
use serde::Deserialize;

struct PamLoadKeys;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BitwardenStatusOutput {
    status: String
}

impl PamServiceModule for PamLoadKeys {
    fn open_session(pam: Pam, _flags: PamFlags, args: Vec<String>) -> PamError {
        init_logging(&args);

        match pam.get_user(None) {
            Ok(Some(user)) => {
                let output = run_bw(user, vec!["status"], None).unwrap();

                debug!("bw status: {}", str::from_utf8(output.stdout.as_slice()).unwrap());

                let status: BitwardenStatusOutput = serde_json::from_slice(output.stdout.as_slice()).unwrap();

                if status.status == "locked" {
                    debug!("Attempting unlock ...");

                    if let Ok(password) = pam.get_authtok(None) {
                        match run_bw(user, vec!["unlock", "--raw"], password) {
                            Ok(session_output) => {
                                debug!("Successfully unlocked vault");
                                match str::from_utf8(session_output.stdout.as_slice()) {
                                    Ok(session_id) => {
                                        debug!("Setting BW_SESSION={}", session_id);
                                        if let Err(_) = pam.putenv(format!("BW_SESSION={}", session_id).as_str()) {
                                            error!("Failed to set BW_SESSION environment variable");
                                            return PamError::SYSTEM_ERR;
                                        }
                                    }
                                    Err(e) => {
                                        error!("Failed to parse session id: {:?}", e);
                                        return PamError::AUTH_ERR;
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Failed to unlock vault: {:?}", e);
                                return PamError::AUTH_ERR;
                            }
                        }
                    }
                }
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

        PamError::SUCCESS
    }

    fn close_session(pam: Pam, _flags: PamFlags, args: Vec<String>) -> PamError {
        init_logging(&args);
        match pam.get_user(None) {
            Ok(Some(user)) => {
                if let Err(e) = run_bw(user, vec!["lock"], None) {
                    error!("Failed to lock vault: {:?}", e);
                    return PamError::SYSTEM_ERR;
                }
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
        PamError::SUCCESS
    }
}

fn run_bw(user: &CStr, cmd: Vec<&str>, password: Option<&CStr>) -> Result<Output> {
    let password_args = match password {
        Some(_) => vec!["--passwordenv", "BW_PASSWORD"],
        None => vec![],
    };
    let pw = password.map(|p| p.to_str().unwrap()).unwrap_or("");
    let child = Command::new("/usr/bin/sudo")
        .env("BW_PASSWORD", pw)
        .arg("--preserve-env=BW_PASSWORD")
        .arg("--non-interactive")
        .arg("--set-home")
        .arg("--user")
        .arg(user.to_str().unwrap())
        .arg("/snap/bin/bw")
        .args(cmd)
        .args(password_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| anyhow::anyhow!("failed to execute process: {}", e))?;
    let output = child
        .wait_with_output()
        .map_err(|e| anyhow::anyhow!("failed to wait on child: {}", e))?;
    if !output.status.success() {
        Err(anyhow::anyhow!(
            "command exited with status: {}",
            output.status
        ))?
    }

    Ok(output)
}

fn init_logging(args: &Vec<String>) {
    let level = if args.len() > 0 {
        args[0].as_str()
    } else {
        "warn"
    };
    env_logger::builder()
        .filter_level(log::LevelFilter::from_str(level).unwrap_or(log::LevelFilter::Warn))
        .parse_default_env()
        .format(|buf, record| {
            let ts = buf.timestamp();
            writeln!(buf, "pam_bitwarden: {}: {}: {}", ts, record.level(), record.args())
        })
        .init();
    debug!("Logging initialized");
}

pam_module!(PamLoadKeys);
