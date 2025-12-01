/// Loads the input file for the specified event, quest, and part
/// from the inputs directory in the workspace root.
pub fn load_event_input(event: u32, quest: u32, part: u32) -> String {
    let workspace = env!("CARGO_MANIFEST_DIR");
    let path = format!("{workspace}/../inputs/everybody_codes_e{event}_q{quest:02}_p{part}.txt");

    let input = std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read input file at {path}"));
    input.trim().to_string()
}
