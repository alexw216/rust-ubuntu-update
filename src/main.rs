use std::process::Command;
use std::path::Path;
use syslog::{Facility};
use std::{thread, time};

fn main() {
    syslog::init(Facility::LOG_USER,
                         log::LevelFilter::Debug,
                         Some("Ubuntu System Update"))
                         .expect("Failed to initialize log facility.");
    let duration = time::Duration::from_secs(600);
    loop {
    update_system();
    thread::sleep(duration);
    }
}

fn update_system() {

    println!("Checking System update!");
    log::debug!("{}", "Checking System update!");

    let apt_update_status = Command::new("sh")
                                     .arg("-c")
                                     .arg("sudo apt-get update -y")
                                     .status()
                                     .expect("failed to execure process");

    println!("Upadte process finsihed with: {}", apt_update_status);
    log::debug!("Upadte process finsihed with: {}", apt_update_status);

    println!("Upgrading System!");
    log::debug!("{}", "Upgrading System update!");

    let apt_upgrade_status = Command::new("sh")
                                      .arg("-c")
                                      .arg("sudo apt-get upgrade -y")
                                      .status()
                                      .expect("failed to execure process");

    println!("Upgrade process finsihed with: {}", apt_upgrade_status);
    log::debug!("Upgrade process finsihed with: {}", apt_upgrade_status);

    println!("Performing dist-upgrade!");
    log::debug!("{}", "Performing dist-upgrade!");

    let apt_dist_upgrade_status = Command::new("sh")
                                           .arg("-c")
                                           .arg("sudo apt-get dist-upgrade -y")
                                           .status()
                                           .expect("failed to execure process");
 

    println!("Dist-upgrade process finsihed with: {}", apt_dist_upgrade_status); 
    log::debug!("Dist-upgrade process finsihed with: {}", apt_dist_upgrade_status);



    println!("Performing autoremove!");
    log::debug!("{}", "Performing autoremove!");

    let apt_autoremove_status = Command::new("sh")
                                           .arg("-c")
                                           .arg("sudo apt-get autoremove -y")
                                           .status()
                                           .expect("failed to execure process");


    println!("Autoremove process finsihed with: {}", apt_autoremove_status);
    log::debug!("Autoremove process finsihed with: {}", apt_autoremove_status);


    println!("Performing autoclean!");
    log::debug!("{}", "Performing autoclean!");

    let apt_autoclean_status = Command::new("sh")
                                           .arg("-c")
                                           .arg("sudo apt-get autoclean -y")
                                           .status()
                                           .expect("failed to execure process");


    println!("Autoclean process finsihed with: {}", apt_autoclean_status);
    log::debug!("Autoclean process finsihed with: {}", apt_autoclean_status);


    /*
    println!("Performing do-release-upgrade!");
    log::debug!("{}", "Performing do-release-upgrade!");

    
    let do_release_upgrade_status = Command::new("sh")
                                             .arg("-c")
                                             .arg("sudo do-release-upgrade")
                                             .status()
                                             .expect("failed to execure process");

    println!("Do-release-upgrade process finsihed with: {}", do_release_upgrade_status);
    log::debug!("Do-release-upgrade process finsihed with: {}", do_release_upgrade_status); 

    */

    if Path::new("/var/run/reboot-required").exists() {
       println!("System needs a reboot, rebooting server.");
       log::debug!("{}", "System needs a reboot, rebooting server."); 
       Command::new("sh")
                .arg("-c")
                .arg("sudo reboot")
                .spawn()
                .expect("failed to execure process");

    }

}
