use serde::Deserialize;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::{env, fs};
use steven_protocol::protocol::mapped_packet::MappablePacket;
use steven_protocol::protocol::packet;
use steven_protocol::protocol::{Conn, Direction, PacketType, State};

const CONFIGURATION_KEEPALIVE_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_keepalive_codec.test-manifest.json";
const CONFIGURATION_KEEPALIVE_CASE_ID: &str = "configuration_keepalive_codec";
const CONFIGURATION_KEEPALIVE_CONTRACT: &str =
    "oracle/contracts/775/configuration_keepalive_codec.contract.json";
const CONFIGURATION_KEEPALIVE_ANSWER: &str =
    "oracle/answers/775/configuration_keepalive_codec.answer.jsonl";
const ORACLE_CONTRACTS_RUST_TARGET: &str = "oracle/rust-tests/tests/oracle_contracts.rs";
const HANDSHAKE_INTENTION_MANIFEST: &str =
    "oracle/test-manifests/775/handshake_intention_framed_dispatch.test-manifest.json";
const HANDSHAKE_INTENTION_CASE_ID: &str = "handshake_intention_framed_dispatch";
const HANDSHAKE_INTENTION_CONTRACT: &str =
    "oracle/contracts/775/handshake_intention_framed_dispatch.contract.json";
const HANDSHAKE_INTENTION_ANSWER: &str =
    "oracle/answers/775/handshake_intention_framed_dispatch.answer.jsonl";
const HANDSHAKE_INTENTION_TEST_NAME: &str =
    "handshake_intention_framed_dispatch_matches_official_oracle_answer";
const HANDSHAKE_INTENTION_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const LOGIN_HELLO_SERVERBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/login_hello_serverbound_framed_dispatch.test-manifest.json";
const LOGIN_HELLO_SERVERBOUND_CASE_ID: &str = "login_hello_serverbound_framed_dispatch";
const LOGIN_HELLO_SERVERBOUND_CONTRACT: &str =
    "oracle/contracts/775/login_hello_serverbound_framed_dispatch.contract.json";
const LOGIN_HELLO_SERVERBOUND_ANSWER: &str =
    "oracle/answers/775/login_hello_serverbound_framed_dispatch.answer.jsonl";
const LOGIN_HELLO_SERVERBOUND_TEST_NAME: &str =
    "login_hello_serverbound_framed_dispatch_matches_official_oracle_answer";
const LOGIN_HELLO_SERVERBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const LOGIN_KEY_SERVERBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/login_key_serverbound_framed_dispatch.test-manifest.json";
const LOGIN_KEY_SERVERBOUND_CASE_ID: &str = "login_key_serverbound_framed_dispatch";
const LOGIN_KEY_SERVERBOUND_CONTRACT: &str =
    "oracle/contracts/775/login_key_serverbound_framed_dispatch.contract.json";
const LOGIN_KEY_SERVERBOUND_ANSWER: &str =
    "oracle/answers/775/login_key_serverbound_framed_dispatch.answer.jsonl";
const LOGIN_KEY_SERVERBOUND_TEST_NAME: &str =
    "login_key_serverbound_framed_dispatch_matches_official_oracle_answer";
const LOGIN_KEY_SERVERBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const LOGIN_CUSTOM_QUERY_ANSWER_SERVERBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/login_custom_query_answer_serverbound_framed_dispatch.test-manifest.json";
const LOGIN_CUSTOM_QUERY_ANSWER_SERVERBOUND_CASE_ID: &str =
    "login_custom_query_answer_serverbound_framed_dispatch";
const LOGIN_CUSTOM_QUERY_ANSWER_SERVERBOUND_CONTRACT: &str =
    "oracle/contracts/775/login_custom_query_answer_serverbound_framed_dispatch.contract.json";
const LOGIN_CUSTOM_QUERY_ANSWER_SERVERBOUND_ANSWER: &str =
    "oracle/answers/775/login_custom_query_answer_serverbound_framed_dispatch.answer.jsonl";
const LOGIN_CUSTOM_QUERY_ANSWER_SERVERBOUND_TEST_NAME: &str =
    "login_custom_query_answer_serverbound_framed_dispatch_matches_official_oracle_answer";
const LOGIN_CUSTOM_QUERY_ANSWER_SERVERBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const LOGIN_ACKNOWLEDGED_SERVERBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/login_acknowledged_serverbound_framed_dispatch.test-manifest.json";
const LOGIN_ACKNOWLEDGED_SERVERBOUND_CASE_ID: &str =
    "login_acknowledged_serverbound_framed_dispatch";
const LOGIN_ACKNOWLEDGED_SERVERBOUND_CONTRACT: &str =
    "oracle/contracts/775/login_acknowledged_serverbound_framed_dispatch.contract.json";
const LOGIN_ACKNOWLEDGED_SERVERBOUND_ANSWER: &str =
    "oracle/answers/775/login_acknowledged_serverbound_framed_dispatch.answer.jsonl";
const LOGIN_ACKNOWLEDGED_SERVERBOUND_TEST_NAME: &str =
    "login_acknowledged_serverbound_framed_dispatch_matches_official_oracle_answer";
const LOGIN_ACKNOWLEDGED_SERVERBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const LOGIN_COOKIE_RESPONSE_SERVERBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/login_cookie_response_serverbound_framed_dispatch.test-manifest.json";
const LOGIN_COOKIE_RESPONSE_SERVERBOUND_CASE_ID: &str =
    "login_cookie_response_serverbound_framed_dispatch";
const LOGIN_COOKIE_RESPONSE_SERVERBOUND_CONTRACT: &str =
    "oracle/contracts/775/login_cookie_response_serverbound_framed_dispatch.contract.json";
const LOGIN_COOKIE_RESPONSE_SERVERBOUND_ANSWER: &str =
    "oracle/answers/775/login_cookie_response_serverbound_framed_dispatch.answer.jsonl";
const LOGIN_COOKIE_RESPONSE_SERVERBOUND_TEST_NAME: &str =
    "login_cookie_response_serverbound_framed_dispatch_matches_official_oracle_answer";
const LOGIN_COOKIE_RESPONSE_SERVERBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const LOGIN_DISCONNECT_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/login_disconnect_clientbound_framed_dispatch.test-manifest.json";
const LOGIN_DISCONNECT_CLIENTBOUND_CASE_ID: &str = "login_disconnect_clientbound_framed_dispatch";
const LOGIN_DISCONNECT_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/login_disconnect_clientbound_framed_dispatch.contract.json";
const LOGIN_DISCONNECT_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/login_disconnect_clientbound_framed_dispatch.answer.jsonl";
const LOGIN_DISCONNECT_CLIENTBOUND_TEST_NAME: &str =
    "login_disconnect_clientbound_framed_dispatch_matches_official_oracle_answer";
const LOGIN_DISCONNECT_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const LOGIN_HELLO_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/login_hello_clientbound_framed_dispatch.test-manifest.json";
const LOGIN_HELLO_CLIENTBOUND_CASE_ID: &str = "login_hello_clientbound_framed_dispatch";
const LOGIN_HELLO_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/login_hello_clientbound_framed_dispatch.contract.json";
const LOGIN_HELLO_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/login_hello_clientbound_framed_dispatch.answer.jsonl";
const LOGIN_HELLO_CLIENTBOUND_TEST_NAME: &str =
    "login_hello_clientbound_framed_dispatch_matches_official_oracle_answer";
const LOGIN_HELLO_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const LOGIN_FINISHED_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/login_finished_clientbound_framed_dispatch.test-manifest.json";
const LOGIN_FINISHED_CLIENTBOUND_CASE_ID: &str = "login_finished_clientbound_framed_dispatch";
const LOGIN_FINISHED_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/login_finished_clientbound_framed_dispatch.contract.json";
const LOGIN_FINISHED_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/login_finished_clientbound_framed_dispatch.answer.jsonl";
const LOGIN_FINISHED_CLIENTBOUND_TEST_NAME: &str =
    "login_finished_clientbound_framed_dispatch_matches_official_oracle_answer";
const LOGIN_FINISHED_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const LOGIN_COMPRESSION_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/login_compression_clientbound_framed_dispatch.test-manifest.json";
const LOGIN_COMPRESSION_CLIENTBOUND_CASE_ID: &str = "login_compression_clientbound_framed_dispatch";
const LOGIN_COMPRESSION_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/login_compression_clientbound_framed_dispatch.contract.json";
const LOGIN_COMPRESSION_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/login_compression_clientbound_framed_dispatch.answer.jsonl";
const LOGIN_COMPRESSION_CLIENTBOUND_TEST_NAME: &str =
    "login_compression_clientbound_framed_dispatch_matches_official_oracle_answer";
const LOGIN_COMPRESSION_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const LOGIN_CUSTOM_QUERY_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/login_custom_query_clientbound_framed_dispatch.test-manifest.json";
const LOGIN_CUSTOM_QUERY_CLIENTBOUND_CASE_ID: &str =
    "login_custom_query_clientbound_framed_dispatch";
const LOGIN_CUSTOM_QUERY_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/login_custom_query_clientbound_framed_dispatch.contract.json";
const LOGIN_CUSTOM_QUERY_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/login_custom_query_clientbound_framed_dispatch.answer.jsonl";
const LOGIN_CUSTOM_QUERY_CLIENTBOUND_TEST_NAME: &str =
    "login_custom_query_clientbound_framed_dispatch_matches_official_oracle_answer";
const LOGIN_CUSTOM_QUERY_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const LOGIN_COOKIE_REQUEST_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/login_cookie_request_clientbound_framed_dispatch.test-manifest.json";
const LOGIN_COOKIE_REQUEST_CLIENTBOUND_CASE_ID: &str =
    "login_cookie_request_clientbound_framed_dispatch";
const LOGIN_COOKIE_REQUEST_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/login_cookie_request_clientbound_framed_dispatch.contract.json";
const LOGIN_COOKIE_REQUEST_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/login_cookie_request_clientbound_framed_dispatch.answer.jsonl";
const LOGIN_COOKIE_REQUEST_CLIENTBOUND_TEST_NAME: &str =
    "login_cookie_request_clientbound_framed_dispatch_matches_official_oracle_answer";
const LOGIN_COOKIE_REQUEST_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const PLAY_BUNDLE_DELIMITER_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/play_bundle_delimiter_clientbound_framed_dispatch.test-manifest.json";
const PLAY_BUNDLE_DELIMITER_CLIENTBOUND_CASE_ID: &str =
    "play_bundle_delimiter_clientbound_framed_dispatch";
const PLAY_BUNDLE_DELIMITER_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/play_bundle_delimiter_clientbound_framed_dispatch.contract.json";
const PLAY_BUNDLE_DELIMITER_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/play_bundle_delimiter_clientbound_framed_dispatch.answer.jsonl";
const PLAY_BUNDLE_DELIMITER_CLIENTBOUND_TEST_NAME: &str =
    "play_bundle_delimiter_clientbound_framed_dispatch_matches_official_oracle_answer";
const PLAY_BUNDLE_DELIMITER_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const PLAY_ADD_ENTITY_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/play_add_entity_clientbound_framed_dispatch.test-manifest.json";
const PLAY_ADD_ENTITY_CLIENTBOUND_CASE_ID: &str = "play_add_entity_clientbound_framed_dispatch";
const PLAY_ADD_ENTITY_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/play_add_entity_clientbound_framed_dispatch.contract.json";
const PLAY_ADD_ENTITY_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/play_add_entity_clientbound_framed_dispatch.answer.jsonl";
const PLAY_ADD_ENTITY_CLIENTBOUND_TEST_NAME: &str =
    "play_add_entity_clientbound_framed_dispatch_matches_official_oracle_answer";
const PLAY_ADD_ENTITY_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const PLAY_ANIMATE_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/play_animate_clientbound_framed_dispatch.test-manifest.json";
const PLAY_ANIMATE_CLIENTBOUND_CASE_ID: &str = "play_animate_clientbound_framed_dispatch";
const PLAY_ANIMATE_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/play_animate_clientbound_framed_dispatch.contract.json";
const PLAY_ANIMATE_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/play_animate_clientbound_framed_dispatch.answer.jsonl";
const PLAY_ANIMATE_CLIENTBOUND_TEST_NAME: &str =
    "play_animate_clientbound_framed_dispatch_matches_official_oracle_answer";
const PLAY_ANIMATE_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const PLAY_AWARD_STATS_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/play_award_stats_clientbound_framed_dispatch.test-manifest.json";
const PLAY_AWARD_STATS_CLIENTBOUND_CASE_ID: &str = "play_award_stats_clientbound_framed_dispatch";
const PLAY_AWARD_STATS_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/play_award_stats_clientbound_framed_dispatch.contract.json";
const PLAY_AWARD_STATS_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/play_award_stats_clientbound_framed_dispatch.answer.jsonl";
const PLAY_AWARD_STATS_CLIENTBOUND_TEST_NAME: &str =
    "play_award_stats_clientbound_framed_dispatch_matches_official_oracle_answer";
const PLAY_AWARD_STATS_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const PLAY_BLOCK_CHANGED_ACK_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/play_block_changed_ack_clientbound_framed_dispatch.test-manifest.json";
const PLAY_BLOCK_CHANGED_ACK_CLIENTBOUND_CASE_ID: &str =
    "play_block_changed_ack_clientbound_framed_dispatch";
const PLAY_BLOCK_CHANGED_ACK_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/play_block_changed_ack_clientbound_framed_dispatch.contract.json";
const PLAY_BLOCK_CHANGED_ACK_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/play_block_changed_ack_clientbound_framed_dispatch.answer.jsonl";
const PLAY_BLOCK_CHANGED_ACK_CLIENTBOUND_TEST_NAME: &str =
    "play_block_changed_ack_clientbound_framed_dispatch_matches_official_oracle_answer";
const PLAY_BLOCK_CHANGED_ACK_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const PLAY_BLOCK_DESTRUCTION_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/play_block_destruction_clientbound_framed_dispatch.test-manifest.json";
const PLAY_BLOCK_DESTRUCTION_CLIENTBOUND_CASE_ID: &str =
    "play_block_destruction_clientbound_framed_dispatch";
const PLAY_BLOCK_DESTRUCTION_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/play_block_destruction_clientbound_framed_dispatch.contract.json";
const PLAY_BLOCK_DESTRUCTION_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/play_block_destruction_clientbound_framed_dispatch.answer.jsonl";
const PLAY_BLOCK_DESTRUCTION_CLIENTBOUND_TEST_NAME: &str =
    "play_block_destruction_clientbound_framed_dispatch_matches_official_oracle_answer";
const PLAY_BLOCK_DESTRUCTION_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const PLAY_BLOCK_ENTITY_DATA_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/play_block_entity_data_clientbound_framed_dispatch.test-manifest.json";
const PLAY_BLOCK_ENTITY_DATA_CLIENTBOUND_CASE_ID: &str =
    "play_block_entity_data_clientbound_framed_dispatch";
const PLAY_BLOCK_ENTITY_DATA_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/play_block_entity_data_clientbound_framed_dispatch.contract.json";
const PLAY_BLOCK_ENTITY_DATA_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/play_block_entity_data_clientbound_framed_dispatch.answer.jsonl";
const PLAY_BLOCK_ENTITY_DATA_CLIENTBOUND_TEST_NAME: &str =
    "play_block_entity_data_clientbound_framed_dispatch_matches_official_oracle_answer";
const PLAY_BLOCK_ENTITY_DATA_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const PLAY_BLOCK_EVENT_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/play_block_event_clientbound_framed_dispatch.test-manifest.json";
const PLAY_BLOCK_EVENT_CLIENTBOUND_CASE_ID: &str = "play_block_event_clientbound_framed_dispatch";
const PLAY_BLOCK_EVENT_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/play_block_event_clientbound_framed_dispatch.contract.json";
const PLAY_BLOCK_EVENT_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/play_block_event_clientbound_framed_dispatch.answer.jsonl";
const PLAY_BLOCK_EVENT_CLIENTBOUND_TEST_NAME: &str =
    "play_block_event_clientbound_framed_dispatch_matches_official_oracle_answer";
const PLAY_BLOCK_EVENT_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const PLAY_BLOCK_UPDATE_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/play_block_update_clientbound_framed_dispatch.test-manifest.json";
const PLAY_BLOCK_UPDATE_CLIENTBOUND_CASE_ID: &str = "play_block_update_clientbound_framed_dispatch";
const PLAY_BLOCK_UPDATE_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/play_block_update_clientbound_framed_dispatch.contract.json";
const PLAY_BLOCK_UPDATE_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/play_block_update_clientbound_framed_dispatch.answer.jsonl";
const PLAY_BLOCK_UPDATE_CLIENTBOUND_TEST_NAME: &str =
    "play_block_update_clientbound_framed_dispatch_matches_official_oracle_answer";
const PLAY_BLOCK_UPDATE_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const PLAY_BOSS_EVENT_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/play_boss_event_clientbound_framed_dispatch.test-manifest.json";
const PLAY_BOSS_EVENT_CLIENTBOUND_CASE_ID: &str = "play_boss_event_clientbound_framed_dispatch";
const PLAY_BOSS_EVENT_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/play_boss_event_clientbound_framed_dispatch.contract.json";
const PLAY_BOSS_EVENT_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/play_boss_event_clientbound_framed_dispatch.answer.jsonl";
const PLAY_BOSS_EVENT_CLIENTBOUND_TEST_NAME: &str =
    "play_boss_event_clientbound_framed_dispatch_matches_official_oracle_answer";
const PLAY_BOSS_EVENT_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const PLAY_CHANGE_DIFFICULTY_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/play_change_difficulty_clientbound_framed_dispatch.test-manifest.json";
const PLAY_CHANGE_DIFFICULTY_CLIENTBOUND_CASE_ID: &str =
    "play_change_difficulty_clientbound_framed_dispatch";
const PLAY_CHANGE_DIFFICULTY_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/play_change_difficulty_clientbound_framed_dispatch.contract.json";
const PLAY_CHANGE_DIFFICULTY_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/play_change_difficulty_clientbound_framed_dispatch.answer.jsonl";
const PLAY_CHANGE_DIFFICULTY_CLIENTBOUND_TEST_NAME: &str =
    "play_change_difficulty_clientbound_framed_dispatch_matches_official_oracle_answer";
const PLAY_CHANGE_DIFFICULTY_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const PLAY_CHUNK_BATCH_FINISHED_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/play_chunk_batch_finished_clientbound_framed_dispatch.test-manifest.json";
const PLAY_CHUNK_BATCH_FINISHED_CLIENTBOUND_CASE_ID: &str =
    "play_chunk_batch_finished_clientbound_framed_dispatch";
const PLAY_CHUNK_BATCH_FINISHED_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/play_chunk_batch_finished_clientbound_framed_dispatch.contract.json";
const PLAY_CHUNK_BATCH_FINISHED_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/play_chunk_batch_finished_clientbound_framed_dispatch.answer.jsonl";
const PLAY_CHUNK_BATCH_FINISHED_CLIENTBOUND_TEST_NAME: &str =
    "play_chunk_batch_finished_clientbound_framed_dispatch_matches_official_oracle_answer";
const PLAY_CHUNK_BATCH_FINISHED_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const PLAY_CHUNK_BATCH_START_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/play_chunk_batch_start_clientbound_framed_dispatch.test-manifest.json";
const PLAY_CHUNK_BATCH_START_CLIENTBOUND_CASE_ID: &str =
    "play_chunk_batch_start_clientbound_framed_dispatch";
const PLAY_CHUNK_BATCH_START_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/play_chunk_batch_start_clientbound_framed_dispatch.contract.json";
const PLAY_CHUNK_BATCH_START_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/play_chunk_batch_start_clientbound_framed_dispatch.answer.jsonl";
const PLAY_CHUNK_BATCH_START_CLIENTBOUND_TEST_NAME: &str =
    "play_chunk_batch_start_clientbound_framed_dispatch_matches_official_oracle_answer";
const PLAY_CHUNK_BATCH_START_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const PLAY_CHUNKS_BIOMES_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/play_chunks_biomes_clientbound_framed_dispatch.test-manifest.json";
const PLAY_CHUNKS_BIOMES_CLIENTBOUND_CASE_ID: &str =
    "play_chunks_biomes_clientbound_framed_dispatch";
const PLAY_CHUNKS_BIOMES_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/play_chunks_biomes_clientbound_framed_dispatch.contract.json";
const PLAY_CHUNKS_BIOMES_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/play_chunks_biomes_clientbound_framed_dispatch.answer.jsonl";
const PLAY_CHUNKS_BIOMES_CLIENTBOUND_TEST_NAME: &str =
    "play_chunks_biomes_clientbound_framed_dispatch_matches_official_oracle_answer";
const PLAY_CHUNKS_BIOMES_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const PLAY_CLEAR_TITLES_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/play_clear_titles_clientbound_framed_dispatch.test-manifest.json";
const PLAY_CLEAR_TITLES_CLIENTBOUND_CASE_ID: &str = "play_clear_titles_clientbound_framed_dispatch";
const PLAY_CLEAR_TITLES_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/play_clear_titles_clientbound_framed_dispatch.contract.json";
const PLAY_CLEAR_TITLES_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/play_clear_titles_clientbound_framed_dispatch.answer.jsonl";
const PLAY_CLEAR_TITLES_CLIENTBOUND_TEST_NAME: &str =
    "play_clear_titles_clientbound_framed_dispatch_matches_official_oracle_answer";
const PLAY_CLEAR_TITLES_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const PLAY_COMMAND_SUGGESTIONS_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/play_command_suggestions_clientbound_framed_dispatch.test-manifest.json";
const PLAY_COMMAND_SUGGESTIONS_CLIENTBOUND_CASE_ID: &str =
    "play_command_suggestions_clientbound_framed_dispatch";
const PLAY_COMMAND_SUGGESTIONS_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/play_command_suggestions_clientbound_framed_dispatch.contract.json";
const PLAY_COMMAND_SUGGESTIONS_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/play_command_suggestions_clientbound_framed_dispatch.answer.jsonl";
const PLAY_COMMAND_SUGGESTIONS_CLIENTBOUND_TEST_NAME: &str =
    "play_command_suggestions_clientbound_framed_dispatch_matches_official_oracle_answer";
const PLAY_COMMAND_SUGGESTIONS_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const PLAY_COMMANDS_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/play_commands_clientbound_framed_dispatch.test-manifest.json";
const PLAY_COMMANDS_CLIENTBOUND_CASE_ID: &str = "play_commands_clientbound_framed_dispatch";
const PLAY_COMMANDS_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/play_commands_clientbound_framed_dispatch.contract.json";
const PLAY_COMMANDS_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/play_commands_clientbound_framed_dispatch.answer.jsonl";
const PLAY_COMMANDS_CLIENTBOUND_TEST_NAME: &str =
    "play_commands_clientbound_framed_dispatch_matches_official_oracle_answer";
const PLAY_COMMANDS_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const PLAY_CONTAINER_CLOSE_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/play_container_close_clientbound_framed_dispatch.test-manifest.json";
const PLAY_CONTAINER_CLOSE_CLIENTBOUND_CASE_ID: &str =
    "play_container_close_clientbound_framed_dispatch";
const PLAY_CONTAINER_CLOSE_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/play_container_close_clientbound_framed_dispatch.contract.json";
const PLAY_CONTAINER_CLOSE_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/play_container_close_clientbound_framed_dispatch.answer.jsonl";
const PLAY_CONTAINER_CLOSE_CLIENTBOUND_TEST_NAME: &str =
    "play_container_close_clientbound_framed_dispatch_matches_official_oracle_answer";
const PLAY_CONTAINER_CLOSE_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
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
const CONFIGURATION_KEEPALIVE_RUNTIME_SEND_MANIFEST: &str =
    "oracle/test-manifests/775/runtime/configuration_keepalive_runtime_send_helper.test-manifest.json";
const CONFIGURATION_KEEPALIVE_RUNTIME_SEND_CASE_ID: &str =
    "configuration_keepalive_runtime_send_helper";
const CONFIGURATION_KEEPALIVE_RUNTIME_SEND_CONTRACT: &str =
    "oracle/contracts/775/runtime/configuration_keepalive_runtime_send_helper.contract.json";
const CONFIGURATION_KEEPALIVE_RUNTIME_SEND_TEST_NAME: &str =
    "configuration_keepalive_runtime_send_helper_sends_official_configuration_frame";
const CONFIGURATION_KEEPALIVE_RUNTIME_SEND_COMPARISON_SURFACE: &str = "runtime_send_helper_frame";
const CONFIGURATION_KEEPALIVE_RUNTIME_PROTOCOL_ECHO_MANIFEST: &str =
    "oracle/test-manifests/775/runtime/configuration_keepalive_runtime_protocol_echo.test-manifest.json";
const CONFIGURATION_KEEPALIVE_RUNTIME_PROTOCOL_ECHO_CASE_ID: &str =
    "configuration_keepalive_runtime_protocol_echo";
const CONFIGURATION_KEEPALIVE_RUNTIME_PROTOCOL_ECHO_CONTRACT: &str =
    "oracle/contracts/775/runtime/configuration_keepalive_runtime_protocol_echo.contract.json";
const CONFIGURATION_KEEPALIVE_RUNTIME_PROTOCOL_ECHO_TEST_NAME: &str =
    "configuration_keepalive_runtime_protocol_echo_reads_maps_and_sends_official_frame";
const CONFIGURATION_KEEPALIVE_RUNTIME_PROTOCOL_ECHO_COMPARISON_SURFACE: &str =
    "runtime_protocol_echo_frame";
const CONFIGURATION_KEEPALIVE_RUNTIME_SPAWN_READER_MANIFEST: &str =
    "oracle/test-manifests/775/runtime/configuration_keepalive_runtime_spawn_reader_reaction.test-manifest.json";
const CONFIGURATION_KEEPALIVE_RUNTIME_SPAWN_READER_CASE_ID: &str =
    "configuration_keepalive_runtime_spawn_reader_reaction";
const CONFIGURATION_KEEPALIVE_RUNTIME_SPAWN_READER_CONTRACT: &str =
    "oracle/contracts/775/runtime/configuration_keepalive_runtime_spawn_reader_reaction.contract.json";
const CONFIGURATION_KEEPALIVE_RUNTIME_SPAWN_READER_TEST_NAME: &str =
    "configuration_keepalive_runtime_spawn_reader_reaction_echoes_official_frame";
const CONFIGURATION_KEEPALIVE_RUNTIME_SPAWN_READER_COMPARISON_SURFACE: &str =
    "runtime_spawn_reader_reaction_frame";
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
const CONFIGURATION_COOKIE_REQUEST_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_cookie_request_framed_dispatch.test-manifest.json";
const CONFIGURATION_COOKIE_REQUEST_CASE_ID: &str = "configuration_cookie_request_framed_dispatch";
const CONFIGURATION_COOKIE_REQUEST_CONTRACT: &str =
    "oracle/contracts/775/configuration_cookie_request_framed_dispatch.contract.json";
