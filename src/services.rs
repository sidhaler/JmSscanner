// MAYBE ONE TIME IT WILL BE USEFUL
/*#[derive(Debug, Clone, Copy)]
pub enum services {
    unknown,
    htpp,
    https,
    ftp,
    ftp_data,
    ssh,
    netbios_ssn,
    microsoft_ds,
    domain,
}
*/
// there must be a better way to do this
pub fn match_service(port: u16) -> String {
    let mut res = String::new();
   // let mut matched_service = services::unknown;
   // DON'T EVEN THINK ABOUT THAT MESS
    match port {
        53 => res = "domain".to_string(),
        22 => res = "ssh".to_string(),
        21 => res = "ftp".to_string(),
        20 => res= "ftp_data".to_string(),
        80 | 8080 => res= "htpp".to_string(),
        443 | 8443 => res = "https".to_string(),
        3306 => res = "mysql".to_string(),
        139 => res = "netbios-ssn".to_string(),
        113 => res = "ident".to_string(),
        115 => res = "simple file transfer protocol".to_string(),
        118 => res = "(SQL) services".to_string(),
        135 => res = "msrpc".to_string(),
        137 => res = "netbios-ns".to_string(),
        138 => res = "netbios-dgm".to_string(),
        445 => res = "microsoft-ds".to_string(),
        631 => res = "ipp".to_string(),
        2222 => res = "EtherNetIP".to_string(),
        10000 => res = "snet-sensor-mgmt".to_string(),
        9 => res = "Discard Protocol".to_string(),
        _ => res = "service unknown".to_string(),
    }

 return res;
}
