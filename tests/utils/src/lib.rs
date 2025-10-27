use tantivy_tokenizer_api::BoxTokenStream;

pub fn reduce_token_stream_to_string<'a>(token_stream: &mut BoxTokenStream<'a>) -> String {
    let mut tokens: Vec<String> = Vec::new();

    loop {
        match token_stream.next() {
            Some(token) => {
                tokens.push(token.text.clone());
            },
            None => break,
        }
    }

    return tokens.join(" ");
}
