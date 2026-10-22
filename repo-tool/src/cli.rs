/* Terminal frontend :) */

use dialoguer::Select;

pub fn ask() -> String {
    let options = vec!["Commit", "Push", "Exit"];
    let selection = Select::new()
        .with_prompt("Select an option")
        .default(0)
        .items(&options)
        .interact()
        .unwrap();

    return options[selection].to_string();
}