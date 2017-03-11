/* This example doesn't compile because lifetime parameter is not declared
in function skip_prefix.
The function skip_prefix return type contains a borrowed value,
but the signature does not say whether it is borrowed from `line` or `prefix`

fn skip_prefix(line: &str, prefix: &str) -> &str {
    line
}

fn main(){
    let line = "lang:en=Hello World!";
    let lang = "en";

    let v;
    {
        let p = format!("lang:{}=", lang);  // -+ `p` comes into scope.
        v = skip_prefix(line, p.as_str());  //  |
    }                                       // -+ `p` goes out of scope.
    println!("{}", v);
} */

// FIXED:
fn skip_prefix<'a,'b>(line: &'a str, prefix: &'b str) -> &'a str {
    line
}

fn main(){
    let line = "lang:en=Hello World!";
    let lang = "en";

    let v;
    {
        let p = format!("lang:{}=", lang);  // -+ `p` comes into scope.
        v = skip_prefix(line, p.as_str());  //  |
    }                                       // -+ `p` goes out of scope.
    println!("{}", v);
}
