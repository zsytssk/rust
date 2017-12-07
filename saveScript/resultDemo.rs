fn main() {
    let result = parse_context(Context("hello"));
    println!("{:?}", result);
}

fn parse_context(context: Context) -> Result<&str, String> {
    Parser{context: &context}.parse()
}

struct Context<'s>(&'s str);

struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c,'s> {
    fn parse(&self) -> Result<&'s str, String> {
        let len = &self.context.0.len();
        let max = 10;
        if *len > max {
            Ok(&self.context.0[..])
        } else {
            let err_str =format!("string not long enogh:{}", max);
            Err(String::from(&err_str[..]))
        }
    }
}

struct Ref<'a, T: 'a>(&'a T);