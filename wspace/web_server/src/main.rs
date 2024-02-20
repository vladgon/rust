use wg_macro::sql;

mod log;

fn main() {
    warn!("Test Warn1");
    warn!("Test Warn2");
}

#[sql(db_url = "mysql://localhost:3306/sample")]
//having derive in multi line to test macro
#[derive(Default)]
#[derive(Debug)]
pub struct Sample<'a, T> where T: Into<String> {
    pub name: T,
    last_name: &'a str,
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod test {
        use crate::sample;
        use crate::Sample;

        #[test]
        fn sample_macro() {
            println!("{:?}", Sample::<String>::default());
            println!("{}", sample);
            println!("{}", Sample::<String>::default().get_url())
        }
    }
}
