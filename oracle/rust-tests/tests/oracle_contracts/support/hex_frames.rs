fn decode_hex(value: &str, label: &str) -> Vec<u8> {
    hex::decode(value).unwrap_or_else(|err| panic!("invalid hex for {label}: {err}"))
}

fn encode_varint(mut value: i32) -> Vec<u8> {
    let mut out = Vec::new();
    loop {
        if (value & !0x7f) == 0 {
            out.push(value as u8);
            return out;
        }

        out.push(((value & 0x7f) | 0x80) as u8);
        value = ((value as u32) >> 7) as i32;
    }
}

fn read_varint_prefix(bytes: &[u8]) -> (i32, usize) {
    let mut value = 0i32;

    for (index, byte) in bytes.iter().copied().enumerate() {
        value |= ((byte & 0x7f) as i32) << (7 * index);
        if byte & 0x80 == 0 {
            return (value, index + 1);
        }

        assert!(
            index < 4,
            "VarInt prefix exceeds Minecraft's 5-byte packet id limit"
        );
    }

    panic!("missing complete VarInt prefix in framed packet")
}

fn try_read_varint_from_reader<R: Read>(reader: &mut R) -> Result<i32, String> {
    let mut value = 0i32;

    for index in 0..5 {
        let mut byte = [0u8; 1];
        reader
            .read_exact(&mut byte)
            .map_err(|err| format!("failed to read network VarInt byte {index}: {err}"))?;
        value |= ((byte[0] & 0x7f) as i32) << (7 * index);
        if byte[0] & 0x80 == 0 {
            return Ok(value);
        }
    }

    Err("network VarInt exceeded Minecraft's 5-byte limit".to_owned())
}

fn read_network_frame_from_reader<R: Read>(reader: &mut R, label: &str) -> Result<Vec<u8>, String> {
    let packet_len = try_read_varint_from_reader(reader)? as usize;
    let mut body = vec![0; packet_len];
    reader
        .read_exact(&mut body)
        .map_err(|err| format!("read {label} packet body: {err}"))?;

    let mut frame = encode_varint(packet_len as i32);
    frame.extend_from_slice(&body);
    Ok(frame)
}

fn official_network_frame_from_framed_payload(framed: &[u8]) -> Vec<u8> {
    let mut expected = encode_varint(framed.len() as i32);
    expected.extend_from_slice(framed);
    expected
}
