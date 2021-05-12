pub fn match_service(port: u16) -> String {

   // TODO: ADD MORE PORTS SERVICE MATCHES
    match port {
        9 => return "Discard Protocol".to_string(),
        22 => return "ssh".to_string(),
        21 => return "ftp".to_string(),
        20 => return "ftp_data".to_string(),
        43 => return "who is".to_string(),
        53 => return "domain".to_string(),
        80 | 8080 | 8280=> return "http".to_string(),
        443 | 8443 | 8243 => return "https".to_string(),
        139 => return "netbios-ssn".to_string(),
        113 => return "ident".to_string(),
        115 => return "simple file transfer protocol".to_string(),
        118 => return "SQL services".to_string(),
        135 => return "msrpc".to_string(),
        137 => return "netbios-ns".to_string(),
        138 => return "netbios-dgm".to_string(),
        445 => return "microsoft-ds".to_string(),
        631 => return "ipp".to_string(),
        2222 => return "EtherNetIP".to_string(),
        3306 => return "mysql".to_string(),
        1990 => return "stun-p1".to_string(),
        8000 => return "django/dynamodb local".to_string(),
        8005 => return "tomcat remote shutdown".to_string(),
        9987 => return "teamspeak 3 server default port".to_string(),
        10000 => return "snet-sensor-mgmt".to_string(),
        25565 => return "minecraft server default port".to_string(),
        10011 => return "teamspeak 3 ServerQuery".to_string(),
        16080 => return "macOS Server Web (HTTP) service with performance cache".to_string(),
        30033 => return "TeamSpeak 3 File Transfer".to_string(),
        _ => return "service unknown".to_string(),
    }
 //return res;
}
