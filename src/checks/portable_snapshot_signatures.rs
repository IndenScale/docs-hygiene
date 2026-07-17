fn validate_trusted_snapshot_keys(config: &Config, diagnostics: &mut Vec<Diagnostic>) {
    for (id, key) in &config.governance.portable_snapshots.trusted_keys {
        if !valid_claim_identity(id) || decode_hex::<32>(key).is_none() {
            push_snapshot_diagnostic(
                "DH_SNAPSHOT_006",
                "docs-hygiene.yml".to_owned(),
                format!(
                    "Portable snapshot trusted key '{id}' requires a stable identity and 64-hex Ed25519 public key."
                ),
                diagnostics,
            );
        }
    }
}

fn validate_snapshot_signature(
    config: &Config,
    manifest: &PortableSnapshotManifest,
    path: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(signature) = &manifest.signature else {
        if config.governance.portable_snapshots.require_signatures {
            push_snapshot_diagnostic(
                "DH_SNAPSHOT_006",
                path.to_owned(),
                format!("Portable snapshot '{}' requires a signature.", manifest.id),
                diagnostics,
            );
        }
        return;
    };
    let Some(key_hex) = config
        .governance
        .portable_snapshots
        .trusted_keys
        .get(&signature.key_id)
    else {
        push_snapshot_diagnostic(
            "DH_SNAPSHOT_006",
            path.to_owned(),
            format!(
                "Portable snapshot '{}' signature key '{}' is not trusted.",
                manifest.id, signature.key_id
            ),
            diagnostics,
        );
        return;
    };
    let verified = if signature.algorithm == "ed25519" {
        decode_hex::<32>(key_hex)
            .and_then(|bytes| ed25519_dalek::VerifyingKey::from_bytes(&bytes).ok())
            .zip(decode_hex::<64>(&signature.value))
            .zip(manifest.signing_bytes().ok())
            .is_some_and(|((key, signature), bytes)| {
                let signature = ed25519_dalek::Signature::from_bytes(&signature);
                ed25519_dalek::Verifier::verify(&key, &bytes, &signature).is_ok()
            })
    } else {
        false
    };
    if !verified {
        push_snapshot_diagnostic(
            "DH_SNAPSHOT_006",
            path.to_owned(),
            format!(
                "Portable snapshot '{}' has an invalid Ed25519 signature.",
                manifest.id
            ),
            diagnostics,
        );
    }
}

fn decode_hex<const N: usize>(value: &str) -> Option<[u8; N]> {
    if value.len() != N * 2 {
        return None;
    }
    let mut output = [0; N];
    for (index, chunk) in value.as_bytes().chunks_exact(2).enumerate() {
        output[index] = (hex_nibble(chunk[0])? << 4) | hex_nibble(chunk[1])?;
    }
    Some(output)
}

fn hex_nibble(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}
