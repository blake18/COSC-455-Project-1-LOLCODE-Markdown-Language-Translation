//hi Prof Joshua Dehlinger, its Oscar, this is the code we had for lab 5 and I am basically modifying it for my project
//any changes I made will be commented with ******************* so that you will be able to find it and whatever, here is example
//******************* Oscar changes:
//added comments at top to notify professor on how I will be commenting the code, also that this code was orignally the lab 5 code
//(tip, press ctrl+f and just put in "******************* Oscar changes:" to see where I made all my changes)
//There is a big change to comments once we get to my Grammar, please make sure to read the big comment block right above it 

//imports for file reading and output
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;

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

    // ******************* Oscar changes:
    // add vector for variables 
    pub varName: Vec<String>,
    pub varContent: Vec<String>,
}

impl LolspeakCompiler {
    pub fn new() -> Self {
        Self {
            lexer: SimpleLexicalAnalyzer::new(""),
            current_tok: String::new(),
            // ******************* Oscar changes
            //add output for constructor
            html_output: String::new(), 

            // ******************* Oscar changes:
            //add the variables for contructor
            varName: Vec::new(),
            varContent: Vec::new(),
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
    this is where your gonna see the bulk of my code ~~and also I was super lazy with commenting here~~
                                                    [NO LONGER TRUE], I did a bunch of commenting afterwards
    This will basically look like my bnf except that:
        I change the logic for checking since I didnt want to use weird category thingy fucntions anymore (the "is_x" functions)
        I format things weirdly for conveinece or just it was how I was thinking in the moment,
        most the functions here will not have the "******************* Oscar changes" for comments relating to them to not flood
        the code with comments and also because I am lazy, Please let me know if you have any questions professor!  
    */

    fn Start_End(&mut self) {
        // start html Wrapper
        self.html_output.push_str("<html>\n<body>\n");

        // <Start_End> ::= "#HAI" <Heading_Part> <Body_Repeatable> "#KTHXBYE"
        self.hai();          
        self.headingPart();    
        self.bodyRepeatable(); 
        self.kThxBye();    

        //End html wrapper
        self.html_output.push_str("</body>\n</html>\n");
    }

    fn hai(&mut self) {
        if self.current_tok == "#HAI" { //check if first token is #HAI
            println!("Found start token: {}", self.current_tok);
            self.next_token(); // move to next token
        } else {
            eprintln!( //if first token is not #HAI, get mad
                "Syntax error: '{}' was found when an article (#HAI) was expected.", //spit out error 
                self.current_tok
            );
            std::process::exit(1); //exit program
        }
    }

    fn kThxBye(&mut self) {
        if self.current_tok == "#KTHXBYE" { //check if next token is #KTHXBYE
            println!("Found end token: {}", self.current_tok);
            self.next_token(); // move to next token
        } else {
            eprintln!( //if next token is not #KTHXBYE, get mad
                "Syntax error: '{}' was found when an article (#KTHXBYE) was expected.", //spit out error 
                self.current_tok
            );
            std::process::exit(1); //exit program
        }
    }

    fn headingPart(&mut self) {
        // <Heading_Part> ::= <Comment> <Head> | ""
        //checks for optional comments after #HAI,
        while self.current_tok == "#OBTW" { 
            self.comment(); 
        }

        if self.current_tok == "#MAEK" { //makes sure it starts with #MAEK
            let peek = self.lexer.tokens.last().cloned().unwrap_or_default(); //checks if next token is head
            if peek == "HEAD" {
                self.head(); //If there is no comment, just start head
            }
        } else {// if not a comment or head, continue parsing 
             println!("No heading found."); 
        }
    }

    fn head(&mut self) { //Parses through head
        // <Head> ::= "#MAEK" "HEAD" "#GIMMEH" "TITLE" <Text> "#MKAY" "#OIC"
        println!("Parsing <Head> block..."); 

        if self.current_tok != "#MAEK" { //Makes sure head starts with #MAEK
            eprintln!("Syntax error: Expected '#MAEK' to start head block, found '{}'", self.current_tok); //if not get mad, throw error
            std::process::exit(1); //and exit program
        }
        self.next_token(); // move to next token

        if self.current_tok != "HEAD" { // makes sure next token after #MAEK is HEAD
            eprintln!("Syntax error: Expected 'HEAD' after #MAEK, found '{}'", self.current_tok); //if not get mad, throw error
            std::process::exit(1); //and exit program
        }
        self.next_token(); // move to next token

        if self.current_tok != "#GIMMEH" { // makes sure next token is #GIMMEH

            eprintln!("Syntax error: Expected '#GIMMEH' after HEAD, found '{}'", self.current_tok); //if not get mad, throw error
            std::process::exit(1); //and exit program
        }
        self.next_token(); // move to next token

        if self.current_tok != "TITLE" { // maker sure next token after #GIMMEH is TITLE
            eprintln!("Syntax error: Expected 'TITLE' after #GIMMEH, found '{}'", self.current_tok); //if not get mad, throw error
            std::process::exit(1); //and exit program
        }
        self.next_token(); // move to next token

        let mut title_text = Vec::new(); //make vector for all tokens of title 
        while self.current_tok != "#MKAY" && !self.current_tok.is_empty() { //collect it in all the tokens from title stopping at #MKAY
            title_text.push(self.current_tok.clone());
            self.next_token(); // move to next token
        }

        if self.current_tok != "#MKAY" { //makes sure titles ends with #MKAY Token
            eprintln!("Syntax error: Missing '#MKAY' after title text."); //if not get mad, throw error
            std::process::exit(1); //and exit program
        }
        self.next_token(); // move to next token

        if self.current_tok != "#OIC" { //Makes sure Head ends with #OIC token
            eprintln!("Syntax error: Missing '#OIC' at end of head block."); //if not get mad, throw error
            std::process::exit(1); //and exit program
        }

        let title_joined = title_text.join(" ");
        // generate HTML for the head/title section
        self.html_output.push_str(&format!(
            "<head><title>{}</title></head>\n",
            title_joined
        ));
        println!("Parsed head title: {}", title_text.join(" ")); 
        self.next_token(); // move to next token
    }

    fn comment(&mut self) {
        if self.current_tok != "#OBTW" { // check to make sure comments begin with #OBTW
            eprintln!("Syntax error: expected '#OBTW' to start a comment, found '{}'", self.current_tok); //if not get mad, throw error
            std::process::exit(1); //and exit program
        }
        self.next_token(); // move to next token

        let mut comment_text = String::new(); //make a string for the comment
        while self.current_tok != "#TLDR" && !self.current_tok.is_empty() { //check for all text until #TLDR and put in string
            comment_text.push_str(&self.current_tok);
            comment_text.push(' ');
            self.next_token(); // move to next token
        }

        if self.current_tok != "#TLDR" {// makes sure it ends with #TLDR to end the comment
            eprintln!("Syntax error: missing '#TLDR' to close comment block."); //if not get mad, throw error
            std::process::exit(1); //and exit program
        }

        self.next_token(); // move to next token

        //html
        self.html_output
            .push_str(&format!("Comment written: {} -->\n", comment_text.trim()));
    }

    fn bodyRepeatable(&mut self) {
        // <Body_Repeatable> ::= <Body> <Body_Repeatable> | ""
        println!("Parsing <Body_Repeatable> (stub)");
        //uses recursion, Keeps parising body tokens until out of tokens or until #KTHXBYE
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
            "#I" => self.variable(),
            "#LEMME" => self.var_use(),
            _ => {
                // Plain text or variable use, gets added to html later
                println!("Encountered text or variable: {}", self.current_tok);
                self.next_token();
            }
        }
    }

    fn paragraph(&mut self) {
        println!("Parsing <Paragraph>...");
        // <Paragraph> ::= "#MAEK" "PARAGRAF" ... "#OIC"
        // we actually skip #MAEK because we already did that earlier.
        if self.current_tok != "PARAGRAF" { //makes sure next token is PARAGRAF
            eprintln!("Syntax error: Expected 'PARAGRAF' after #MAEK, found '{}'", self.current_tok); //if not get mad, throw error
            std::process::exit(1); //and exit program
        }
        self.next_token(); // move to next token

        // start HTML paragraph
        self.html_output.push_str("<p>");

        //parse through paragraph until #OIC
        while self.current_tok != "#OIC" && !self.current_tok.is_empty() {
            match self.current_tok.as_str() {
                "#GIMMEH" => self.format(), // detects format
                "#LEMME" => self.var_use(), // adds variables 
                "#MAEK" => self.list(),     // allows lists
                _ => {//adds the plain text to html
                    self.html_output.push_str(&format!(" {}", self.current_tok)); //added for html
                    println!("Paragraph content: {}", self.current_tok);
                    self.next_token(); //and exit program
                }
            }
        }

        if self.current_tok == "#OIC" {
            // close HTML paragraph
            self.html_output.push_str("</p>\n");
            println!("End of paragraph.");
            self.next_token();
        } else { // makes sure it ends with #OIC to end the paragraph
            eprintln!("Syntax error: Missing #OIC at end of paragraph block."); //if not get mad, throw error
            std::process::exit(1); //and exit program
        }
    }

    fn list(&mut self) {
        println!("Parsing <List>...");
        // <List> ::= "#MAEK" "LIST" <Item> <List> "#OIC" | "#MAEK" "LIST" <Item> "#OIC"
        if self.current_tok != "LIST" {// checl to make sure next token starts with LIST
            eprintln!("Syntax error: Expected 'LIST' after #MAEK, found '{}'", self.current_tok); //if not get mad, throw error
            std::process::exit(1); //and exit program
        }
        self.next_token(); // move to next token

        //start HTML list output
        self.html_output.push_str("<ul>\n");

        //parse each item until #OIC
        while self.current_tok != "#OIC" && self.current_tok != "#MKAY" && !self.current_tok.is_empty() { //start loop, ends when out of tokens or we see #OIC or #MKAY
            if self.current_tok == "#GIMMEH" { //make sure list starts with #GIMMEH
                self.next_token(); // move to next token
                if self.current_tok == "ITEM" { //makes sure next token is ITEM
                    self.next_token(); // move to next token
                    self.html_output.push_str("<li>"); //start html list item
                    println!("List item: {}", self.current_tok);
                    
                    //starts loop that collects all text in the ITEM, ends when we see #OIC or #MKAY
                    while self.current_tok != "#MKAY" && self.current_tok != "#OIC" && !self.current_tok.is_empty() {
                        if self.current_tok == "#LEMME" {
                            // lets variables be used inside lists
                            self.var_use();
                        } else if self.current_tok == "#GIMMEH" {
                            // allow for formats
                            self.format();
                        } else {
                            //add to html
                            self.html_output.push_str(&format!(" {}", self.current_tok));
                            self.next_token(); // move to next token
                        }
                    }       

                    self.html_output.push_str("</li>\n"); //close html list item

                    if self.current_tok == "#MKAY" { //if current token is #MKAY, end list
                        self.next_token(); // move to next token
                    }
                }
            } else {//if for some reason something doesnt match up, skip it
                self.next_token(); // move to next token
            }
        }

        if self.current_tok == "#OIC" {// makes sure lists end with #OIC 
            self.html_output.push_str("</ul>\n"); //close HTML list
            println!("End of list.");
            self.next_token(); // move to next token
        } else {//if it does not end with #OIC
            eprintln!("Syntax error: Missing #OIC at end of list block.");// get mad, throw error 
            std::process::exit(1); // move to next token
        }
    }

    fn paragraphOrList(&mut self) {
        self.next_token(); // move to next token
        match self.current_tok.as_str() { //if the next token is--
            "PARAGRAF" => self.paragraph(), // PARAGRAF, Uses Paragraph function
            "LIST" => self.list(),          //LIST, use List Function
            _ => { // Anything else is invalid 
                eprintln!("Syntax error: expected PARAGRAF or LIST after #MAEK, found '{}'", self.current_tok);// get mad throw error
                std::process::exit(1); // move to next token
            }
        }
    }

    fn format(&mut self) {
        // Handles stuff like bold, italics, newline, sounds, etc.
        self.next_token(); // move to next token
        match self.current_tok.as_str() { //if the next token is any of the following--
            "NEWLINE" => { // if it is NEWLINE,
                println!("Newline element");
                self.html_output.push_str("<br>\n"); //add a line break to the html
                self.next_token(); // move to next token
            }

            "BOLD" => { // if next token is BOLD
                println!("Bold element start");
                self.next_token(); // move to next token
                self.html_output.push_str("<b>"); //start bold html
                while self.current_tok != "#MKAY" && !self.current_tok.is_empty() { //read everything inside bold until #MKAY
                    match self.current_tok.as_str() {
                        "#LEMME" => { //if next token is #LEMME
                            // allows variables inside bold text
                            self.var_use();
                        }
                        _ => {
                            println!("Bold content: {}", self.current_tok); 
                            self.html_output.push_str(&format!(" {}", self.current_tok)); //add plain text to html 
                            self.next_token(); // move to next token
                        }
                    }
                }
                self.html_output.push_str("</b>"); // end bold html
                if self.current_tok == "#MKAY" { //moves to next token after #MKAY
                    self.next_token();
                }
            }

            "ITALICS" => { // if next token is ITALICS
                println!("Italics text star t");
                self.next_token(); // move to next token
                self.html_output.push_str("<i>"); //start italic html
                while self.current_tok != "#MKAY" && !self.current_tok.is_empty() { //read everything inside bold until #MKAY
                    match self.current_tok.as_str() {
                        "#LEMME" => { //if next token is #LEMME
                             // allows variables inside italics text
                            self.var_use();
                        }
                        _ => {
                            println!("Italics content: {}", self.current_tok);
                            self.html_output.push_str(&format!(" {}", self.current_tok)); //add plain text to html 
                            self.next_token(); // move to next token
                        }
                    }
                }
                self.html_output.push_str("</i>"); // end italic html
                if self.current_tok == "#MKAY" { //moves to next token after #MKAY
                    self.next_token();
                }
            }

            "SOUNDZ" => { // if next token is SOUNDZ
                println!("Sound element start");
                self.next_token(); // move to next token
                let url = self.current_tok.clone(); // store token as "url"
                self.next_token(); // move to next token
                if self.current_tok != "#MKAY" { //if it does not end with #MKAY, 
                    eprintln!("Syntax Error: Missing #MKAY after SOUNDZ link"); //spit out error
                    std::process::exit(1); //exit program
                }
                self.next_token(); // move to next token
                self.html_output.push_str(&format!( //add for html
                    "<audio controls><source src=\"{}\" type=\"audio/mpeg\"></audio>", url 
                ));
            },

            "VIDZ" => { // if next token is VIDZ
                println!("Video element start");
                self.next_token(); // move to next token
                let url = self.current_tok.clone(); // store token as "url"
                self.next_token(); // move to next token
                if self.current_tok != "#MKAY" { //if it does not end with #MKAY, spit out error
                    eprintln!("Syntax error: Missing #MKAY after VIDZ link");
                    std::process::exit(1); ////exit program
                }
                self.next_token(); // move to next token
                self.html_output.push_str(&format!( //add for html
                        "<iframe src=\"{}\"></iframe>", url
                ));
            },

            "NEWLINE" => { // if next token is NEWLINE
                println!("Newline element");
                self.html_output.push_str("<br>\n"); //put <br> in html and do linebreak
                self.next_token(); // move to next token 
            },

            _ => {
                println!("Unknown inline: {}", self.current_tok); //all other tokens are unknown, spit out error
                self.next_token(); // move to next token 
            }
        }
    }

    fn variable(&mut self) {
        // <Variable> ::= "#I" "HAZ" <Var_Name> "#IT" "IZ" <Text> "#MKAY"
        println!("Parsing <Variable>...");
        self.next_token(); // move to next token 

        if self.current_tok != "HAZ" {//make sure next variable is HAZ
            eprintln!("Syntax error: expected 'HAZ' after #I, found '{}'", self.current_tok); //if not get mad, throw error 
            std::process::exit(1);  //exit program
        }
        self.next_token(); // move to next token 

        let name = self.current_tok.clone(); //// Get variable name
        println!("Variable name: {}", name); 
        self.next_token(); // move to next token 

        if self.current_tok != "#IT" { //make sure next token is #IT
            eprintln!("Syntax error: expected '#IT' after, found '{}'", self.current_tok); //if not get mad, throw error 
            std::process::exit(1); //exit program
        }
        self.next_token(); // move to next token 

        if self.current_tok != "IZ" { //make sure next token is #IZ
            eprintln!("Syntax error: expected 'IZ' after, found '{}'", self.current_tok); //if not get mad, throw error 
            std::process::exit(1); //exit program
        }
        self.next_token(); // move to next token 

        //store tokens until #MKAY
        let mut value_parts = Vec::new(); 
        while self.current_tok != "#MKAY" && !self.current_tok.is_empty() { 
            value_parts.push(self.current_tok.clone());
            self.next_token(); // move to next token 
        }

        let value = value_parts.join(" ");
        println!("Variable value: {}", value);

        // Store variable name and content
        self.varName.push(name);
        self.varContent.push(value);

        if self.current_tok == "#MKAY" {// makes sure variable definition ends with #MKAY
            self.next_token(); // move to next token 
        } else { //other wise get mad
            eprintln!("Syntax error: missing '#MKAY' at end of variable definition."); // throw error 
            std::process::exit(1); //exit program
        }
    }

    fn var_use(&mut self) {
        // <Var_Use> ::= "#LEMME" "SEE" <Var_Name> "#MKAY"
        println!("Parsing <Var_Use>...");
        self.next_token();  // move to next token 

        if self.current_tok != "SEE" { //make sure next token is SEE
            eprintln!("Syntax error: expected 'SEE' after #LEMME, found '{}'", self.current_tok); //if not get mad, throw error
            std::process::exit(1); //exit program
        }
        self.next_token(); // move to next token 

        let name = self.current_tok.clone(); //make a string that is a copy of this token
        println!("Using variable: {}", name);

        // Look up variable name in the varName list, "pos" is the index that we are looking up to match
        if let Some(pos) = self.varName.iter().position(|n| *n == name) {
            let value = &self.varContent[pos];
            self.html_output.push_str(&format!("{}", value)); //put the variable in the html
            println!("Inserted variable value into HTML: {}", value); 
        } else { //could not find variable name in the varName list
            eprintln!("Semantic error: variable '{}' not defined.", name); //get mad, throw Semantic error
            std::process::exit(1); //and exit program
        }

        self.next_token(); // move to next token 
        if self.current_tok == "#MKAY" { //make sure next token is #MKAY
            self.next_token(); // move to next token 
        } else { //otherwise 
            eprintln!("Syntax error: missing '#MKAY' after variable use."); //get mad, throw error
            std::process::exit(1); //and exit program
        }
    }

}

    
    //******************* Oscar changes:
    /*
    This is the end of my compiler code for my LOLspeak Grammar
    For the rest of this, we are going back to lab5 code.
    Expect me to do the "******************* Oscar changes:" for comments and to see what changed between lab 5 
    */

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

    // ******************* Oscar changes:
    //mostly for debugging, just a section of the output that I see the parser parse through
    println!("\n        Parse Log:");

    compiler.parse();

    // ******************* Oscar changes:
    //mostly for debugging, prints out the html output
    println!("\n        HTML OUTPUT:");
    println!("{}", compiler.html_output);

    // ******************* Oscar changes:
    //for outputing the html into a file
    let mut output_file = File::create("output.html")
        .expect("Error: Could not create html file.");
    write!(output_file, "{}", compiler.html_output)
        .expect("Error: Failed to write HTML output.");
    println!("\n HTML output has been written to output.txt");

    
}
