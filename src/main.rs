//hi Prof Joshua Dehlinger, its Oscar, this is the code we had for lab 5 and I am basically modifying it for my project
//any changes I made will be commented with ******************* so that you will be able to find it and whatever, here is example
//******************* Oscar changes:
//added comments at top to notify professor on how I will be commenting the code, also that this code was orignally the lab 5 code
//(tip, press ctrl+f and just put in "******************* Oscar changes:" to see where I made all my changes)


use std::env;
use std::fs;

//
// ===================== Compiler Trait =====================
//

/// Compiler trait
pub trait Compiler {
    fn compile(&mut self, source: &str);
    fn next_token(&mut self) -> String;
    fn parse(&mut self);
    fn current_token(&self) -> String;
    fn set_current_token(&mut self, tok: String);
}

//
// ===================== Lexical Analyzer =====================
//

/// Trait for a simple lexical analyzer.
/// Implements a character-by-character analysis.
pub trait LexicalAnalyzer {
    fn get_char(&mut self) -> char;
    fn add_char(&mut self, c: char);
    fn lookup(&self, s: &str) -> bool;
}

/// A concrete implementation of the lexical analyzer.
pub struct SimpleLexicalAnalyzer {
    input: Vec<char>,
    position: usize,
    current_build: String,
    //******************* Oscar changes:
    //change the words lists to be my word lists, with two categories, words starting with the hash tag and words without
    pub tokens: Vec<String>,  
    pub hashWords: Vec<String>,
    pub plainWords: Vec<String>,
}


impl SimpleLexicalAnalyzer {
    pub fn new(source: &str) -> Self {
        Self {
            input: source.chars().collect(),
            position: 0,
            current_build: String::new(),
            tokens: Vec::new(),
            //******************* Oscar changes:
            //initializing all the tokens for the changes we made earlier
            hashWords: vec![
                "#HAI".into(), "#KTHXBYE".into(),
                "#OBTW".into(), "#TLDR".into(),
                "#MAEK".into(), "#GIMMEH".into(),
                "#OIC".into(), "#MKAY".into(),
                "#I".into(), "#IT".into(),
                "#LEMME".into()],
            plainWords: vec![
                "HEAD".into(), "TITLE".into(), "PARAGRAF".into(), "LIST".into(), "ITEM".into(),
                "BOLD".into(), "ITALICS".into(), "NEWLINE".into(), "SOUNDZ".into(), "VIDZ".into(),
                "HAZ".into(), "IZ".into(), "SEE".into(),]
        }
    }

    pub fn tokenize(&mut self) {
        loop {
            let c = self.get_char();
            if c == '\0' {
                break;
            }
            if c.is_whitespace() {
                if !self.current_build.is_empty() {
                    let token = std::mem::take(&mut self.current_build);
                    self.tokens.push(token.to_uppercase());
                }
            } else {
                self.add_char(c);
            }
        }
        if !self.current_build.is_empty() {
            let token = std::mem::take(&mut self.current_build);
            self.tokens.push(token.to_uppercase());
        }
        self.tokens.reverse();
    }
}

impl LexicalAnalyzer for SimpleLexicalAnalyzer {
    fn get_char(&mut self) -> char {
        if self.position < self.input.len() {
            let c = self.input[self.position];
            self.position += 1;
            c
        } else {
            '\0'
        }
    }

    fn add_char(&mut self, c: char) {
        self.current_build.push(c);
    }

    fn lookup(&self, s: &str) -> bool {
        //******************* Oscar changes:
        //tells the compiler only to check words that start with #”
        if s.starts_with('#') {
            self.hashWords.iter().any(|k| k == s)
        } else {
            //******************* Oscar changes:
            // non hash tokens are allowed like text and URLs
            true
        }
    }
}

//
// ===================== Syntax Analyzer =====================
//

pub struct LolspeakCompiler {
    lexer: SimpleLexicalAnalyzer,
    current_tok: String,
    // ******************* Oscar changes:
    // add field to build output
    pub html_output: String,
}

impl LolspeakCompiler {
    pub fn new() -> Self {
        Self {
            lexer: SimpleLexicalAnalyzer::new(""),
            current_tok: String::new(),
            // ******************* Oscar changes
            //add output for constructor
            html_output: String::new(), 
        }
    }

    fn start(&mut self) {
        let candidate = self.lexer.tokens.pop().unwrap_or_default();
        if self.lexer.lookup(&candidate) {
            self.current_tok = candidate;
        } else if !candidate.is_empty() {
            eprintln!("Lexical error: '{}' is not a recognized token.", candidate);
            std::process::exit(1);
        } else {
            eprintln!("User error: The provided sentence is empty.");
            std::process::exit(1);
        }
    }

