#![allow(dead_code, unused_variables, unused_imports)]
use fabric_ledger_protos::{common_messages, ledger_messages};
use protobuf::{parse_from_bytes, Message};

#[derive(Debug, Clone)]
pub enum ROLE {
    MEMBER,
    PEER,
    ADMIN,
    CLIENT,
}

#[derive(Debug, Clone)]
pub enum Expression {
    AND(Vec<Expression>),
    OR(Vec<Expression>),
    OUTOF(Vec<Expression>, usize),
    Principal(String, ROLE),
}

#[derive(Debug)]
pub struct StateBasedEndorsement {
    root: Expression,
}

impl StateBasedEndorsement {
    pub fn build(expr: Expression) -> Self {
        StateBasedEndorsement { root: expr }
    }
}
use Expression::*;
fn main() {
    println!("Hello");

    let p1 = Principal("ORG1".to_string(), ROLE::PEER);
    let p2 = Principal("ORG2".to_string(), ROLE::PEER);
    let p3 = Principal("ORG3".to_string(), ROLE::PEER);

    //let sbe = StateBasedEndorsement::build(p1);
    // println!("Rust Structure.... {:#?}", sbe);

    let p1 = Principal("ORG1".to_string(), ROLE::PEER);
    let p2 = Principal("ORG2".to_string(), ROLE::PEER);
    let p3 = Principal("ORG3".to_string(), ROLE::PEER);
    let p4 = Principal("ORG4".to_string(), ROLE::PEER);
    let sbe = StateBasedEndorsement::build(AND(vec![p1, OR(vec![p3, p4])]));

    //let sbe = StateBasedEndorsement::build(AND(vec![OR(vec![p1, p2]), OR(vec![p3, p4])]));
    //let sbe = StateBasedEndorsement::build(AND(vec![p1, p2]));
    println!("Rust Structure....  {:#?}", sbe);

    let mut ep = ledger_messages::EndorsementPolicy::new();
    let mut r = ledger_messages::EndorsementRule::new();
    match_expr(&sbe.root, &mut r);
    ep.set_rule(r);

    // create the buffer to send
    let buffer = ep.write_to_bytes().unwrap();
    let ep = parse_from_bytes::<ledger_messages::EndorsementPolicy>(&buffer).unwrap();
    let e = match read_policy(&ep.get_rule()) {
        OUTOF(e, 0) => e[0].clone(),
        _ => unreachable!(),
    };
    let sbe = StateBasedEndorsement::build(e);
    println!("Rust Structure.... {:#?}", sbe);
}

fn read_policy(r: &ledger_messages::EndorsementRule) -> Expression {
    let min = r.get_min_endorsements();
    let rules = r.get_rules();
    let principals = r.get_principals();

    let mut vec_rules = Vec::new();
    let rules_iter = rules.into_iter();
    for r in rules_iter {
        vec_rules.push(read_policy(&r));
    }

    let principals_iter = principals.into_iter();

    let mut vec_principals = Vec::new();
    for p in principals_iter {
        match p.get_role() {
            ledger_messages::EndorsementPrincipal_Role::MEMBER => {
                vec_principals.push(Principal(p.get_msp_id().to_string(), ROLE::MEMBER));
            }
            ledger_messages::EndorsementPrincipal_Role::ADMIN => {
                vec_principals.push(Principal(p.get_msp_id().to_string(), ROLE::PEER));
            }
            ledger_messages::EndorsementPrincipal_Role::CLIENT => {
                vec_principals.push(Principal(p.get_msp_id().to_string(), ROLE::PEER));
            }
            ledger_messages::EndorsementPrincipal_Role::PEER => {
                vec_principals.push(Principal(p.get_msp_id().to_string(), ROLE::PEER));
            }
        }
    }

    let num_elements = (vec_principals.len() + vec_rules.len()) as i32;

    if (vec_principals.len() == 1 && vec_rules.len() == 0) {
        return vec_principals[0].clone();
    } else if min == 1 {
        let concatenated = [&vec_rules[..], &vec_principals[..]].concat();
        return Expression::OR(concatenated);
    } else if min == num_elements {
        let concatenated = [&vec_rules[..], &vec_principals[..]].concat();
        return Expression::AND(concatenated);
    } else {
        let concatenated = [&vec_rules[..], &vec_principals[..]].concat();
        return Expression::OUTOF(concatenated, min as usize);
    };
}

fn match_expr(expr: &Expression, rule: &mut ledger_messages::EndorsementRule) {
    match expr {
        AND(e) => {
            let mut r = ledger_messages::EndorsementRule::new();
            let repeated_rules: Vec<ledger_messages::EndorsementRule> = Vec::new();
            let e_iter = e.iter();
            for subexpre in e_iter {
                match_expr(subexpre, &mut r);
            }
            let min_endoresemtns: usize = r.get_principals().len() + r.get_rules().len();
            r.set_min_endorsements(min_endoresemtns as i32); // OR so it is set to 1

            rule.mut_rules().push(r);
        }
        OR(e) => {
            let mut r = ledger_messages::EndorsementRule::new();
            r.set_min_endorsements(1); // OR so it is set to 1
            let e_iter = e.iter();
            for subexpre in e_iter {
                match_expr(subexpre, &mut r);
            }
            rule.mut_rules().push(r);
        }
        OUTOF(e, i) => {
            let mut r = ledger_messages::EndorsementRule::new();
            let e_iter = e.iter();
            for subexpre in e_iter {
                match_expr(subexpre, &mut r);
            }
            r.set_min_endorsements(*i as i32);
            rule.mut_rules().push(r);
        }
        Principal(name, role) => {
            let mut p = ledger_messages::EndorsementPrincipal::new();
            p.set_msp_id(name.clone());
            match role {
                ROLE::MEMBER => p.set_role(ledger_messages::EndorsementPrincipal_Role::MEMBER),
                ROLE::PEER => {
                    p.set_role(ledger_messages::EndorsementPrincipal_Role::PEER);
                }
                ROLE::ADMIN => p.set_role(ledger_messages::EndorsementPrincipal_Role::ADMIN),
                ROLE::CLIENT => p.set_role(ledger_messages::EndorsementPrincipal_Role::CLIENT),
            };
            rule.mut_principals().push(p);
        }
    }
}
