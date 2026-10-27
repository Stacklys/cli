use std::io::{self, Write};

use crate::util::Logger;

pub fn user_reply_valid(response: &str) -> bool {
  let response = response.trim().to_lowercase();
  if response == "yes" || response == "y" || response == "no" || response == "n" {
    return true;
  }
  false
}

pub fn ask(question: &str) -> String {
  let mut value = String::from("yes");

  loop {
    Logger.info(question.to_string().as_str());
    io::stdout().flush().expect("Error");
    io::stdin().read_line(&mut value).expect("Error");

    if user_reply_valid(&value) {
      break;
    }
    Logger.error("Invalid response. Please enter (y/n)");
    value.clear();
  }

  value.trim().to_string()
}

pub fn ask_confirm(question: &str) -> bool {
  let response = ask(question);
  let reply = response.to_lowercase();
  reply == "yes" || reply == "y"
}
