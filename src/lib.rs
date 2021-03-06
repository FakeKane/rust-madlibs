#[feature(plugin)]
#[macro_use] #[plugin] #[no_link]
extern crate regex_macros;
extern crate regex;
use std::io;
use regex::Regex;

// read_input keeps taking in lines from stdin until a single newline
// is entered. It returns a String.
pub fn read_input() -> String {
    let mut story = "".to_string();
    println!("Please enter a line of the story. Enter a blank line to stop");
    loop {
        let input = io::stdin().read_line().ok().expect("Failed to read line.");
        let line = input.as_slice();
        if line == "\n" {
            break;
        }
        else {
            story.push_str(line);
        }
    }
    story.to_string()
}

// fill_blanks takes in a string, representing the current template
// and both prompts and replaces for an item in brackets [].
// A string, the modified story, is returned.
#[allow(unused_variables)]
pub fn fill_blanks(s: &str) -> String {
    let mut l_iter = s.match_indices("[");
    let mut r_iter = s.match_indices("]");
    let l_duple = l_iter.next();
    let r_duple = r_iter.next();
    let l_index = match l_duple {
        Some((x, y)) => x,
        None => -1,
    };
    let r_index = match r_duple {
        Some((x, y)) => y,
        None => -1,
    };
    if l_index == -1 {
        s.to_string()
    }
    else {
        println!("Give me a/an {}!", s.slice(l_index + 1, r_index - 1));
        let input = std::io::stdin().read_line().ok().expect("Failed to fill blank.");
        let re = regex!(s.as_slice());
        re.replace(s.slice(l_index, r_index), input.as_slice().trim());
        re.as_str().to_string();
    }
}
