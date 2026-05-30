use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::{env, fs};
use steven_protocol::protocol::packet;
use steven_protocol::protocol::{Direction, PacketType, State};

const CONFIGURATION_KEEPALIVE_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_keepalive_codec.test-manifest.json";
const CONFIGURATION_KEEPALIVE_CASE_ID: &str = "configuration_keepalive_codec";
const CONFIGURATION_KEEPALIVE_CONTRACT: &str =
    "oracle/contracts/775/configuration_keepalive_codec.contract.json";
const CONFIGURATION_KEEPALIVE_ANSWER: &str =
    "oracle/answers/775/configuration_keepalive_codec.answer.jsonl";
const ORACLE_CONTRACTS_RUST_TARGET: &str = "oracle/rust-tests/tests/oracle_contracts.rs";
const CONFIGURATION_KEEPALIVE_TEST_NAME: &str =
    "configuration_keepalive_matches_official_oracle_answer";
const CONFIGURATION_KEEPALIVE_COMPARISON_SURFACE: &str = "codec_body_only";
const CONFIGURATION_KEEPALIVE_FRAMED_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_keepalive_framed_dispatch.test-manifest.json";
const CONFIGURATION_KEEPALIVE_FRAMED_CASE_ID: &str = "configuration_keepalive_framed_dispatch";
const CONFIGURATION_KEEPALIVE_FRAMED_CONTRACT: &str =
    "oracle/contracts/775/configuration_keepalive_framed_dispatch.contract.json";
const CONFIGURATION_KEEPALIVE_FRAMED_ANSWER: &str =
    "oracle/answers/775/configuration_keepalive_framed_dispatch.answer.jsonl";
const CONFIGURATION_KEEPALIVE_FRAMED_TEST_NAME: &str =
    "configuration_keepalive_framed_dispatch_decodes_official_oracle_answer";
const CONFIGURATION_KEEPALIVE_FRAMED_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const CONFIGURATION_KEEPALIVE_CLIENTBOUND_FRAMED_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_keepalive_clientbound_framed_dispatch.test-manifest.json";
const CONFIGURATION_KEEPALIVE_CLIENTBOUND_FRAMED_CASE_ID: &str =
    "configuration_keepalive_clientbound_framed_dispatch";
const CONFIGURATION_KEEPALIVE_CLIENTBOUND_FRAMED_CONTRACT: &str =
    "oracle/contracts/775/configuration_keepalive_clientbound_framed_dispatch.contract.json";
const CONFIGURATION_KEEPALIVE_CLIENTBOUND_FRAMED_ANSWER: &str =
    "oracle/answers/775/configuration_keepalive_clientbound_framed_dispatch.answer.jsonl";
const CONFIGURATION_KEEPALIVE_CLIENTBOUND_FRAMED_TEST_NAME: &str =
    "configuration_keepalive_clientbound_framed_dispatch_decodes_official_oracle_answer";
const CONFIGURATION_KEEPALIVE_CLIENTBOUND_FRAMED_COMPARISON_SURFACE: &str =
    "framed_dispatch_decode";
const CONFIGURATION_FINISH_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_finish_framed_terminal.test-manifest.json";
const CONFIGURATION_FINISH_CASE_ID: &str = "configuration_finish_framed_terminal";
const CONFIGURATION_FINISH_CONTRACT: &str =
    "oracle/contracts/775/configuration_finish_framed_terminal.contract.json";
const CONFIGURATION_FINISH_ANSWER: &str =
    "oracle/answers/775/configuration_finish_framed_terminal.answer.jsonl";
const CONFIGURATION_FINISH_TEST_NAME: &str =
    "configuration_finish_framed_terminal_matches_official_oracle_answer";
const CONFIGURATION_FINISH_COMPARISON_SURFACE: &str = "decoded_fields";

#[derive(Debug, Deserialize)]
struct TestManifest {
    case_id: String,
    contract_path: String,
    answer_path: String,
    rust_test_target: String,
    rust_test_name: String,
    comparison_surface: String,
}

#[derive(Debug, Deserialize)]
struct OracleAnswer {
    case_id: String,
    answer: ConfigurationOracleAnswer,
}

#[derive(Debug, Deserialize)]
struct ConfigurationOracleAnswer {
    #[serde(default)]
    input_id: i64,
    #[serde(default)]
    encoded_body_hex: String,
    encoded_framed_hex: Option<String>,
    decoded_id: Option<i64>,
    decoded_packet_type: Option<String>,
    decoded_packet_class: Option<String>,
    remaining_after_official_decode: Option<i32>,
    packet_type: Option<String>,
    serverbound: Option<FinishDirectionAnswer>,
    clientbound: Option<FinishDirectionAnswer>,
    #[serde(default)]
    configuration_serverbound_packet_table: Vec<PacketTableRow>,
    #[serde(default)]
    configuration_clientbound_packet_table: Vec<PacketTableRow>,
}

#[derive(Debug, Deserialize)]
struct PacketTableRow {
    packet_id: i32,
    packet_type: String,
}

