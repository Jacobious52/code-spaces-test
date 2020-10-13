extern crate nom;
use nom::{
  IResult,
  bytes::complete::{tag, take_until},
  character::{
    complete::{multispace1},
  }
};

fn main() {}

#[derive(Debug, PartialEq)]
struct Rule {

}

fn rule(input: &str) -> IResult<&str, Rule> {

    let (input, _) = tag("-A")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, chain) = take_until(" ")(input)?;
    Ok((input, Rule {}))
}

#[test]
fn parse_rule() {
    assert_eq!(rule("-A INPUT -m conntrack --ctstate RELATED,ESTABLISHED -j ACCEPT"), Ok(("", Rule{

    })));
}