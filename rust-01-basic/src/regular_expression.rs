// -------------------------------------------
// 			Regular Expression Basics
// -------------------------------------------

extern crate regex;
use regex::Regex;
#[test]
fn main() {
    // let re = Regex::new(r"[prt].ain").unwrap();
    // let text = "rrrain spain none";

    // // println!("The text has a match {:?}", re.is_match(text));
    // // println!("The text has a match {:?}", re.find(text));

    // for cap in re.captures_iter(text) {
    //     println!("match: {:?}", &cap[0]);
    // }

    // let re = Regex::new(r"gr[ae]y").unwrap();
    // let text = "gray grey graye";

    // for cap in re.captures_iter(text) {
    //     println!("match: {:?}", &cap[0]);
    // }

    // let re = Regex::new(r"[^a-z]ain").unwrap();
    // let text = "main pain tain rain but not 0ain";

    // for cap in re.captures_iter(text) {
    //     println!("match: {:?}", &cap[0]);
    // }


    // let re = Regex::new(r"\d\d\d\d\d\d").unwrap();
    // let re = Regex::new(r"^\d......").unwrap();
    // let text = "My phone number is 816030 and the second phone number is 816694";

    // for cap in re.captures_iter(text) {
    //     println!("match: {:?}", &cap[0]);
    // }

    // let re = Regex::new(r"^aba").unwrap();
    // let text  = "ba abaa bc";

    // for cap in re.captures_iter(text) {
    //     println!("match: {:?}", &cap[0]);
    // }

    //let re = Regex::new(r"bc$").unwrap();
    // let re = Regex::new(r"^bc$").unwrap();
    // let text = "bc";

    // //let text = "aba abc bc";

    // for cap in re.captures_iter(text) {
    //     println!("match: {:?}", &cap[0]);
    // }

    // let re = Regex::new(r"^\d\d$");
    // let text = "89";

    let re = Regex::new(r"\b\w*").unwrap();
    let text = "Hi my name is nouman";

    for cap in re.captures_iter(text) {
        println!("match: {:?}", &cap[0]);
    }
}



// -------------------------------------------
// 			Regexes- Repeatitions and Quantifiers
// -------------------------------------------

#[test]
fn main_3(){
    //let re = Regex::new(r"a?aa").unwrap();
    //let text = "aa aaa";

    // let re = Regex::new(r"ba?").unwrap();
    // let text = "a ba b ba";

    // let re = Regex::new(r"\w?\w?\w?.rs").unwrap();
    // let text = "fil.rs t1.rs file.rs";

    // for cap in re.captures_iter(text) {
    //     println!("Match: {} ", &cap[0]);
    // }

    // let re = Regex::new(r"a+").unwrap();
    // let text = "a aa aaa baab bab";

    // let re = Regex::new(r"\w+\.gif").unwrap();
    // let text = "image1.gif and background.gif";

    // let re = Regex::new(r"ab*").unwrap();
    // let text = "a ab abbbbb";

    // for cap in re.captures_iter(text) {
    //     println!("Match: {} ", &cap[0]);
    // }

    //    let re = Regex::new(r"\b\w{3,5}\b").unwrap();
    //    let text = "Hello i think you are happy becuase i have a gift for you";

    // let re = Regex::new(r"\b\d{1,3}\.\d{1,3}\b").unwrap();
    // let text = "921.583 0.0 1456.25";

    //     let re = Regex::new(r"\d{3,}").unwrap();
    //     let text = "5321 500 5698 12";

    //    for cap in re.captures_iter(text) {
    //     println!("Match: {} ", &cap[0]);
    // }


    let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    let text = "2012-03-14, 2013-01-01 and 2014-07-05";

    for cap in re.captures_iter(text) {
        println!("Month: {} Day: {} Year: {}, the whole: {}", &cap[2], &cap[3], &cap[1], &cap[0]);
    }

}



// -------------------------------------------
//           	     String Literal
// -------------------------------------------

#[test]
fn main4() {
    let str = r"The main said _Hello world_ \n \t ' ";
    println!("{}", str);

    let jason_str = "{
        \" name \": \"Micheal\",
        \"age\": 40,
        \"sex\": Male
    }";

    let jason_str1 = r#"{
        " name ": "Micheal",
        "age": 40,
        "sex": Male
    }"#;

    let str = r###"Hello"## World!"###;

    // Exercise for you

    let string1 = r#"""#; // "
    let string2 = r#""""""""#; // """"""
    let string3 = r#" He asked,"Is rust awesome?"""#; // He asked,"Is rust awesome?"
    println!("{}", string1);
    println!("{}", string2);
    println!("{}", string3);
}
