use crypto::CryptoEngine;
use log::info;
use storage::StorageEngine;
use webauthn::WebAuthnAuthenticator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!("FIDO2 WebAuthn Example");
    info!("This example demonstrates registration and authentication via WebAuthnAuthenticator.");

    let crypto = CryptoEngine::new()?;
    let storage = StorageEngine::new()?;
    let mut authenticator = WebAuthnAuthenticator::new(
        [
            0xaa, 0xbb, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0x00, 0x11, 0x22,
            0x33, 0x44,
        ],
        crypto,
        storage,
    )?;

    info!("Authenticator ready");

    // 1. Get capabilities via GetInfo
    let info = authenticator.get_info()?;
    info!("Firmware version: {}", info.firmware_version);

    // 2. Registration (MakeCredential with WebAuthn validation)
    let req = webauthn::MakeCredentialRequest {
        client_data_hash: [0u8; 32].to_vec(),
        rp: webauthn::RelyingParty {
            id: "example.com".to_string(),
            name: None,
            icon: None,
        },
        user: webauthn::User {
            id: b"user123".to_vec(),
            name: None,
            display_name: None,
            icon_url: None,
        },
        pub_key_cred_params: vec![webauthn::PublicKeyCredParams {
            r#type: "public-key".to_string(),
            algorithms: -8,
        }],
        exclude_list: vec![],
        extensions: None,
        options: webauthn::MakeCredentialOptions {
            rk: false,
            uv: true,
            up: true,
            extended: false,
        },
        pin_uv_auth_param: None,
        pin_protocol: None,
        enterprise_protections: None,
    };

    info!("Registering credential...");
    match authenticator.make_credential(req) {
        Ok(_) => info!("Credential registered successfully"),
        Err(e) => info!("MakeCredential error: {}", e),
    }

    // 3. Authentication (GetAssertion with WebAuthn validation)
    info!("Getting assertion...");
    let req = webauthn::GetAssertionRequest {
        rp_id: "example.com".to_string(),
        credentials: vec![],
        allow_list: None,
        client_data_hash: [0u8; 32].to_vec(),
        extensions: None,
        options: webauthn::GetAssertionOptions { up: true, uv: true },
        pin_uv_auth_param: None,
        pin_protocol: None,
        uv: None,
    };

    match authenticator.get_assertion(req) {
        Ok(_) => info!("Assertion completed successfully"),
        Err(e) => info!("GetAssertion error: {}", e),
    }

    info!("Example complete.");
    Ok(())
}