const CONFIGURATION_COOKIE_REQUEST_ANSWER: &str =
    "oracle/answers/775/configuration_cookie_request_framed_dispatch.answer.jsonl";
const CONFIGURATION_COOKIE_REQUEST_TEST_NAME: &str =
    "configuration_cookie_request_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_COOKIE_REQUEST_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
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
const CONFIGURATION_PING_PONG_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_ping_pong_framed_dispatch.test-manifest.json";
const CONFIGURATION_PING_PONG_CASE_ID: &str = "configuration_ping_pong_framed_dispatch";
const CONFIGURATION_PING_PONG_CONTRACT: &str =
    "oracle/contracts/775/configuration_ping_pong_framed_dispatch.contract.json";
const CONFIGURATION_PING_PONG_ANSWER: &str =
    "oracle/answers/775/configuration_ping_pong_framed_dispatch.answer.jsonl";
const CONFIGURATION_PING_PONG_TEST_NAME: &str =
    "configuration_ping_pong_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_PING_PONG_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const CONFIGURATION_CLIENT_INFORMATION_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_client_information_framed_dispatch.test-manifest.json";
const CONFIGURATION_CLIENT_INFORMATION_CASE_ID: &str =
    "configuration_client_information_framed_dispatch";
const CONFIGURATION_CLIENT_INFORMATION_CONTRACT: &str =
    "oracle/contracts/775/configuration_client_information_framed_dispatch.contract.json";
const CONFIGURATION_CLIENT_INFORMATION_ANSWER: &str =
    "oracle/answers/775/configuration_client_information_framed_dispatch.answer.jsonl";
const CONFIGURATION_CLIENT_INFORMATION_TEST_NAME: &str =
    "configuration_client_information_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_CLIENT_INFORMATION_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const CONFIGURATION_COOKIE_RESPONSE_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_cookie_response_framed_dispatch.test-manifest.json";
const CONFIGURATION_COOKIE_RESPONSE_CASE_ID: &str = "configuration_cookie_response_framed_dispatch";
const CONFIGURATION_COOKIE_RESPONSE_CONTRACT: &str =
    "oracle/contracts/775/configuration_cookie_response_framed_dispatch.contract.json";
const CONFIGURATION_COOKIE_RESPONSE_ANSWER: &str =
    "oracle/answers/775/configuration_cookie_response_framed_dispatch.answer.jsonl";
const CONFIGURATION_COOKIE_RESPONSE_TEST_NAME: &str =
    "configuration_cookie_response_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_COOKIE_RESPONSE_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const CONFIGURATION_CUSTOM_PAYLOAD_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_custom_payload_framed_dispatch.test-manifest.json";
const CONFIGURATION_CUSTOM_PAYLOAD_CASE_ID: &str = "configuration_custom_payload_framed_dispatch";
const CONFIGURATION_CUSTOM_PAYLOAD_CONTRACT: &str =
    "oracle/contracts/775/configuration_custom_payload_framed_dispatch.contract.json";
const CONFIGURATION_CUSTOM_PAYLOAD_ANSWER: &str =
    "oracle/answers/775/configuration_custom_payload_framed_dispatch.answer.jsonl";
const CONFIGURATION_CUSTOM_PAYLOAD_TEST_NAME: &str =
    "configuration_custom_payload_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_CUSTOM_PAYLOAD_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const CONFIGURATION_CUSTOM_PAYLOAD_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_custom_payload_clientbound_framed_dispatch.test-manifest.json";
const CONFIGURATION_CUSTOM_PAYLOAD_CLIENTBOUND_CASE_ID: &str =
    "configuration_custom_payload_clientbound_framed_dispatch";
const CONFIGURATION_CUSTOM_PAYLOAD_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/configuration_custom_payload_clientbound_framed_dispatch.contract.json";
const CONFIGURATION_CUSTOM_PAYLOAD_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/configuration_custom_payload_clientbound_framed_dispatch.answer.jsonl";
const CONFIGURATION_CUSTOM_PAYLOAD_CLIENTBOUND_TEST_NAME: &str =
    "configuration_custom_payload_clientbound_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_CUSTOM_PAYLOAD_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const CONFIGURATION_DISCONNECT_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_disconnect_clientbound_framed_dispatch.test-manifest.json";
const CONFIGURATION_DISCONNECT_CLIENTBOUND_CASE_ID: &str =
    "configuration_disconnect_clientbound_framed_dispatch";
const CONFIGURATION_DISCONNECT_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/configuration_disconnect_clientbound_framed_dispatch.contract.json";
const CONFIGURATION_DISCONNECT_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/configuration_disconnect_clientbound_framed_dispatch.answer.jsonl";
const CONFIGURATION_DISCONNECT_CLIENTBOUND_TEST_NAME: &str =
    "configuration_disconnect_clientbound_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_DISCONNECT_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const CONFIGURATION_RESET_CHAT_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_reset_chat_clientbound_framed_dispatch.test-manifest.json";
const CONFIGURATION_RESET_CHAT_CLIENTBOUND_CASE_ID: &str =
    "configuration_reset_chat_clientbound_framed_dispatch";
const CONFIGURATION_RESET_CHAT_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/configuration_reset_chat_clientbound_framed_dispatch.contract.json";
const CONFIGURATION_RESET_CHAT_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/configuration_reset_chat_clientbound_framed_dispatch.answer.jsonl";
const CONFIGURATION_RESET_CHAT_CLIENTBOUND_TEST_NAME: &str =
    "configuration_reset_chat_clientbound_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_RESET_CHAT_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const CONFIGURATION_REGISTRY_DATA_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_registry_data_clientbound_framed_dispatch.test-manifest.json";
const CONFIGURATION_REGISTRY_DATA_CLIENTBOUND_CASE_ID: &str =
    "configuration_registry_data_clientbound_framed_dispatch";
const CONFIGURATION_REGISTRY_DATA_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/configuration_registry_data_clientbound_framed_dispatch.contract.json";
const CONFIGURATION_REGISTRY_DATA_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/configuration_registry_data_clientbound_framed_dispatch.answer.jsonl";
const CONFIGURATION_REGISTRY_DATA_CLIENTBOUND_TEST_NAME: &str =
    "configuration_registry_data_clientbound_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_REGISTRY_DATA_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const CONFIGURATION_RESOURCE_PACK_POP_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_resource_pack_pop_clientbound_framed_dispatch.test-manifest.json";
const CONFIGURATION_RESOURCE_PACK_POP_CLIENTBOUND_CASE_ID: &str =
    "configuration_resource_pack_pop_clientbound_framed_dispatch";
const CONFIGURATION_RESOURCE_PACK_POP_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/configuration_resource_pack_pop_clientbound_framed_dispatch.contract.json";
const CONFIGURATION_RESOURCE_PACK_POP_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/configuration_resource_pack_pop_clientbound_framed_dispatch.answer.jsonl";
const CONFIGURATION_RESOURCE_PACK_POP_CLIENTBOUND_TEST_NAME: &str =
    "configuration_resource_pack_pop_clientbound_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_RESOURCE_PACK_POP_CLIENTBOUND_COMPARISON_SURFACE: &str =
    "framed_dispatch_decode";
const CONFIGURATION_RESOURCE_PACK_PUSH_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_resource_pack_push_clientbound_framed_dispatch.test-manifest.json";
const CONFIGURATION_RESOURCE_PACK_PUSH_CLIENTBOUND_CASE_ID: &str =
    "configuration_resource_pack_push_clientbound_framed_dispatch";
const CONFIGURATION_RESOURCE_PACK_PUSH_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/configuration_resource_pack_push_clientbound_framed_dispatch.contract.json";
const CONFIGURATION_RESOURCE_PACK_PUSH_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/configuration_resource_pack_push_clientbound_framed_dispatch.answer.jsonl";
const CONFIGURATION_RESOURCE_PACK_PUSH_CLIENTBOUND_TEST_NAME: &str =
    "configuration_resource_pack_push_clientbound_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_RESOURCE_PACK_PUSH_CLIENTBOUND_COMPARISON_SURFACE: &str =
    "framed_dispatch_decode";
const CONFIGURATION_STORE_COOKIE_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_store_cookie_clientbound_framed_dispatch.test-manifest.json";
const CONFIGURATION_STORE_COOKIE_CLIENTBOUND_CASE_ID: &str =
    "configuration_store_cookie_clientbound_framed_dispatch";
const CONFIGURATION_STORE_COOKIE_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/configuration_store_cookie_clientbound_framed_dispatch.contract.json";
const CONFIGURATION_STORE_COOKIE_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/configuration_store_cookie_clientbound_framed_dispatch.answer.jsonl";
const CONFIGURATION_STORE_COOKIE_CLIENTBOUND_TEST_NAME: &str =
    "configuration_store_cookie_clientbound_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_STORE_COOKIE_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const CONFIGURATION_TRANSFER_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_transfer_clientbound_framed_dispatch.test-manifest.json";
const CONFIGURATION_TRANSFER_CLIENTBOUND_CASE_ID: &str =
    "configuration_transfer_clientbound_framed_dispatch";
const CONFIGURATION_TRANSFER_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/configuration_transfer_clientbound_framed_dispatch.contract.json";
const CONFIGURATION_TRANSFER_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/configuration_transfer_clientbound_framed_dispatch.answer.jsonl";
const CONFIGURATION_TRANSFER_CLIENTBOUND_TEST_NAME: &str =
    "configuration_transfer_clientbound_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_TRANSFER_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const CONFIGURATION_UPDATE_ENABLED_FEATURES_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_update_enabled_features_clientbound_framed_dispatch.test-manifest.json";
const CONFIGURATION_UPDATE_ENABLED_FEATURES_CLIENTBOUND_CASE_ID: &str =
    "configuration_update_enabled_features_clientbound_framed_dispatch";
const CONFIGURATION_UPDATE_ENABLED_FEATURES_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/configuration_update_enabled_features_clientbound_framed_dispatch.contract.json";
const CONFIGURATION_UPDATE_ENABLED_FEATURES_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/configuration_update_enabled_features_clientbound_framed_dispatch.answer.jsonl";
const CONFIGURATION_UPDATE_ENABLED_FEATURES_CLIENTBOUND_TEST_NAME: &str =
    "configuration_update_enabled_features_clientbound_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_UPDATE_ENABLED_FEATURES_CLIENTBOUND_COMPARISON_SURFACE: &str =
    "framed_dispatch_decode";
const CONFIGURATION_UPDATE_TAGS_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_update_tags_clientbound_framed_dispatch.test-manifest.json";
const CONFIGURATION_UPDATE_TAGS_CLIENTBOUND_CASE_ID: &str =
    "configuration_update_tags_clientbound_framed_dispatch";
const CONFIGURATION_UPDATE_TAGS_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/configuration_update_tags_clientbound_framed_dispatch.contract.json";
const CONFIGURATION_UPDATE_TAGS_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/configuration_update_tags_clientbound_framed_dispatch.answer.jsonl";
const CONFIGURATION_UPDATE_TAGS_CLIENTBOUND_TEST_NAME: &str =
    "configuration_update_tags_clientbound_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_UPDATE_TAGS_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const CONFIGURATION_SELECT_KNOWN_PACKS_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_select_known_packs_clientbound_framed_dispatch.test-manifest.json";
const CONFIGURATION_SELECT_KNOWN_PACKS_CLIENTBOUND_CASE_ID: &str =
    "configuration_select_known_packs_clientbound_framed_dispatch";
const CONFIGURATION_SELECT_KNOWN_PACKS_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/configuration_select_known_packs_clientbound_framed_dispatch.contract.json";
const CONFIGURATION_SELECT_KNOWN_PACKS_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/configuration_select_known_packs_clientbound_framed_dispatch.answer.jsonl";
const CONFIGURATION_SELECT_KNOWN_PACKS_CLIENTBOUND_TEST_NAME: &str =
    "configuration_select_known_packs_clientbound_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_SELECT_KNOWN_PACKS_CLIENTBOUND_COMPARISON_SURFACE: &str =
    "framed_dispatch_decode";
const CONFIGURATION_CUSTOM_REPORT_DETAILS_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_custom_report_details_clientbound_framed_dispatch.test-manifest.json";
const CONFIGURATION_CUSTOM_REPORT_DETAILS_CLIENTBOUND_CASE_ID: &str =
    "configuration_custom_report_details_clientbound_framed_dispatch";
const CONFIGURATION_CUSTOM_REPORT_DETAILS_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/configuration_custom_report_details_clientbound_framed_dispatch.contract.json";
const CONFIGURATION_CUSTOM_REPORT_DETAILS_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/configuration_custom_report_details_clientbound_framed_dispatch.answer.jsonl";
const CONFIGURATION_CUSTOM_REPORT_DETAILS_CLIENTBOUND_TEST_NAME: &str =
    "configuration_custom_report_details_clientbound_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_CUSTOM_REPORT_DETAILS_CLIENTBOUND_COMPARISON_SURFACE: &str =
    "framed_dispatch_decode";
const CONFIGURATION_SERVER_LINKS_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_server_links_clientbound_framed_dispatch.test-manifest.json";
const CONFIGURATION_SERVER_LINKS_CLIENTBOUND_CASE_ID: &str =
    "configuration_server_links_clientbound_framed_dispatch";
const CONFIGURATION_SERVER_LINKS_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/configuration_server_links_clientbound_framed_dispatch.contract.json";
const CONFIGURATION_SERVER_LINKS_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/configuration_server_links_clientbound_framed_dispatch.answer.jsonl";
const CONFIGURATION_SERVER_LINKS_CLIENTBOUND_TEST_NAME: &str =
    "configuration_server_links_clientbound_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_SERVER_LINKS_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const CONFIGURATION_CLEAR_DIALOG_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_clear_dialog_clientbound_framed_dispatch.test-manifest.json";
const CONFIGURATION_CLEAR_DIALOG_CLIENTBOUND_CASE_ID: &str =
    "configuration_clear_dialog_clientbound_framed_dispatch";
const CONFIGURATION_CLEAR_DIALOG_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/configuration_clear_dialog_clientbound_framed_dispatch.contract.json";
const CONFIGURATION_CLEAR_DIALOG_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/configuration_clear_dialog_clientbound_framed_dispatch.answer.jsonl";
const CONFIGURATION_CLEAR_DIALOG_CLIENTBOUND_TEST_NAME: &str =
    "configuration_clear_dialog_clientbound_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_CLEAR_DIALOG_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const CONFIGURATION_SHOW_DIALOG_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_show_dialog_clientbound_framed_dispatch.test-manifest.json";
const CONFIGURATION_SHOW_DIALOG_CLIENTBOUND_CASE_ID: &str =
    "configuration_show_dialog_clientbound_framed_dispatch";
const CONFIGURATION_SHOW_DIALOG_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/configuration_show_dialog_clientbound_framed_dispatch.contract.json";
const CONFIGURATION_SHOW_DIALOG_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/configuration_show_dialog_clientbound_framed_dispatch.answer.jsonl";
const CONFIGURATION_SHOW_DIALOG_CLIENTBOUND_TEST_NAME: &str =
    "configuration_show_dialog_clientbound_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_SHOW_DIALOG_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const CONFIGURATION_CODE_OF_CONDUCT_CLIENTBOUND_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_code_of_conduct_clientbound_framed_dispatch.test-manifest.json";
const CONFIGURATION_CODE_OF_CONDUCT_CLIENTBOUND_CASE_ID: &str =
    "configuration_code_of_conduct_clientbound_framed_dispatch";
const CONFIGURATION_CODE_OF_CONDUCT_CLIENTBOUND_CONTRACT: &str =
    "oracle/contracts/775/configuration_code_of_conduct_clientbound_framed_dispatch.contract.json";
const CONFIGURATION_CODE_OF_CONDUCT_CLIENTBOUND_ANSWER: &str =
    "oracle/answers/775/configuration_code_of_conduct_clientbound_framed_dispatch.answer.jsonl";
const CONFIGURATION_CODE_OF_CONDUCT_CLIENTBOUND_TEST_NAME: &str =
    "configuration_code_of_conduct_clientbound_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_CODE_OF_CONDUCT_CLIENTBOUND_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const CONFIGURATION_RESOURCE_PACK_RESPONSE_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_resource_pack_response_framed_dispatch.test-manifest.json";
const CONFIGURATION_RESOURCE_PACK_RESPONSE_CASE_ID: &str =
    "configuration_resource_pack_response_framed_dispatch";
const CONFIGURATION_RESOURCE_PACK_RESPONSE_CONTRACT: &str =
    "oracle/contracts/775/configuration_resource_pack_response_framed_dispatch.contract.json";
const CONFIGURATION_RESOURCE_PACK_RESPONSE_ANSWER: &str =
    "oracle/answers/775/configuration_resource_pack_response_framed_dispatch.answer.jsonl";
const CONFIGURATION_RESOURCE_PACK_RESPONSE_TEST_NAME: &str =
    "configuration_resource_pack_response_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_RESOURCE_PACK_RESPONSE_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const CONFIGURATION_SELECT_KNOWN_PACKS_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_select_known_packs_framed_dispatch.test-manifest.json";
const CONFIGURATION_SELECT_KNOWN_PACKS_CASE_ID: &str =
    "configuration_select_known_packs_framed_dispatch";
const CONFIGURATION_SELECT_KNOWN_PACKS_CONTRACT: &str =
    "oracle/contracts/775/configuration_select_known_packs_framed_dispatch.contract.json";
const CONFIGURATION_SELECT_KNOWN_PACKS_ANSWER: &str =
    "oracle/answers/775/configuration_select_known_packs_framed_dispatch.answer.jsonl";
const CONFIGURATION_SELECT_KNOWN_PACKS_TEST_NAME: &str =
    "configuration_select_known_packs_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_SELECT_KNOWN_PACKS_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const CONFIGURATION_CUSTOM_CLICK_ACTION_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_custom_click_action_framed_dispatch.test-manifest.json";
const CONFIGURATION_CUSTOM_CLICK_ACTION_CASE_ID: &str =
    "configuration_custom_click_action_framed_dispatch";
const CONFIGURATION_CUSTOM_CLICK_ACTION_CONTRACT: &str =
    "oracle/contracts/775/configuration_custom_click_action_framed_dispatch.contract.json";
const CONFIGURATION_CUSTOM_CLICK_ACTION_ANSWER: &str =
    "oracle/answers/775/configuration_custom_click_action_framed_dispatch.answer.jsonl";
const CONFIGURATION_CUSTOM_CLICK_ACTION_TEST_NAME: &str =
    "configuration_custom_click_action_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_CUSTOM_CLICK_ACTION_COMPARISON_SURFACE: &str = "framed_dispatch_decode";
const CONFIGURATION_ACCEPT_CODE_OF_CONDUCT_MANIFEST: &str =
    "oracle/test-manifests/775/configuration_accept_code_of_conduct_framed_dispatch.test-manifest.json";
const CONFIGURATION_ACCEPT_CODE_OF_CONDUCT_CASE_ID: &str =
    "configuration_accept_code_of_conduct_framed_dispatch";
const CONFIGURATION_ACCEPT_CODE_OF_CONDUCT_CONTRACT: &str =
    "oracle/contracts/775/configuration_accept_code_of_conduct_framed_dispatch.contract.json";
const CONFIGURATION_ACCEPT_CODE_OF_CONDUCT_ANSWER: &str =
    "oracle/answers/775/configuration_accept_code_of_conduct_framed_dispatch.answer.jsonl";
const CONFIGURATION_ACCEPT_CODE_OF_CONDUCT_TEST_NAME: &str =
    "configuration_accept_code_of_conduct_framed_dispatch_matches_official_oracle_answer";
const CONFIGURATION_ACCEPT_CODE_OF_CONDUCT_COMPARISON_SURFACE: &str = "framed_dispatch_decode";

#[derive(Debug, Deserialize)]
struct TestManifest {
    case_id: String,
    contract_path: String,
    answer_path: String,
    rust_test_target: String,
    rust_test_name: String,
    comparison_surface: String,
    #[serde(default)]
    response_answer_path: Option<String>,
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
    input_protocol_version: Option<i32>,
    decoded_protocol_version: Option<i32>,
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
    clientbound_ping: Option<FramedDirectionAnswer>,
    serverbound_pong: Option<FramedDirectionAnswer>,
    input_information: Option<ClientInformationRecord>,
    decoded_information: Option<ClientInformationRecord>,
    input_key: Option<String>,
    decoded_key: Option<String>,
    input_payload_hex: Option<String>,
    decoded_payload_hex: Option<String>,
    input_payload_length: Option<usize>,
    decoded_payload_length: Option<usize>,
    decoded_payload_equals_input: Option<bool>,
    input_custom_payload_id: Option<String>,
    decoded_custom_payload_id: Option<String>,
    input_payload_class: Option<String>,
    decoded_payload_class: Option<String>,
    input_brand: Option<String>,
    decoded_brand: Option<String>,
    encoded_payload_body_hex: Option<String>,
    reason_fixture: Option<String>,
    input_reason_text: Option<String>,
    decoded_reason_text: Option<String>,
    input_uuid: Option<String>,
    body_uuid: Option<String>,
    stream_decoded_uuid: Option<String>,
    decoded_uuid: Option<String>,
    input_uuid_present: Option<bool>,
    decoded_uuid_present: Option<bool>,
    input_operation: Option<String>,
    stream_decoded_operation: Option<String>,
    decoded_operation: Option<String>,
    decoded_operation_ordinal: Option<i32>,
    remaining_after_operation_read: Option<i32>,
    input_url: Option<String>,
    decoded_url: Option<String>,
    input_hash: Option<String>,
    decoded_hash: Option<String>,
    input_host: Option<String>,
    decoded_host: Option<String>,
    input_port: Option<i32>,
    decoded_port: Option<i32>,
    input_name: Option<String>,
    decoded_name: Option<String>,
    input_profile_id: Option<String>,
    decoded_profile_id: Option<String>,
    input_profile_name: Option<String>,
    decoded_profile_name: Option<String>,
    input_property_count: Option<usize>,
    decoded_property_count: Option<usize>,
    input_server_id: Option<String>,
    decoded_server_id: Option<String>,
    input_public_key_hex: Option<String>,
    decoded_public_key_hex: Option<String>,
    input_public_key_length: Option<usize>,
    decoded_public_key_length: Option<usize>,
    input_challenge_hex: Option<String>,
    decoded_challenge_hex: Option<String>,
    input_challenge_length: Option<usize>,
    decoded_challenge_length: Option<usize>,
    input_should_authenticate: Option<bool>,
    decoded_should_authenticate: Option<bool>,
    input_compression_threshold: Option<i32>,
    decoded_compression_threshold: Option<i32>,
    input_transaction_id: Option<i32>,
    decoded_transaction_id: Option<i32>,
    input_payload_id: Option<String>,
    decoded_payload_id: Option<String>,
    input_keybytes_hex: Option<String>,
    decoded_keybytes_hex: Option<String>,
    input_keybytes_length: Option<usize>,
    decoded_keybytes_length: Option<usize>,
    input_encrypted_challenge_hex: Option<String>,
    decoded_encrypted_challenge_hex: Option<String>,
    input_encrypted_challenge_length: Option<usize>,
    decoded_encrypted_challenge_length: Option<usize>,
    input_intent: Option<String>,
    decoded_intent: Option<String>,
    input_intent_id: Option<i32>,
    decoded_intent_id: Option<i32>,
    input_is_terminal: Option<bool>,
    decoded_is_terminal: Option<bool>,
    input_fixture: Option<String>,
    fixture: Option<String>,
    official_body_shape: Option<String>,
    input_entity_id: Option<i32>,
    decoded_entity_id: Option<i32>,
    input_stat_count: Option<usize>,
    stream_decoded_stat_count: Option<usize>,
    decoded_stat_count: Option<usize>,
    input_sequence: Option<i32>,
    stream_decoded_sequence: Option<i32>,
    decoded_sequence: Option<i32>,
    input_breaker_id: Option<i32>,
    stream_decoded_breaker_id: Option<i32>,
    decoded_breaker_id: Option<i32>,
    input_block_x: Option<i32>,
    input_block_y: Option<i32>,
    input_block_z: Option<i32>,
    stream_decoded_block_x: Option<i32>,
    stream_decoded_block_y: Option<i32>,
    stream_decoded_block_z: Option<i32>,
    decoded_block_x: Option<i32>,
    decoded_block_y: Option<i32>,
    decoded_block_z: Option<i32>,
    input_progress: Option<i32>,
    stream_decoded_progress: Option<i32>,
    decoded_progress: Option<i32>,
    input_block_entity_type: Option<String>,
    stream_decoded_block_entity_type: Option<String>,
    decoded_block_entity_type: Option<String>,
    decoded_block_entity_type_registry_id: Option<i32>,
    input_event_type: Option<i32>,
    stream_decoded_event_type: Option<i32>,
    decoded_event_type: Option<i32>,
    input_event_data: Option<i32>,
    stream_decoded_event_data: Option<i32>,
    decoded_event_data: Option<i32>,
    input_block: Option<String>,
    stream_decoded_block: Option<String>,
    decoded_block: Option<String>,
    decoded_block_registry_id: Option<i32>,
    stream_decoded_block_state_block: Option<String>,
    decoded_block_state_block: Option<String>,
    decoded_block_state_registry_id: Option<i32>,
    input_difficulty: Option<String>,
    input_difficulty_serialized_name: Option<String>,
    input_difficulty_id: Option<i32>,
    stream_decoded_difficulty: Option<String>,
    stream_decoded_difficulty_serialized_name: Option<String>,
    stream_decoded_difficulty_id: Option<i32>,
    decoded_difficulty: Option<String>,
    decoded_difficulty_serialized_name: Option<String>,
    decoded_difficulty_id: Option<i32>,
    input_locked: Option<bool>,
    stream_decoded_locked: Option<bool>,
    decoded_locked: Option<bool>,
    input_batch_size: Option<i32>,
    stream_decoded_batch_size: Option<i32>,
    decoded_batch_size: Option<i32>,
    stream_decoded_same_instance: Option<bool>,
    decoded_same_instance: Option<bool>,
    input_chunk_biome_count: Option<usize>,
    stream_decoded_chunk_biome_count: Option<usize>,
    decoded_chunk_biome_count: Option<usize>,
    input_reset_times: Option<bool>,
    stream_decoded_reset_times: Option<bool>,
    decoded_reset_times: Option<bool>,
    input_command_id: Option<i32>,
    stream_decoded_command_id: Option<i32>,
    decoded_command_id: Option<i32>,
    input_range_start: Option<i32>,
    stream_decoded_range_start: Option<i32>,
    decoded_range_start: Option<i32>,
    input_range_length: Option<i32>,
    stream_decoded_range_length: Option<i32>,
    decoded_range_length: Option<i32>,
    input_suggestion_count: Option<usize>,
    stream_decoded_suggestion_count: Option<usize>,
    decoded_suggestion_count: Option<usize>,
    decoded_to_suggestions_range_start: Option<i32>,
    decoded_to_suggestions_range_length: Option<i32>,
    decoded_to_suggestions_count: Option<usize>,
    input_root_child_count: Option<usize>,
    stream_decoded_entry_count: Option<usize>,
    stream_decoded_root_index: Option<i32>,
    decoded_root_index: Option<i32>,
    input_container_id: Option<i32>,
    stream_decoded_container_id: Option<i32>,
    decoded_container_id: Option<i32>,
    input_tag_size: Option<usize>,
    stream_decoded_tag_size: Option<usize>,
    decoded_tag_size: Option<usize>,
    input_tag_snbt: Option<String>,
    stream_decoded_tag_snbt: Option<String>,
    decoded_tag_snbt: Option<String>,
    input_entity_type: Option<String>,
    decoded_entity_type: Option<String>,
    decoded_entity_type_registry_id: Option<i32>,
    input_x: Option<f64>,
    decoded_x: Option<f64>,
    input_y: Option<f64>,
    decoded_y: Option<f64>,
    input_z: Option<f64>,
    decoded_z: Option<f64>,
    input_movement_x: Option<f64>,
    decoded_movement_x: Option<f64>,
    input_movement_y: Option<f64>,
    decoded_movement_y: Option<f64>,
    input_movement_z: Option<f64>,
    decoded_movement_z: Option<f64>,
    encoded_movement_lp_hex: Option<String>,
    input_x_rot_degrees: Option<f64>,
    decoded_x_rot_degrees: Option<f64>,
    decoded_x_rot_byte: Option<i8>,
    input_y_rot_degrees: Option<f64>,
    decoded_y_rot_degrees: Option<f64>,
    decoded_y_rot_byte: Option<i8>,
    input_y_head_rot_degrees: Option<f64>,
    decoded_y_head_rot_degrees: Option<f64>,
    decoded_y_head_rot_byte: Option<i8>,
    input_data: Option<i32>,
    decoded_data: Option<i32>,
    input_animation_action_name: Option<String>,
    input_animation_action: Option<i32>,
    decoded_animation_action: Option<i32>,
    remaining_after_packet_stream_decode: Option<i32>,
    fixture_body_hex: Option<String>,
    input_feature_count: Option<usize>,
    decoded_feature_count: Option<usize>,
    input_features: Option<Vec<String>>,
    decoded_features: Option<Vec<String>>,
    input_tag_registry_count: Option<usize>,
    decoded_tag_registry_count: Option<usize>,
    input_required: Option<bool>,
    decoded_required: Option<bool>,
    input_prompt_present: Option<bool>,
    decoded_prompt_present: Option<bool>,
    input_action: Option<String>,
    decoded_action: Option<String>,
    input_action_is_terminal: Option<bool>,
    decoded_action_is_terminal: Option<bool>,
    input_known_pack_count: Option<usize>,
    decoded_known_pack_count: Option<usize>,
    input_known_packs: Option<Vec<KnownPackRecord>>,
    decoded_known_packs: Option<Vec<KnownPackRecord>>,
    input_detail_count: Option<usize>,
    decoded_detail_count: Option<usize>,
    input_details: Option<std::collections::BTreeMap<String, String>>,
    decoded_details: Option<std::collections::BTreeMap<String, String>>,
    input_link_count: Option<usize>,
    decoded_link_count: Option<usize>,
    input_links: Option<Vec<serde_json::Value>>,
    decoded_links: Option<Vec<serde_json::Value>>,
    input_dialog_class: Option<String>,
    decoded_dialog_class: Option<String>,
    input_dialog_title: Option<String>,
    decoded_dialog_title: Option<String>,
    input_dialog_body_count: Option<usize>,
    decoded_dialog_body_count: Option<usize>,
    input_dialog_input_count: Option<usize>,
    decoded_dialog_input_count: Option<usize>,
    input_can_close_with_escape: Option<bool>,
    decoded_can_close_with_escape: Option<bool>,
    input_pause: Option<bool>,
    decoded_pause: Option<bool>,
    input_after_action: Option<String>,
    decoded_after_action: Option<String>,
    input_code_of_conduct: Option<String>,
    decoded_code_of_conduct: Option<String>,
    input_custom_click_id: Option<String>,
    decoded_custom_click_id: Option<String>,
    input_payload_present: Option<bool>,
    decoded_payload_present: Option<bool>,
    input_payload_tag_id: Option<i32>,
    decoded_payload_tag_id: Option<i32>,
    input_payload_type: Option<String>,
    decoded_payload_type: Option<String>,
    input_payload_snbt: Option<String>,
    decoded_payload_snbt: Option<String>,
    instance_packet_type: Option<String>,
    decoded_equals_instance: Option<bool>,
    registry_fixture: Option<String>,
    input_registry_key: Option<String>,
    decoded_registry_key: Option<String>,
    input_entry_count: Option<usize>,
    decoded_entry_count: Option<usize>,
    #[serde(default)]
    handshaking_serverbound_packet_table: Vec<PacketTableRow>,
    #[serde(default)]
    login_serverbound_packet_table: Vec<PacketTableRow>,
    #[serde(default)]
    login_clientbound_packet_table: Vec<PacketTableRow>,
    #[serde(default)]
    configuration_serverbound_packet_table: Vec<PacketTableRow>,
    #[serde(default)]
    configuration_clientbound_packet_table: Vec<PacketTableRow>,
    #[serde(default)]
    play_clientbound_packet_table: Vec<PacketTableRow>,
    #[serde(default)]
    resource_pack_action_table: Vec<ResourcePackActionRow>,
}

