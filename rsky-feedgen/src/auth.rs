use base64::{Engine as _, engine::{general_purpose}};
use crate::models::JwtParts;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn verify_jwt(jwtstr: &String, service_did: &String) -> Result<String, String> {
	let parts = jwtstr
	    .split(".")
	    .map(String::from)
	    .collect::<Vec<_>>();
    
    if parts.len() != 3 {
        return Err("poorly formatted jwt".into());
    }

    let bytes = general_purpose::STANDARD_NO_PAD
        .decode(&parts[1]).unwrap();
    let payload = std::str::from_utf8(&bytes).unwrap();
    let payload: JwtParts = serde_json::from_str(payload).unwrap();
    
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    if since_the_epoch.as_millis() / 1000 > payload.exp {
    	return Err("jwt expired".into());
    }
    if service_did != &payload.aud {
    	return Err("jwt audience does not match service did".into());
    }
    // TO DO: Verify cryptographic signature
    if let Ok(jwtstr) = serde_json::to_string(&payload) {
    	Ok(jwtstr)
    } else {
    	Err("error parsing payload".into())
    }
}
