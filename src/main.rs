use std::{env::args, fs::File, io::BufRead, str::FromStr};

use num_bigint::BigInt;
use num_traits::ToPrimitive;

struct Estate {
    stack: Vec<BigInt>,
    before: BigInt,
}

impl Estate {
    fn new() -> Estate {
        Estate {
            stack: Vec::new(),
            before: BigInt::ZERO,
        }
    }

    fn top(&self) -> BigInt {
        if self.stack.is_empty() {
            BigInt::ZERO
        } else {
            self.stack.last().unwrap().clone()
        }
    }

    fn execute_instruction(&mut self, ls: Vec<String>) {
        let ls: Vec<_> = ls
            .into_iter()
            .map(|e| {
                if !e.is_empty() && e.ends_with("\n") {
                    e.chars().take(e.len() - 1).collect()
                } else {
                    e
                }
            })
            .collect();
        if ls.is_empty() {
            return;
        }
        let ant = if self.stack.is_empty() {
            BigInt::ZERO
        } else {
            self.stack.last().unwrap().clone()
        };
        if ls.len() == 1 && ls.first().unwrap().to_uppercase() == "PRINT" {
            println!("{}", ant);
            return;
        }
        let mut newvars = self.stack.clone();
        for c in ls {
            if let Ok(num) = BigInt::from_str(c.as_str()) {
                newvars.push(num);
            } else {
                match c.to_uppercase().as_str() {
                    "POP" => {
                        let _ = newvars.pop();
                    }
                    "BEFORE" => newvars.push(ant.clone()),
                    "+" | "-" | "/" | "*" | "POW" => {
                        if newvars.len() < 2 {
                            eprintln!(
                                "Bad instruction, {}, not enough values in stack: {:?}",
                                c, self.stack
                            );
                            return;
                        }
                        let a = newvars.pop().unwrap();
                        let b = newvars.pop().unwrap();
                        if c.to_uppercase().as_str() == "POW" {
                            // No exacto, igualmente petara de igual manera
                            newvars.push(a.pow(b.to_u32().unwrap_or(u32::MAX)));
                        } else {
                            match c.as_str() {
                                "+" => newvars.push(&a + &b),
                                "-" => newvars.push(&a - &b),
                                "/" => newvars.push(if b == BigInt::ZERO {
                                    BigInt::from(10).pow(10000)
                                } else {
                                    &a / &b
                                }),
                                "*" => newvars.push(&a * &b),
                                _ => {}
                            }
                        }
                    }
                    _ => {
                        eprintln!("Bad instruction, {} stack:{:?}", c, self.stack);
                        return;
                    }
                }
            }
        }
        // Al final para evitar fallos de excepcion
        self.stack = newvars;
        self.before = ant;
    }
}

fn interactivo() {
    let mut est = Estate::new();
    loop {
        let mut buff = String::new();
        let _ = std::io::stdin().read_line(&mut buff);
        let inst: Vec<_> = buff.split(" ").map(String::from).collect();
        if inst.len() >= 1 && inst.first().unwrap().starts_with("EXIT") {
            break;
        }
        est.execute_instruction(inst);
        println!("{}", est.top())
    }
}

fn ejecutar_archivo(path: String, est: &mut Estate) {
    let Ok(arch) = File::open(path.clone()) else {
        eprintln!("Archivo no exitente: {}", path);
        return;
    };
    let ln = std::io::BufReader::new(arch);
    for inst in ln.lines() {
        let inst: Vec<_> = inst.unwrap().split(" ").map(String::from).collect();
        est.execute_instruction(inst);
    }
}

fn main() {
    let args: Vec<_> = args().skip(1).collect();
    if args.is_empty() {
        interactivo();
    } else {
        let mut est = Estate::new();
        for path in args {
            ejecutar_archivo(path, &mut est);
        }
    }
}