#[derive(Debug, Deserialize)]
struct PacketTableRow {
    packet_id: i32,
    packet_type: String,
}

#[derive(Debug, Deserialize)]
struct ResourcePackActionRow {
    name: String,
    is_terminal: bool,
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

#[derive(Debug, Deserialize)]
struct FramedDirectionAnswer {
    flow: String,
    packet_type: String,
    decoded_packet_type: String,
    decoded_packet_class: String,
    input_id: i32,
    encoded_framed_hex: String,
    encoded_body_hex: String,
    decoded_id: i32,
    remaining_after_official_decode: i32,
    configuration_packet_table: Vec<PacketTableRow>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct ClientInformationRecord {
    language: String,
    view_distance: i32,
    chat_visibility: String,
    chat_colors: bool,
    model_customisation: i32,
    main_hand: String,
    text_filtering_enabled: bool,
    allows_listing: bool,
    particle_status: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct KnownPackRecord {
    namespace: String,
    id: String,
    version: String,
    is_vanilla: bool,
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

#[test]
fn handshake_intention_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("handshake_intention_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(handshake_intention_framed_dispatch_body)
        .expect("spawn handshake intention oracle stack")
        .join()
        .expect("handshake intention oracle thread panicked");
}

fn handshake_intention_framed_dispatch_body() {
    let manifest: TestManifest = read_json(HANDSHAKE_INTENTION_MANIFEST);
    assert_eq!(manifest.case_id, HANDSHAKE_INTENTION_CASE_ID);
    assert_eq!(manifest.contract_path, HANDSHAKE_INTENTION_CONTRACT);
    assert_eq!(manifest.answer_path, HANDSHAKE_INTENTION_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(manifest.rust_test_name, HANDSHAKE_INTENTION_TEST_NAME);
    assert_eq!(
        manifest.comparison_surface,
        HANDSHAKE_INTENTION_COMPARISON_SURFACE
    );
    assert_runner_scope(HANDSHAKE_INTENTION_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:intention")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.handshake.ClientIntentionPacket")
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));
    assert_eq!(oracle.answer.input_protocol_version, Some(775));
    assert_eq!(oracle.answer.decoded_protocol_version, Some(775));
    assert_eq!(oracle.answer.input_host.as_deref(), Some("localhost"));
    assert_eq!(oracle.answer.decoded_host.as_deref(), Some("localhost"));
    assert_eq!(oracle.answer.input_port, Some(25565));
    assert_eq!(oracle.answer.decoded_port, Some(25565));
    assert_eq!(oracle.answer.input_intent.as_deref(), Some("LOGIN"));
    assert_eq!(oracle.answer.decoded_intent.as_deref(), Some("LOGIN"));
    assert_eq!(oracle.answer.input_intent_id, Some(2));
    assert_eq!(oracle.answer.decoded_intent_id, Some(2));
    assert_eq!(oracle.answer.input_is_terminal, Some(true));
    assert_eq!(oracle.answer.decoded_is_terminal, Some(true));

    let expected_packet_id = packet_id_for(
        &oracle.answer.handshaking_serverbound_packet_table,
        "minecraft:intention",
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
        State::Handshaking,
        Direction::Serverbound,
        framed_packet_id,
        &mut body_slice,
    )
    .unwrap()
    .expect("expected Handshaking serverbound intention to dispatch");

    match decoded {
        packet::Packet::Handshake(packet) => {
            assert_eq!(
                packet.protocol_version.0,
                oracle.answer.decoded_protocol_version.unwrap()
            );
            assert_eq!(packet.host, oracle.answer.decoded_host.as_deref().unwrap());
            assert_eq!(i32::from(packet.port), oracle.answer.decoded_port.unwrap());
            assert_eq!(packet.next.0, oracle.answer.decoded_intent_id.unwrap());
        }
        other => panic!("expected intention dispatch, got {other:?}"),
    }
    assert!(
        body_slice.is_empty(),
        "decoded packet did not consume the official body bytes"
    );
}

#[test]
fn login_hello_serverbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("login_hello_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(login_hello_serverbound_framed_dispatch_body)
        .expect("spawn login hello oracle stack")
        .join()
        .expect("login hello oracle thread panicked");
}

fn login_hello_serverbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(LOGIN_HELLO_SERVERBOUND_MANIFEST);
    assert_eq!(manifest.case_id, LOGIN_HELLO_SERVERBOUND_CASE_ID);
    assert_eq!(manifest.contract_path, LOGIN_HELLO_SERVERBOUND_CONTRACT);
    assert_eq!(manifest.answer_path, LOGIN_HELLO_SERVERBOUND_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(manifest.rust_test_name, LOGIN_HELLO_SERVERBOUND_TEST_NAME);
    assert_eq!(
        manifest.comparison_surface,
        LOGIN_HELLO_SERVERBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(LOGIN_HELLO_SERVERBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:hello")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:hello")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.login.ServerboundHelloPacket")
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));
    assert_eq!(oracle.answer.input_name, oracle.answer.decoded_name);
    assert_eq!(
        oracle.answer.input_profile_id,
        oracle.answer.decoded_profile_id
    );

    let expected_packet_id = packet_id_for(
        &oracle.answer.login_serverbound_packet_table,
        "minecraft:hello",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("login hello answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Login,
            Direction::Serverbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Login serverbound hello packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding login hello packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Login serverbound hello packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::LoginStart(packet) => {
            assert_eq!(
                packet.username,
                oracle.answer.decoded_name.as_deref().unwrap()
            );
        }
        other => panic!("expected Login serverbound hello dispatch, got {other:?}"),
    }
    assert!(
        body_slice.is_empty(),
        "decoded login hello packet did not consume the official body bytes"
    );
}

#[test]
fn login_key_serverbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("login_key_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(login_key_serverbound_framed_dispatch_body)
        .expect("spawn login key oracle stack")
        .join()
        .expect("login key oracle thread panicked");
}

fn login_key_serverbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(LOGIN_KEY_SERVERBOUND_MANIFEST);
    assert_eq!(manifest.case_id, LOGIN_KEY_SERVERBOUND_CASE_ID);
    assert_eq!(manifest.contract_path, LOGIN_KEY_SERVERBOUND_CONTRACT);
    assert_eq!(manifest.answer_path, LOGIN_KEY_SERVERBOUND_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(manifest.rust_test_name, LOGIN_KEY_SERVERBOUND_TEST_NAME);
    assert_eq!(
        manifest.comparison_surface,
        LOGIN_KEY_SERVERBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(LOGIN_KEY_SERVERBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(oracle.answer.packet_type.as_deref(), Some("minecraft:key"));
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:key")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.login.ServerboundKeyPacket")
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));
    assert_eq!(
        oracle.answer.input_keybytes_hex,
        oracle.answer.decoded_keybytes_hex
    );
    assert_eq!(
        oracle.answer.input_keybytes_length,
        oracle.answer.decoded_keybytes_length
    );
    assert_eq!(
        oracle.answer.input_encrypted_challenge_hex,
        oracle.answer.decoded_encrypted_challenge_hex
    );
    assert_eq!(
        oracle.answer.input_encrypted_challenge_length,
        oracle.answer.decoded_encrypted_challenge_length
    );

    let expected_packet_id = packet_id_for(
        &oracle.answer.login_serverbound_packet_table,
        "minecraft:key",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("login key answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Login,
            Direction::Serverbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Login serverbound key packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| panic!("Stevenarella errored while decoding login key packet: {err}"))
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Login serverbound key packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::EncryptionResponse(packet) => {
            assert_eq!(
                hex::encode(packet.shared_secret.data),
                oracle.answer.decoded_keybytes_hex.as_deref().unwrap()
            );
            assert_eq!(
                hex::encode(packet.verify_token.data),
                oracle
                    .answer
                    .decoded_encrypted_challenge_hex
                    .as_deref()
                    .unwrap()
            );
        }
        other => panic!("expected Login serverbound key dispatch, got {other:?}"),
    }
    assert!(
        body_slice.is_empty(),
        "decoded login key packet did not consume the official body bytes"
    );
}

#[test]
fn login_custom_query_answer_serverbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("login_custom_query_answer_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(login_custom_query_answer_serverbound_framed_dispatch_body)
        .expect("spawn login custom_query_answer oracle stack")
        .join()
        .expect("login custom_query_answer oracle thread panicked");
}

fn login_custom_query_answer_serverbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(LOGIN_CUSTOM_QUERY_ANSWER_SERVERBOUND_MANIFEST);
    assert_eq!(
        manifest.case_id,
        LOGIN_CUSTOM_QUERY_ANSWER_SERVERBOUND_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        LOGIN_CUSTOM_QUERY_ANSWER_SERVERBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        LOGIN_CUSTOM_QUERY_ANSWER_SERVERBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        LOGIN_CUSTOM_QUERY_ANSWER_SERVERBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        LOGIN_CUSTOM_QUERY_ANSWER_SERVERBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(LOGIN_CUSTOM_QUERY_ANSWER_SERVERBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:custom_query_answer")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:custom_query_answer")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.login.ServerboundCustomQueryAnswerPacket")
    );
    assert_eq!(
        oracle.answer.decoded_payload_class.as_deref(),
        Some("net.minecraft.network.protocol.login.custom.DiscardedQueryAnswerPayload")
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));
    assert_eq!(
        oracle.answer.input_transaction_id,
        oracle.answer.decoded_transaction_id
    );
    assert_eq!(oracle.answer.input_payload_present, Some(false));
    assert_eq!(oracle.answer.decoded_payload_present, Some(true));

    let expected_packet_id = packet_id_for(
        &oracle.answer.login_serverbound_packet_table,
        "minecraft:custom_query_answer",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("login custom_query_answer answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Login,
            Direction::Serverbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Login serverbound custom_query_answer packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding login custom_query_answer packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Login serverbound custom_query_answer packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::LoginPluginResponse(packet) => {
            assert_eq!(
                packet.message_id.0,
                oracle.answer.decoded_transaction_id.unwrap()
            );
            assert_eq!(
                packet.successful,
                oracle.answer.input_payload_present.unwrap()
            );
            assert!(
                packet.data.is_empty(),
                "decoded login custom_query_answer compatibility packet carried unexpected data"
            );
        }
        other => panic!("expected Login serverbound custom_query_answer dispatch, got {other:?}"),
    }
    assert!(
        body_slice.is_empty(),
        "decoded login custom_query_answer packet did not consume the official body bytes"
    );
}

#[test]
fn login_acknowledged_serverbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("login_acknowledged_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(login_acknowledged_serverbound_framed_dispatch_body)
        .expect("spawn login_acknowledged oracle stack")
        .join()
        .expect("login_acknowledged oracle thread panicked");
}

fn login_acknowledged_serverbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(LOGIN_ACKNOWLEDGED_SERVERBOUND_MANIFEST);
    assert_eq!(manifest.case_id, LOGIN_ACKNOWLEDGED_SERVERBOUND_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        LOGIN_ACKNOWLEDGED_SERVERBOUND_CONTRACT
    );
    assert_eq!(manifest.answer_path, LOGIN_ACKNOWLEDGED_SERVERBOUND_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        LOGIN_ACKNOWLEDGED_SERVERBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        LOGIN_ACKNOWLEDGED_SERVERBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(LOGIN_ACKNOWLEDGED_SERVERBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:login_acknowledged")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:login_acknowledged")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.login.ServerboundLoginAcknowledgedPacket")
    );
    assert_eq!(
        oracle.answer.instance_packet_type.as_deref(),
        Some("minecraft:login_acknowledged")
    );
    assert_eq!(oracle.answer.decoded_equals_instance, Some(true));
    assert_eq!(oracle.answer.input_is_terminal, Some(true));
    assert_eq!(oracle.answer.decoded_is_terminal, Some(true));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.login_serverbound_packet_table,
        "minecraft:login_acknowledged",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("login_acknowledged answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Login,
            Direction::Serverbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Login serverbound login_acknowledged packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding login_acknowledged packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Login serverbound login_acknowledged packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::LoginAcknowledged(packet) => {
            assert_eq!(packet.empty, ());
        }
        other => panic!("expected Login serverbound login_acknowledged dispatch, got {other:?}"),
    }
    assert!(
        body_slice.is_empty(),
        "decoded login_acknowledged packet did not consume the official body bytes"
    );
}

#[test]
fn login_cookie_response_serverbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("login_cookie_response_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(login_cookie_response_serverbound_framed_dispatch_body)
        .expect("spawn login cookie_response oracle stack")
        .join()
        .expect("login cookie_response oracle thread panicked");
}

fn login_cookie_response_serverbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(LOGIN_COOKIE_RESPONSE_SERVERBOUND_MANIFEST);
    assert_eq!(manifest.case_id, LOGIN_COOKIE_RESPONSE_SERVERBOUND_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        LOGIN_COOKIE_RESPONSE_SERVERBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        LOGIN_COOKIE_RESPONSE_SERVERBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        LOGIN_COOKIE_RESPONSE_SERVERBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        LOGIN_COOKIE_RESPONSE_SERVERBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(LOGIN_COOKIE_RESPONSE_SERVERBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:cookie_response")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:cookie_response")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.cookie.ServerboundCookieResponsePacket")
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));
    assert_eq!(
        oracle.answer.input_key, oracle.answer.decoded_key,
        "official decoded login cookie_response key differs from the official input key"
    );
    assert_eq!(
        oracle.answer.input_payload_hex, oracle.answer.decoded_payload_hex,
        "official decoded login cookie_response payload differs from the official input payload"
    );
    assert_eq!(
        oracle.answer.input_payload_length, oracle.answer.decoded_payload_length,
        "official decoded login cookie_response payload length differs from the official input payload"
    );
    assert_eq!(
        oracle.answer.decoded_payload_equals_input,
        Some(true),
        "official decoded login cookie_response payload was not byte-equal to the input payload"
    );

    let expected_packet_id = packet_id_for(
        &oracle.answer.login_serverbound_packet_table,
        "minecraft:cookie_response",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("login cookie_response answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Login,
            Direction::Serverbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Login serverbound cookie_response packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding login cookie_response packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Login serverbound cookie_response packet id {}",
                framed_packet_id
            )
        });
    let expected_payload_hex = oracle
        .answer
        .decoded_payload_hex
        .as_deref()
        .expect("login cookie_response answer missing decoded_payload_hex");
    let expected_payload = decode_hex(expected_payload_hex, "decoded_payload_hex");
    match decoded {
        packet::Packet::LoginCookieResponse(packet) => {
            assert_eq!(
                packet.key,
                oracle.answer.decoded_key.clone().unwrap_or_default(),
                "decoded login cookie_response packet did not preserve key"
            );
            assert!(
                packet.has_payload,
                "decoded login cookie_response packet did not preserve the non-null payload marker"
            );
            assert_eq!(
                packet.payload.data, expected_payload,
                "decoded login cookie_response packet did not preserve payload bytes"
            );
        }
        other => panic!("expected Login serverbound cookie_response dispatch, got {other:?}"),
    }
    assert!(
        body_slice.is_empty(),
        "decoded login cookie_response packet did not consume the official body bytes"
    );
}

#[test]
fn login_disconnect_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("login_disconnect_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(login_disconnect_clientbound_framed_dispatch_body)
        .expect("spawn login_disconnect oracle stack")
        .join()
        .expect("login_disconnect oracle thread panicked");
}

