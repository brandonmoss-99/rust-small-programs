struct Letters { 
    /* Using String instead of str, because want the String to live inside the
    struct in memory, not have a reference to a String */
    string: String 
}

impl Letters {
    fn new(text: String) -> Letters {
        Letters{string: text}
    }

    /// Get the total value of all the characters' values
    fn lettersum(&self) -> usize {
        let mut total = 0;
    
        match self.string.len() {
            0 => total,
            _ => {
                let text = self.string.to_ascii_lowercase();
    
                /* For each character in the String, get the byte value of the unicode,
                and remove 96 from it (where the alphabet starts in the unicode character
                set), and add it to the total sum of character values */
                for c in text.chars() {
                    let byte_val = c as u8 - 96;
                    total += byte_val as usize;
                }
    
                total
            }
        }
    }

    fn letter_sum_multiplied(&self, mult: isize) -> isize {
        let mut total: isize = 0;
    
        match self.string.len() {
            0 => total,
            _ => {
                let text = self.string.to_ascii_lowercase();
    
                /* For each character in the String, get the byte value of the unicode,
                and remove 96 from it (where the alphabet starts in the unicode character
                set), and add it to the total sum of character values */
                for c in text.chars() {
                    let byte_val = c as u8 - 96;
                    total += byte_val as isize;
                }
    
                total * mult
            }
        }
    }
}

fn main() {
    let new_str: Letters = Letters::new("microspectrophotometries".to_string());
    let mult: isize = 5;
    println!("The ascii character sum of {} is: {}", new_str.string, new_str.lettersum());
    println!("The ascii character sum of {} multiplied by {} is {}", new_str.string, mult, new_str.letter_sum_multiplied(mult));
}
