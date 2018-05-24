use token;

fn peekchar(index: usize) -> Option<char> {
    let sentence = "idtest1 ";
    let bytes = sentence.as_bytes();
    if index >= sentence.len() { 
        return None
    } else {
        return Some(bytes[index] as char)
    }
}

pub fn identifier() -> token::Token {

    let mut token = token::Token::new();
    let mut buffer: Vec<char> = vec!['\0'; 15];
    let mut index: usize = 0;
    loop { 
        let c = peekchar(index);
        match c {
            Some(x) => {
                if x.is_alphanumeric() {
                    if index < 15 {
                        buffer[index] = x;
                    }
                    index = index + 1;
                } else {
                    break;
                }
            }

            None => {
                break;
            }

        }
    }
    println!("{:?}", buffer);
    return token;
}