fn login_disconnect_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(LOGIN_DISCONNECT_CLIENTBOUND_MANIFEST);
    assert_eq!(manifest.case_id, LOGIN_DISCONNECT_CLIENTBOUND_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        LOGIN_DISCONNECT_CLIENTBOUND_CONTRACT
    );
    assert_eq!(manifest.answer_path, LOGIN_DISCONNECT_CLIENTBOUND_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        LOGIN_DISCONNECT_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        LOGIN_DISCONNECT_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(LOGIN_DISCONNECT_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:login_disconnect")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:login_disconnect")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.login.ClientboundLoginDisconnectPacket")
    );
    assert_eq!(
        oracle.answer.reason_fixture.as_deref(),
        Some("Component.literal(\"\")")
    );
    assert_eq!(
        oracle.answer.input_reason_text, oracle.answer.decoded_reason_text,
        "official decoded login_disconnect reason text differs from the official input reason text"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.login_clientbound_packet_table,
        "minecraft:login_disconnect",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("login_disconnect answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Login,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Login clientbound login_disconnect packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding Login clientbound login_disconnect packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Login clientbound login_disconnect packet id {}",
                framed_packet_id
            )
        });
    let expected_reason_text = oracle
        .answer
        .decoded_reason_text
        .as_deref()
        .expect("login_disconnect answer missing decoded_reason_text");
    match decoded {
        packet::Packet::LoginDisconnect(packet) => {
            assert_eq!(
                packet.reason.to_string(),
                expected_reason_text,
                "decoded Login clientbound login_disconnect reason text did not match the official reason text"
            );
        }
        other => {
            panic!("decoded packet did not preserve Login clientbound login_disconnect identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Login clientbound login_disconnect packet did not consume the official body bytes"
    );
}

#[test]
fn login_hello_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("login_hello_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(login_hello_clientbound_framed_dispatch_body)
        .expect("spawn login_hello_clientbound oracle stack")
        .join()
        .expect("login_hello_clientbound oracle thread panicked");
}

fn login_hello_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(LOGIN_HELLO_CLIENTBOUND_MANIFEST);
    assert_eq!(manifest.case_id, LOGIN_HELLO_CLIENTBOUND_CASE_ID);
    assert_eq!(manifest.contract_path, LOGIN_HELLO_CLIENTBOUND_CONTRACT);
    assert_eq!(manifest.answer_path, LOGIN_HELLO_CLIENTBOUND_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(manifest.rust_test_name, LOGIN_HELLO_CLIENTBOUND_TEST_NAME);
    assert_eq!(
        manifest.comparison_surface,
        LOGIN_HELLO_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(LOGIN_HELLO_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:hello")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:hello")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.login.ClientboundHelloPacket")
    );
    assert_eq!(
        oracle.answer.input_server_id, oracle.answer.decoded_server_id,
        "official decoded Login clientbound hello serverId differs from input"
    );
    assert_eq!(
        oracle.answer.input_public_key_hex, oracle.answer.decoded_public_key_hex,
        "official decoded Login clientbound hello publicKey differs from input"
    );
    assert_eq!(
        oracle.answer.input_public_key_length, oracle.answer.decoded_public_key_length,
        "official decoded Login clientbound hello publicKey length differs from input"
    );
    assert_eq!(
        oracle.answer.input_challenge_hex, oracle.answer.decoded_challenge_hex,
        "official decoded Login clientbound hello challenge differs from input"
    );
    assert_eq!(
        oracle.answer.input_challenge_length, oracle.answer.decoded_challenge_length,
        "official decoded Login clientbound hello challenge length differs from input"
    );
    assert_eq!(
        oracle.answer.input_should_authenticate, oracle.answer.decoded_should_authenticate,
        "official decoded Login clientbound hello shouldAuthenticate differs from input"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.login_clientbound_packet_table,
        "minecraft:hello",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("login_hello clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Login,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Login clientbound hello packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding Login clientbound hello packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Login clientbound hello packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::EncryptionRequest_ShouldAuthenticate(packet) => {
            assert_eq!(
                packet.server_id,
                oracle.answer.decoded_server_id.clone().unwrap_or_default(),
                "decoded Login clientbound hello server_id did not match the official serverId"
            );
            assert_eq!(
                hex::encode(packet.public_key.data),
                oracle
                    .answer
                    .decoded_public_key_hex
                    .clone()
                    .unwrap_or_default(),
                "decoded Login clientbound hello public_key did not match the official publicKey"
            );
            assert_eq!(
                hex::encode(packet.verify_token.data),
                oracle
                    .answer
                    .decoded_challenge_hex
                    .clone()
                    .unwrap_or_default(),
                "decoded Login clientbound hello verify_token did not match the official challenge"
            );
            assert_eq!(
                packet.should_authenticate,
                oracle.answer.decoded_should_authenticate.unwrap_or(false),
                "decoded Login clientbound hello should_authenticate did not match the official shouldAuthenticate"
            );
        }
        other => {
            panic!("decoded packet did not preserve Login clientbound hello identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Login clientbound hello packet did not consume the official body bytes"
    );
}

#[test]
fn login_finished_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("login_finished_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(login_finished_clientbound_framed_dispatch_body)
        .expect("spawn login_finished_clientbound oracle stack")
        .join()
        .expect("login_finished_clientbound oracle thread panicked");
}

fn login_finished_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(LOGIN_FINISHED_CLIENTBOUND_MANIFEST);
    assert_eq!(manifest.case_id, LOGIN_FINISHED_CLIENTBOUND_CASE_ID);
    assert_eq!(manifest.contract_path, LOGIN_FINISHED_CLIENTBOUND_CONTRACT);
    assert_eq!(manifest.answer_path, LOGIN_FINISHED_CLIENTBOUND_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        LOGIN_FINISHED_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        LOGIN_FINISHED_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(LOGIN_FINISHED_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:login_finished")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:login_finished")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.login.ClientboundLoginFinishedPacket")
    );
    assert_eq!(
        oracle.answer.input_profile_id, oracle.answer.decoded_profile_id,
        "official decoded Login clientbound login_finished profile id differs from input"
    );
    assert_eq!(
        oracle.answer.input_profile_name, oracle.answer.decoded_profile_name,
        "official decoded Login clientbound login_finished profile name differs from input"
    );
    assert_eq!(
        oracle.answer.input_property_count, oracle.answer.decoded_property_count,
        "official decoded Login clientbound login_finished property count differs from input"
    );
    assert_eq!(oracle.answer.input_property_count, Some(0));
    assert_eq!(oracle.answer.input_is_terminal, Some(true));
    assert_eq!(
        oracle.answer.input_is_terminal, oracle.answer.decoded_is_terminal,
        "official decoded Login clientbound login_finished terminal flag differs from input"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.login_clientbound_packet_table,
        "minecraft:login_finished",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("login_finished clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Login,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Login clientbound login_finished packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding Login clientbound login_finished packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Login clientbound login_finished packet id {}",
                framed_packet_id
            )
        });

    let expected_uuid: steven_protocol::protocol::UUID = oracle
        .answer
        .decoded_profile_id
        .as_deref()
        .expect("login_finished answer missing decoded_profile_id")
        .parse()
        .expect("official login_finished decoded_profile_id is not a Stevenarella UUID");
    let expected_name = oracle
        .answer
        .decoded_profile_name
        .as_deref()
        .expect("login_finished answer missing decoded_profile_name");

    match decoded {
        packet::Packet::LoginSuccess_UUID(packet) => {
            assert_eq!(
                packet.uuid, expected_uuid,
                "decoded Login clientbound login_finished uuid did not match the official GameProfile id"
            );
            assert_eq!(
                packet.username, expected_name,
                "decoded Login clientbound login_finished username did not match the official GameProfile name"
            );
        }
        other => {
            panic!("decoded packet did not preserve Login clientbound login_finished identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Login clientbound login_finished packet did not consume the official body bytes"
    );
}

#[test]
fn login_compression_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("login_compression_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(login_compression_clientbound_framed_dispatch_body)
        .expect("spawn login_compression_clientbound oracle stack")
        .join()
        .expect("login_compression_clientbound oracle thread panicked");
}

fn login_compression_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(LOGIN_COMPRESSION_CLIENTBOUND_MANIFEST);
    assert_eq!(manifest.case_id, LOGIN_COMPRESSION_CLIENTBOUND_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        LOGIN_COMPRESSION_CLIENTBOUND_CONTRACT
    );
    assert_eq!(manifest.answer_path, LOGIN_COMPRESSION_CLIENTBOUND_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        LOGIN_COMPRESSION_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        LOGIN_COMPRESSION_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(LOGIN_COMPRESSION_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:login_compression")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:login_compression")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.login.ClientboundLoginCompressionPacket")
    );
    assert_eq!(
        oracle.answer.input_compression_threshold, oracle.answer.decoded_compression_threshold,
        "official decoded Login clientbound login_compression threshold differs from input"
    );
    assert_eq!(
        oracle.answer.input_compression_threshold,
        Some(0),
        "fixture changed: expected smallest accepted compression threshold"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.login_clientbound_packet_table,
        "minecraft:login_compression",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("login_compression clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Login,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Login clientbound login_compression packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding Login clientbound login_compression packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Login clientbound login_compression packet id {}",
                framed_packet_id
            )
        });

    let expected_threshold = oracle
        .answer
        .decoded_compression_threshold
        .expect("login_compression answer missing decoded_compression_threshold");

    match decoded {
        packet::Packet::SetInitialCompression(packet) => {
            assert_eq!(
                packet.threshold.0, expected_threshold,
                "decoded Login clientbound login_compression threshold did not match official compressionThreshold"
            );
        }
        other => {
            panic!("decoded packet did not preserve Login clientbound login_compression identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Login clientbound login_compression packet did not consume the official body bytes"
    );
}

#[test]
fn login_custom_query_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("login_custom_query_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(login_custom_query_clientbound_framed_dispatch_body)
        .expect("spawn login_custom_query_clientbound oracle stack")
        .join()
        .expect("login_custom_query_clientbound oracle thread panicked");
}

fn login_custom_query_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(LOGIN_CUSTOM_QUERY_CLIENTBOUND_MANIFEST);
    assert_eq!(manifest.case_id, LOGIN_CUSTOM_QUERY_CLIENTBOUND_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        LOGIN_CUSTOM_QUERY_CLIENTBOUND_CONTRACT
    );
    assert_eq!(manifest.answer_path, LOGIN_CUSTOM_QUERY_CLIENTBOUND_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        LOGIN_CUSTOM_QUERY_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        LOGIN_CUSTOM_QUERY_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(LOGIN_CUSTOM_QUERY_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:custom_query")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:custom_query")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.login.ClientboundCustomQueryPacket")
    );
    assert_eq!(
        oracle.answer.input_payload_class.as_deref(),
        Some("net.minecraft.network.protocol.login.custom.DiscardedQueryPayload")
    );
    assert_eq!(
        oracle.answer.decoded_payload_class.as_deref(),
        Some("net.minecraft.network.protocol.login.custom.DiscardedQueryPayload")
    );
    assert_eq!(
        oracle.answer.input_transaction_id, oracle.answer.decoded_transaction_id,
        "official decoded Login clientbound custom_query transaction id differs from input"
    );
    assert_eq!(
        oracle.answer.input_payload_id, oracle.answer.decoded_payload_id,
        "official decoded Login clientbound custom_query payload id differs from input"
    );
    assert_eq!(
        oracle.answer.input_payload_length, oracle.answer.decoded_payload_length,
        "official decoded Login clientbound custom_query payload length differs from input"
    );
    assert_eq!(
        oracle.answer.input_payload_length,
        Some(0),
        "fixture changed: expected empty DiscardedQueryPayload body"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.login_clientbound_packet_table,
        "minecraft:custom_query",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("login_custom_query clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Login,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Login clientbound custom_query packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding Login clientbound custom_query packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Login clientbound custom_query packet id {}",
                framed_packet_id
            )
        });

    let expected_transaction_id = oracle
        .answer
        .decoded_transaction_id
        .expect("login_custom_query answer missing decoded_transaction_id");
    let expected_payload_id = oracle
        .answer
        .decoded_payload_id
        .as_deref()
        .expect("login_custom_query answer missing decoded_payload_id");
    let expected_payload_hex = oracle
        .answer
        .encoded_payload_body_hex
        .as_deref()
        .expect("login_custom_query answer missing encoded_payload_body_hex");
    let expected_payload = decode_hex(expected_payload_hex, "encoded_payload_body_hex");

    match decoded {
        packet::Packet::LoginPluginRequest(packet) => {
            assert_eq!(
                packet.message_id.0, expected_transaction_id,
                "decoded Login clientbound custom_query transaction id did not match official transactionId"
            );
            assert_eq!(
                packet.channel, expected_payload_id,
                "decoded Login clientbound custom_query channel did not match official payload id"
            );
            assert_eq!(
                packet.data, expected_payload,
                "decoded Login clientbound custom_query compatibility packet did not preserve payload bytes"
            );
        }
        other => {
            panic!("decoded packet did not preserve Login clientbound custom_query identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Login clientbound custom_query packet did not consume the official body bytes"
    );
}

#[test]
fn login_cookie_request_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("login_cookie_request_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(login_cookie_request_clientbound_framed_dispatch_body)
        .expect("spawn login_cookie_request_clientbound oracle stack")
        .join()
        .expect("login_cookie_request_clientbound oracle thread panicked");
}

fn login_cookie_request_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(LOGIN_COOKIE_REQUEST_CLIENTBOUND_MANIFEST);
    assert_eq!(manifest.case_id, LOGIN_COOKIE_REQUEST_CLIENTBOUND_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        LOGIN_COOKIE_REQUEST_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        LOGIN_COOKIE_REQUEST_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        LOGIN_COOKIE_REQUEST_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        LOGIN_COOKIE_REQUEST_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(LOGIN_COOKIE_REQUEST_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:cookie_request")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:cookie_request")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.cookie.ClientboundCookieRequestPacket")
    );
    assert_eq!(
        oracle.answer.input_key, oracle.answer.decoded_key,
        "official decoded Login clientbound cookie_request key differs from input"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.login_clientbound_packet_table,
        "minecraft:cookie_request",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("login_cookie_request clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Login,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Login clientbound cookie_request packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding Login clientbound cookie_request packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Login clientbound cookie_request packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::LoginCookieRequest(packet) => {
            assert_eq!(
                packet.key,
                oracle.answer.decoded_key.clone().unwrap_or_default(),
                "decoded Login clientbound cookie_request key did not match official key"
            );
        }
        other => {
            panic!("decoded packet did not preserve Login clientbound cookie_request identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Login clientbound cookie_request packet did not consume the official body bytes"
    );
}

#[test]
fn play_bundle_delimiter_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_bundle_delimiter_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_bundle_delimiter_clientbound_framed_dispatch_body)
        .expect("spawn play_bundle_delimiter_clientbound oracle stack")
        .join()
        .expect("play_bundle_delimiter_clientbound oracle thread panicked");
}

fn play_bundle_delimiter_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(PLAY_BUNDLE_DELIMITER_CLIENTBOUND_MANIFEST);
    assert_eq!(manifest.case_id, PLAY_BUNDLE_DELIMITER_CLIENTBOUND_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        PLAY_BUNDLE_DELIMITER_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        PLAY_BUNDLE_DELIMITER_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        PLAY_BUNDLE_DELIMITER_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        PLAY_BUNDLE_DELIMITER_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(PLAY_BUNDLE_DELIMITER_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:bundle_delimiter")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:bundle_delimiter")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundBundleDelimiterPacket")
    );
    assert_eq!(oracle.answer.encoded_body_hex, "");
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:bundle_delimiter",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_bundle_delimiter clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Play,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Play clientbound bundle_delimiter packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding Play clientbound bundle_delimiter packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound bundle_delimiter packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::BundleDelimiterClientbound(packet) => {
            assert_eq!(
                packet.empty,
                (),
                "decoded Play clientbound bundle_delimiter must preserve empty body"
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound bundle_delimiter identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound bundle_delimiter packet did not consume the official body bytes"
    );
}

#[test]
fn play_add_entity_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_add_entity_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_add_entity_clientbound_framed_dispatch_body)
        .expect("spawn play_add_entity_clientbound oracle stack")
        .join()
        .expect("play_add_entity_clientbound oracle thread panicked");
}

fn play_add_entity_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(PLAY_ADD_ENTITY_CLIENTBOUND_MANIFEST);
    assert_eq!(manifest.case_id, PLAY_ADD_ENTITY_CLIENTBOUND_CASE_ID);
    assert_eq!(manifest.contract_path, PLAY_ADD_ENTITY_CLIENTBOUND_CONTRACT);
    assert_eq!(manifest.answer_path, PLAY_ADD_ENTITY_CLIENTBOUND_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        PLAY_ADD_ENTITY_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        PLAY_ADD_ENTITY_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(PLAY_ADD_ENTITY_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:add_entity")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:add_entity")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundAddEntityPacket")
    );
    assert_eq!(
        oracle.answer.fixture.as_deref(),
        Some(
            "direct official ClientboundAddEntityPacket constructor with bootstrapped built-in EntityType.PIG and zero Vec3.LP movement"
        )
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));
    assert_eq!(
        oracle.answer.input_entity_id,
        oracle.answer.decoded_entity_id
    );
    assert_eq!(oracle.answer.input_uuid, oracle.answer.decoded_uuid);
    assert_eq!(
        oracle.answer.input_entity_type,
        oracle.answer.decoded_entity_type
    );
    assert_eq!(
        oracle.answer.input_entity_type.as_deref(),
        Some("minecraft:pig")
    );
    assert_eq!(oracle.answer.decoded_entity_type_registry_id, Some(100));
    assert_eq!(oracle.answer.input_x, oracle.answer.decoded_x);
    assert_eq!(oracle.answer.input_y, oracle.answer.decoded_y);
    assert_eq!(oracle.answer.input_z, oracle.answer.decoded_z);
    assert_eq!(
        oracle.answer.input_movement_x,
        oracle.answer.decoded_movement_x
    );
    assert_eq!(
        oracle.answer.input_movement_y,
        oracle.answer.decoded_movement_y
    );
    assert_eq!(
        oracle.answer.input_movement_z,
        oracle.answer.decoded_movement_z
    );
    assert_eq!(oracle.answer.encoded_movement_lp_hex.as_deref(), Some("00"));
    assert_eq!(
        oracle.answer.input_x_rot_degrees,
        oracle.answer.decoded_x_rot_degrees
    );
    assert_eq!(oracle.answer.decoded_x_rot_byte, Some(32));
    assert_eq!(
        oracle.answer.input_y_rot_degrees,
        oracle.answer.decoded_y_rot_degrees
    );
    assert_eq!(oracle.answer.decoded_y_rot_byte, Some(64));
    assert_eq!(oracle.answer.input_y_head_rot_degrees, Some(180.0));
    assert_eq!(oracle.answer.decoded_y_head_rot_degrees, Some(-180.0));
    assert_eq!(oracle.answer.decoded_y_head_rot_byte, Some(-128));
    assert_eq!(oracle.answer.input_data, oracle.answer.decoded_data);

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:add_entity",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_add_entity clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Play,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Play clientbound add_entity packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding Play clientbound add_entity packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound add_entity packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::PlayAddEntityClientbound(packet) => {
            let expected_uuid: steven_protocol::protocol::UUID = oracle
                .answer
                .decoded_uuid
                .as_deref()
                .expect("play_add_entity answer missing decoded_uuid")
                .parse()
                .expect("play_add_entity answer decoded_uuid must parse as UUID");
            assert_eq!(packet.entity_id.0, oracle.answer.decoded_entity_id.unwrap());
            assert_eq!(packet.uuid, expected_uuid);
            assert_eq!(
                packet.ty.0,
                oracle.answer.decoded_entity_type_registry_id.unwrap()
            );
            assert_eq!(packet.x, oracle.answer.decoded_x.unwrap());
            assert_eq!(packet.y, oracle.answer.decoded_y.unwrap());
            assert_eq!(packet.z, oracle.answer.decoded_z.unwrap());
            assert_eq!(
                packet.movement_lp_zero.0,
                0,
                "decoded Play clientbound add_entity fixture must preserve zero Vec3.LP movement marker"
            );
            assert_eq!(packet.x_rot, oracle.answer.decoded_x_rot_byte.unwrap());
            assert_eq!(packet.y_rot, oracle.answer.decoded_y_rot_byte.unwrap());
            assert_eq!(
                packet.y_head_rot,
                oracle.answer.decoded_y_head_rot_byte.unwrap()
            );
            assert_eq!(packet.data.0, oracle.answer.decoded_data.unwrap());
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound add_entity identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound add_entity packet did not consume the official body bytes"
    );
}

#[test]
fn play_animate_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_animate_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_animate_clientbound_framed_dispatch_body)
        .expect("spawn play_animate_clientbound oracle stack")
        .join()
        .expect("play_animate_clientbound oracle thread panicked");
}

fn play_animate_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(PLAY_ANIMATE_CLIENTBOUND_MANIFEST);
    assert_eq!(manifest.case_id, PLAY_ANIMATE_CLIENTBOUND_CASE_ID);
    assert_eq!(manifest.contract_path, PLAY_ANIMATE_CLIENTBOUND_CONTRACT);
    assert_eq!(manifest.answer_path, PLAY_ANIMATE_CLIENTBOUND_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(manifest.rust_test_name, PLAY_ANIMATE_CLIENTBOUND_TEST_NAME);
    assert_eq!(
        manifest.comparison_surface,
        PLAY_ANIMATE_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(PLAY_ANIMATE_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:animate")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:animate")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundAnimatePacket")
    );
    assert_eq!(
        oracle.answer.fixture.as_deref(),
        Some(
            "official ClientboundAnimatePacket.STREAM_CODEC decode fixture with entity id and SWING_MAIN_HAND action; no initialized Entity, Level, or game state"
        )
    );
    assert_eq!(oracle.answer.remaining_after_packet_stream_decode, Some(0));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));
    assert_eq!(
        oracle.answer.input_entity_id,
        oracle.answer.decoded_entity_id
    );
    assert_eq!(
        oracle.answer.input_animation_action_name.as_deref(),
        Some("SWING_MAIN_HAND")
    );
    assert_eq!(
        oracle.answer.input_animation_action,
        oracle.answer.decoded_animation_action
    );
    assert_eq!(oracle.answer.decoded_animation_action, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:animate",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_animate clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let fixture_body = decode_hex(
        oracle
            .answer
            .fixture_body_hex
            .as_deref()
            .expect("play_animate answer missing fixture_body_hex"),
        "fixture_body_hex",
    );
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body, fixture_body,
        "official animate frame body must match the official STREAM_CODEC decode fixture body"
    );

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Play,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Play clientbound animate packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding Play clientbound animate packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound animate packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::PlayAnimateClientbound(packet) => {
            assert_eq!(packet.entity_id.0, oracle.answer.decoded_entity_id.unwrap());
            assert_eq!(
                packet.action,
                oracle.answer.decoded_animation_action.unwrap() as u8
            );
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound animate identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound animate packet did not consume the official body bytes"
    );
}

#[test]
fn play_award_stats_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_award_stats_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_award_stats_clientbound_framed_dispatch_body)
        .expect("spawn play_award_stats_clientbound oracle stack")
        .join()
        .expect("play_award_stats_clientbound oracle thread panicked");
}

fn play_award_stats_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(PLAY_AWARD_STATS_CLIENTBOUND_MANIFEST);
    assert_eq!(manifest.case_id, PLAY_AWARD_STATS_CLIENTBOUND_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        PLAY_AWARD_STATS_CLIENTBOUND_CONTRACT
    );
    assert_eq!(manifest.answer_path, PLAY_AWARD_STATS_CLIENTBOUND_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        PLAY_AWARD_STATS_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        PLAY_AWARD_STATS_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(PLAY_AWARD_STATS_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:award_stats")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:award_stats")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundAwardStatsPacket")
    );
    assert_eq!(
        oracle.answer.fixture.as_deref(),
        Some(
            "official ClientboundAwardStatsPacket empty Object2IntOpenHashMap<Stat<?>> fixture; no initialized Minecraft/game state or stat registry entries"
        )
    );
    assert_eq!(
        oracle.answer.official_body_shape.as_deref(),
        Some(
            "Object2IntMap<Stat<?>> encoded as VarInt count, then per entry Stat.STREAM_CODEC key followed by VarInt value; empty fixture encodes only count 0"
        )
    );
    assert_eq!(oracle.answer.input_stat_count, Some(0));
    assert_eq!(oracle.answer.stream_decoded_stat_count, Some(0));
    assert_eq!(oracle.answer.decoded_stat_count, Some(0));
    assert_eq!(oracle.answer.remaining_after_packet_stream_decode, Some(0));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:award_stats",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_award_stats clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let fixture_body = decode_hex(
        oracle
            .answer
            .fixture_body_hex
            .as_deref()
            .expect("play_award_stats answer missing fixture_body_hex"),
        "fixture_body_hex",
    );
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body, fixture_body,
        "official award_stats frame body must match the official STREAM_CODEC empty-stats fixture body"
    );

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Play,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Play clientbound award_stats packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding Play clientbound award_stats packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound award_stats packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::PlayAwardStatsClientbound(packet) => {
            assert_eq!(
                packet.stat_count.0 as usize,
                oracle.answer.decoded_stat_count.unwrap()
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound award_stats identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound award_stats packet did not consume the official body bytes"
    );
}

#[test]
fn play_block_changed_ack_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_block_changed_ack_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_block_changed_ack_clientbound_framed_dispatch_body)
        .expect("spawn play_block_changed_ack_clientbound oracle stack")
        .join()
        .expect("play_block_changed_ack_clientbound oracle thread panicked");
}

fn play_block_changed_ack_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(PLAY_BLOCK_CHANGED_ACK_CLIENTBOUND_MANIFEST);
    assert_eq!(manifest.case_id, PLAY_BLOCK_CHANGED_ACK_CLIENTBOUND_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        PLAY_BLOCK_CHANGED_ACK_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        PLAY_BLOCK_CHANGED_ACK_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        PLAY_BLOCK_CHANGED_ACK_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        PLAY_BLOCK_CHANGED_ACK_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(PLAY_BLOCK_CHANGED_ACK_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:block_changed_ack")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:block_changed_ack")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundBlockChangedAckPacket")
    );
    assert_eq!(
        oracle.answer.fixture.as_deref(),
        Some(
            "official ClientboundBlockChangedAckPacket sequence constructor fixture; no initialized Minecraft/game state"
        )
    );
    assert_eq!(
        oracle.answer.official_body_shape.as_deref(),
        Some("sequence encoded as one FriendlyByteBuf VarInt")
    );
    assert_eq!(oracle.answer.input_sequence, Some(12345));
    assert_eq!(oracle.answer.stream_decoded_sequence, Some(12345));
    assert_eq!(oracle.answer.decoded_sequence, Some(12345));
    assert_eq!(oracle.answer.remaining_after_packet_stream_decode, Some(0));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:block_changed_ack",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_block_changed_ack clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let fixture_body = decode_hex(
        oracle
            .answer
            .fixture_body_hex
            .as_deref()
            .expect("play_block_changed_ack answer missing fixture_body_hex"),
        "fixture_body_hex",
    );
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body, fixture_body,
        "official block_changed_ack frame body must match the official STREAM_CODEC sequence fixture body"
    );

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Play,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Play clientbound block_changed_ack packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!(
                "Stevenarella errored while decoding Play clientbound block_changed_ack packet: {err}"
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound block_changed_ack packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::PlayBlockChangedAckClientbound(packet) => {
            assert_eq!(packet.sequence.0, oracle.answer.decoded_sequence.unwrap());
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound block_changed_ack identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound block_changed_ack packet did not consume the official body bytes"
    );
}

#[test]
fn play_block_destruction_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_block_destruction_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_block_destruction_clientbound_framed_dispatch_body)
        .expect("spawn play_block_destruction_clientbound oracle stack")
        .join()
        .expect("play_block_destruction_clientbound oracle thread panicked");
}

fn play_block_destruction_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(PLAY_BLOCK_DESTRUCTION_CLIENTBOUND_MANIFEST);
    assert_eq!(manifest.case_id, PLAY_BLOCK_DESTRUCTION_CLIENTBOUND_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        PLAY_BLOCK_DESTRUCTION_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        PLAY_BLOCK_DESTRUCTION_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        PLAY_BLOCK_DESTRUCTION_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        PLAY_BLOCK_DESTRUCTION_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(PLAY_BLOCK_DESTRUCTION_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:block_destruction")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:block_destruction")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundBlockDestructionPacket")
    );
    assert_eq!(
        oracle.answer.fixture.as_deref(),
        Some(
            "official ClientboundBlockDestructionPacket breaker id, BlockPos, and progress constructor fixture; no initialized Minecraft/game state"
        )
    );
    assert_eq!(
        oracle.answer.official_body_shape.as_deref(),
        Some(
            "breaker id encoded as FriendlyByteBuf VarInt, block position encoded with FriendlyByteBuf BlockPos, progress encoded as one unsigned byte"
        )
    );
    assert_eq!(oracle.answer.input_breaker_id, Some(123));
    assert_eq!(oracle.answer.stream_decoded_breaker_id, Some(123));
    assert_eq!(oracle.answer.decoded_breaker_id, Some(123));
    assert_eq!(oracle.answer.input_block_x, Some(12));
    assert_eq!(oracle.answer.input_block_y, Some(64));
    assert_eq!(oracle.answer.input_block_z, Some(-7));
    assert_eq!(oracle.answer.stream_decoded_block_x, Some(12));
    assert_eq!(oracle.answer.stream_decoded_block_y, Some(64));
    assert_eq!(oracle.answer.stream_decoded_block_z, Some(-7));
    assert_eq!(oracle.answer.decoded_block_x, Some(12));
    assert_eq!(oracle.answer.decoded_block_y, Some(64));
    assert_eq!(oracle.answer.decoded_block_z, Some(-7));
    assert_eq!(oracle.answer.input_progress, Some(5));
    assert_eq!(oracle.answer.stream_decoded_progress, Some(5));
    assert_eq!(oracle.answer.decoded_progress, Some(5));
    assert_eq!(oracle.answer.remaining_after_packet_stream_decode, Some(0));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:block_destruction",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_block_destruction clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let fixture_body = decode_hex(
        oracle
            .answer
            .fixture_body_hex
            .as_deref()
            .expect("play_block_destruction answer missing fixture_body_hex"),
        "fixture_body_hex",
    );
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body, fixture_body,
        "official block_destruction frame body must match the official STREAM_CODEC fixture body"
    );

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Play,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Play clientbound block_destruction packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!(
                "Stevenarella errored while decoding Play clientbound block_destruction packet: {err}"
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound block_destruction packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::PlayBlockDestructionClientbound(packet) => {
            assert_eq!(
                packet.breaker_id.0,
                oracle.answer.decoded_breaker_id.unwrap()
            );
            assert_eq!(packet.location.x, oracle.answer.decoded_block_x.unwrap());
            assert_eq!(packet.location.y, oracle.answer.decoded_block_y.unwrap());
            assert_eq!(packet.location.z, oracle.answer.decoded_block_z.unwrap());
            assert_eq!(
                i32::from(packet.progress),
                oracle.answer.decoded_progress.unwrap()
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound block_destruction identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound block_destruction packet did not consume the official body bytes"
    );
}

#[test]
fn play_block_entity_data_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_block_entity_data_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_block_entity_data_clientbound_framed_dispatch_body)
        .expect("spawn play_block_entity_data_clientbound oracle stack")
        .join()
        .expect("play_block_entity_data_clientbound oracle thread panicked");
}

