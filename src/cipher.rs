pub struct Cipher {
    pub original: String,
    pub rot13: String,
}

impl Cipher {
    pub fn new(input: &String) -> Self {
        Cipher {
            original: input.clone(),
            rot13: to_rot13(input.clone()),
        }
    } 
}

// converts text to its rot13 cipher equivalent
fn to_rot13(input: String) -> String {
    let mut modified_string: String = String::new();

    // get all indexes of the inputted string
    for i in 0..input.len() {
        modified_string.push(get_char_code(input.chars().nth(i).unwrap()));
    }

    modified_string
}

// for rot13 cipher
fn get_char_code(letter: char) -> char {
    match letter {
      'A' => 'N',
      'B' => 'O',
      'C' => 'P',
      'D' => 'Q',
      'E' => 'R',
      'F' => 'S',
      'G' => 'T',
      'H' => 'U',
      'I' => 'V',
      'J' => 'W',
      'K' => 'X',
      'L' => 'Y',
      'M' => 'Z',
      'N' => 'A',
      'O' => 'B',
      'P' => 'C',
      'Q' => 'D',
      'R' => 'E',
      'S' => 'F',
      'T' => 'G',
      'U' => 'H',
      'V' => 'I',
      'W' => 'J',
      'X' => 'K',
      'Y' => 'L',
      'Z' => 'M',
      'a' => 'n',
      'b' => 'o',
      'c' => 'p',
      'd' => 'q',
      'e' => 'r',
      'f' => 's',
      'g' => 't',
      'h' => 'u',
      'i' => 'v',
      'j' => 'w',
      'k' => 'x',
      'l' => 'y',
      'm' => 'z',
      'n' => 'a',
      'o' => 'b',
      'p' => 'c',
      'q' => 'd',
      'r' => 'e',
      's' => 'f',
      't' => 'g',
      'u' => 'h',
      'v' => 'i',
      'w' => 'j',
      'x' => 'k',
      'y' => 'l',
      'z' => 'm',
      _ => ' '
    }
}