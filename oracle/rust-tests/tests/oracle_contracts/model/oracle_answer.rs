#[derive(Debug, Deserialize)]
struct OracleAnswer {
    case_id: String,
    answer: ConfigurationOracleAnswer,
}