    //******************* Oscar changes:
    //deleted all old grammar functions (from lab5)
    //replaced them with the grammar for my project
    /*
    this is where your gonna see the bulk of my code and also I was super lazy with commenting here
    This will basically look like my bnf except that:
       I change the logic for checking since I didnt want to use weird category thingy fucntions anymore (the "is_x" functions)
       I format things weirdly for conveinece or just it was how I was thinking in the moment, Please let me know if you have
       any questions professor, this is honestly a mess to read and I am willing to go back and comment it to look nice and stuff
    */

    fn Start_End(&mut self) {
        // <Start_End> ::= "#HAI" <Heading_Part> <Body_Repeatable> "#KTHXBYE"
        self.hai();          
        self.headingPart();    
        self.bodyRepeatable(); 
        self.kThxBye();    
    }

    fn hai(&mut self) {
        if self.current_tok == "#HAI" {
            println!("Found start token: {}", self.current_tok);
            self.next_token();
        } else {
            eprintln!(
                "Syntax error: '{}' was found when an article (#HAI) was expected.", 
                self.current_tok
            );
            std::process::exit(1);
        }
    }

    fn kThxBye(&mut self) {
        if self.current_tok == "#KTHXBYE" {
            println!("Found start token: {}", self.current_tok);
            self.next_token();
        } else {
            eprintln!(
                "Syntax error: '{}' was found when an article (#KTHXBYE) was expected.", 
                self.current_tok
            );
            std::process::exit(1);
        }
    }

    fn headingPart(&mut self) {
        // <Heading_Part> ::= <Comment> <Head> | ""
        if self.current_tok == "#OBTW" || self.current_tok == "#MAEK" {
            self.comment();
            if self.current_tok == "#MAEK" {
                self.head();
            }
        } else {
            println!("No heading found.");
        }
    }

    fn head(&mut self) {
        // <Head> ::= "#MAEK" "HEAD" "#GIMMEH" "TITLE" <Text> "#MKAY" "#OIC"
        //basically one big switch case but idk rust so we do it bad
        println!("Parsing <Head> block...");

        if self.current_tok != "#MAEK" {
            eprintln!("Syntax error: Expected '#MAEK' to start head block, found '{}'", self.current_tok);
            std::process::exit(1);
        }
        self.next_token();

        if self.current_tok != "HEAD" {
            eprintln!("Syntax error: Expected 'HEAD' after #MAEK, found '{}'", self.current_tok);
            std::process::exit(1);
        }
        self.next_token();

        if self.current_tok != "#GIMMEH" {
            eprintln!("Syntax error: Expected '#GIMMEH' after HEAD, found '{}'", self.current_tok);
            std::process::exit(1);
        }
        self.next_token();

        if self.current_tok != "TITLE" {
            eprintln!("Syntax error: Expected 'TITLE' after #GIMMEH, found '{}'", self.current_tok);
            std::process::exit(1);
        }
        self.next_token();

        let mut title_text = Vec::new();
        while self.current_tok != "#MKAY" && !self.current_tok.is_empty() {
            title_text.push(self.current_tok.clone());
            self.next_token();
        }

        if self.current_tok != "#MKAY" {
            eprintln!("Syntax error: Missing '#MKAY' after title text.");
            std::process::exit(1);
        }
        self.next_token();

        if self.current_tok != "#OIC" {
            eprintln!("Syntax error: Missing '#OIC' at end of head block.");
            std::process::exit(1);
        }

        println!("Parsed head title: {}", title_text.join(" "));
        self.next_token();
    }

    fn comment(&mut self) {
        while self.current_tok == "#OBTW" {
            println!("Found comment start: {}", self.current_tok);
            self.next_token();

            let mut comment_text = Vec::new();
            while self.current_tok != "#TLDR" && !self.current_tok.is_empty() {
                comment_text.push(self.current_tok.clone());
                self.next_token();
            }

            if self.current_tok != "#TLDR" {
                eprintln!("Syntax error: missing '#TLDR' to close comment block.");
                std::process::exit(1);
            }

            println!("Comment captured: {}", comment_text.join(" "));
            self.next_token();

        }
    }

    fn bodyRepeatable(&mut self) {
        // <Body_Repeatable> ::= <Body> <Body_Repeatable> | ""
        println!("Parsing <Body_Repeatable> (stub)");
        while !self.current_tok.is_empty() && self.current_tok != "#KTHXBYE" {
            self.body();
        }
    }

    fn body(&mut self) {
        // <Body> ::= <Comment> | <Paragraph> | <List> | <Bold> | <Italics> | <Sound> | <Video> | <Newline> | <Variable> | <Var_Use> | <Text> | ""
        match self.current_tok.as_str() {
            "#OBTW" => self.comment(),
            "#MAEK" => self.paragraphOrList(),
            "#GIMMEH" => self.format(), 
            _ => {
                // Plain text or variable use
                println!("Encountered text or variable: {}", self.current_tok);
                self.next_token();
            }
        }
    }

