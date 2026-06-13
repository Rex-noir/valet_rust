use std::process::Output;

use anyhow::{Ok, Result, bail};

use crate::core::{App, CommandManager};

#[derive(Debug)]
pub struct Dns;

impl Dns {
    pub fn setup(app: &App) -> Result<()> {
        let cm = CommandManager::init();

        // check systemd version

        let version = cm
            .run("systemctl", &["--version"])
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .and_then(|s| Self::parse_systemd_version(&s));

        if let Some(v) = version {
            println!("Systemd version detected : {}", v);
            if v < 258 {
                eprintln!("Not supported systemd version less than 258.")
            } else {
                println!("Setting up systemd resolved dns delegate config ...");
                Self::setup_dns_delegate_config()?;

                println!("Disable systemd resolved DNSStubListener");
                Self::disable_systemd_resolved_dns_stub_listener()?;

                println!("Installing dnsmasq ...");
                cm.install_package("dnsmasq")?;

                println!("Setting up dnsmasq configuration ...");
                Self::setup_dnsmasq_configuration()?;

                println!("Restarting dnsmasq");
                Self::restart_dnsmasq()?;

                println!("Restarting systemd-resolved");
                Self::restart_systemd_resolved()?;
            }
        } else {
            println!("Systemd version can't be determined! Aborting!!")
        }

        Ok(())
    }

    fn parse_systemd_version(stdout: &str) -> Option<u32> {
        let first_line = stdout.lines().next()?;

        let version_str = first_line.split_whitespace().nth(1)?;

        version_str.parse::<u32>().ok()
    }

    fn setup_dns_delegate_config() -> Result<()> {
        let cm = CommandManager::init();

        let status = cm.run_elevated(&["mkdir", "-p", "/etc/systemd/dns-delegate.d"])?;

        if !status.success() {
            eprintln!("Failed to ensure ensure_delegate_directory_exists");
        }
        let path = "/etc/systemd/dns-delegate.d/valet-rust.dns-delegate";
        let content = "[Delegate]\nDNS=127.0.0.1\nDomains=~test\nDNSSECMode=no\n";
        let cmd = format!("printf '%s' '{}' | sudo tee {} > /dev/null", content, path);
        let status = cm.run_elevated(&["sh", "-c", &cmd])?;
        if !status.success() {
            bail!("Failed to write DNS delegate config");
        }
        println!("DNS delegate config written to {}", path);
        Ok(())
    }

    fn restart_systemd_resolved() -> Result<()> {
        let cm = CommandManager::init();
        let status = cm.run_elevated(&["systemctl", "restart", "systemd-resolved"])?;
        if !status.success() {
            bail!("Error restarting systemd-resolved");
        }
        Ok(())
    }

    fn setup_dnsmasq_configuration() -> Result<()> {
        let cm = CommandManager::init();
        let status = cm.run_elevated(&["mkdir", "-p", "/etc/dnsmasq.d"])?;
        if !status.success() {
            bail!("Failed to ensure /etc/dnsmasq.d exists");
        }
        let config =
            "listen-address=127.0.0.1\nbind-interfaces\nno-resolv\naddress=/.test/127.0.0.1\n";
        let cmd = format!(
            "printf '%s' '{}' | sudo tee /etc/dnsmasq.d/valet-rust.conf > /dev/null",
            config
        );
        let status = cm.run_elevated(&["sh", "-c", &cmd])?;
        if !status.success() {
            bail!("Failed to write dnsmasq config");
        }
        println!("dnsmasq config written to /etc/dnsmasq.d/valet-rust.conf");
        Ok(())
    }

    fn restart_dnsmasq() -> Result<Output> {
        let cm = CommandManager::init();
        let run: std::result::Result<Output, anyhow::Error> =
            cm.run("systemctl", &["restart", "dnsmasq"]);
        run
    }

    fn disable_systemd_resolved_dns_stub_listener() -> Result<()> {
        let cm = CommandManager::init();

        let status = cm.run_elevated(&["mkdir", "-p", "/etc/systemd/resolved.conf.d"])?;
        if !status.success() {
            bail!("Failed to create /etc/systemd/resolved.conf.d");
        }

        let path = "/etc/systemd/resolved.conf.d/no-stub-listener.conf";
        let content = "[Resolve]\nDNSStubListener=no\n";
        let cmd = format!("printf '%s' '{}' | sudo tee {} > /dev/null", content, path);

        let status = cm.run_elevated(&["sh", "-c", &cmd])?;
        if !status.success() {
            bail!("Failed to write resolved stub listener config to {}", path);
        }

        println!("DNSStubListener disabled via drop-in at {}", path);
        Ok(())
    }
}
