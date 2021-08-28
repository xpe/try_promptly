use promptly::{prompt, ReadlineError};

fn main() {
    let skip_conf_prompt = false;
    dangerous_thing(skip_conf_prompt);
}

fn dangerous_thing(skip_conf_prompt: bool) {
    if skip_conf_prompt || continue_if_yes() {
        println!("Doing dangerous thing...");
    } else {
        println!("Cancelled");
    }
}

fn continue_if_yes() -> bool {
    loop {
        let r: Result<String, ReadlineError> = prompt("Continue? (y/n)");
        match r {
            Ok(response) => {
                if response == "n" || response == "N" {
                    return false;
                } else if response == "y" || response == "Y" {
                    return true;
                } else {
                    println!("Response not recognized: {}\nPlease type 'y' or 'n' and press enter.", response);
                }
            },
            Err(e) => {
                println!("{}", e);
                return false;
            }
        }
    }
}
