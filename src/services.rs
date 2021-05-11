// there must be a better way to do this
pub fn match_service(port: u16) -> String {
    let mut res = String::new();
   // let mut matched_service = services::unknown;
   // DON'T EVEN THINK ABOUT THAT MESS
    match port {
        9 => res = "Discard Protocol".to_string(),
        22 => res = "ssh".to_string(),
        21 => res = "ftp".to_string(),
        20 => res= "ftp_data".to_string(),
        43 => res = "who is".to_string(),
        53 => res = "domain".to_string(),
        80 | 8080 | 8280=> res= "http".to_string(),
        443 | 8443 | 8243 => res = "https".to_string(),
        139 => res = "netbios-ssn".to_string(),
        113 => res = "ident".to_string(),
        115 => res = "simple file transfer protocol".to_string(),
        118 => res = "SQL services".to_string(),
        135 => res = "msrpc".to_string(),
        137 => res = "netbios-ns".to_string(),
        138 => res = "netbios-dgm".to_string(),
        445 => res = "microsoft-ds".to_string(),
        631 => res = "ipp".to_string(),
        2222 => res = "EtherNetIP".to_string(),
        3306 => res = "mysql".to_string(),
        1990 => res = "stun-p1".to_string(),
        8000 => res = "django/dynamodb local".to_string(),
        8005 => res = "tomcat remote shutdown".to_string(),
        9987 => res = "teamspeak 3 server default port".to_string(),
        10000 => res = "snet-sensor-mgmt".to_string(),
        25565 => res = "minecraft server default port".to_string(),
        10011 => res = "teamspeak 3 ServerQuery".to_string(),
        16080 => res = "macOS Server Web (HTTP) service with performance cache".to_string(),
        30033 => res = "TeamSpeak 3 File Transfer".to_string(),
        _ => res = "service unknown".to_string(),
    }
 return res;
}
