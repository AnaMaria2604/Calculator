use std::io;
#[derive(Debug, Clone, PartialEq)]
enum CaracterePosibile {
    ParantezaDeschisa,
    ParantezaInchisa,
    Numar(f64),
    Op(char),
    Rad,
    Log,
    Sin,
    Cos,
    Tangenta,
}

enum ExpresieData {
    Operatie(Box<ExpresieData>, char, Box<ExpresieData>), //enum la op
    Rad(Box<ExpresieData>),
    Log(Box<ExpresieData>),
    Sin(Box<ExpresieData>),
    Cos(Box<ExpresieData>),
    Tangenta(Box<ExpresieData>),
    Numar(f64),
}

fn caractere_expresie(caractere: CaracterePosibile) -> Option<ExpresieData> {
    //transformare dintr-un tip de enum in altul
    match caractere {
        CaracterePosibile::Numar(val) => Some(ExpresieData::Numar(val)),
        CaracterePosibile::Rad => Some(ExpresieData::Rad(Box::new(ExpresieData::Numar(0.0)))),
        CaracterePosibile::Sin => Some(ExpresieData::Sin(Box::new(ExpresieData::Numar(0.0)))),
        CaracterePosibile::Cos => Some(ExpresieData::Cos(Box::new(ExpresieData::Numar(0.0)))),
        CaracterePosibile::Tangenta => {
            Some(ExpresieData::Tangenta(Box::new(ExpresieData::Numar(0.0))))
        }
        _ => None,
    }
}

fn expresie_numar(expresie: &ExpresieData) -> Option<f64> {
    //transformam din enum in numar
    match expresie {
        ExpresieData::Numar(nr) => Some(*nr),
        _ => None,
    }
}

fn prioritatea_operatorilor(operator: char) -> i64 {
    match operator {
        '^' => 3,
        '*' => 2,
        '/' => 2,
        '+' => 1,
        '-' => 1,
        _ => 0,
    }
}

fn formare_operatia_finala(operatie_data: &str) -> Result<Vec<CaracterePosibile>, &'static str> {
    let mut caractere = Vec::new();
    let mut numere = String::new();

    for ch in operatie_data.chars() {
        match ch {
            '0'..='9' | '.' => {
                numere.push(ch);
            }
            _ => {
                if !numere.is_empty() {
                    let nr_nou = if let Ok(ok) = numere.parse::<f64>() {
                        ok
                    } else {
                        return Err("Nu se poate efectua aceasta operatie.");
                    };
                    caractere.push(CaracterePosibile::Numar(nr_nou));
                    numere.clear();
                }

                match ch {
                    'r' => caractere.push(CaracterePosibile::Rad),
                    'l' => caractere.push(CaracterePosibile::Log),
                    's' => caractere.push(CaracterePosibile::Sin),
                    'c' => caractere.push(CaracterePosibile::Cos),
                    't' => caractere.push(CaracterePosibile::Tangenta),

                    '(' => caractere.push(CaracterePosibile::ParantezaDeschisa),
                    ')' => caractere.push(CaracterePosibile::ParantezaInchisa),

                    '^' => caractere.push(CaracterePosibile::Op('^')),
                    '+' => caractere.push(CaracterePosibile::Op('+')),
                    '-' => caractere.push(CaracterePosibile::Op('-')),
                    '*' => caractere.push(CaracterePosibile::Op('*')),
                    '/' => caractere.push(CaracterePosibile::Op('/')),
                    _ => (),
                }
            }
        }
    }

    if !numere.is_empty() {
        let nr_nou = if let Ok(ok) = numere.parse::<f64>() {
            ok
        } else {
            return Err("Nu se poate efectua aceasta operatie.");
        };
        caractere.push(CaracterePosibile::Numar(nr_nou));
    }

    Ok(caractere)
}