fn play_block_entity_data_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(PLAY_BLOCK_ENTITY_DATA_CLIENTBOUND_MANIFEST);
    assert_eq!(manifest.case_id, PLAY_BLOCK_ENTITY_DATA_CLIENTBOUND_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        PLAY_BLOCK_ENTITY_DATA_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        PLAY_BLOCK_ENTITY_DATA_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        PLAY_BLOCK_ENTITY_DATA_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        PLAY_BLOCK_ENTITY_DATA_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(PLAY_BLOCK_ENTITY_DATA_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:block_entity_data")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:block_entity_data")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundBlockEntityDataPacket")
    );
    assert_eq!(
        oracle.answer.fixture.as_deref(),
        Some(
            "official private ClientboundBlockEntityDataPacket BlockPos, built-in BlockEntityType.CHEST, and empty CompoundTag constructor fixture; requires bootstrapped built-in registries but no initialized Level, BlockEntity, or game state"
        )
    );
    assert_eq!(
        oracle.answer.official_body_shape.as_deref(),
        Some(
            "block position encoded with BlockPos.STREAM_CODEC, block entity type encoded with ByteBufCodecs.registry(Registries.BLOCK_ENTITY_TYPE), and tag encoded with ByteBufCodecs.TRUSTED_COMPOUND_TAG"
        )
    );
    assert_eq!(oracle.answer.input_block_x, Some(12));
    assert_eq!(oracle.answer.input_block_y, Some(64));
    assert_eq!(oracle.answer.input_block_z, Some(-7));
    assert_eq!(oracle.answer.stream_decoded_block_x, Some(12));
    assert_eq!(oracle.answer.stream_decoded_block_y, Some(64));
    assert_eq!(oracle.answer.stream_decoded_block_z, Some(-7));
    assert_eq!(oracle.answer.decoded_block_x, Some(12));
    assert_eq!(oracle.answer.decoded_block_y, Some(64));
    assert_eq!(oracle.answer.decoded_block_z, Some(-7));
    assert_eq!(
        oracle.answer.input_block_entity_type.as_deref(),
        Some("minecraft:chest")
    );
    assert_eq!(
        oracle.answer.stream_decoded_block_entity_type.as_deref(),
        Some("minecraft:chest")
    );
    assert_eq!(
        oracle.answer.decoded_block_entity_type.as_deref(),
        Some("minecraft:chest")
    );
    assert_eq!(oracle.answer.decoded_block_entity_type_registry_id, Some(1));
    assert_eq!(oracle.answer.input_tag_size, Some(0));
    assert_eq!(oracle.answer.stream_decoded_tag_size, Some(0));
    assert_eq!(oracle.answer.decoded_tag_size, Some(0));
    assert_eq!(oracle.answer.input_tag_snbt.as_deref(), Some("{}"));
    assert_eq!(oracle.answer.stream_decoded_tag_snbt.as_deref(), Some("{}"));
    assert_eq!(oracle.answer.decoded_tag_snbt.as_deref(), Some("{}"));
    assert_eq!(oracle.answer.remaining_after_packet_stream_decode, Some(0));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:block_entity_data",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_block_entity_data clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let fixture_body = decode_hex(
        oracle
            .answer
            .fixture_body_hex
            .as_deref()
            .expect("play_block_entity_data answer missing fixture_body_hex"),
        "fixture_body_hex",
    );
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body, fixture_body,
        "official block_entity_data frame body must match the official STREAM_CODEC fixture body"
    );

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Play,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Play clientbound block_entity_data packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!(
                "Stevenarella errored while decoding Play clientbound block_entity_data packet: {err}"
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound block_entity_data packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::PlayBlockEntityDataClientbound(packet) => {
            assert_eq!(packet.location.x, oracle.answer.decoded_block_x.unwrap());
            assert_eq!(packet.location.y, oracle.answer.decoded_block_y.unwrap());
            assert_eq!(packet.location.z, oracle.answer.decoded_block_z.unwrap());
            assert_eq!(
                packet.block_entity_type.0,
                oracle.answer.decoded_block_entity_type_registry_id.unwrap()
            );
            assert_eq!(packet.nbt_tag_type, 10);
            assert_eq!(
                packet.tag,
                vec![0],
                "official empty trusted compound tag should contain only the compound end marker after the root tag id"
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound block_entity_data identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound block_entity_data packet did not consume the official body bytes"
    );
}

#[test]
fn play_block_event_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_block_event_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_block_event_clientbound_framed_dispatch_body)
        .expect("spawn play_block_event_clientbound oracle stack")
        .join()
        .expect("play_block_event_clientbound oracle thread panicked");
}

fn play_block_event_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(PLAY_BLOCK_EVENT_CLIENTBOUND_MANIFEST);
    assert_eq!(manifest.case_id, PLAY_BLOCK_EVENT_CLIENTBOUND_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        PLAY_BLOCK_EVENT_CLIENTBOUND_CONTRACT
    );
    assert_eq!(manifest.answer_path, PLAY_BLOCK_EVENT_CLIENTBOUND_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        PLAY_BLOCK_EVENT_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        PLAY_BLOCK_EVENT_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(PLAY_BLOCK_EVENT_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:block_event")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:block_event")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundBlockEventPacket")
    );
    assert_eq!(
        oracle.answer.fixture.as_deref(),
        Some(
            "official ClientboundBlockEventPacket BlockPos, built-in Blocks.NOTE_BLOCK, event type, and event data constructor fixture; requires bootstrapped built-in registries but no initialized Level, BlockEntity, or game state"
        )
    );
    assert_eq!(
        oracle.answer.official_body_shape.as_deref(),
        Some(
            "block position encoded with RegistryFriendlyByteBuf BlockPos, event type encoded as one unsigned byte, event data encoded as one unsigned byte, and block encoded with ByteBufCodecs.registry(Registries.BLOCK)"
        )
    );
    assert_eq!(oracle.answer.input_block_x, Some(12));
    assert_eq!(oracle.answer.input_block_y, Some(64));
    assert_eq!(oracle.answer.input_block_z, Some(-7));
    assert_eq!(oracle.answer.stream_decoded_block_x, Some(12));
    assert_eq!(oracle.answer.stream_decoded_block_y, Some(64));
    assert_eq!(oracle.answer.stream_decoded_block_z, Some(-7));
    assert_eq!(oracle.answer.decoded_block_x, Some(12));
    assert_eq!(oracle.answer.decoded_block_y, Some(64));
    assert_eq!(oracle.answer.decoded_block_z, Some(-7));
    assert_eq!(oracle.answer.input_event_type, Some(1));
    assert_eq!(oracle.answer.stream_decoded_event_type, Some(1));
    assert_eq!(oracle.answer.decoded_event_type, Some(1));
    assert_eq!(oracle.answer.input_event_data, Some(2));
    assert_eq!(oracle.answer.stream_decoded_event_data, Some(2));
    assert_eq!(oracle.answer.decoded_event_data, Some(2));
    assert_eq!(
        oracle.answer.input_block.as_deref(),
        Some("minecraft:note_block")
    );
    assert_eq!(
        oracle.answer.stream_decoded_block.as_deref(),
        Some("minecraft:note_block")
    );
    assert_eq!(
        oracle.answer.decoded_block.as_deref(),
        Some("minecraft:note_block")
    );
    assert_eq!(oracle.answer.decoded_block_registry_id, Some(109));
    assert_eq!(oracle.answer.remaining_after_packet_stream_decode, Some(0));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:block_event",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_block_event clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let fixture_body = decode_hex(
        oracle
            .answer
            .fixture_body_hex
            .as_deref()
            .expect("play_block_event answer missing fixture_body_hex"),
        "fixture_body_hex",
    );
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body, fixture_body,
        "official block_event frame body must match the official STREAM_CODEC fixture body"
    );

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Play,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Play clientbound block_event packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding Play clientbound block_event packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound block_event packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::PlayBlockEventClientbound(packet) => {
            assert_eq!(packet.location.x, oracle.answer.decoded_block_x.unwrap());
            assert_eq!(packet.location.y, oracle.answer.decoded_block_y.unwrap());
            assert_eq!(packet.location.z, oracle.answer.decoded_block_z.unwrap());
            assert_eq!(
                i32::from(packet.event_type),
                oracle.answer.decoded_event_type.unwrap()
            );
            assert_eq!(
                i32::from(packet.event_data),
                oracle.answer.decoded_event_data.unwrap()
            );
            assert_eq!(
                packet.block.0,
                oracle.answer.decoded_block_registry_id.unwrap()
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound block_event identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound block_event packet did not consume the official body bytes"
    );
}

#[test]
fn play_block_update_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_block_update_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_block_update_clientbound_framed_dispatch_body)
        .expect("spawn play_block_update_clientbound oracle stack")
        .join()
        .expect("play_block_update_clientbound oracle thread panicked");
}

fn play_block_update_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(PLAY_BLOCK_UPDATE_CLIENTBOUND_MANIFEST);
    assert_eq!(manifest.case_id, PLAY_BLOCK_UPDATE_CLIENTBOUND_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        PLAY_BLOCK_UPDATE_CLIENTBOUND_CONTRACT
    );
    assert_eq!(manifest.answer_path, PLAY_BLOCK_UPDATE_CLIENTBOUND_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        PLAY_BLOCK_UPDATE_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        PLAY_BLOCK_UPDATE_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(PLAY_BLOCK_UPDATE_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:block_update")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:block_update")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket")
    );
    assert_eq!(
        oracle.answer.fixture.as_deref(),
        Some(
            "official ClientboundBlockUpdatePacket BlockPos and built-in Blocks.STONE.defaultBlockState() constructor fixture; requires bootstrapped built-in block state registry but no initialized Level or game state"
        )
    );
    assert_eq!(
        oracle.answer.official_body_shape.as_deref(),
        Some(
            "block position encoded with BlockPos.STREAM_CODEC and block state encoded with ByteBufCodecs.idMapper(Block.BLOCK_STATE_REGISTRY)"
        )
    );
    assert_eq!(oracle.answer.input_block_x, Some(12));
    assert_eq!(oracle.answer.input_block_y, Some(64));
    assert_eq!(oracle.answer.input_block_z, Some(-7));
    assert_eq!(oracle.answer.stream_decoded_block_x, Some(12));
    assert_eq!(oracle.answer.stream_decoded_block_y, Some(64));
    assert_eq!(oracle.answer.stream_decoded_block_z, Some(-7));
    assert_eq!(oracle.answer.decoded_block_x, Some(12));
    assert_eq!(oracle.answer.decoded_block_y, Some(64));
    assert_eq!(oracle.answer.decoded_block_z, Some(-7));
    assert_eq!(
        oracle.answer.input_block.as_deref(),
        Some("minecraft:stone")
    );
    assert_eq!(
        oracle.answer.stream_decoded_block_state_block.as_deref(),
        Some("minecraft:stone")
    );
    assert_eq!(
        oracle.answer.decoded_block_state_block.as_deref(),
        Some("minecraft:stone")
    );
    assert_eq!(oracle.answer.decoded_block_state_registry_id, Some(1));
    assert_eq!(oracle.answer.remaining_after_packet_stream_decode, Some(0));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:block_update",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_block_update clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let fixture_body = decode_hex(
        oracle
            .answer
            .fixture_body_hex
            .as_deref()
            .expect("play_block_update answer missing fixture_body_hex"),
        "fixture_body_hex",
    );
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body, fixture_body,
        "official block_update frame body must match the official STREAM_CODEC fixture body"
    );

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Play,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Play clientbound block_update packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!(
                "Stevenarella errored while decoding Play clientbound block_update packet: {err}"
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound block_update packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::PlayBlockUpdateClientbound(packet) => {
            assert_eq!(packet.location.x, oracle.answer.decoded_block_x.unwrap());
            assert_eq!(packet.location.y, oracle.answer.decoded_block_y.unwrap());
            assert_eq!(packet.location.z, oracle.answer.decoded_block_z.unwrap());
            assert_eq!(
                packet.block_state.0,
                oracle.answer.decoded_block_state_registry_id.unwrap()
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound block_update identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound block_update packet did not consume the official body bytes"
    );
}

#[test]
fn play_boss_event_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_boss_event_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_boss_event_clientbound_framed_dispatch_body)
        .expect("spawn play_boss_event_clientbound oracle stack")
        .join()
        .expect("play_boss_event_clientbound oracle thread panicked");
}

fn play_boss_event_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(PLAY_BOSS_EVENT_CLIENTBOUND_MANIFEST);
    assert_eq!(manifest.case_id, PLAY_BOSS_EVENT_CLIENTBOUND_CASE_ID);
    assert_eq!(manifest.contract_path, PLAY_BOSS_EVENT_CLIENTBOUND_CONTRACT);
    assert_eq!(manifest.answer_path, PLAY_BOSS_EVENT_CLIENTBOUND_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        PLAY_BOSS_EVENT_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        PLAY_BOSS_EVENT_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(PLAY_BOSS_EVENT_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:boss_event")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:boss_event")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundBossEventPacket")
    );
    assert_eq!(
        oracle.answer.fixture.as_deref(),
        Some(
            "official ClientboundBossEventPacket.createRemovePacket(UUID) fixture; context-free remove operation with no initialized BossEvent, Level, or game state"
        )
    );
    assert_eq!(
        oracle.answer.official_body_shape.as_deref(),
        Some(
            "UUID encoded with RegistryFriendlyByteBuf.writeUUID, operation encoded with RegistryFriendlyByteBuf.writeEnum/readEnum, and REMOVE operation writes no additional body"
        )
    );
    assert_eq!(oracle.answer.input_uuid, oracle.answer.body_uuid);
    assert_eq!(oracle.answer.input_uuid, oracle.answer.stream_decoded_uuid);
    assert_eq!(oracle.answer.input_uuid, oracle.answer.decoded_uuid);
    assert_eq!(oracle.answer.input_operation.as_deref(), Some("REMOVE"));
    assert_eq!(
        oracle.answer.stream_decoded_operation.as_deref(),
        Some("REMOVE")
    );
    assert_eq!(oracle.answer.decoded_operation.as_deref(), Some("REMOVE"));
    assert_eq!(oracle.answer.decoded_operation_ordinal, Some(1));
    assert_eq!(oracle.answer.remaining_after_operation_read, Some(0));
    assert_eq!(oracle.answer.remaining_after_packet_stream_decode, Some(0));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:boss_event",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_boss_event clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let fixture_body = decode_hex(
        oracle
            .answer
            .fixture_body_hex
            .as_deref()
            .expect("play_boss_event answer missing fixture_body_hex"),
        "fixture_body_hex",
    );
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body, fixture_body,
        "official boss_event frame body must match the official STREAM_CODEC fixture body"
    );

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Play,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Play clientbound boss_event packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding Play clientbound boss_event packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound boss_event packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::BossBar(packet) => {
            let expected_uuid: steven_protocol::protocol::UUID = oracle
                .answer
                .decoded_uuid
                .as_deref()
                .expect("play_boss_event answer missing decoded_uuid")
                .parse()
                .expect("play_boss_event answer decoded_uuid must parse as UUID");
            assert_eq!(packet.uuid, expected_uuid);
            assert_eq!(
                packet.action.0,
                oracle.answer.decoded_operation_ordinal.unwrap()
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound boss_event identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound boss_event packet did not consume the official body bytes"
    );
}

#[test]
fn play_change_difficulty_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_change_difficulty_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_change_difficulty_clientbound_framed_dispatch_body)
        .expect("spawn play_change_difficulty_clientbound oracle stack")
        .join()
        .expect("play_change_difficulty_clientbound oracle thread panicked");
}

fn play_change_difficulty_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(PLAY_CHANGE_DIFFICULTY_CLIENTBOUND_MANIFEST);
    assert_eq!(manifest.case_id, PLAY_CHANGE_DIFFICULTY_CLIENTBOUND_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        PLAY_CHANGE_DIFFICULTY_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        PLAY_CHANGE_DIFFICULTY_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        PLAY_CHANGE_DIFFICULTY_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        PLAY_CHANGE_DIFFICULTY_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(PLAY_CHANGE_DIFFICULTY_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:change_difficulty")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:change_difficulty")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundChangeDifficultyPacket")
    );
    assert_eq!(
        oracle.answer.fixture.as_deref(),
        Some(
            "official ClientboundChangeDifficultyPacket(Difficulty, boolean) constructor fixture; context-free difficulty/locked record with no initialized Level or game state"
        )
    );
    assert_eq!(
        oracle.answer.official_body_shape.as_deref(),
        Some(
            "difficulty encoded with Difficulty.STREAM_CODEC followed by locked encoded with ByteBufCodecs.BOOL"
        )
    );
    assert_eq!(oracle.answer.input_difficulty.as_deref(), Some("HARD"));
    assert_eq!(
        oracle.answer.input_difficulty_serialized_name.as_deref(),
        Some("hard")
    );
    assert_eq!(oracle.answer.input_difficulty_id, Some(3));
    assert_eq!(
        oracle.answer.stream_decoded_difficulty,
        oracle.answer.input_difficulty
    );
    assert_eq!(
        oracle.answer.stream_decoded_difficulty_serialized_name,
        oracle.answer.input_difficulty_serialized_name
    );
    assert_eq!(
        oracle.answer.stream_decoded_difficulty_id,
        oracle.answer.input_difficulty_id
    );
    assert_eq!(
        oracle.answer.decoded_difficulty,
        oracle.answer.input_difficulty
    );
    assert_eq!(
        oracle.answer.decoded_difficulty_serialized_name,
        oracle.answer.input_difficulty_serialized_name
    );
    assert_eq!(
        oracle.answer.decoded_difficulty_id,
        oracle.answer.input_difficulty_id
    );
    assert_eq!(oracle.answer.input_locked, Some(true));
    assert_eq!(
        oracle.answer.stream_decoded_locked,
        oracle.answer.input_locked
    );
    assert_eq!(oracle.answer.decoded_locked, oracle.answer.input_locked);
    assert_eq!(oracle.answer.remaining_after_packet_stream_decode, Some(0));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:change_difficulty",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_change_difficulty clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let fixture_body = decode_hex(
        oracle
            .answer
            .fixture_body_hex
            .as_deref()
            .expect("play_change_difficulty answer missing fixture_body_hex"),
        "fixture_body_hex",
    );
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body, fixture_body,
        "official change_difficulty frame body must match the official STREAM_CODEC fixture body"
    );

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Play,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Play clientbound change_difficulty packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!(
                "Stevenarella errored while decoding Play clientbound change_difficulty packet: {err}"
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound change_difficulty packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::ServerDifficulty_Locked(packet) => {
            assert_eq!(
                i32::from(packet.difficulty),
                oracle.answer.decoded_difficulty_id.unwrap()
            );
            assert_eq!(packet.locked, oracle.answer.decoded_locked.unwrap());
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound change_difficulty identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound change_difficulty packet did not consume the official body bytes"
    );
}

#[test]
fn play_chunk_batch_finished_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_chunk_batch_finished_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_chunk_batch_finished_clientbound_framed_dispatch_body)
        .expect("spawn play_chunk_batch_finished_clientbound oracle stack")
        .join()
        .expect("play_chunk_batch_finished_clientbound oracle thread panicked");
}

fn play_chunk_batch_finished_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(PLAY_CHUNK_BATCH_FINISHED_CLIENTBOUND_MANIFEST);
    assert_eq!(
        manifest.case_id,
        PLAY_CHUNK_BATCH_FINISHED_CLIENTBOUND_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        PLAY_CHUNK_BATCH_FINISHED_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        PLAY_CHUNK_BATCH_FINISHED_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        PLAY_CHUNK_BATCH_FINISHED_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        PLAY_CHUNK_BATCH_FINISHED_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(PLAY_CHUNK_BATCH_FINISHED_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:chunk_batch_finished")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:chunk_batch_finished")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundChunkBatchFinishedPacket")
    );
    assert_eq!(
        oracle.answer.fixture.as_deref(),
        Some(
            "official ClientboundChunkBatchFinishedPacket(int) constructor fixture; context-free batch-size record with no initialized Level, chunk storage, or game state"
        )
    );
    assert_eq!(
        oracle.answer.official_body_shape.as_deref(),
        Some("batchSize encoded as FriendlyByteBuf VarInt")
    );
    assert_eq!(oracle.answer.input_batch_size, Some(7));
    assert_eq!(
        oracle.answer.stream_decoded_batch_size,
        oracle.answer.input_batch_size
    );
    assert_eq!(
        oracle.answer.decoded_batch_size,
        oracle.answer.input_batch_size
    );
    assert_eq!(oracle.answer.remaining_after_packet_stream_decode, Some(0));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:chunk_batch_finished",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_chunk_batch_finished clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let fixture_body = decode_hex(
        oracle
            .answer
            .fixture_body_hex
            .as_deref()
            .expect("play_chunk_batch_finished answer missing fixture_body_hex"),
        "fixture_body_hex",
    );
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body, fixture_body,
        "official chunk_batch_finished frame body must match the official STREAM_CODEC fixture body"
    );

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Play,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Play clientbound chunk_batch_finished packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!(
                "Stevenarella errored while decoding Play clientbound chunk_batch_finished packet: {err}"
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound chunk_batch_finished packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::PlayChunkBatchFinishedClientbound(packet) => {
            assert_eq!(
                packet.batch_size.0,
                oracle.answer.decoded_batch_size.unwrap()
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound chunk_batch_finished identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound chunk_batch_finished packet did not consume the official body bytes"
    );
}

#[test]
fn play_chunk_batch_start_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_chunk_batch_start_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_chunk_batch_start_clientbound_framed_dispatch_body)
        .expect("spawn play_chunk_batch_start_clientbound oracle stack")
        .join()
        .expect("play_chunk_batch_start_clientbound oracle thread panicked");
}

fn play_chunk_batch_start_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(PLAY_CHUNK_BATCH_START_CLIENTBOUND_MANIFEST);
    assert_eq!(manifest.case_id, PLAY_CHUNK_BATCH_START_CLIENTBOUND_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        PLAY_CHUNK_BATCH_START_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        PLAY_CHUNK_BATCH_START_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        PLAY_CHUNK_BATCH_START_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        PLAY_CHUNK_BATCH_START_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(PLAY_CHUNK_BATCH_START_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:chunk_batch_start")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:chunk_batch_start")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundChunkBatchStartPacket")
    );
    assert_eq!(
        oracle.answer.fixture.as_deref(),
        Some(
            "official ClientboundChunkBatchStartPacket.INSTANCE singleton fixture; context-free empty-body packet with no initialized Level, chunk storage, or game state"
        )
    );
    assert_eq!(
        oracle.answer.official_body_shape.as_deref(),
        Some("empty body encoded by StreamCodec.unit(INSTANCE)")
    );
    assert_eq!(oracle.answer.stream_decoded_same_instance, Some(true));
    assert_eq!(oracle.answer.decoded_same_instance, Some(true));
    assert_eq!(oracle.answer.remaining_after_packet_stream_decode, Some(0));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:chunk_batch_start",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_chunk_batch_start clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let fixture_body = decode_hex(
        oracle
            .answer
            .fixture_body_hex
            .as_deref()
            .expect("play_chunk_batch_start answer missing fixture_body_hex"),
        "fixture_body_hex",
    );
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body, fixture_body,
        "official chunk_batch_start frame body must match the official STREAM_CODEC fixture body"
    );
    assert!(
        body.is_empty(),
        "official chunk_batch_start body must be empty"
    );

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Play,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Play clientbound chunk_batch_start packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!(
                "Stevenarella errored while decoding Play clientbound chunk_batch_start packet: {err}"
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound chunk_batch_start packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::PlayChunkBatchStartClientbound(_) => {}
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound chunk_batch_start identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound chunk_batch_start packet did not consume the official body bytes"
    );
}

#[test]
fn play_chunks_biomes_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_chunks_biomes_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_chunks_biomes_clientbound_framed_dispatch_body)
        .expect("spawn play_chunks_biomes_clientbound oracle stack")
        .join()
        .expect("play_chunks_biomes_clientbound oracle thread panicked");
}

fn play_chunks_biomes_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(PLAY_CHUNKS_BIOMES_CLIENTBOUND_MANIFEST);
    assert_eq!(manifest.case_id, PLAY_CHUNKS_BIOMES_CLIENTBOUND_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        PLAY_CHUNKS_BIOMES_CLIENTBOUND_CONTRACT
    );
    assert_eq!(manifest.answer_path, PLAY_CHUNKS_BIOMES_CLIENTBOUND_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        PLAY_CHUNKS_BIOMES_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        PLAY_CHUNKS_BIOMES_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(PLAY_CHUNKS_BIOMES_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:chunks_biomes")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:chunks_biomes")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundChunksBiomesPacket")
    );
    assert_eq!(
        oracle.answer.fixture.as_deref(),
        Some(
            "official ClientboundChunksBiomesPacket(List<ChunkBiomeData>) constructor fixture with an empty chunkBiomeData list; context-free list-count body with no initialized Level, LevelChunk, biome registry, chunk storage, or game state"
        )
    );
    assert_eq!(
        oracle.answer.official_body_shape.as_deref(),
        Some("chunkBiomeData encoded as FriendlyByteBuf VarInt list count followed by each ChunkBiomeData as ChunkPos via FriendlyByteBuf.writeChunkPos and biome byte array via FriendlyByteBuf.writeByteArray/readByteArray(max 2097152); empty fixture encodes only list count 0")
    );
    assert_eq!(oracle.answer.input_chunk_biome_count, Some(0));
    assert_eq!(oracle.answer.stream_decoded_chunk_biome_count, Some(0));
    assert_eq!(oracle.answer.decoded_chunk_biome_count, Some(0));
    assert_eq!(oracle.answer.remaining_after_packet_stream_decode, Some(0));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:chunks_biomes",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_chunks_biomes clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let fixture_body = decode_hex(
        oracle
            .answer
            .fixture_body_hex
            .as_deref()
            .expect("play_chunks_biomes answer missing fixture_body_hex"),
        "fixture_body_hex",
    );
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body, fixture_body,
        "official chunks_biomes frame body must match the official STREAM_CODEC fixture body"
    );
    assert_eq!(
        body,
        vec![0],
        "official chunks_biomes empty-list body must encode only VarInt list count 0"
    );

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Play,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Play clientbound chunks_biomes packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!(
                "Stevenarella errored while decoding Play clientbound chunks_biomes packet: {err}"
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound chunks_biomes packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::PlayChunksBiomesClientbound(chunks_biomes) => {
            assert!(
                chunks_biomes.chunk_biome_data.data.is_empty(),
                "decoded Play clientbound chunks_biomes fixture should have zero chunkBiomeData entries"
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound chunks_biomes identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound chunks_biomes packet did not consume the official body bytes"
    );
}

#[test]
fn play_clear_titles_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_clear_titles_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_clear_titles_clientbound_framed_dispatch_body)
        .expect("spawn play_clear_titles_clientbound oracle stack")
        .join()
        .expect("play_clear_titles_clientbound oracle thread panicked");
}