#[derive(Debug, Deserialize)]
struct FinishDirectionAnswer {
    flow: String,
    packet_type: String,
    decoded_packet_type: String,
    decoded_packet_class: String,
    instance_is_terminal: bool,
    decoded_is_terminal: bool,
    encoded_framed_hex: String,
    encoded_body_hex: String,
    remaining_after_official_decode: i32,
    configuration_packet_table: Vec<PacketTableRow>,
}

fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_json<T: for<'de> Deserialize<'de>>(relative_path: &str) -> T {
    let path = project_root().join(relative_path);
    let contents = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&contents)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

fn read_answer(relative_path: &str, expected_case_id: &str) -> OracleAnswer {
    let path = project_root().join(relative_path);
    let contents = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    let mut answers = Vec::new();

    for (index, line) in contents.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }

        let answer: OracleAnswer = serde_json::from_str(line).unwrap_or_else(|err| {
            panic!(
                "failed to parse oracle answer JSONL row {} from {}: {err}",
                index + 1,
                path.display()
            )
        });

        assert_eq!(
            answer.case_id,
            expected_case_id,
            "oracle answer JSONL row {} from {} has the wrong case_id",
            index + 1,
            path.display()
        );
        answers.push(answer);
    }

    assert_eq!(
        answers.len(),
        1,
        "expected exactly one non-empty oracle answer row for {} in {}; found {}",
        expected_case_id,
        path.display(),
        answers.len()
    );

    answers.pop().unwrap()
}

fn packet_id_for(table: &[PacketTableRow], packet_type: &str) -> i32 {
    table
        .iter()
        .find(|row| row.packet_type == packet_type)
        .unwrap_or_else(|| panic!("missing packet id for {packet_type}"))
        .packet_id
}

fn assert_optional_runner_env(key: &str, actual: &str) {
    if let Ok(expected) = env::var(key) {
        assert_eq!(
            actual,
            expected.as_str(),
            "runner scope env {key} did not match"
        );
    }
}

fn assert_runner_scope(manifest_path: &str, manifest: &TestManifest) {
    assert_optional_runner_env("ORACLE_EXPECTED_MANIFEST", manifest_path);
    assert_optional_runner_env("ORACLE_EXPECTED_CASE_ID", &manifest.case_id);
    assert_optional_runner_env(
        "ORACLE_EXPECTED_RUST_TEST_TARGET",
        &manifest.rust_test_target,
    );
    assert_optional_runner_env("ORACLE_EXPECTED_RUST_TEST_NAME", &manifest.rust_test_name);
}

fn decode_hex(value: &str, label: &str) -> Vec<u8> {
    hex::decode(value).unwrap_or_else(|err| panic!("invalid hex for {label}: {err}"))
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

#[test]
fn configuration_keepalive_matches_official_oracle_answer() {
    // Case-local typed adapter. Future cases should be dispatched from manifest
    // topology instead of copying this shape as the general oracle runner model.
    let manifest: TestManifest = read_json(CONFIGURATION_KEEPALIVE_MANIFEST);
    assert_eq!(manifest.case_id, CONFIGURATION_KEEPALIVE_CASE_ID);
    assert_eq!(manifest.contract_path, CONFIGURATION_KEEPALIVE_CONTRACT);
    assert_eq!(manifest.answer_path, CONFIGURATION_KEEPALIVE_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(manifest.rust_test_name, CONFIGURATION_KEEPALIVE_TEST_NAME);
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_KEEPALIVE_COMPARISON_SURFACE
    );
    assert_runner_scope(CONFIGURATION_KEEPALIVE_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_serverbound_packet_table,
        "minecraft:keep_alive",
    );
    let packet = packet::configuration::serverbound::ConfigurationKeepAliveServerbound_i64 {
        id: oracle.answer.input_id,
    };

    assert_eq!(packet.packet_id(775), expected_packet_id);

    let mut body = Vec::new();
    packet.write(&mut body).unwrap();

    assert_eq!(hex::encode(body), oracle.answer.encoded_body_hex);
}

#[test]
fn configuration_keepalive_framed_dispatch_decodes_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_KEEPALIVE_FRAMED_MANIFEST);
    assert_eq!(manifest.case_id, CONFIGURATION_KEEPALIVE_FRAMED_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_KEEPALIVE_FRAMED_CONTRACT
    );
    assert_eq!(manifest.answer_path, CONFIGURATION_KEEPALIVE_FRAMED_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_KEEPALIVE_FRAMED_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_KEEPALIVE_FRAMED_COMPARISON_SURFACE
    );
    assert_runner_scope(CONFIGURATION_KEEPALIVE_FRAMED_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(oracle.answer.decoded_id, Some(oracle.answer.input_id));
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:keep_alive")
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ServerboundKeepAlivePacket")
    );

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_serverbound_packet_table,
        "minecraft:keep_alive",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("framed dispatch answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded = packet::packet_by_id(
        775,
        State::Configuration,
        Direction::Serverbound,
        framed_packet_id,
        &mut body_slice,
    )
    .unwrap()
    .expect("expected Configuration serverbound keep_alive to dispatch");

    match decoded {
        packet::Packet::KeepAliveServerbound_i64(packet) => {
            assert_eq!(packet.id, oracle.answer.input_id);
        }
        other => panic!("expected keep_alive dispatch, got {other:?}"),
    }
    assert!(
        body_slice.is_empty(),
        "decoded packet did not consume the official body bytes"
    );
}

