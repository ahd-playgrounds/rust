#![allow(dead_code)]
use anyhow::Context;
use anyhow::Result;
use lib::failer;

pub fn add(left: i32, right: i32) -> Result<i32> {
    if left > 20 {
        failer().context("oh dear")?;
        Ok(2)
    } else {
        Ok(left + right)
    }
}

mod lib {
    use thiserror::Error;

    #[derive(Error, Debug, PartialEq)]
    pub enum MyErr {
        #[error("we've all been there")]
        MathHard(#[from] std::num::ParseIntError),
        #[error("number was invalid `{0}`")]
        OhDear(String),
        #[error("go {found} expected {expected}")]
        DarnIt { expected: String, found: String },
        #[error("raise a ticket")]
        Mystery,
    }

    pub fn failer() -> Result<(), MyErr> {
        let err = "NaN".parse::<u32>().unwrap_err();
        Err(MyErr::MathHard(err))
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_failer() {
            let e = failer().unwrap_err();
            assert_eq!(e, MyErr::Mystery);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lib::MyErr;

    use super::*;

    #[test]
    fn it_works() {
        let result = add(22, 2);

        match result {
            Ok(_) => (),
            Err(e) => panic!("{e:#?}"),
        }

        // let err = result.unwrap_err();
        // err.chain().for_each(|res| {
        //     println!("{res}");
        // });
        // assert!(false)
        //
        // assert_eq!(err.root_cause().to_string(), MyErr::Mystery.to_string());
    }
}