    fn paragraph(&mut self) {
        println!("Parsing <Paragraph>...");
        // <Paragraph> ::= "#MAEK" "PARAGRAF" ... "#OIC"
        self.next_token();

        while self.current_tok != "#OIC" && !self.current_tok.is_empty() {
            match self.current_tok.as_str() {
                "#GIMMEH" => self.format(), // detect format
                "#MAEK" => self.list(),     // nested list in paragraph
                _ => {
                    println!("Paragraph content: {}", self.current_tok);
                    self.next_token();
                }
            }
        }

        if self.current_tok == "#OIC" {
            println!("End of paragraph.");
            self.next_token();
        } else {
            eprintln!("Syntax error: Missing #OIC at end of paragraph block.");
            std::process::exit(1);
        }
    }

    fn list(&mut self) {
        println!("Parsing <List>...");
        // <List> ::= "#MAEK" "LIST" <Item> ... "#OIC"
        self.next_token();

        while self.current_tok != "#OIC" && !self.current_tok.is_empty() {
            if self.current_tok == "#GIMMEH" {
                self.next_token();
                if self.current_tok == "ITEM" {
                    self.next_token();
                    println!("List item: {}", self.current_tok);
                    while self.current_tok != "#MKAY" && self.current_tok != "#OIC" && !self.current_tok.is_empty() {
                        self.next_token();
                    }
                    if self.current_tok == "#MKAY" {
                        self.next_token();
                    }
                }
            } else {
                self.next_token();
            }
        }

        if self.current_tok == "#OIC" {
            println!("End of list.");
            self.next_token();
        } else {
            eprintln!("Syntax error: Missing #OIC at end of list block.");
            std::process::exit(1);
        }
    }

    fn paragraphOrList(&mut self) {
        self.next_token();
        match self.current_tok.as_str() {
            "PARAGRAF" => self.paragraph(),
            "LIST" => self.list(),
            _ => {
                eprintln!("Syntax error: expected PARAGRAF or LIST after #MAEK, found '{}'", self.current_tok);
                std::process::exit(1);
            }
        }
    }

    fn format(&mut self) {
        // Handles stuff like bold, italics, newline, sounds, etc.
        self.next_token();
        match self.current_tok.as_str() {
            "BOLD" => println!("Bold text start"),
            "ITALICS" => println!("Italics text start"),
            "NEWLINE" => println!("Newline element"),
            "SOUNDZ" => println!("Sound element start"),
            "VIDZ" => println!("Video element start"),
            _ => println!("Unknown inline element: {}", self.current_tok),
        }

        while self.current_tok != "#MKAY" && self.current_tok != "#OIC" && !self.current_tok.is_empty() {
            self.next_token();
        }

        if self.current_tok == "#MKAY" {
            self.next_token();
        }
    }


}

impl Compiler for LolspeakCompiler {
    fn compile(&mut self, source: &str) {
        self.lexer = SimpleLexicalAnalyzer::new(source);
        self.lexer.tokenize();
        self.start();
    }

    fn next_token(&mut self) -> String {
        let candidate = self.lexer.tokens.pop().unwrap_or_default();
        if self.lexer.lookup(&candidate) {
            self.current_tok = candidate.clone();
            candidate
        } else if self.lexer.tokens.is_empty() {
            self.current_tok.clear();
            String::new()
        } else {
            eprintln!("Lexical error: '{}' is not a recognized token.", candidate);
            std::process::exit(1);
        }
    }

    fn parse(&mut self) {
        //******************* Oscar changes:
        //change to use my grammar function, Start_End();
        self.Start_End();
        if !self.lexer.tokens.is_empty() || !self.current_tok.is_empty() {
            eprintln!("Syntax error: Additional tokens found after the sentence.");
            std::process::exit(1);
        }
    }

    fn current_token(&self) -> String {
        self.current_tok.clone()
    }
    fn set_current_token(&mut self, tok: String) {
        self.current_tok = tok;
    }
}

//
// ===================== Main =====================
//

fn main() {
    // Usage in VS Code terminal:
    //   cargo run -- lolspeak.txt
    // where lolspeak.txt is in the project root (not src/) of the project.

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let sentence = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", filename, err);
        std::process::exit(1);
    });

    let mut compiler = LolspeakCompiler::new();
    compiler.compile(&sentence);

    /* 
    println!("TOKENS:");
    for tok in &compiler.lexer.tokens {
        println!("{}", tok);
    }
    */

    compiler.parse();

    println!(
        "The sentence '{}' follows the lolspeak grammar!",
        sentence.trim()
    );
}