fn play_clear_titles_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(PLAY_CLEAR_TITLES_CLIENTBOUND_MANIFEST);
    assert_eq!(manifest.case_id, PLAY_CLEAR_TITLES_CLIENTBOUND_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        PLAY_CLEAR_TITLES_CLIENTBOUND_CONTRACT
    );
    assert_eq!(manifest.answer_path, PLAY_CLEAR_TITLES_CLIENTBOUND_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        PLAY_CLEAR_TITLES_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        PLAY_CLEAR_TITLES_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(PLAY_CLEAR_TITLES_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:clear_titles")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:clear_titles")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundClearTitlesPacket")
    );
    assert_eq!(
        oracle.answer.fixture.as_deref(),
        Some(
            "official ClientboundClearTitlesPacket(boolean) constructor fixture with resetTimes=true; context-free boolean body with no initialized client title state, screen, Level, or game state"
        )
    );
    assert_eq!(
        oracle.answer.official_body_shape.as_deref(),
        Some("resetTimes encoded as one FriendlyByteBuf boolean via ClientboundClearTitlesPacket.write(FriendlyByteBuf) and read by the private ClientboundClearTitlesPacket(FriendlyByteBuf) constructor")
    );
    assert_eq!(oracle.answer.input_reset_times, Some(true));
    assert_eq!(oracle.answer.stream_decoded_reset_times, Some(true));
    assert_eq!(oracle.answer.decoded_reset_times, Some(true));
    assert_eq!(oracle.answer.remaining_after_packet_stream_decode, Some(0));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:clear_titles",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_clear_titles clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let fixture_body = decode_hex(
        oracle
            .answer
            .fixture_body_hex
            .as_deref()
            .expect("play_clear_titles answer missing fixture_body_hex"),
        "fixture_body_hex",
    );
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body, fixture_body,
        "official clear_titles frame body must match the official STREAM_CODEC fixture body"
    );
    assert_eq!(
        body,
        vec![1],
        "official clear_titles resetTimes=true body must encode a single boolean true byte"
    );

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Play,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Play clientbound clear_titles packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!(
                "Stevenarella errored while decoding Play clientbound clear_titles packet: {err}"
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound clear_titles packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::PlayClearTitlesClientbound(clear_titles) => {
            assert!(
                clear_titles.reset_times,
                "decoded Play clientbound clear_titles fixture should preserve resetTimes=true"
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound clear_titles identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound clear_titles packet did not consume the official body bytes"
    );
}

#[test]
fn play_command_suggestions_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_command_suggestions_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_command_suggestions_clientbound_framed_dispatch_body)
        .expect("spawn play_command_suggestions_clientbound oracle stack")
        .join()
        .expect("play_command_suggestions_clientbound oracle thread panicked");
}

fn play_command_suggestions_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(PLAY_COMMAND_SUGGESTIONS_CLIENTBOUND_MANIFEST);
    assert_eq!(
        manifest.case_id,
        PLAY_COMMAND_SUGGESTIONS_CLIENTBOUND_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        PLAY_COMMAND_SUGGESTIONS_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        PLAY_COMMAND_SUGGESTIONS_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        PLAY_COMMAND_SUGGESTIONS_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        PLAY_COMMAND_SUGGESTIONS_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(PLAY_COMMAND_SUGGESTIONS_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:command_suggestions")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:command_suggestions")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundCommandSuggestionsPacket")
    );
    assert_eq!(
        oracle.answer.fixture.as_deref(),
        Some(
            "official ClientboundCommandSuggestionsPacket(int, Suggestions) constructor fixture with command id, StringRange.between(rangeStart, rangeEnd), and an empty suggestion list; context-free Brigadier suggestions body with no command tree, command context, Level, or game state"
        )
    );
    assert_eq!(
        oracle.answer.official_body_shape.as_deref(),
        Some("command id VarInt, range start VarInt, range length VarInt, then a VarInt suggestion count followed by Entry records; each Entry is text STRING_UTF8 plus optional trusted Component tooltip, and this fixture uses zero entries")
    );
    assert_eq!(oracle.answer.input_command_id, Some(123));
    assert_eq!(oracle.answer.stream_decoded_command_id, Some(123));
    assert_eq!(oracle.answer.decoded_command_id, Some(123));
    assert_eq!(oracle.answer.input_range_start, Some(1));
    assert_eq!(oracle.answer.stream_decoded_range_start, Some(1));
    assert_eq!(oracle.answer.decoded_range_start, Some(1));
    assert_eq!(oracle.answer.input_range_length, Some(3));
    assert_eq!(oracle.answer.stream_decoded_range_length, Some(3));
    assert_eq!(oracle.answer.decoded_range_length, Some(3));
    assert_eq!(oracle.answer.input_suggestion_count, Some(0));
    assert_eq!(oracle.answer.stream_decoded_suggestion_count, Some(0));
    assert_eq!(oracle.answer.decoded_suggestion_count, Some(0));
    assert_eq!(oracle.answer.decoded_to_suggestions_range_start, Some(1));
    assert_eq!(oracle.answer.decoded_to_suggestions_range_length, Some(3));
    assert_eq!(oracle.answer.decoded_to_suggestions_count, Some(0));
    assert_eq!(oracle.answer.remaining_after_packet_stream_decode, Some(0));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:command_suggestions",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_command_suggestions clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let fixture_body = decode_hex(
        oracle
            .answer
            .fixture_body_hex
            .as_deref()
            .expect("play_command_suggestions answer missing fixture_body_hex"),
        "fixture_body_hex",
    );
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    let mut expected_body = Vec::new();
    expected_body.extend_from_slice(&encode_varint(
        oracle
            .answer
            .input_command_id
            .expect("missing input_command_id"),
    ));
    expected_body.extend_from_slice(&encode_varint(
        oracle
            .answer
            .input_range_start
            .expect("missing input_range_start"),
    ));
    expected_body.extend_from_slice(&encode_varint(
        oracle
            .answer
            .input_range_length
            .expect("missing input_range_length"),
    ));
    expected_body.extend_from_slice(&encode_varint(
        oracle
            .answer
            .input_suggestion_count
            .expect("missing input_suggestion_count") as i32,
    ));

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body, fixture_body,
        "official command_suggestions frame body must match the official STREAM_CODEC fixture body"
    );
    assert_eq!(
        body, expected_body,
        "official command_suggestions empty fixture body should encode id/start/length/count as VarInts"
    );

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Play,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Play clientbound command_suggestions packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!(
                "Stevenarella errored while decoding Play clientbound command_suggestions packet: {err}"
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound command_suggestions packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::PlayCommandSuggestionsClientbound(command_suggestions) => {
            assert_eq!(
                command_suggestions.id.0,
                oracle
                    .answer
                    .decoded_command_id
                    .expect("missing decoded_command_id")
            );
            assert_eq!(
                command_suggestions.start.0,
                oracle
                    .answer
                    .decoded_range_start
                    .expect("missing decoded_range_start")
            );
            assert_eq!(
                command_suggestions.length.0,
                oracle
                    .answer
                    .decoded_range_length
                    .expect("missing decoded_range_length")
            );
            assert_eq!(
                command_suggestions.suggestion_count.0,
                oracle
                    .answer
                    .decoded_suggestion_count
                    .expect("missing decoded_suggestion_count") as i32
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound command_suggestions identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound command_suggestions packet did not consume the official body bytes"
    );
}

#[test]
fn play_commands_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_commands_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_commands_clientbound_framed_dispatch_body)
        .expect("spawn play_commands_clientbound oracle stack")
        .join()
        .expect("play_commands_clientbound oracle thread panicked");
}

fn play_commands_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(PLAY_COMMANDS_CLIENTBOUND_MANIFEST);
    assert_eq!(manifest.case_id, PLAY_COMMANDS_CLIENTBOUND_CASE_ID);
    assert_eq!(manifest.contract_path, PLAY_COMMANDS_CLIENTBOUND_CONTRACT);
    assert_eq!(manifest.answer_path, PLAY_COMMANDS_CLIENTBOUND_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(manifest.rust_test_name, PLAY_COMMANDS_CLIENTBOUND_TEST_NAME);
    assert_eq!(
        manifest.comparison_surface,
        PLAY_COMMANDS_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(PLAY_COMMANDS_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:commands")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:commands")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundCommandsPacket")
    );
    assert_eq!(
        oracle.answer.fixture.as_deref(),
        Some(
            "official ClientboundCommandsPacket(RootCommandNode<S>, NodeInspector<S>) constructor fixture with an empty Brigadier CommandDispatcher root; context-free root-only command tree with no argument nodes, command context, Level, or game state"
        )
    );
    assert_eq!(
        oracle.answer.official_body_shape.as_deref(),
        Some("VarInt node count, then each Entry as flags byte, VarInt child index array, optional redirect index, and node-specific payload, followed by root index VarInt; this root-only fixture has one root Entry with flags 0, zero children, no redirect, no stub payload, and root index 0")
    );
    assert_eq!(oracle.answer.input_root_child_count, Some(0));
    assert_eq!(oracle.answer.stream_decoded_entry_count, Some(1));
    assert_eq!(oracle.answer.decoded_entry_count, Some(1));
    assert_eq!(oracle.answer.stream_decoded_root_index, Some(0));
    assert_eq!(oracle.answer.decoded_root_index, Some(0));
    assert_eq!(oracle.answer.remaining_after_packet_stream_decode, Some(0));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:commands",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_commands clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let fixture_body = decode_hex(
        oracle
            .answer
            .fixture_body_hex
            .as_deref()
            .expect("play_commands answer missing fixture_body_hex"),
        "fixture_body_hex",
    );
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    let mut expected_body = Vec::new();
    expected_body.extend_from_slice(&encode_varint(
        oracle
            .answer
            .decoded_entry_count
            .expect("missing decoded_entry_count") as i32,
    ));
    expected_body.push(0);
    expected_body.extend_from_slice(&encode_varint(0));
    expected_body.extend_from_slice(&encode_varint(
        oracle
            .answer
            .decoded_root_index
            .expect("missing decoded_root_index"),
    ));

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body, fixture_body,
        "official commands frame body must match the official STREAM_CODEC fixture body"
    );
    assert_eq!(
        body, expected_body,
        "official commands root-only fixture body should encode one root entry and root index zero"
    );

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Play,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Play clientbound commands packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding Play clientbound commands packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound commands packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::DeclareCommands(commands) => {
            assert_eq!(
                commands.nodes.data.len(),
                oracle
                    .answer
                    .decoded_entry_count
                    .expect("missing decoded_entry_count")
            );
            assert_eq!(
                commands.root_index.0,
                oracle
                    .answer
                    .decoded_root_index
                    .expect("missing decoded_root_index")
            );
            let root = commands
                .nodes
                .data
                .first()
                .expect("official commands fixture should contain a root node");
            assert_eq!(root.flags, 0);
            assert!(
                root.children.data.is_empty(),
                "root-only commands fixture must not contain child nodes"
            );
            assert!(root.redirect_node.is_none());
            assert!(root.name.is_none());
            assert!(root.parser.is_none());
            assert!(root.properties.is_none());
            assert!(root.suggestions_type.is_none());
        }
        other => {
            panic!("decoded packet did not preserve Play clientbound commands identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound commands packet did not consume the official body bytes"
    );
}

#[test]
fn play_container_close_clientbound_framed_dispatch_matches_official_oracle_answer() {
    thread::Builder::new()
        .name("play_container_close_clientbound_oracle".to_owned())
        .stack_size(8 * 1024 * 1024)
        .spawn(play_container_close_clientbound_framed_dispatch_body)
        .expect("spawn play_container_close_clientbound oracle stack")
        .join()
        .expect("play_container_close_clientbound oracle thread panicked");
}

fn play_container_close_clientbound_framed_dispatch_body() {
    let manifest: TestManifest = read_json(PLAY_CONTAINER_CLOSE_CLIENTBOUND_MANIFEST);
    assert_eq!(manifest.case_id, PLAY_CONTAINER_CLOSE_CLIENTBOUND_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        PLAY_CONTAINER_CLOSE_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        PLAY_CONTAINER_CLOSE_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        PLAY_CONTAINER_CLOSE_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        PLAY_CONTAINER_CLOSE_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(PLAY_CONTAINER_CLOSE_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:container_close")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:container_close")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.game.ClientboundContainerClosePacket")
    );
    assert_eq!(
        oracle.answer.fixture.as_deref(),
        Some(
            "official ClientboundContainerClosePacket(int) constructor fixture with containerId from the case; context-free container id body with no initialized Menu, screen, Level, or game state"
        )
    );
    assert_eq!(
        oracle.answer.official_body_shape.as_deref(),
        Some("containerId encoded by ClientboundContainerClosePacket.write(FriendlyByteBuf) via FriendlyByteBuf.writeContainerId and read by the private ClientboundContainerClosePacket(FriendlyByteBuf) constructor via FriendlyByteBuf.readContainerId")
    );
    assert_eq!(oracle.answer.input_container_id, Some(7));
    assert_eq!(oracle.answer.stream_decoded_container_id, Some(7));
    assert_eq!(oracle.answer.decoded_container_id, Some(7));
    assert_eq!(oracle.answer.remaining_after_packet_stream_decode, Some(0));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.play_clientbound_packet_table,
        "minecraft:container_close",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("play_container_close clientbound answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let fixture_body = decode_hex(
        oracle
            .answer
            .fixture_body_hex
            .as_deref()
            .expect("play_container_close answer missing fixture_body_hex"),
        "fixture_body_hex",
    );
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body, fixture_body,
        "official container_close frame body must match the official STREAM_CODEC fixture body"
    );

    let mut body_slice = body.as_slice();
    let decoded_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::packet_by_id(
            775,
            State::Play,
            Direction::Clientbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Play clientbound container_close packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!(
                "Stevenarella errored while decoding Play clientbound container_close packet: {err}"
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Play clientbound container_close packet id {}",
                framed_packet_id
            )
        });

    match decoded {
        packet::Packet::WindowClose(close_window) => {
            assert_eq!(
                i32::from(close_window.id),
                oracle
                    .answer
                    .decoded_container_id
                    .expect("missing decoded_container_id")
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve Play clientbound container_close identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded Play clientbound container_close packet did not consume the official body bytes"
    );
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
fn configuration_keepalive_runtime_send_helper_sends_official_configuration_frame() {
    let manifest: TestManifest = read_json(CONFIGURATION_KEEPALIVE_RUNTIME_SEND_MANIFEST);
    assert_eq!(
        manifest.case_id,
        CONFIGURATION_KEEPALIVE_RUNTIME_SEND_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_KEEPALIVE_RUNTIME_SEND_CONTRACT
    );
    assert_eq!(manifest.answer_path, CONFIGURATION_KEEPALIVE_FRAMED_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_KEEPALIVE_RUNTIME_SEND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_KEEPALIVE_RUNTIME_SEND_COMPARISON_SURFACE
    );
    assert_runner_scope(CONFIGURATION_KEEPALIVE_RUNTIME_SEND_MANIFEST, &manifest);

    let oracle = read_answer(
        &manifest.answer_path,
        CONFIGURATION_KEEPALIVE_FRAMED_CASE_ID,
    );
    assert_eq!(oracle.case_id, CONFIGURATION_KEEPALIVE_FRAMED_CASE_ID);
    assert_eq!(oracle.answer.decoded_id, Some(oracle.answer.input_id));
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:keep_alive")
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let official_framed = decode_hex(
        oracle
            .answer
            .encoded_framed_hex
            .as_deref()
            .expect("framed dispatch answer missing encoded_framed_hex"),
        "configuration_keepalive_framed_dispatch.encoded_framed_hex",
    );
    let expected_network_frame = official_network_frame_from_framed_payload(&official_framed);

    let listener = TcpListener::bind("127.0.0.1:0").expect("bind localhost probe server");
    let server_addr = listener.local_addr().expect("read localhost probe addr");
    let mut server = Some(thread::spawn(move || -> Result<Vec<u8>, String> {
        let (mut stream, _) = listener
            .accept()
            .map_err(|err| format!("accept runtime send probe client: {err}"))?;
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .map_err(|err| format!("set runtime send probe read timeout: {err}"))?;
        let packet_len = try_read_varint_from_reader(&mut stream)? as usize;
        let mut body = vec![0; packet_len];
        stream
            .read_exact(&mut body)
            .map_err(|err| format!("read runtime send probe packet body: {err}"))?;

        let mut observed = encode_varint(packet_len as i32);
        observed.extend_from_slice(&body);
        Ok(observed)
    }));

    let mut conn = Conn::new(&server_addr.to_string(), 775).expect("connect runtime send probe");
    conn.state = State::Configuration;
    let send_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        packet::send_keep_alive(&mut conn, oracle.answer.input_id)
    }));
    match send_result {
        Ok(Ok(())) => {}
        Ok(Err(err)) => {
            drop(conn);
            if let Some(server) = server.take() {
                let _ = server.join();
            }
            panic!("packet::send_keep_alive returned an error in Configuration state: {err}");
        }
        Err(_) => {
            drop(conn);
            if let Some(server) = server.take() {
                let _ = server.join();
            }
            panic!(
                "packet::send_keep_alive panicked in Configuration state before sending the official Protocol 775 Configuration serverbound keep_alive frame"
            );
        }
    }

    let observed_network_frame = server
        .take()
        .expect("runtime send probe server was already joined")
        .join()
        .expect("runtime send probe server thread panicked")
        .expect("runtime send probe server did not observe a complete packet");

    assert_eq!(
        hex::encode(&observed_network_frame),
        hex::encode(&expected_network_frame),
        "packet::send_keep_alive in Configuration state must send the official Protocol 775 Configuration serverbound keep_alive framed packet with the outer network length prefix"
    );
}

#[test]
fn configuration_keepalive_runtime_protocol_echo_reads_maps_and_sends_official_frame() {
    let manifest: TestManifest = read_json(CONFIGURATION_KEEPALIVE_RUNTIME_PROTOCOL_ECHO_MANIFEST);
    assert_eq!(
        manifest.case_id,
        CONFIGURATION_KEEPALIVE_RUNTIME_PROTOCOL_ECHO_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_KEEPALIVE_RUNTIME_PROTOCOL_ECHO_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_KEEPALIVE_CLIENTBOUND_FRAMED_ANSWER
    );
    assert_eq!(
        manifest.response_answer_path.as_deref(),
        Some(CONFIGURATION_KEEPALIVE_FRAMED_ANSWER)
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_KEEPALIVE_RUNTIME_PROTOCOL_ECHO_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_KEEPALIVE_RUNTIME_PROTOCOL_ECHO_COMPARISON_SURFACE
    );
    assert_runner_scope(
        CONFIGURATION_KEEPALIVE_RUNTIME_PROTOCOL_ECHO_MANIFEST,
        &manifest,
    );

    let inbound_oracle = read_answer(
        &manifest.answer_path,
        CONFIGURATION_KEEPALIVE_CLIENTBOUND_FRAMED_CASE_ID,
    );
    let outbound_answer_path = manifest
        .response_answer_path
        .as_deref()
        .expect("runtime protocol echo manifest missing response_answer_path");
    let outbound_oracle = read_answer(outbound_answer_path, CONFIGURATION_KEEPALIVE_FRAMED_CASE_ID);

    assert_eq!(
        inbound_oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:keep_alive")
    );
    assert_eq!(
        inbound_oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundKeepAlivePacket")
    );
    assert_eq!(
        inbound_oracle.answer.remaining_after_official_decode,
        Some(0)
    );
    assert_eq!(
        outbound_oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:keep_alive")
    );
    assert_eq!(
        outbound_oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ServerboundKeepAlivePacket")
    );
    assert_eq!(
        outbound_oracle.answer.remaining_after_official_decode,
        Some(0)
    );
    assert_eq!(
        inbound_oracle.answer.input_id, outbound_oracle.answer.input_id,
        "runtime echo fixture requires the inbound and outbound official answers to use the same keep_alive id"
    );

    let inbound_framed = decode_hex(
        inbound_oracle
            .answer
            .encoded_framed_hex
            .as_deref()
            .expect("clientbound framed dispatch answer missing encoded_framed_hex"),
        "configuration_keepalive_clientbound_framed_dispatch.encoded_framed_hex",
    );
    let outbound_framed = decode_hex(
        outbound_oracle
            .answer
            .encoded_framed_hex
            .as_deref()
            .expect("serverbound framed dispatch answer missing encoded_framed_hex"),
        "configuration_keepalive_framed_dispatch.encoded_framed_hex",
    );
    let inbound_network_frame = official_network_frame_from_framed_payload(&inbound_framed);
    let expected_outbound_network_frame =
        official_network_frame_from_framed_payload(&outbound_framed);

    let listener = TcpListener::bind("127.0.0.1:0").expect("bind localhost echo probe server");
    let server_addr = listener
        .local_addr()
        .expect("read localhost echo probe addr");
    let mut server = Some(thread::spawn(move || -> Result<Vec<u8>, String> {
        let (mut stream, _) = listener
            .accept()
            .map_err(|err| format!("accept runtime protocol echo probe client: {err}"))?;
        stream
            .set_read_timeout(Some(Duration::from_secs(2)))
            .map_err(|err| format!("set runtime protocol echo probe read timeout: {err}"))?;
        stream
            .set_write_timeout(Some(Duration::from_secs(2)))
            .map_err(|err| format!("set runtime protocol echo probe write timeout: {err}"))?;
        stream
            .write_all(&inbound_network_frame)
            .map_err(|err| format!("write official clientbound keep_alive frame: {err}"))?;

        read_network_frame_from_reader(&mut stream, "runtime protocol echo response")
    }));

    let mut conn = Conn::new(&server_addr.to_string(), 775).expect("connect protocol echo probe");
    conn.state = State::Configuration;
    let mapped = conn
        .read_packet()
        .expect("read official Configuration clientbound keep_alive frame")
        .map();
    let keep_alive_id = match mapped {
        steven_protocol::protocol::mapped_packet::MappedPacket::KeepAliveClientbound(
            keep_alive,
        ) => keep_alive.id,
        other => panic!("expected mapped clientbound keep_alive packet, got {other:?}"),
    };
    assert_eq!(keep_alive_id, inbound_oracle.answer.input_id);

    packet::send_keep_alive(&mut conn, keep_alive_id)
        .expect("send official Configuration serverbound keep_alive response");

    let observed_outbound_network_frame = server
        .take()
        .expect("runtime protocol echo probe server was already joined")
        .join()
        .expect("runtime protocol echo probe server thread panicked")
        .expect("runtime protocol echo probe server did not observe a complete response packet");

    assert_eq!(
        hex::encode(&observed_outbound_network_frame),
        hex::encode(&expected_outbound_network_frame),
        "runtime protocol echo path must map the official Configuration clientbound keep_alive id and send the official Configuration serverbound keep_alive framed packet with the outer network length prefix"
    );
}

#[test]
fn configuration_keepalive_runtime_spawn_reader_reaction_echoes_official_frame() {
    let manifest: TestManifest = read_json(CONFIGURATION_KEEPALIVE_RUNTIME_SPAWN_READER_MANIFEST);
    assert_eq!(
        manifest.case_id,
        CONFIGURATION_KEEPALIVE_RUNTIME_SPAWN_READER_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_KEEPALIVE_RUNTIME_SPAWN_READER_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_KEEPALIVE_CLIENTBOUND_FRAMED_ANSWER
    );
    assert_eq!(
        manifest.response_answer_path.as_deref(),
        Some(CONFIGURATION_KEEPALIVE_FRAMED_ANSWER)
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_KEEPALIVE_RUNTIME_SPAWN_READER_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_KEEPALIVE_RUNTIME_SPAWN_READER_COMPARISON_SURFACE
    );
    assert_runner_scope(
        CONFIGURATION_KEEPALIVE_RUNTIME_SPAWN_READER_MANIFEST,
        &manifest,
    );

    let inbound_oracle = read_answer(
        &manifest.answer_path,
        CONFIGURATION_KEEPALIVE_CLIENTBOUND_FRAMED_CASE_ID,
    );
    let outbound_answer_path = manifest
        .response_answer_path
        .as_deref()
        .expect("runtime spawn_reader manifest missing response_answer_path");
    let outbound_oracle = read_answer(outbound_answer_path, CONFIGURATION_KEEPALIVE_FRAMED_CASE_ID);

    assert_eq!(
        inbound_oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:keep_alive")
    );
    assert_eq!(
        inbound_oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundKeepAlivePacket")
    );
    assert_eq!(
        inbound_oracle.answer.remaining_after_official_decode,
        Some(0)
    );
    assert_eq!(
        outbound_oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:keep_alive")
    );
    assert_eq!(
        outbound_oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ServerboundKeepAlivePacket")
    );
    assert_eq!(
        outbound_oracle.answer.remaining_after_official_decode,
        Some(0)
    );
    assert_eq!(
        inbound_oracle.answer.input_id, outbound_oracle.answer.input_id,
        "spawn_reader fixture requires the inbound and outbound official answers to use the same keep_alive id"
    );

    let inbound_framed = decode_hex(
        inbound_oracle
            .answer
            .encoded_framed_hex
            .as_deref()
            .expect("clientbound framed dispatch answer missing encoded_framed_hex"),
        "configuration_keepalive_clientbound_framed_dispatch.encoded_framed_hex",
    );
    let outbound_framed = decode_hex(
        outbound_oracle
            .answer
            .encoded_framed_hex
            .as_deref()
            .expect("serverbound framed dispatch answer missing encoded_framed_hex"),
        "configuration_keepalive_framed_dispatch.encoded_framed_hex",
    );
    let _inbound_network_frame = official_network_frame_from_framed_payload(&inbound_framed);
    let _expected_outbound_network_frame =
        official_network_frame_from_framed_payload(&outbound_framed);

    let server_source_path = project_root().join("stevenarella/src/server/mod.rs");
    let server_source = fs::read_to_string(&server_source_path).unwrap_or_else(|err| {
        panic!(
            "failed to read spawn_reader owner source {}: {err}",
            server_source_path.display()
        )
    });
    assert!(
        server_source.contains("fn spawn_reader(mut read: protocol::Conn"),
        "expected current spawn_reader owner symbol in {}",
        server_source_path.display()
    );
    assert!(
        server_source.contains("MappedPacket::KeepAliveClientbound(keep_alive)"),
        "expected current spawn_reader keep_alive reaction branch in {}",
        server_source_path.display()
    );
    assert!(
        server_source.contains("pub fn handle_next_reader_packet_for_oracle"),
        "expected narrow oracle reader-loop helper in {}",
        server_source_path.display()
    );
    assert!(
        server_source.contains("conn.state = protocol::State::Play;"),
        "expected current public Server::connect state transition evidence in {}",
        server_source_path.display()
    );

    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let output = Command::new(cargo)
        .current_dir(project_root())
        .args([
            "test",
            "--manifest-path",
            "stevenarella/Cargo.toml",
            "server::tests::configuration_keepalive_runtime_spawn_reader_reaction_echoes_official_frame",
            "--",
            "--exact",
        ])
        .output()
        .expect("run stevenarella spawn_reader oracle probe");

    assert!(
        output.status.success(),
        "stevenarella spawn_reader oracle probe failed\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
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
fn configuration_cookie_request_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_COOKIE_REQUEST_MANIFEST);
    assert_eq!(manifest.case_id, CONFIGURATION_COOKIE_REQUEST_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_COOKIE_REQUEST_CONTRACT
    );
    assert_eq!(manifest.answer_path, CONFIGURATION_COOKIE_REQUEST_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_COOKIE_REQUEST_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_COOKIE_REQUEST_COMPARISON_SURFACE
    );
    assert_runner_scope(CONFIGURATION_COOKIE_REQUEST_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:cookie_request")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:cookie_request")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.cookie.ClientboundCookieRequestPacket")
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));
    assert_eq!(
        oracle.answer.input_key, oracle.answer.decoded_key,
        "official decoded cookie_request key differs from the official input key"
    );

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:cookie_request",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("cookie_request answer missing encoded_framed_hex");
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
            "Stevenarella panicked while dispatching official Configuration clientbound cookie_request packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding cookie_request packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound cookie_request packet id {}",
                framed_packet_id
            )
        });
    match decoded {
        packet::Packet::PluginMessageClientbound(packet) => {
            assert_eq!(
                packet.channel, "CookieRequest",
                "decoded packet did not preserve cookie_request compatibility channel"
            );
            assert!(
                packet.data.is_empty(),
                "decoded cookie_request compatibility packet carried unexpected data"
            );
        }
        other => panic!("decoded packet did not preserve cookie_request identity: {other:?}"),
    }
    assert!(
        body_slice.is_empty(),
        "decoded cookie_request packet did not consume the official body bytes"
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

#[test]
fn configuration_ping_pong_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_PING_PONG_MANIFEST);
    assert_eq!(manifest.case_id, CONFIGURATION_PING_PONG_CASE_ID);
    assert_eq!(manifest.contract_path, CONFIGURATION_PING_PONG_CONTRACT);
    assert_eq!(manifest.answer_path, CONFIGURATION_PING_PONG_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(manifest.rust_test_name, CONFIGURATION_PING_PONG_TEST_NAME);
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_PING_PONG_COMPARISON_SURFACE
    );
    assert_runner_scope(CONFIGURATION_PING_PONG_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);

    let clientbound_ping = oracle
        .answer
        .clientbound_ping
        .as_ref()
        .expect("ping/pong answer missing clientbound_ping direction");
    assert_ping_pong_direction_matches_official_frame(clientbound_ping, Direction::Clientbound);

    let serverbound_pong = oracle
        .answer
        .serverbound_pong
        .as_ref()
        .expect("ping/pong answer missing serverbound_pong direction");
    assert_ping_pong_direction_matches_official_frame(serverbound_pong, Direction::Serverbound);
}