fn impartire_operatii(
    caractere: &mut Vec<CaracterePosibile>,
    poz: &mut usize,
) -> Result<ExpresieData, &'static str> {
    let mut numere: Vec<ExpresieData> = Vec::new();
    let mut operatori: Vec<char> = Vec::new();

    while caractere.len() > *poz {
        match &caractere[*poz] {
            CaracterePosibile::Rad => {
                *poz += 1;
                let nr = if let Some(ok) = caractere_expresie(caractere[*poz].clone()) {
                    ok
                } else {
                    eprintln!("Nu se poate efectua aceasta operatie.");
                    ExpresieData::Numar(0.0)
                };
                numere.push(ExpresieData::Numar(calculare(&ExpresieData::Rad(
                    Box::new(nr),
                ))?));
            }
            CaracterePosibile::Log => {
                *poz += 1;
                let nr = if let Some(ok) = caractere_expresie(caractere[*poz].clone()) {
                    ok
                } else {
                    eprintln!("Nu se poate efectua aceasta operatie.");
                    ExpresieData::Numar(0.0)
                };
                numere.push(ExpresieData::Numar(calculare(&ExpresieData::Log(
                    Box::new(nr),
                ))?));
            }
            CaracterePosibile::Sin => {
                *poz += 1;
                let nr = if let Some(ok) = caractere_expresie(caractere[*poz].clone()) {
                    ok
                } else {
                    eprintln!("Nu se poate efectua aceasta operatie.");
                    ExpresieData::Numar(0.0)
                };
                let number = if let Some(nmb) = expresie_numar(&nr) {
                    nmb.to_radians()
                } else {
                    0.0
                };
                numere.push(ExpresieData::Numar(calculare(&ExpresieData::Sin(
                    Box::new(ExpresieData::Numar(number)),
                ))?));
            }
            CaracterePosibile::Cos => {
                *poz += 1;
                let nr = if let Some(ok) = caractere_expresie(caractere[*poz].clone()) {
                    ok
                } else {
                    eprintln!("Nu se poate efectua aceasta operatie.");
                    ExpresieData::Numar(0.0)
                };
                let number = if let Some(nmb) = expresie_numar(&nr) {
                    nmb.to_radians()
                } else {
                    0.0
                };
                numere.push(ExpresieData::Numar(calculare(&ExpresieData::Cos(
                    Box::new(ExpresieData::Numar(number)),
                ))?));
            }
            CaracterePosibile::Tangenta => {
                *poz += 1;
                let nr = if let Some(ok) = caractere_expresie(caractere[*poz].clone()) {
                    ok
                } else {
                    eprintln!("Nu se poate efectua aceasta operatie.");
                    ExpresieData::Numar(0.0)
                };
                let number = if let Some(nmb) = expresie_numar(&nr) {
                    nmb.to_radians()
                } else {
                    0.0
                };
                numere.push(ExpresieData::Numar(calculare(&ExpresieData::Tangenta(
                    Box::new(ExpresieData::Numar(number)),
                ))?));
            }
            CaracterePosibile::ParantezaDeschisa => {
                *poz += 1;
                let op_intre_paranteze: ExpresieData = impartire_operatii(caractere, poz)?;
                numere.push(op_intre_paranteze);
                *poz -= 1;
            }
            CaracterePosibile::ParantezaInchisa => {
                *poz += 1;
                break;
            }
            CaracterePosibile::Numar(nr) => {
                numere.push(ExpresieData::Numar(*nr));
            }
            CaracterePosibile::Op(operator) => {
                while let Some(&varf_op) = operatori.last() {
                    if prioritatea_operatorilor(varf_op) >= prioritatea_operatorilor(*operator) {
                        let partea_dr = if let Some(ok) = numere.pop() {
                            ok
                        } else {
                            return Err("Nu se poate efectua aceasta operatie.");
                        };
                        let partea_st = if let Some(ok) = numere.pop() {
                            ok
                        } else {
                            return Err("Nu se poate efectua aceasta operatie.");
                        };
                        numere.push(ExpresieData::Operatie(
                            Box::new(partea_st),
                            varf_op,
                            Box::new(partea_dr),
                        ));
                        operatori.pop();
                    } else {
                        break;
                    }
                }
                operatori.push(*operator);
                if let Some(CaracterePosibile::Op('^')) = caractere.get(*poz + 1) {
                    caractere.remove(*poz + 1);
                } else if let Some(CaracterePosibile::Op('+')) = caractere.get(*poz + 1) {
                    caractere.remove(*poz + 1);
                } else if let Some(CaracterePosibile::Op('-')) = caractere.get(*poz + 1) {
                    caractere.remove(*poz + 1);
                } else if let Some(CaracterePosibile::Op('*')) = caractere.get(*poz + 1) {
                    caractere.remove(*poz + 1);
                } else if let Some(CaracterePosibile::Op('/')) = caractere.get(*poz + 1) {
                    caractere.remove(*poz + 1);
                }
            }
        }
        *poz += 1;
    }

    while let Some(&varf_op) = operatori.last() {
        let partea_dr = if let Some(ok) = numere.pop() {
            ok
        } else {
            return Err("Nu se poate efectua aceasta operatie.");
        };
        let partea_st = if let Some(ok) = numere.pop() {
            ok
        } else {
            return Err("Nu se poate efectua aceasta operatie.");
        };
        numere.push(ExpresieData::Operatie(
            Box::new(partea_st),
            varf_op,
            Box::new(partea_dr),
        ));
        operatori.pop();
    }

    if let Some(ok) = numere.pop() {
        Ok(ok)
    } else {
        Err("Nu se poate efectua aceasta operatie.")
    }
}

