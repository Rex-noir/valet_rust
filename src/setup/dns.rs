use std::fs;
use std::process::Output;

use anyhow::{Ok, Result, bail};

use crate::core::{App, CommandManager};

#[derive(Debug)]
pub struct Dns;

impl Dns {
    pub fn setup(_app: &App) -> Result<()> {
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

                println!("Enable dnsmasq systemd service ...");
                Self::enable_dnsmasq_systemd_service()?;

                println!("Restarting systemd-resolved");
                Self::restart_systemd_resolved()?;
            }
        } else {
            bail!("Systemd version can't be determined! Aborting!!")
        }

        Ok(())
    }

    fn parse_systemd_version(stdout: &str) -> Option<u32> {
        let first_line = stdout.lines().next()?;

        let version_str = first_line.split_whitespace().nth(1)?;

        version_str.parse::<u32>().ok()
    }

    fn setup_dns_delegate_config() -> Result<()> {
        let dir = "/etc/systemd/dns-delegate.d";
        fs::create_dir_all(dir).map_err(|e| anyhow::anyhow!("Failed to create {}: {}", dir, e))?;

        let path = "/etc/systemd/dns-delegate.d/valet-rust.dns-delegate";
        let content = "[Delegate]\nDNS=127.0.0.1\nDomains=~test\nDNSSECMode=no\n";
        fs::write(path, content).map_err(|e| {
            anyhow::anyhow!("Failed to write DNS delegate config to {}: {}", path, e)
        })?;

        println!("DNS delegate config written to {}", path);
        Ok(())
    }

    fn restart_systemd_resolved() -> Result<()> {
        let cm = CommandManager::init();
        let output = cm.run("systemctl", &["restart", "systemd-resolved"])?;
        if !output.status.success() {
            bail!("Error restarting systemd-resolved");
        }
        Ok(())
    }

    fn setup_dnsmasq_configuration() -> Result<()> {
        let main_dnsmasq_conf_path = "/etc/dnsmasq.conf";
        let stub_dnsmasq_conf = include_str!("../stubs/dnsmasq.conf");

        fs::write(main_dnsmasq_conf_path, stub_dnsmasq_conf)?;

        let dir = "/etc/dnsmasq.d";
        fs::create_dir_all(dir).map_err(|e| anyhow::anyhow!("Failed to create {}: {}", dir, e))?;

        let path = "/etc/dnsmasq.d/valet-rust.conf";
        let config =
            "listen-address=127.0.0.1\nbind-interfaces\nno-resolv\naddress=/.test/127.0.0.1\n";
        fs::write(path, config)
            .map_err(|e| anyhow::anyhow!("Failed to write dnsmasq config to {}: {}", path, e))?;

        println!("dnsmasq config written to {}", path);
        Ok(())
    }

    fn restart_dnsmasq() -> Result<Output> {
        let cm = CommandManager::init();
        let run: std::result::Result<Output, anyhow::Error> =
            cm.run("systemctl", &["restart", "dnsmasq"]);
        run
    }

    fn disable_systemd_resolved_dns_stub_listener() -> Result<()> {
        let dir = "/etc/systemd/resolved.conf.d";
        fs::create_dir_all(dir).map_err(|e| anyhow::anyhow!("Failed to create {}: {}", dir, e))?;

        let path = "/etc/systemd/resolved.conf.d/no-stub-listener.conf";
        let content = "[Resolve]\nDNSStubListener=no\n";
        fs::write(path, content).map_err(|e| {
            anyhow::anyhow!(
                "Failed to write resolved stub listener config to {}: {}",
                path,
                e
            )
        })?;

        println!("DNSStubListener disabled via drop-in at {}", path);
        Ok(())
    }

    fn enable_dnsmasq_systemd_service() -> Result<Output> {
        let cm = CommandManager::init();
        let output = cm.run("systemctl", &["enable", "dnsmasq", "--now"])?;
        Ok(output)
    }
}