#[test]
fn configuration_client_information_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_CLIENT_INFORMATION_MANIFEST);
    assert_eq!(manifest.case_id, CONFIGURATION_CLIENT_INFORMATION_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_CLIENT_INFORMATION_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_CLIENT_INFORMATION_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_CLIENT_INFORMATION_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_CLIENT_INFORMATION_COMPARISON_SURFACE
    );
    assert_runner_scope(CONFIGURATION_CLIENT_INFORMATION_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:client_information")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:client_information")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ServerboundClientInformationPacket")
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));
    assert_eq!(
        oracle.answer.input_information, oracle.answer.decoded_information,
        "official decoded ClientInformation record differs from the official input record"
    );

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_serverbound_packet_table,
        "minecraft:client_information",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("client_information answer missing encoded_framed_hex");
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
            Direction::Serverbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Configuration serverbound client_information packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding client_information packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration serverbound client_information packet id {}",
                framed_packet_id
            )
        });
    let decoded_debug = format!("{decoded:?}");
    assert!(
        decoded_debug.contains("ClientInformation"),
        "decoded packet did not preserve client_information identity: {decoded_debug}"
    );
    assert!(
        body_slice.is_empty(),
        "decoded client_information packet did not consume the official body bytes"
    );
}

#[test]
fn configuration_cookie_response_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_COOKIE_RESPONSE_MANIFEST);
    assert_eq!(manifest.case_id, CONFIGURATION_COOKIE_RESPONSE_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_COOKIE_RESPONSE_CONTRACT
    );
    assert_eq!(manifest.answer_path, CONFIGURATION_COOKIE_RESPONSE_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_COOKIE_RESPONSE_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_COOKIE_RESPONSE_COMPARISON_SURFACE
    );
    assert_runner_scope(CONFIGURATION_COOKIE_RESPONSE_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:cookie_response")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:cookie_response")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.cookie.ServerboundCookieResponsePacket")
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));
    assert_eq!(
        oracle.answer.input_key, oracle.answer.decoded_key,
        "official decoded cookie_response key differs from the official input key"
    );
    assert_eq!(
        oracle.answer.input_payload_hex, oracle.answer.decoded_payload_hex,
        "official decoded cookie_response payload differs from the official input payload"
    );
    assert_eq!(
        oracle.answer.input_payload_length, oracle.answer.decoded_payload_length,
        "official decoded cookie_response payload length differs from the official input payload"
    );
    assert_eq!(
        oracle.answer.decoded_payload_equals_input,
        Some(true),
        "official decoded cookie_response payload was not byte-equal to the input payload"
    );

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_serverbound_packet_table,
        "minecraft:cookie_response",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("cookie_response answer missing encoded_framed_hex");
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
            Direction::Serverbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Configuration serverbound cookie_response packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding cookie_response packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration serverbound cookie_response packet id {}",
                framed_packet_id
            )
        });
    let expected_payload_hex = oracle
        .answer
        .decoded_payload_hex
        .as_deref()
        .expect("cookie_response answer missing decoded_payload_hex");
    let expected_payload = decode_hex(expected_payload_hex, "decoded_payload_hex");
    match decoded {
        packet::Packet::PluginMessageServerbound(packet) => {
            assert_eq!(
                packet.channel, "CookieResponse",
                "decoded packet did not preserve cookie_response compatibility channel"
            );
            assert_eq!(
                packet.data, expected_payload,
                "decoded cookie_response compatibility packet did not preserve payload bytes"
            );
        }
        other => panic!("decoded packet did not preserve cookie_response identity: {other:?}"),
    }
    assert!(
        body_slice.is_empty(),
        "decoded cookie_response packet did not consume the official body bytes"
    );
}

#[test]
fn configuration_custom_payload_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_CUSTOM_PAYLOAD_MANIFEST);
    assert_eq!(manifest.case_id, CONFIGURATION_CUSTOM_PAYLOAD_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_CUSTOM_PAYLOAD_CONTRACT
    );
    assert_eq!(manifest.answer_path, CONFIGURATION_CUSTOM_PAYLOAD_ANSWER);
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_CUSTOM_PAYLOAD_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_CUSTOM_PAYLOAD_COMPARISON_SURFACE
    );
    assert_runner_scope(CONFIGURATION_CUSTOM_PAYLOAD_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:custom_payload")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:custom_payload")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ServerboundCustomPayloadPacket")
    );
    assert_eq!(
        oracle.answer.input_payload_class.as_deref(),
        Some("net.minecraft.network.protocol.common.custom.BrandPayload")
    );
    assert_eq!(
        oracle.answer.decoded_payload_class.as_deref(),
        Some("net.minecraft.network.protocol.common.custom.BrandPayload")
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));
    assert_eq!(
        oracle.answer.input_custom_payload_id, oracle.answer.decoded_custom_payload_id,
        "official decoded custom_payload id differs from the official input payload id"
    );
    assert_eq!(
        oracle.answer.input_brand, oracle.answer.decoded_brand,
        "official decoded custom_payload brand differs from the official input brand"
    );

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_serverbound_packet_table,
        "minecraft:custom_payload",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("custom_payload answer missing encoded_framed_hex");
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
            Direction::Serverbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Configuration serverbound custom_payload packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding custom_payload packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration serverbound custom_payload packet id {}",
                framed_packet_id
            )
        });
    let expected_channel = oracle
        .answer
        .decoded_custom_payload_id
        .as_deref()
        .expect("custom_payload answer missing decoded_custom_payload_id");
    let expected_payload_hex = oracle
        .answer
        .encoded_payload_body_hex
        .as_deref()
        .expect("custom_payload answer missing encoded_payload_body_hex");
    let expected_payload = decode_hex(expected_payload_hex, "encoded_payload_body_hex");
    match decoded {
        packet::Packet::PluginMessageServerbound(packet) => {
            assert_eq!(
                packet.channel, expected_channel,
                "decoded packet did not preserve custom_payload channel"
            );
            assert_eq!(
                packet.data, expected_payload,
                "decoded custom_payload compatibility packet did not preserve payload bytes"
            );
        }
        other => panic!("decoded packet did not preserve custom_payload identity: {other:?}"),
    }
    assert!(
        body_slice.is_empty(),
        "decoded custom_payload packet did not consume the official body bytes"
    );
}

#[test]
fn configuration_custom_payload_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_CUSTOM_PAYLOAD_CLIENTBOUND_MANIFEST);
    assert_eq!(
        manifest.case_id,
        CONFIGURATION_CUSTOM_PAYLOAD_CLIENTBOUND_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_CUSTOM_PAYLOAD_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_CUSTOM_PAYLOAD_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_CUSTOM_PAYLOAD_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_CUSTOM_PAYLOAD_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(CONFIGURATION_CUSTOM_PAYLOAD_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:custom_payload")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:custom_payload")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundCustomPayloadPacket")
    );
    assert_eq!(
        oracle.answer.input_payload_class.as_deref(),
        Some("net.minecraft.network.protocol.common.custom.BrandPayload")
    );
    assert_eq!(
        oracle.answer.decoded_payload_class.as_deref(),
        Some("net.minecraft.network.protocol.common.custom.BrandPayload")
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));
    assert_eq!(
        oracle.answer.input_custom_payload_id, oracle.answer.decoded_custom_payload_id,
        "official decoded clientbound custom_payload id differs from the official input payload id"
    );
    assert_eq!(
        oracle.answer.input_brand, oracle.answer.decoded_brand,
        "official decoded clientbound custom_payload brand differs from the official input brand"
    );

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:custom_payload",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("clientbound custom_payload answer missing encoded_framed_hex");
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
            "Stevenarella panicked while dispatching official Configuration clientbound custom_payload packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding clientbound custom_payload packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound custom_payload packet id {}",
                framed_packet_id
            )
        });
    let expected_channel = oracle
        .answer
        .decoded_custom_payload_id
        .as_deref()
        .expect("clientbound custom_payload answer missing decoded_custom_payload_id");
    let expected_payload_hex = oracle
        .answer
        .encoded_payload_body_hex
        .as_deref()
        .expect("clientbound custom_payload answer missing encoded_payload_body_hex");
    let expected_payload = decode_hex(expected_payload_hex, "encoded_payload_body_hex");
    match decoded {
        packet::Packet::PluginMessageClientbound(packet) => {
            assert_eq!(
                packet.channel, expected_channel,
                "decoded clientbound packet did not preserve custom_payload channel"
            );
            assert_eq!(
                packet.data, expected_payload,
                "decoded clientbound custom_payload compatibility packet did not preserve payload bytes"
            );
        }
        other => {
            panic!("decoded packet did not preserve clientbound custom_payload identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded clientbound custom_payload packet did not consume the official body bytes"
    );
}

#[test]
fn configuration_disconnect_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_DISCONNECT_CLIENTBOUND_MANIFEST);
    assert_eq!(
        manifest.case_id,
        CONFIGURATION_DISCONNECT_CLIENTBOUND_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_DISCONNECT_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_DISCONNECT_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_DISCONNECT_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_DISCONNECT_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(CONFIGURATION_DISCONNECT_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:disconnect")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:disconnect")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundDisconnectPacket")
    );
    assert_eq!(
        oracle.answer.reason_fixture.as_deref(),
        Some("Component.literal(\"\")")
    );
    assert_eq!(
        oracle.answer.input_reason_text, oracle.answer.decoded_reason_text,
        "official decoded disconnect reason text differs from the official input reason text"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:disconnect",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("clientbound disconnect answer missing encoded_framed_hex");
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
            "Stevenarella panicked while dispatching official Configuration clientbound disconnect packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding clientbound disconnect packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound disconnect packet id {}",
                framed_packet_id
            )
        });
    let expected_reason_text = oracle
        .answer
        .decoded_reason_text
        .as_deref()
        .expect("clientbound disconnect answer missing decoded_reason_text");
    match decoded {
        packet::Packet::Disconnect(packet) => {
            assert_eq!(
                packet.reason.to_string(),
                expected_reason_text,
                "decoded clientbound disconnect reason text did not match the official reason text"
            );
        }
        other => {
            panic!("decoded packet did not preserve clientbound disconnect identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded clientbound disconnect packet did not consume the official body bytes"
    );
}

#[test]
fn configuration_reset_chat_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_RESET_CHAT_CLIENTBOUND_MANIFEST);
    assert_eq!(
        manifest.case_id,
        CONFIGURATION_RESET_CHAT_CLIENTBOUND_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_RESET_CHAT_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_RESET_CHAT_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_RESET_CHAT_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_RESET_CHAT_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(CONFIGURATION_RESET_CHAT_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:reset_chat")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:reset_chat")
    );
    assert_eq!(
        oracle.answer.instance_packet_type.as_deref(),
        Some("minecraft:reset_chat")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.configuration.ClientboundResetChatPacket")
    );
    assert_eq!(oracle.answer.decoded_equals_instance, Some(true));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:reset_chat",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("clientbound reset_chat answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert!(
        body.is_empty(),
        "official reset_chat singleton body should be empty because ClientboundResetChatPacket.STREAM_CODEC is StreamCodec.unit(INSTANCE)"
    );

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
            "Stevenarella panicked while dispatching official Configuration clientbound reset_chat packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding clientbound reset_chat packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound reset_chat packet id {}",
                framed_packet_id
            )
        });
    match decoded {
        packet::Packet::PluginMessageClientbound(packet) => {
            assert_eq!(
                packet.channel, "ResetChat",
                "decoded packet did not preserve reset_chat compatibility channel"
            );
            assert!(
                packet.data.is_empty(),
                "decoded reset_chat compatibility packet carried unexpected data"
            );
        }
        other => {
            panic!("decoded packet did not preserve clientbound reset_chat identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded clientbound reset_chat packet did not consume the official empty body"
    );
}

#[test]
fn configuration_registry_data_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_REGISTRY_DATA_CLIENTBOUND_MANIFEST);
    assert_eq!(
        manifest.case_id,
        CONFIGURATION_REGISTRY_DATA_CLIENTBOUND_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_REGISTRY_DATA_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_REGISTRY_DATA_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_REGISTRY_DATA_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_REGISTRY_DATA_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(CONFIGURATION_REGISTRY_DATA_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:registry_data")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:registry_data")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.configuration.ClientboundRegistryDataPacket")
    );
    assert_eq!(
        oracle.answer.registry_fixture.as_deref(),
        Some("Registries.DIMENSION_TYPE with List.of() entries")
    );
    assert_eq!(
        oracle.answer.input_registry_key.as_deref(),
        Some("minecraft:dimension_type")
    );
    assert_eq!(
        oracle.answer.input_registry_key, oracle.answer.decoded_registry_key,
        "official decoded registry key differs from the official input registry key"
    );
    assert_eq!(
        oracle.answer.input_entry_count,
        Some(0),
        "registry_data fixture must not invent registry entries"
    );
    assert_eq!(
        oracle.answer.input_entry_count, oracle.answer.decoded_entry_count,
        "official decoded registry entry count differs from the official input entry count"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:registry_data",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("clientbound registry_data answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert!(
        !body.is_empty(),
        "official registry_data body should include registry key and empty entry-list length"
    );

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
            "Stevenarella panicked while dispatching official Configuration clientbound registry_data packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding clientbound registry_data packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound registry_data packet id {}",
                framed_packet_id
            )
        });
    match decoded {
        packet::Packet::PluginMessageClientbound(packet) => {
            assert_eq!(
                packet.channel, "RegistryData",
                "decoded packet did not preserve registry_data compatibility channel"
            );
            assert!(
                packet.data.is_empty(),
                "decoded registry_data compatibility packet carried unexpected data"
            );
        }
        other => {
            panic!("decoded packet did not preserve clientbound registry_data identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded clientbound registry_data packet did not consume the official body bytes"
    );
}

#[test]
fn configuration_resource_pack_pop_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_RESOURCE_PACK_POP_CLIENTBOUND_MANIFEST);
    assert_eq!(
        manifest.case_id,
        CONFIGURATION_RESOURCE_PACK_POP_CLIENTBOUND_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_RESOURCE_PACK_POP_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_RESOURCE_PACK_POP_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_RESOURCE_PACK_POP_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_RESOURCE_PACK_POP_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(
        CONFIGURATION_RESOURCE_PACK_POP_CLIENTBOUND_MANIFEST,
        &manifest,
    );

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:resource_pack_pop")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:resource_pack_pop")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundResourcePackPopPacket")
    );
    assert_eq!(oracle.answer.input_uuid_present, Some(true));
    assert_eq!(
        oracle.answer.input_uuid_present, oracle.answer.decoded_uuid_present,
        "official decoded resource_pack_pop UUID presence differs from the official input UUID presence"
    );
    assert_eq!(
        oracle.answer.input_uuid, oracle.answer.decoded_uuid,
        "official decoded resource_pack_pop UUID differs from the official input UUID"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:resource_pack_pop",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("resource_pack_pop answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert!(
        !body.is_empty(),
        "official resource_pack_pop body should include optional UUID presence and UUID bytes"
    );

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
            "Stevenarella panicked while dispatching official Configuration clientbound resource_pack_pop packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding clientbound resource_pack_pop packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound resource_pack_pop packet id {}",
                framed_packet_id
            )
        });
    match decoded {
        packet::Packet::PluginMessageClientbound(packet) => {
            assert_eq!(
                packet.channel, "ResourcePackPop",
                "decoded packet did not preserve resource_pack_pop compatibility channel"
            );
            assert!(
                packet.data.is_empty(),
                "decoded resource_pack_pop compatibility packet carried unexpected data"
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve clientbound resource_pack_pop identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded clientbound resource_pack_pop packet did not consume the official body bytes"
    );
}

#[test]
fn configuration_resource_pack_push_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_RESOURCE_PACK_PUSH_CLIENTBOUND_MANIFEST);
    assert_eq!(
        manifest.case_id,
        CONFIGURATION_RESOURCE_PACK_PUSH_CLIENTBOUND_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_RESOURCE_PACK_PUSH_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_RESOURCE_PACK_PUSH_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_RESOURCE_PACK_PUSH_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_RESOURCE_PACK_PUSH_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(
        CONFIGURATION_RESOURCE_PACK_PUSH_CLIENTBOUND_MANIFEST,
        &manifest,
    );

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:resource_pack_push")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:resource_pack_push")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundResourcePackPushPacket")
    );
    assert_eq!(
        oracle.answer.input_uuid, oracle.answer.decoded_uuid,
        "official decoded resource_pack_push UUID differs from the official input UUID"
    );
    assert_eq!(
        oracle.answer.input_url, oracle.answer.decoded_url,
        "official decoded resource_pack_push URL differs from the official input URL"
    );
    assert_eq!(
        oracle.answer.input_hash, oracle.answer.decoded_hash,
        "official decoded resource_pack_push hash differs from the official input hash"
    );
    assert_eq!(
        oracle.answer.input_required, oracle.answer.decoded_required,
        "official decoded resource_pack_push required flag differs from the official input required flag"
    );
    assert_eq!(oracle.answer.input_prompt_present, Some(false));
    assert_eq!(
        oracle.answer.input_prompt_present, oracle.answer.decoded_prompt_present,
        "official decoded resource_pack_push prompt presence differs from the official input prompt presence"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:resource_pack_push",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("resource_pack_push answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert!(
        !body.is_empty(),
        "official resource_pack_push body should include UUID, URL, hash, required flag, and prompt presence"
    );

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
            "Stevenarella panicked while dispatching official Configuration clientbound resource_pack_push packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding clientbound resource_pack_push packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound resource_pack_push packet id {}",
                framed_packet_id
            )
        });
    match decoded {
        packet::Packet::PluginMessageClientbound(packet) => {
            assert_eq!(
                packet.channel, "ResourcePackPush",
                "decoded packet did not preserve resource_pack_push compatibility channel"
            );
            assert!(
                packet.data.is_empty(),
                "decoded resource_pack_push compatibility packet carried unexpected data"
            );
        }
        other => {
            panic!("decoded packet did not preserve clientbound resource_pack_push identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded clientbound resource_pack_push packet did not consume the official body bytes"
    );
}

#[test]
fn configuration_store_cookie_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_STORE_COOKIE_CLIENTBOUND_MANIFEST);
    assert_eq!(
        manifest.case_id,
        CONFIGURATION_STORE_COOKIE_CLIENTBOUND_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_STORE_COOKIE_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_STORE_COOKIE_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_STORE_COOKIE_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_STORE_COOKIE_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(CONFIGURATION_STORE_COOKIE_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:store_cookie")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:store_cookie")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundStoreCookiePacket")
    );
    assert_eq!(
        oracle.answer.input_key, oracle.answer.decoded_key,
        "official decoded store_cookie key differs from the official input key"
    );
    assert_eq!(
        oracle.answer.input_payload_hex, oracle.answer.decoded_payload_hex,
        "official decoded store_cookie payload differs from the official input payload"
    );
    assert_eq!(
        oracle.answer.input_payload_length, oracle.answer.decoded_payload_length,
        "official decoded store_cookie payload length differs from the official input payload length"
    );
    assert_eq!(oracle.answer.decoded_payload_equals_input, Some(true));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:store_cookie",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("store_cookie answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert!(
        !body.is_empty(),
        "official store_cookie body should include Identifier key and byte-array payload"
    );

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
            "Stevenarella panicked while dispatching official Configuration clientbound store_cookie packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding clientbound store_cookie packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound store_cookie packet id {}",
                framed_packet_id
            )
        });
    match decoded {
        packet::Packet::PluginMessageClientbound(packet) => {
            assert_eq!(
                packet.channel, "StoreCookie",
                "decoded packet did not preserve store_cookie compatibility channel"
            );
            let expected_payload_hex = oracle
                .answer
                .decoded_payload_hex
                .as_deref()
                .expect("store_cookie answer missing decoded_payload_hex");
            let expected_payload = decode_hex(expected_payload_hex, "decoded_payload_hex");
            assert_eq!(
                packet.data, expected_payload,
                "decoded store_cookie compatibility packet carried unexpected payload"
            );
        }
        other => {
            panic!("decoded packet did not preserve clientbound store_cookie identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded clientbound store_cookie packet did not consume the official body bytes"
    );
}

#[test]
fn configuration_transfer_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_TRANSFER_CLIENTBOUND_MANIFEST);
    assert_eq!(manifest.case_id, CONFIGURATION_TRANSFER_CLIENTBOUND_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_TRANSFER_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_TRANSFER_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_TRANSFER_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_TRANSFER_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(CONFIGURATION_TRANSFER_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:transfer")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:transfer")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundTransferPacket")
    );
    assert_eq!(
        oracle.answer.input_host, oracle.answer.decoded_host,
        "official decoded transfer host differs from the official input host"
    );
    assert_eq!(
        oracle.answer.input_port, oracle.answer.decoded_port,
        "official decoded transfer port differs from the official input port"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:transfer",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("transfer answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert!(
        !body.is_empty(),
        "official transfer body should include host String and port VarInt"
    );

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
            "Stevenarella panicked while dispatching official Configuration clientbound transfer packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding clientbound transfer packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound transfer packet id {}",
                framed_packet_id
            )
        });
    match decoded {
        packet::Packet::PluginMessageClientbound(packet) => {
            assert_eq!(
                packet.channel, "Transfer",
                "decoded packet did not preserve transfer compatibility channel"
            );
            assert!(
                packet.data.is_empty(),
                "decoded transfer compatibility packet carried unexpected data"
            );
        }
        other => {
            panic!("decoded packet did not preserve clientbound transfer identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded clientbound transfer packet did not consume the official body bytes"
    );
}

#[test]
fn configuration_update_enabled_features_clientbound_framed_dispatch_matches_official_oracle_answer(
) {
    let manifest: TestManifest =
        read_json(CONFIGURATION_UPDATE_ENABLED_FEATURES_CLIENTBOUND_MANIFEST);
    assert_eq!(
        manifest.case_id,
        CONFIGURATION_UPDATE_ENABLED_FEATURES_CLIENTBOUND_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_UPDATE_ENABLED_FEATURES_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_UPDATE_ENABLED_FEATURES_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_UPDATE_ENABLED_FEATURES_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_UPDATE_ENABLED_FEATURES_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(
        CONFIGURATION_UPDATE_ENABLED_FEATURES_CLIENTBOUND_MANIFEST,
        &manifest,
    );

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:update_enabled_features")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:update_enabled_features")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.configuration.ClientboundUpdateEnabledFeaturesPacket")
    );
    assert_eq!(
        oracle.answer.input_fixture.as_deref(),
        Some("Set.of() features")
    );
    assert_eq!(
        oracle.answer.input_feature_count, oracle.answer.decoded_feature_count,
        "official decoded update_enabled_features count differs from the official input count"
    );
    assert_eq!(
        oracle.answer.input_features, oracle.answer.decoded_features,
        "official decoded update_enabled_features set differs from the official input set"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:update_enabled_features",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("update_enabled_features answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body,
        encode_varint(0),
        "official empty update_enabled_features fixture should encode a zero-length feature collection"
    );

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
            "Stevenarella panicked while dispatching official Configuration clientbound update_enabled_features packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!(
                "Stevenarella errored while decoding clientbound update_enabled_features packet: {err}"
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound update_enabled_features packet id {}",
                framed_packet_id
            )
        });
    match decoded {
        packet::Packet::PluginMessageClientbound(packet) => {
            assert_eq!(
                packet.channel, "UpdateEnabledFeatures",
                "decoded packet did not preserve update_enabled_features compatibility channel"
            );
            assert!(
                packet.data.is_empty(),
                "decoded update_enabled_features compatibility packet carried unexpected data"
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve clientbound update_enabled_features identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded clientbound update_enabled_features packet did not consume the official body bytes"
    );
}

#[test]
fn configuration_update_tags_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_UPDATE_TAGS_CLIENTBOUND_MANIFEST);
    assert_eq!(
        manifest.case_id,
        CONFIGURATION_UPDATE_TAGS_CLIENTBOUND_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_UPDATE_TAGS_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_UPDATE_TAGS_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_UPDATE_TAGS_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_UPDATE_TAGS_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(CONFIGURATION_UPDATE_TAGS_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:update_tags")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:update_tags")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundUpdateTagsPacket")
    );
    assert_eq!(
        oracle.answer.input_fixture.as_deref(),
        Some("Map.of() tags")
    );
    assert_eq!(
        oracle.answer.input_tag_registry_count,
        Some(0),
        "update_tags fixture must not invent tag registry payloads"
    );
    assert_eq!(
        oracle.answer.input_tag_registry_count, oracle.answer.decoded_tag_registry_count,
        "official decoded update_tags registry-payload count differs from the official input count"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:update_tags",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("update_tags answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body,
        encode_varint(0),
        "official empty update_tags fixture should encode a zero-length registry-payload map"
    );

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
            "Stevenarella panicked while dispatching official Configuration clientbound update_tags packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding clientbound update_tags packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound update_tags packet id {}",
                framed_packet_id
            )
        });
    match decoded {
        packet::Packet::PluginMessageClientbound(packet) => {
            assert_eq!(
                packet.channel, "UpdateTags",
                "decoded packet did not preserve update_tags compatibility channel"
            );
            assert!(
                packet.data.is_empty(),
                "decoded update_tags compatibility packet carried unexpected data"
            );
        }
        other => {
            panic!("decoded packet did not preserve clientbound update_tags identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded clientbound update_tags packet did not consume the official body bytes"
    );
}

#[test]
fn configuration_select_known_packs_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_SELECT_KNOWN_PACKS_CLIENTBOUND_MANIFEST);
    assert_eq!(
        manifest.case_id,
        CONFIGURATION_SELECT_KNOWN_PACKS_CLIENTBOUND_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_SELECT_KNOWN_PACKS_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_SELECT_KNOWN_PACKS_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_SELECT_KNOWN_PACKS_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_SELECT_KNOWN_PACKS_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(
        CONFIGURATION_SELECT_KNOWN_PACKS_CLIENTBOUND_MANIFEST,
        &manifest,
    );

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:select_known_packs")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:select_known_packs")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.configuration.ClientboundSelectKnownPacks")
    );
    assert_eq!(
        oracle.answer.input_fixture.as_deref(),
        Some("List.of() known_packs")
    );
    assert_eq!(
        oracle.answer.input_known_pack_count,
        Some(0),
        "clientbound select_known_packs fixture must not invent known-pack entries"
    );
    assert_eq!(
        oracle.answer.input_known_pack_count, oracle.answer.decoded_known_pack_count,
        "official decoded clientbound select_known_packs list length differs from the official input length"
    );
    assert_eq!(
        oracle.answer.input_known_packs, oracle.answer.decoded_known_packs,
        "official decoded clientbound select_known_packs list differs from the official input list"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:select_known_packs",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("clientbound select_known_packs answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body,
        encode_varint(0),
        "official empty clientbound select_known_packs fixture should encode a zero-length known-pack list"
    );

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
            "Stevenarella panicked while dispatching official Configuration clientbound select_known_packs packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!(
                "Stevenarella errored while decoding clientbound select_known_packs packet: {err}"
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound select_known_packs packet id {}",
                framed_packet_id
            )
        });
    match decoded {
        packet::Packet::PluginMessageClientbound(packet) => {
            assert_eq!(
                packet.channel, "SelectKnownPacks",
                "decoded packet did not preserve clientbound select_known_packs compatibility channel"
            );
            assert!(
                packet.data.is_empty(),
                "decoded clientbound select_known_packs compatibility packet carried unexpected data"
            );
        }
        other => {
            panic!(
                "decoded packet did not preserve clientbound select_known_packs identity: {other:?}"
            )
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded clientbound select_known_packs packet did not consume the official body bytes"
    );
}

