use crate::util::file_validation::file_exists;
use crate::util::logger::Logger;
use crate::util::user_input::ask_confirm;
use std::fs;

pub fn overwrite_template(message: &str, path: &str) -> bool {
  if file_exists(path) {
    Logger.info(&format!("{} already exists", message));
    let response = ask_confirm("Do you want to overwrite it? (y/n): ");
    return response;
  }
  true
}

pub fn create_template(value: &str) {
  if value == "editorconfig" {
    let origin = "./src/template/editorconfig/.editorconfig";

    if !overwrite_template("editorconfig", ".editorconfig") {
      return;
    }

    fs::File::create(".editorconfig").unwrap();
    fs::copy(origin, ".editorconfig").unwrap();

    Logger.setup("File .editorconfig successful created");
  }
}
