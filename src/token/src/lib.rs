enum Token {
    AND,
    OR,
    DSEMI,
    DLESS,
    DGREAT,
    LESSAND,
    GREATAND,
    LESSGREAT,
    DLESSDASH,
    CLOBBER,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        let str = match self {
            Token::AND => "&&",
            Token::OR => "||",
            Token::DSEMI => ";;",
            Token::DLESS => "<<",
            Token::DGREAT => ">>",
            Token::LESSAND => "<&",
            Token::GREATAND => ">&",
            Token::LESSGREAT => "<>",
            Token::DLESSDASH => "<<-",
            Token::CLOBBER => ">|",
        };
        String::from(str)
    }
}