fn ridicare_la_putere(a: f64, b: f64) -> f64 {
    let mut nr = 0.0;
    let mut rezultat = 1.0;
    while nr != b {
        rezultat *= a;
        nr += 1.0;
    }
    rezultat
}

fn calculare(operatie: &ExpresieData) -> Result<f64, &'static str> {
    match operatie {
        ExpresieData::Operatie(partea_st, varf_op, partea_dr) => {
            let st_rezultat = calculare(partea_st)?;
            let dr_rezultat = calculare(partea_dr)?;

            let rezultat_final = match varf_op {
                '*' => st_rezultat * dr_rezultat,
                '^' => ridicare_la_putere(st_rezultat, dr_rezultat),
                '/' => st_rezultat / dr_rezultat,
                '+' => st_rezultat + dr_rezultat,
                '-' => st_rezultat - dr_rezultat,
                _ => return Err("Nu este un operator bun."),
            };
            println!(
                "={}{}{}={}",
                st_rezultat, varf_op, dr_rezultat, rezultat_final
            );
            Ok(rezultat_final)
        }
        ExpresieData::Numar(nr) => Ok(*nr),
        ExpresieData::Rad(nr) => {
            let val = calculare(nr)?;
            Ok(val.sqrt())
        }
        ExpresieData::Log(nr) => {
            let val = calculare(nr)?;
            Ok(val.ln())
        }
        ExpresieData::Sin(nr) => {
            let val = calculare(nr)?;
            Ok(val.sin())
        }
        ExpresieData::Cos(nr) => {
            let val = calculare(nr)?;
            Ok(val.cos())
        }
        ExpresieData::Tangenta(nr) => {
            let val = calculare(nr)?;
            Ok(val.tan())
        }
    }
}

fn main() {
    println!(" ");
    println!("Operatii posibile si acceptate, avand in vedere ca avem variabilele dumneavoastra var, var1:");
    println!("Adunare: var + var1");
    println!("Scadere: var - var1");
    println!("Ridicare la putere: var ^ var1");
    println!("Inmultire: var * var1");
    println!("Impartire: var / var1");
    println!("Radical: r var");
    println!("Logaritm: l var");
    println!("Sinus: s var");
    println!("Cosinus: c var");
    println!("Tangenta: t var");
    println!(" ");

    println!("Verificare: 2^3={}", ridicare_la_putere(2.0, 3.0));
    println!(" ");

    println!("Introduceti operatia ce doriti sa fie calculata pe pasi:");
    let mut operatie_de_calculat = String::new();
    match io::stdin().read_line(&mut operatie_de_calculat) {
        Err(eroare) => {
            eprintln!("A existat o eroare in incercarea de citire de la tastatura: {eroare}");
        }
        Ok(_) => {
            println!("S-a putut citi operatia ce doriti sa fie calculata;");
            println!("Operatia introdusa arata astfel: {}", operatie_de_calculat);

            let operatie_data: String = operatie_de_calculat
                .chars()
                .filter(|&ch| !ch.is_whitespace())
                .collect();

            match formare_operatia_finala(&operatie_data) {
                Err(eroare) => {
                    eprintln!(
                        "A existat o eroare la formarea elementelor din operatia data: {eroare}"
                    );
                }
                Ok(caractere) => {
                    let mut poz = 0;
                    match impartire_operatii(&mut caractere.clone(), &mut poz) {
                        Err(eroare) => {
                            eprintln!(
                                "A existat o eroare la impartirea in operatii mai mici a operatiei data: {eroare}"
                            );
                        }
                        Ok(operatie) => match calculare(&operatie) {
                            Err(eroare) => {
                                eprintln!(
                                    "A existat o eroare la calcularea operatiei data: {eroare}"
                                );
                            }
                            Ok(nr) => {
                                println!("Rezultat final: {}", nr);
                                println!("S-a putut calcula cu succes operatia data!");
                            }
                        },
                    }
                }
            }
        }
    }
}
