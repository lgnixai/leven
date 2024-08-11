use tsr_lexer::globals::Span;


#[cfg(test)]
mod tests {
    use tsr_lexer::globals::{BytesSpan, Positioned, Span};
    use tsr_lexer::token::Token;

    #[test]
    fn test_lexer() {



        //let position: Span = BytesSpan::new(b"foobar").from();
        let position: Span = Span::from(BytesSpan::new(b"foobar".as_ref()));
        print!("{:?}",position);

      let comment= position.wrap(Token::Comment("//这是啥".parse().unwrap()));
        print!("{:?}",comment);

        //  Ok((input, position.wrap(Token::Comment(comment))))
    }
    #[test]
    fn test_postion(){
        let vec = vec![1, 2, 3, 4];
        let mut iter = vec.iter().peekable();

        // Peek at the next element
        if let Some(&next) = iter.peek() {
            println!("Next element is: {}", next);
        }

        // Now, consume the next element
        if let Some(next) = iter.next() {
            println!("Consumed element: {}", next);
        }
        if let Some(&next) = iter.peek() {
            println!("Next element is: {}", next);
        }


        // Continue with the iterator
        for val in iter {
            println!("Remaining element: {}", val);
        }

    }
}