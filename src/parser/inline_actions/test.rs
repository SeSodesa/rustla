// /// This submodule contains tests for the inline actions of the lexer.

// use super::super::*;

// #[test]
// fn lex_code () {

//   let mut src_iter = r"asdsadas ``some code``  ".chars();

//   let pos = &mut Pos::new();

//   let mut lexer = Lexer::new(&mut src_iter, pos, State::Inline);

//   lexer.lex();

//   let toks = lexer.tokens;

//   println!("{:#?}", toks);

//   assert_eq!(toks[0].t_type, TokenType::Text);
//   assert_eq!(toks[1].t_type, TokenType::Code);
//   assert_eq!(toks[2].t_type, TokenType::Text);
//   assert_eq!(toks[1].lexeme, "some code");

// }

// #[test]
// fn phrase_reference_01 () {

//   let mut src_iter = r"asdsadas ``some code``  
//   asdsadsadsad `alias <link>`__".chars();

//   let pos = &mut Pos::new();

//   let mut lexer = Lexer::new(&mut src_iter, pos, State::Inline);

//   lexer.lex();

//   let toks = lexer.tokens;

//   println!("{:#?}", toks);

//   assert_eq!(toks[0].t_type, TokenType::Text);
//   assert_eq!(toks[1].t_type, TokenType::Code);
//   assert_eq!(toks[2].t_type, TokenType::Text);
//   assert_eq!(toks[3].t_type, TokenType::InlineWhitespace);
//   assert_eq!(toks[4].t_type, TokenType::Text);
//   assert_eq!(toks[5].t_type, TokenType::TargetReference);
//   assert_eq!(toks[6].t_type, TokenType::LinkAlias);
//   assert_eq!(toks[7].t_type, TokenType::Hyperlink);

// }

// #[test]
// fn phrase_reference_02 () {

//   let mut src_iter = r"asdsadas   
//   asdsadsadsad `target`__ adsads".chars();

//   let pos = &mut Pos::new();

//   let mut lexer = Lexer::new(&mut src_iter, pos, State::Inline);

//   lexer.lex();

//   let toks = lexer.tokens;

//   println!("{:#?}", toks);

//   assert_eq!(toks[3].t_type, TokenType::InlineReference);
//   assert_eq!(toks[4].t_type, TokenType::Target);
//   assert_eq!(toks[5].t_type, TokenType::RefAnonOrNot);

// }


// #[test]
// fn role_content_01() {
//   let mut src_iter = r"asdsadas :math:`tekstiä matikkamoodissa`".chars();

//   let pos = &mut Pos::new();

//   let mut lexer = Lexer::new(&mut src_iter, pos, State::Inline);

//   lexer.lex();

//   let toks = lexer.tokens;

//   println!("{:#?}", toks);

//   assert_eq!(toks[1].t_type, TokenType::RoleContent);
//   assert_eq!(toks[2].t_type, TokenType::Role);
//   assert_eq!(toks[3].t_type, TokenType::Content);
// }


// #[test]
// fn content_role_01() {
//   let mut src_iter = r"asdsadas `tekstiä matikkamoodissa`:math:".chars();

//   let pos = &mut Pos::new();

//   let mut lexer = Lexer::new(&mut src_iter, pos, State::Inline);

//   lexer.lex();

//   let toks = lexer.tokens;

//   println!("{:#?}", toks);

//   assert_eq!(toks[1].t_type, TokenType::ContentRole);
//   assert_eq!(toks[2].t_type, TokenType::Content);
//   assert_eq!(toks[3].t_type, TokenType::Role);
// }


// #[test]
// fn strong_emphasis_01() {
//   let mut src_iter = r"asdsadas **korostettua tekstiä** adasdadsad".chars();

//   let pos = &mut Pos::new();

//   let mut lexer = Lexer::new(&mut src_iter, pos, State::Inline);

//   lexer.lex();

//   let toks = lexer.tokens;

//   println!("{:#?}", toks);

//   assert_eq!(toks[0].t_type, TokenType::Text);
//   assert_eq!(toks[1].t_type, TokenType::StrongEmphasis);
//   assert_eq!(toks[2].t_type, TokenType::Text);
// }


// #[test]
// fn emphasis_01() {
//   let mut src_iter = r"asdsadas *korostettua tekstiä* adasdadsad".chars();

//   let pos = &mut Pos::new();

//   let mut lexer = Lexer::new(&mut src_iter, pos, State::Inline);

//   lexer.lex();

//   let toks = lexer.tokens;

//   println!("{:#?}", toks);

//   assert_eq!(toks[0].t_type, TokenType::Text);
//   assert_eq!(toks[1].t_type, TokenType::Emphasis);
//   assert_eq!(toks[2].t_type, TokenType::Text);
// }

// #[test]
// fn footnote_or_citation_01 () {

//   let mut src_iter = r"asdsadas [Sod2011]_ adasdadsad".chars();

//   let pos = &mut Pos::new();

//   let mut lexer = Lexer::new(&mut src_iter, pos, State::Inline);

//   lexer.lex();

//   let toks = lexer.tokens;

//   println!("{:#?}", toks);


//   assert_eq!(toks[0].t_type, TokenType::Text);
//   assert_eq!(toks[1].t_type, TokenType::FootnoteOrCitation);
//   assert_eq!(toks[2].t_type, TokenType::Text);
// }


// #[test]
// fn hyperlink_01 () {

//   let mut src_iter = r"asdsadas <https://www.address.fi/> adasdadsad".chars();

//   let pos = &mut Pos::new();

//   let mut lexer = Lexer::new(&mut src_iter, pos, State::Inline);

//   lexer.lex();

//   let toks = lexer.tokens;

//   println!("{:#?}", toks);


//   assert_eq!(toks[1].t_type, TokenType::URI);
//   assert_eq!(toks[2].t_type, TokenType::Scheme);
//   assert_eq!(toks[3].t_type, TokenType::Authority);
//   assert_eq!(toks[4].t_type, TokenType::Path);
// }