#[test]
fn configuration_custom_report_details_clientbound_framed_dispatch_matches_official_oracle_answer()
{
    let manifest: TestManifest =
        read_json(CONFIGURATION_CUSTOM_REPORT_DETAILS_CLIENTBOUND_MANIFEST);
    assert_eq!(
        manifest.case_id,
        CONFIGURATION_CUSTOM_REPORT_DETAILS_CLIENTBOUND_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_CUSTOM_REPORT_DETAILS_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_CUSTOM_REPORT_DETAILS_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_CUSTOM_REPORT_DETAILS_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_CUSTOM_REPORT_DETAILS_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(
        CONFIGURATION_CUSTOM_REPORT_DETAILS_CLIENTBOUND_MANIFEST,
        &manifest,
    );

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:custom_report_details")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:custom_report_details")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundCustomReportDetailsPacket")
    );
    assert_eq!(
        oracle.answer.input_fixture.as_deref(),
        Some("Map.of() details")
    );
    assert_eq!(
        oracle.answer.input_detail_count,
        Some(0),
        "custom_report_details fixture must not invent report-detail entries"
    );
    assert_eq!(
        oracle.answer.input_detail_count, oracle.answer.decoded_detail_count,
        "official decoded custom_report_details map length differs from the official input length"
    );
    assert_eq!(
        oracle.answer.input_details, oracle.answer.decoded_details,
        "official decoded custom_report_details map differs from the official input map"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:custom_report_details",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("custom_report_details answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body,
        encode_varint(0),
        "official empty custom_report_details fixture should encode a zero-length details map"
    );

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
            "Stevenarella panicked while dispatching official Configuration clientbound custom_report_details packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!(
                "Stevenarella errored while decoding clientbound custom_report_details packet: {err}"
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound custom_report_details packet id {}",
                framed_packet_id
            )
        });
    match decoded {
        packet::Packet::PluginMessageClientbound(packet) => {
            assert_eq!(
                packet.channel, "CustomReportDetails",
                "decoded packet did not preserve custom_report_details compatibility channel"
            );
            assert!(
                packet.data.is_empty(),
                "decoded custom_report_details compatibility packet carried unexpected data"
            );
        }
        other => {
            panic!("decoded packet did not preserve custom_report_details identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded clientbound custom_report_details packet did not consume the official body bytes"
    );
}

#[test]
fn configuration_server_links_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_SERVER_LINKS_CLIENTBOUND_MANIFEST);
    assert_eq!(
        manifest.case_id,
        CONFIGURATION_SERVER_LINKS_CLIENTBOUND_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_SERVER_LINKS_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_SERVER_LINKS_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_SERVER_LINKS_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_SERVER_LINKS_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(CONFIGURATION_SERVER_LINKS_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:server_links")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:server_links")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundServerLinksPacket")
    );
    assert_eq!(
        oracle.answer.input_fixture.as_deref(),
        Some("List.of() server_links")
    );
    assert_eq!(
        oracle.answer.input_link_count,
        Some(0),
        "server_links fixture must not invent server-link entries"
    );
    assert_eq!(
        oracle.answer.input_link_count, oracle.answer.decoded_link_count,
        "official decoded server_links list length differs from the official input length"
    );
    assert_eq!(
        oracle.answer.input_links, oracle.answer.decoded_links,
        "official decoded server_links list differs from the official input list"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:server_links",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("server_links answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert_eq!(
        body,
        encode_varint(0),
        "official empty server_links fixture should encode a zero-length links list"
    );

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
            "Stevenarella panicked while dispatching official Configuration clientbound server_links packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding clientbound server_links packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound server_links packet id {}",
                framed_packet_id
            )
        });
    match decoded {
        packet::Packet::PluginMessageClientbound(packet) => {
            assert_eq!(
                packet.channel, "ServerLinks",
                "decoded packet did not preserve server_links compatibility channel"
            );
            assert!(
                packet.data.is_empty(),
                "decoded server_links compatibility packet carried unexpected data"
            );
        }
        other => {
            panic!("decoded packet did not preserve server_links identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded clientbound server_links packet did not consume the official body bytes"
    );
}

#[test]
fn configuration_clear_dialog_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_CLEAR_DIALOG_CLIENTBOUND_MANIFEST);
    assert_eq!(
        manifest.case_id,
        CONFIGURATION_CLEAR_DIALOG_CLIENTBOUND_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_CLEAR_DIALOG_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_CLEAR_DIALOG_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_CLEAR_DIALOG_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_CLEAR_DIALOG_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(CONFIGURATION_CLEAR_DIALOG_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:clear_dialog")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:clear_dialog")
    );
    assert_eq!(
        oracle.answer.instance_packet_type.as_deref(),
        Some("minecraft:clear_dialog")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundClearDialogPacket")
    );
    assert_eq!(
        oracle.answer.input_fixture.as_deref(),
        Some("ClientboundClearDialogPacket.INSTANCE")
    );
    assert_eq!(oracle.answer.decoded_equals_instance, Some(true));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:clear_dialog",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("clear_dialog answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert!(
        body.is_empty(),
        "official clear_dialog singleton body should be empty because ClientboundClearDialogPacket.STREAM_CODEC is StreamCodec.unit(INSTANCE)"
    );

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
            "Stevenarella panicked while dispatching official Configuration clientbound clear_dialog packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding clientbound clear_dialog packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound clear_dialog packet id {}",
                framed_packet_id
            )
        });
    match decoded {
        packet::Packet::PluginMessageClientbound(packet) => {
            assert_eq!(
                packet.channel, "ClearDialog",
                "decoded packet did not preserve clear_dialog compatibility channel"
            );
            assert!(
                packet.data.is_empty(),
                "decoded clear_dialog compatibility packet carried unexpected data"
            );
        }
        other => {
            panic!("decoded packet did not preserve clear_dialog identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded clientbound clear_dialog packet did not consume the official empty body"
    );
}

#[test]
fn configuration_show_dialog_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_SHOW_DIALOG_CLIENTBOUND_MANIFEST);
    assert_eq!(
        manifest.case_id,
        CONFIGURATION_SHOW_DIALOG_CLIENTBOUND_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_SHOW_DIALOG_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_SHOW_DIALOG_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_SHOW_DIALOG_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_SHOW_DIALOG_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(CONFIGURATION_SHOW_DIALOG_CLIENTBOUND_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:show_dialog")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:show_dialog")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ClientboundShowDialogPacket")
    );
    assert_eq!(
        oracle.answer.input_fixture.as_deref(),
        Some("Holder.direct(new NoticeDialog(CommonDialogData literal title, NoticeDialog.DEFAULT_ACTION))")
    );
    assert_eq!(
        oracle.answer.input_dialog_class.as_deref(),
        Some("net.minecraft.server.dialog.NoticeDialog")
    );
    assert_eq!(
        oracle.answer.decoded_dialog_class.as_deref(),
        Some("net.minecraft.server.dialog.NoticeDialog")
    );
    assert_eq!(
        oracle.answer.input_dialog_title, oracle.answer.decoded_dialog_title,
        "official decoded show_dialog title differs from the official input title"
    );
    assert_eq!(
        oracle.answer.input_dialog_body_count, oracle.answer.decoded_dialog_body_count,
        "official decoded show_dialog body count differs from the official input body count"
    );
    assert_eq!(
        oracle.answer.input_dialog_input_count, oracle.answer.decoded_dialog_input_count,
        "official decoded show_dialog input count differs from the official input count"
    );
    assert_eq!(
        oracle.answer.input_can_close_with_escape, oracle.answer.decoded_can_close_with_escape,
        "official decoded show_dialog escape-close flag differs from the official input flag"
    );
    assert_eq!(
        oracle.answer.input_pause, oracle.answer.decoded_pause,
        "official decoded show_dialog pause flag differs from the official input flag"
    );
    assert_eq!(
        oracle.answer.input_after_action, oracle.answer.decoded_after_action,
        "official decoded show_dialog after_action differs from the official input action"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:show_dialog",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("show_dialog answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert!(
        !body.is_empty(),
        "official show_dialog NoticeDialog fixture should encode a context-free dialog body"
    );

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
            "Stevenarella panicked while dispatching official Configuration clientbound show_dialog packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding clientbound show_dialog packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound show_dialog packet id {}",
                framed_packet_id
            )
        });
    match decoded {
        packet::Packet::PluginMessageClientbound(packet) => {
            assert_eq!(
                packet.channel, "ShowDialog",
                "decoded packet did not preserve show_dialog compatibility channel"
            );
            assert_eq!(
                packet.data,
                body,
                "decoded show_dialog compatibility packet did not retain the official context-free dialog body"
            );
        }
        other => {
            panic!("decoded packet did not preserve show_dialog identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded clientbound show_dialog packet did not consume the official body"
    );
}

#[test]
fn configuration_code_of_conduct_clientbound_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_CODE_OF_CONDUCT_CLIENTBOUND_MANIFEST);
    assert_eq!(
        manifest.case_id,
        CONFIGURATION_CODE_OF_CONDUCT_CLIENTBOUND_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_CODE_OF_CONDUCT_CLIENTBOUND_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_CODE_OF_CONDUCT_CLIENTBOUND_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_CODE_OF_CONDUCT_CLIENTBOUND_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_CODE_OF_CONDUCT_CLIENTBOUND_COMPARISON_SURFACE
    );
    assert_runner_scope(
        CONFIGURATION_CODE_OF_CONDUCT_CLIENTBOUND_MANIFEST,
        &manifest,
    );

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:code_of_conduct")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:code_of_conduct")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.configuration.ClientboundCodeOfConductPacket")
    );
    assert_eq!(
        oracle.answer.input_fixture.as_deref(),
        Some("ClientboundCodeOfConductPacket(String)")
    );
    assert_eq!(
        oracle.answer.input_code_of_conduct, oracle.answer.decoded_code_of_conduct,
        "official decoded code_of_conduct String differs from the official input String"
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_clientbound_packet_table,
        "minecraft:code_of_conduct",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("code_of_conduct answer missing encoded_framed_hex");
    let framed = decode_hex(framed_hex, "encoded_framed_hex");
    let body = decode_hex(&oracle.answer.encoded_body_hex, "encoded_body_hex");
    let (framed_packet_id, body_offset) = read_varint_prefix(&framed);

    assert_eq!(framed_packet_id, expected_packet_id);
    assert_eq!(&framed[body_offset..], body.as_slice());
    assert!(
        !body.is_empty(),
        "official code_of_conduct fixture should encode one String body"
    );

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
            "Stevenarella panicked while dispatching official Configuration clientbound code_of_conduct packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding clientbound code_of_conduct packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration clientbound code_of_conduct packet id {}",
                framed_packet_id
            )
        });
    match decoded {
        packet::Packet::PluginMessageClientbound(packet) => {
            assert_eq!(
                packet.channel, "CodeOfConduct",
                "decoded packet did not preserve code_of_conduct compatibility channel"
            );
            assert_eq!(
                packet.data,
                oracle
                    .answer
                    .decoded_code_of_conduct
                    .as_ref()
                    .expect("code_of_conduct answer missing decoded_code_of_conduct")
                    .as_bytes(),
                "decoded code_of_conduct compatibility packet did not retain the official String bytes"
            );
        }
        other => {
            panic!("decoded packet did not preserve code_of_conduct identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded clientbound code_of_conduct packet did not consume the official body"
    );
}

#[test]
fn configuration_resource_pack_response_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_RESOURCE_PACK_RESPONSE_MANIFEST);
    assert_eq!(
        manifest.case_id,
        CONFIGURATION_RESOURCE_PACK_RESPONSE_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_RESOURCE_PACK_RESPONSE_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_RESOURCE_PACK_RESPONSE_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_RESOURCE_PACK_RESPONSE_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_RESOURCE_PACK_RESPONSE_COMPARISON_SURFACE
    );
    assert_runner_scope(CONFIGURATION_RESOURCE_PACK_RESPONSE_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:resource_pack")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:resource_pack")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ServerboundResourcePackPacket")
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));
    assert_eq!(
        oracle.answer.input_uuid, oracle.answer.decoded_uuid,
        "official decoded resource_pack UUID differs from the official input UUID"
    );
    assert_eq!(
        oracle.answer.input_action, oracle.answer.decoded_action,
        "official decoded resource_pack action differs from the official input action"
    );
    assert_eq!(
        oracle.answer.input_action_is_terminal, oracle.answer.decoded_action_is_terminal,
        "official decoded resource_pack action terminal flag differs from the official input action"
    );
    let official_action = oracle
        .answer
        .decoded_action
        .as_deref()
        .expect("resource_pack answer missing decoded_action");
    let official_action_terminal = oracle
        .answer
        .decoded_action_is_terminal
        .expect("resource_pack answer missing decoded_action_is_terminal");
    let action_row = oracle
        .answer
        .resource_pack_action_table
        .iter()
        .find(|row| row.name == official_action)
        .unwrap_or_else(|| panic!("resource_pack action table missing {official_action}"));
    assert_eq!(
        action_row.is_terminal, official_action_terminal,
        "official resource_pack action table terminal flag differs from decoded action"
    );

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_serverbound_packet_table,
        "minecraft:resource_pack",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("resource_pack answer missing encoded_framed_hex");
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
            Direction::Serverbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Configuration serverbound resource_pack packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding resource_pack packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration serverbound resource_pack packet id {}",
                framed_packet_id
            )
        });
    let decoded_debug = format!("{decoded:?}");
    assert!(
        decoded_debug.contains("ResourcePack"),
        "decoded packet did not preserve resource_pack identity: {decoded_debug}"
    );
    assert!(
        body_slice.is_empty(),
        "decoded resource_pack packet did not consume the official body bytes"
    );
}

#[test]
fn configuration_select_known_packs_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_SELECT_KNOWN_PACKS_MANIFEST);
    assert_eq!(manifest.case_id, CONFIGURATION_SELECT_KNOWN_PACKS_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_SELECT_KNOWN_PACKS_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_SELECT_KNOWN_PACKS_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_SELECT_KNOWN_PACKS_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_SELECT_KNOWN_PACKS_COMPARISON_SURFACE
    );
    assert_runner_scope(CONFIGURATION_SELECT_KNOWN_PACKS_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:select_known_packs")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:select_known_packs")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.configuration.ServerboundSelectKnownPacks")
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));
    assert_eq!(
        oracle.answer.input_known_pack_count, oracle.answer.decoded_known_pack_count,
        "official decoded select_known_packs list length differs from the official input length"
    );
    assert_eq!(
        oracle.answer.input_known_packs, oracle.answer.decoded_known_packs,
        "official decoded select_known_packs list differs from the official input list"
    );

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_serverbound_packet_table,
        "minecraft:select_known_packs",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("select_known_packs answer missing encoded_framed_hex");
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
            Direction::Serverbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Configuration serverbound select_known_packs packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding select_known_packs packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration serverbound select_known_packs packet id {}",
                framed_packet_id
            )
        });
    let decoded_debug = format!("{decoded:?}");
    assert!(
        decoded_debug.contains("SelectKnownPacks"),
        "decoded packet did not preserve select_known_packs identity: {decoded_debug}"
    );
    assert!(
        body_slice.is_empty(),
        "decoded select_known_packs packet did not consume the official body bytes"
    );
}

#[test]
fn configuration_custom_click_action_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_CUSTOM_CLICK_ACTION_MANIFEST);
    assert_eq!(manifest.case_id, CONFIGURATION_CUSTOM_CLICK_ACTION_CASE_ID);
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_CUSTOM_CLICK_ACTION_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_CUSTOM_CLICK_ACTION_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_CUSTOM_CLICK_ACTION_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_CUSTOM_CLICK_ACTION_COMPARISON_SURFACE
    );
    assert_runner_scope(CONFIGURATION_CUSTOM_CLICK_ACTION_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:custom_click_action")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:custom_click_action")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.common.ServerboundCustomClickActionPacket")
    );
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));
    assert_eq!(
        oracle.answer.input_custom_click_id, oracle.answer.decoded_custom_click_id,
        "official decoded custom_click_action id differs from the official input id"
    );
    assert_eq!(
        oracle.answer.input_payload_present, oracle.answer.decoded_payload_present,
        "official decoded custom_click_action payload presence differs from the official input payload"
    );
    assert_eq!(
        oracle.answer.input_payload_tag_id, oracle.answer.decoded_payload_tag_id,
        "official decoded custom_click_action payload tag id differs from the official input payload"
    );
    assert_eq!(
        oracle.answer.input_payload_type, oracle.answer.decoded_payload_type,
        "official decoded custom_click_action payload type differs from the official input payload"
    );
    assert_eq!(
        oracle.answer.input_payload_snbt, oracle.answer.decoded_payload_snbt,
        "official decoded custom_click_action payload SNBT differs from the official input payload"
    );

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_serverbound_packet_table,
        "minecraft:custom_click_action",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("custom_click_action answer missing encoded_framed_hex");
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
            Direction::Serverbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Configuration serverbound custom_click_action packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding custom_click_action packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration serverbound custom_click_action packet id {}",
                framed_packet_id
            )
        });
    let decoded_debug = format!("{decoded:?}");
    assert!(
        decoded_debug.contains("CustomClickAction"),
        "decoded packet did not preserve custom_click_action identity: {decoded_debug}"
    );
    assert!(
        body_slice.is_empty(),
        "decoded custom_click_action packet did not consume the official body bytes"
    );
}

#[test]
fn configuration_accept_code_of_conduct_framed_dispatch_matches_official_oracle_answer() {
    let manifest: TestManifest = read_json(CONFIGURATION_ACCEPT_CODE_OF_CONDUCT_MANIFEST);
    assert_eq!(
        manifest.case_id,
        CONFIGURATION_ACCEPT_CODE_OF_CONDUCT_CASE_ID
    );
    assert_eq!(
        manifest.contract_path,
        CONFIGURATION_ACCEPT_CODE_OF_CONDUCT_CONTRACT
    );
    assert_eq!(
        manifest.answer_path,
        CONFIGURATION_ACCEPT_CODE_OF_CONDUCT_ANSWER
    );
    assert_eq!(manifest.rust_test_target, ORACLE_CONTRACTS_RUST_TARGET);
    assert_eq!(
        manifest.rust_test_name,
        CONFIGURATION_ACCEPT_CODE_OF_CONDUCT_TEST_NAME
    );
    assert_eq!(
        manifest.comparison_surface,
        CONFIGURATION_ACCEPT_CODE_OF_CONDUCT_COMPARISON_SURFACE
    );
    assert_runner_scope(CONFIGURATION_ACCEPT_CODE_OF_CONDUCT_MANIFEST, &manifest);

    let oracle = read_answer(&manifest.answer_path, &manifest.case_id);
    assert_eq!(oracle.case_id, manifest.case_id);
    assert_eq!(
        oracle.answer.packet_type.as_deref(),
        Some("minecraft:accept_code_of_conduct")
    );
    assert_eq!(
        oracle.answer.decoded_packet_type.as_deref(),
        Some("minecraft:accept_code_of_conduct")
    );
    assert_eq!(
        oracle.answer.instance_packet_type.as_deref(),
        Some("minecraft:accept_code_of_conduct")
    );
    assert_eq!(
        oracle.answer.decoded_packet_class.as_deref(),
        Some("net.minecraft.network.protocol.configuration.ServerboundAcceptCodeOfConductPacket")
    );
    assert_eq!(oracle.answer.decoded_equals_instance, Some(true));
    assert_eq!(oracle.answer.remaining_after_official_decode, Some(0));

    let expected_packet_id = packet_id_for(
        &oracle.answer.configuration_serverbound_packet_table,
        "minecraft:accept_code_of_conduct",
    );
    let framed_hex = oracle
        .answer
        .encoded_framed_hex
        .as_deref()
        .expect("accept_code_of_conduct answer missing encoded_framed_hex");
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
            Direction::Serverbound,
            framed_packet_id,
            &mut body_slice,
        )
    }))
    .unwrap_or_else(|_| {
        panic!(
            "Stevenarella panicked while dispatching official Configuration serverbound accept_code_of_conduct packet id {}",
            framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| {
            panic!("Stevenarella errored while decoding accept_code_of_conduct packet: {err}")
        })
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration serverbound accept_code_of_conduct packet id {}",
                framed_packet_id
            )
        });
    match decoded {
        packet::Packet::PluginMessageServerbound(packet) => {
            assert_eq!(
                packet.channel, "AcceptCodeOfConduct",
                "decoded packet did not preserve accept_code_of_conduct compatibility channel"
            );
            assert!(
                packet.data.is_empty(),
                "decoded accept_code_of_conduct compatibility packet carried unexpected data"
            );
        }
        other => {
            panic!("decoded packet did not preserve accept_code_of_conduct identity: {other:?}")
        }
    }
    assert!(
        body_slice.is_empty(),
        "decoded accept_code_of_conduct packet did not consume the official body bytes"
    );
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

fn assert_ping_pong_direction_matches_official_frame(
    official: &FramedDirectionAnswer,
    direction: Direction,
) {
    assert_eq!(official.decoded_packet_type, official.packet_type);
    assert_eq!(
        official.decoded_id, official.input_id,
        "official decoded payload id differs from input id for {}",
        official.flow
    );
    assert_eq!(official.remaining_after_official_decode, 0);

    let official_name_fragment = rust_name_fragment_from_packet_type(&official.packet_type);
    assert!(
        official
            .decoded_packet_class
            .contains(&official_name_fragment),
        "official decoded packet class did not preserve packet identity: {}",
        official.decoded_packet_class
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
            "Stevenarella panicked while dispatching official Configuration {} {} packet id {}",
            official.flow, official.packet_type, framed_packet_id
        )
    });

    let decoded = decoded_result
        .unwrap_or_else(|err| panic!("Stevenarella errored while decoding ping/pong packet: {err}"))
        .unwrap_or_else(|| {
            panic!(
                "Stevenarella did not dispatch official Configuration {} {} packet id {}",
                official.flow, official.packet_type, framed_packet_id
            )
        });
    let decoded_debug = format!("{decoded:?}");
    assert!(
        decoded_debug.contains(&official_name_fragment),
        "decoded packet did not preserve ping/pong identity: {decoded_debug}"
    );
    assert!(
        decoded_debug.contains(&official.input_id.to_string()),
        "decoded packet did not expose the official ping/pong payload id {}: {decoded_debug}",
        official.input_id
    );
    assert!(
        body_slice.is_empty(),
        "decoded ping/pong packet did not consume the official body bytes"
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