#[test]
fn configuration_keepalive_clientbound_framed_dispatch_decodes_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_KEEPALIVE_CLIENTBOUND_FRAMED_MANIFEST);
    assert_eq!(
        manifest.case_id,
        CONFIGURATION_KEEPALIVE_CLIENTBOUND_FRAMED_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_KEEPALIVE_CLIENTBOUND_FRAMED_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_KEEPALIVE_CLIENTBOUND_FRAMED_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_KEEPALIVE_CLIENTBOUND_FRAMED_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_KEEPALIVE_CLIENTBOUND_FRAMED_COMPARISON_SURFACE
    );
    assert_runner_scope(
        CONFIGURATION_KEEPALIVE_CLIENTBOUND_FRAMED_MANIFEST,
        &manifest,
    );

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(oracle.answer.decoded_id, Some(oracle.answer.input_id));
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:keep_alive")
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundKeepAlivePacket")
    );

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:keep_alive",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("clientbound framed dispatch answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Configuration,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Configuration clientbound keep_alive packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| panic!("Stevenarella errored while decoding clientbound keep_alive: {err}"))
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound keep_alive packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::KeepAliveClientbound_i64(packet) => {
            assert_eq!(packet.id, oracle.answer.input_id);
        }
        other => panic!("expected clientbound keep_alive dispatch, got {other:?}"),
    }
    assert!(
        body_slice.is_empty(),
        "decoded clientbound keep_alive did not consume the official body bytes"
    );
}

#[test]
fn configuration_finish_framed_terminal_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_FINISH_MANIFEST);
    assert_eq!(manifest.case_id, CONFIGURATION_FINISH_CASE_ID);
    assert_eq!(manifest.contract_path, CONFIGURATION_FINISH_CONTRACT);
    assert_eq!(manifest.answer_path, CONFIGURATION_FINISH_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(manifest.rust_test_name, CONFIGURATION_FINISH_TEST_NAME);
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_FINISH_COMPARISON_SURFACE
    );
    assert_runner_scope(CONFIGURATION_FINISH_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    let packet_type = oracle
        .answer
        .packet_type
        .as_deref()
        .expect("finish_configuration answer missing packet_type");

    let serverbound = oracle
        .answer
        .serverbound
        .as_ref()
        .expect("finish_configuration answer missing serverbound direction");
    assert_eq!(serverbound.packet_type, packet_type);
    assert_finish_direction_matches_official_frame(serverbound, Direction::Serverbound);

    let clientbound = oracle
        .answer
        .clientbound
        .as_ref()
        .expect("finish_configuration answer missing clientbound direction");
    assert_eq!(clientbound.packet_type, packet_type);
    assert_finish_direction_matches_official_frame(clientbound, Direction::Clientbound);
}

fn assert_finish_direction_matches_official_frame(
    official: &FinishDirectionAnswer,
    direction: Direction,
) {
    assert_eq!(official.decoded_packet_type, official.packet_type);
    let official_name_fragment = rust_name_fragment_from_packet_type(&official.packet_type);
    assert!(
        official
            .decoded_packet_class
            .contains(&official_name_fragment),
        "official decoded packet class did not preserve packet identity: {}",
        official.decoded_packet_class
    );
    assert_eq!(official.remaining_after_official_decode, 0);
    assert_eq!(
        official.instance_is_terminal, official.decoded_is_terminal,
        "official INSTANCE and decoded packet terminal flags differ for {}",
        official.flow
    );

    let expected_packet_id =
        packet_id_for(&official.configuration_packet_table, &official.packet_type);
    let framed = decode_hex(&official.encoded_framed_hex, "encoded_framed_hex");
    let body = decode_hex(&official.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Configuration,
            direction,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Configuration {} finish_configuration packet id {}",
            official.flow, framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| panic!("Stevenarella errored while decoding finish_configuration: {err}"))
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration {} finish_configuration packet id {}",
                official.flow, framed_packet_id
            )
        });
    assert!(
        format!("{decoded:?}").contains(&official_name_fragment),
        "decoded packet did not preserve finish_configuration identity: {decoded:?}"
    );
    assert!(
        body_slice.is_empty(),
        "decoded finish_configuration did not consume the official body bytes"
    );
}

fn rust_name_fragment_from_packet_type(packet_type: &str) -> String {
    packet_type
        .rsplit(':')
        .next()
        .unwrap_or(packet_type)
        .split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<String>()
}
