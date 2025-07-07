use wasm_bindgen::prelude::*;

/// Binary format:
/// [4 bytes] Magic header: "RECF"
/// [1 byte ] Version
/// [4 bytes] Metadata length (big endian u32)
/// [4 bytes] Audio length (big endian u32)
/// [X bytes] Metadata (JSON string)
/// [Y bytes] Audio bytes

#[wasm_bindgen]
pub fn encode_audiodom(metadata_json: &str, audio: &[u8]) -> Vec<u8> {
    let mut buffer = Vec::new();

    // Header and version
    buffer.extend_from_slice(b"RECF");
    buffer.push(1u8); // version 1

    // Metadata + audio lengths
    let meta_len = metadata_json.len() as u32;
    let audio_len = audio.len() as u32;
    buffer.extend_from_slice(&meta_len.to_be_bytes());
    buffer.extend_from_slice(&audio_len.to_be_bytes());

    // Payload
    buffer.extend_from_slice(metadata_json.as_bytes());
    buffer.extend_from_slice(audio);

    buffer
}

#[wasm_bindgen]
pub fn decode_audiodom(data: &[u8]) -> Result<JsValue, JsValue> {
    if data.len() < 13 {
        return Err(JsValue::from_str("Data too short"));
    }

    // Check header
    if &data[0..4] != b"RECF" {
        return Err(JsValue::from_str("Invalid .audom file header"));
    }

    let version = data[4];
    if version != 1 {
        return Err(JsValue::from_str("Unsupported .audom version"));
    }

    // Lengths
    let meta_len = u32::from_be_bytes(data[5..9].try_into().unwrap()) as usize;
    let audio_len = u32::from_be_bytes(data[9..13].try_into().unwrap()) as usize;

    let meta_start = 13;
    let meta_end = meta_start + meta_len;
    let audio_start = meta_end;
    let audio_end = audio_start + audio_len;

    if data.len() < audio_end {
        return Err(JsValue::from_str("Data is incomplete or corrupted"));
    }

    // Extract metadata and audio
    let metadata_bytes = &data[meta_start..meta_end];
    let audio_bytes = &data[audio_start..audio_end];

    let metadata_str = std::str::from_utf8(metadata_bytes)
        .map_err(|_| JsValue::from_str("Invalid UTF-8 in metadata"))?;

    // Build JS object
    let result = js_sys::Object::new();
    js_sys::Reflect::set(
        &result,
        &JsValue::from_str("metadata"),
        &JsValue::from_str(metadata_str),
    )?;
    js_sys::Reflect::set(
        &result,
        &JsValue::from_str("audio"),
        &js_sys::Uint8Array::from(audio_bytes),
    )?;

    Ok(JsValue::from(result))
}
