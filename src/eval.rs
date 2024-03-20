use std::iter::Sum;
use image::{DynamicImage, GenericImageView, Rgba};
use rand::prelude::ThreadRng;
use rand::{random, Rng};
use crate::parser::Token;

#[derive(Debug, Clone, Copy)]
struct sum {
    r: u8,
    g: u8,
    b: u8
}

#[derive(Debug)]
struct sum_save {
    v_y: Option<sum>,
    v_b: Option<sum>,
    v_e: Option<sum>,
    v_r: Option<sum>,
    v_h: Option<sum>,
    v_v: Option<sum>,
    v_d: Option<sum>,
    v_H: Option<sum>,
    v_L: Option<sum>
}

impl sum_save {
    fn new() -> Self {
        sum_save {
            v_y: None,
            v_b: None,
            v_e: None,
            v_r: None,
            v_h: None,
            v_v: None,
            v_d: None,
            v_H: None,
            v_L: None
        }
    }
}

pub fn eval(x: u32, y: u32, width: u32, height: u32, r: u8, g: u8, b: u8, a: u8, sr: u8, sg: u8, sb: u8, input: &DynamicImage, rng: &mut ThreadRng, tokens: Vec<Token>) -> Result<Rgba<u8>, String> {
    if a == 0 {
        return Ok(Rgba([0, 0, 0, 0]));
    }

    let mut stack: Vec<sum> = Vec::with_capacity(tokens.len());

    let div = |a: u8, b: u8| -> u8 {
        if b == 0 {
            return a;
        }
        a.wrapping_div(b)
    };

    let modu = |a: u8, b: u8| -> u8 {
        if b == 0 {
            return a;
        }
        a.wrapping_rem(b)
    };

    let bit_and_not = |a: u8, b: u8| -> u8 {
        a & !b
    };

    let weight = |a: u8, b: u8| -> u8 {
        let fuzz = f64::from(b) / 255.0;
        let r = f64::from(a) * fuzz;
        r as u8
    };

    let three_rule = |x: u32, max: u32| -> u8 {
        (((255 * x) / max) & 255) as u8
    };

    let is_in_bounds = |x: u32, y: u32, width: u32, height: u32| -> bool {
        x < width && y < height
    };

    let mut saved = sum_save::new();
    let mut boxxed: [sum; 9] = [sum { r: 0, g: 0, b: 0 }; 9];

    for tok in tokens {
        match tok {
            Token::Num(n) => stack.push(sum { r: n, g: n, b: n }),

            Token::Add => {
                let b = stack.pop().ok_or("Stack underflow")?;
                let a = stack.pop().ok_or("Stack underflow")?;
                stack.push(sum { r: a.r.wrapping_add(b.r), g: a.g.wrapping_add(b.g), b: a.b.wrapping_add(b.b) });
            },

            Token::Sub => {
                let b = stack.pop().ok_or("Stack underflow")?;
                let a = stack.pop().ok_or("Stack underflow")?;
                stack.push(sum { r: a.r.wrapping_sub(b.r), g: a.g.wrapping_sub(b.g), b: a.b.wrapping_sub(b.b) });
            },

            Token::Mul => {
                let b = stack.pop().ok_or("Stack underflow")?;
                let a = stack.pop().ok_or("Stack underflow")?;
                stack.push(sum { r: a.r.wrapping_mul(b.r), g: a.g.wrapping_mul(b.g), b: a.b.wrapping_mul(b.b) });
            },

            Token::Div => {
                let b = stack.pop().ok_or("Stack underflow")?;
                let a = stack.pop().ok_or("Stack underflow")?;
                stack.push(sum { r: div(a.r, b.r), g: div(a.g, b.g), b: div(a.b, b.b) });
            },

            Token::Mod => {
                let b = stack.pop().ok_or("Stack underflow")?;
                let a = stack.pop().ok_or("Stack underflow")?;
                stack.push(sum { r: modu(a.r, b.r), g: modu(a.g, b.g), b: modu(a.b, b.b) });
            },

            Token::Pow => {
                let b = stack.pop().ok_or("Stack underflow")?;
                let a = stack.pop().ok_or("Stack underflow")?;
                stack.push(sum { r: a.r.wrapping_pow(b.r.into()), g: a.g.wrapping_pow(b.g.into()), b: a.b.wrapping_pow(b.b.into()) });
            },

            Token::BitAnd => {
                let b = stack.pop().ok_or("Stack underflow")?;
                let a = stack.pop().ok_or("Stack underflow")?;
                stack.push(sum { r: a.r & b.r, g: a.g & b.g, b: a.b & b.b });
            },

            Token::BitOr => {
                let b = stack.pop().ok_or("Stack underflow")?;
                let a = stack.pop().ok_or("Stack underflow")?;
                stack.push(sum { r: a.r | b.r, g: a.g | b.g, b: a.b | b.b });
            },

            Token::BitAndNot => {
                let b = stack.pop().ok_or("Stack underflow")?;
                let a = stack.pop().ok_or("Stack underflow")?;
                stack.push(sum { r: bit_and_not(a.r, b.r), g: bit_and_not(a.g, b.g), b: bit_and_not(a.b, b.b) });
            },

            Token::BitXor => {
                let b = stack.pop().ok_or("Stack underflow")?;
                let a = stack.pop().ok_or("Stack underflow")?;
                stack.push(sum { r: a.r ^ b.r, g: a.g ^ b.g, b: a.b ^ b.b });
            },

            Token::BitLShift => {
                let b = stack.pop().ok_or("Stack underflow")?;
                let a = stack.pop().ok_or("Stack underflow")?;

                let rr = a.r.wrapping_shl(b.r.into());
                let gg = a.g.wrapping_shl(b.g.into());
                let bb = a.b.wrapping_shl(b.b.into());

                stack.push(sum { r: rr, g: gg, b: bb });
            },

            Token::BitRShift => {
                let b = stack.pop().ok_or("Stack underflow")?;
                let a = stack.pop().ok_or("Stack underflow")?;

                let rr = a.r.wrapping_shr(b.r.into());
                let gg = a.g.wrapping_shr(b.g.into());
                let bb = a.b.wrapping_shr(b.b.into());

                stack.push(sum { r: rr, g: gg, b: bb });
            },

            Token::Weight => {
                let b = stack.pop().ok_or("Stack underflow")?;
                let a = stack.pop().ok_or("Stack underflow")?;

                stack.push(sum { r: weight(a.r, b.r), g: weight(a.g, b.g), b: weight(a.b, b.b) });
            },

            Token::Greater => {
                let b = stack.pop().ok_or("Stack underflow")?;
                let a = stack.pop().ok_or("Stack underflow")?;

                stack.push(sum { r: if a.r > b.r { 255 } else { 0 }, g: if a.g > b.g { 255 } else { 0 }, b: if a.b > b.b { 255 } else { 0 } });
            }

            Token::CharToken(c) => {
                match c {
                    'c' => stack.push(sum { r, g, b }),
                    'R' => stack.push(sum { r: 255, g: 0, b: 0 }),
                    'G' => stack.push(sum { r: 0, g: 255, b: 0 }),
                    'B' => stack.push(sum { r: 0, g: 0, b: 255 }),
                    'Y' => {
                        let v_y = match saved.v_y {
                            Some(v_y) => v_y,
                            None => {
                                let y = f64::from(r) * 0.299 + f64::from(g) * 0.587 + f64::from(b) * 0.0722;
                                let v_y = sum { r: y as u8, g: y as u8, b: y as u8 };
                                saved.v_y = Some(v_y);
                                v_y
                            }
                        };

                        stack.push(v_y);
                    },
                    's' => stack.push(sum { r: sr, g: sg, b: sb }),
                    'x' => {
                        let xu = three_rule(x, width);
                        stack.push(sum { r: xu, g: xu, b: xu });
                    },
                    'y' => {
                        let yu = three_rule(y, height);
                        stack.push(sum { r: yu, g: yu, b: yu });
                    },
                    'r' => {
                        let v_r = match saved.v_r {
                            Some(v_r) => v_r,
                            None => {
                                let x1 = random::<u32>() % 3;
                                let y1 = random::<u32>() % 3;

                                let x2 = random::<u32>() % 3;
                                let y2 = random::<u32>() % 3;

                                let x3 = random::<u32>() % 3;
                                let y3 = random::<u32>() % 3;

                                let p1 = match is_in_bounds(x + x1, y + y1, width, height) {
                                    true => input.get_pixel(x + x1, y + y1).0,
                                    false => [0, 0, 0, 0]
                                };

                                let p2 = match is_in_bounds(x + x2, y + y2, width, height) {
                                    true => input.get_pixel(x + x2, y + y2).0,
                                    false => [0, 0, 0, 0]
                                };

                                let p3 = match is_in_bounds(x + x3, y + y3, width, height) {
                                    true => input.get_pixel(x + x3, y + y3).0,
                                    false => [0, 0, 0, 0]
                                };

                                let v_r = sum {
                                    r: p1[0],
                                    g: p2[1],
                                    b: p3[2]
                                };

                                saved.v_r = Some(v_r);
                                v_r
                            }
                        };

                        stack.push(v_r);
                    },
                    'e' => {
                        let v_e = match saved.v_e {
                            Some(v_e) => v_e,
                            None => {
                                boxxed = fetch_boxed(input, x as i32, y as i32, r, g, b);

                                let rr = boxxed[8].r.wrapping_sub(boxxed[0].r).wrapping_add(boxxed[5].r).wrapping_sub(boxxed[3].r).wrapping_add(boxxed[7].r).
                                            wrapping_sub(boxxed[1].r).wrapping_add(boxxed[6].r).wrapping_sub(boxxed[2].r);

                                let gg = boxxed[8].g.wrapping_sub(boxxed[0].g).wrapping_add(boxxed[5].g).wrapping_sub(boxxed[3].g).wrapping_add(boxxed[7].g).
                                            wrapping_sub(boxxed[1].g).wrapping_add(boxxed[6].g).wrapping_sub(boxxed[2].g);

                                let bb = boxxed[8].b.wrapping_sub(boxxed[0].b).wrapping_add(boxxed[5].b).wrapping_sub(boxxed[3].b).wrapping_add(boxxed[7].b).
                                            wrapping_sub(boxxed[1].b).wrapping_add(boxxed[6].b).wrapping_sub(boxxed[2].b);

                                let v_e = sum {
                                    r: rr,
                                    g: gg,
                                    b: bb
                                };
                                saved.v_e = Some(v_e);
                                v_e
                            }
                        };

                        stack.push(v_e);
                    },
                    'b' => {
                        let v_b = match saved.v_b {
                            Some(v_b) => v_b,
                            None => {
                                boxxed = fetch_boxed(input, x as i32, y as i32, r, g, b);

                                let rr = wrapping_vec_add_u32(vec![boxxed[0].r, boxxed[1].r, boxxed[2].r, boxxed[3].r, boxxed[5].r, boxxed[6].r, boxxed[7].r, boxxed[8].r]);
                                let gg = wrapping_vec_add_u32(vec![boxxed[0].g, boxxed[1].g, boxxed[2].g, boxxed[3].g, boxxed[5].g, boxxed[6].g, boxxed[7].g, boxxed[8].g]);
                                let bb = wrapping_vec_add_u32(vec![boxxed[0].b, boxxed[1].b, boxxed[2].b, boxxed[3].b, boxxed[5].b, boxxed[6].b, boxxed[7].b, boxxed[8].b]);

                                let v_b = sum {
                                    r: (rr / 9) as u8,
                                    g: (gg / 9) as u8,
                                    b: (bb / 9) as u8,
                                };
                                saved.v_b = Some(v_b);
                                v_b
                            }
                        };

                        stack.push(v_b);
                    },
                    'H' => {
                        let v_h = match saved.v_H {
                            Some(v_h) => v_h,
                            None => {
                                boxxed = fetch_boxed(input, x as i32, y as i32, r, g, b);

                                let r_m = max(vec![boxxed[0].r, boxxed[1].r, boxxed[2].r, boxxed[3].r, boxxed[5].r, boxxed[6].r, boxxed[7].r, boxxed[8].r]).expect("max");
                                let g_m = max(vec![boxxed[0].g, boxxed[1].g, boxxed[2].g, boxxed[3].g, boxxed[5].g, boxxed[6].g, boxxed[7].g, boxxed[8].g]).expect("max");
                                let b_m = max(vec![boxxed[0].b, boxxed[1].b, boxxed[2].b, boxxed[3].b, boxxed[5].b, boxxed[6].b, boxxed[7].b, boxxed[8].b]).expect("max");

                                let v_h = sum {
                                    r: r_m,
                                    g: g_m,
                                    b: b_m
                                };

                                saved.v_H = Some(v_h);
                                v_h
                            }
                        };

                        stack.push(v_h);
                    },
                    'L' => {
                        let v_l = match saved.v_L {
                            Some(v_l) => v_l,
                            None => {
                                boxxed = fetch_boxed(input, x as i32, y as i32, r, g, b);
                                let r_m = min(vec![boxxed[0].r, boxxed[1].r, boxxed[2].r, boxxed[3].r, boxxed[5].r, boxxed[6].r, boxxed[7].r, boxxed[8].r]).expect("min");
                                let g_m = min(vec![boxxed[0].g, boxxed[1].g, boxxed[2].g, boxxed[3].g, boxxed[5].g, boxxed[6].g, boxxed[7].g, boxxed[8].g]).expect("min");
                                let b_m = min(vec![boxxed[0].b, boxxed[1].b, boxxed[2].b, boxxed[3].b, boxxed[5].b, boxxed[6].b, boxxed[7].b, boxxed[8].b]).expect("min");

                                let v_l = sum {
                                    r: r_m,
                                    g: g_m,
                                    b: b_m
                                };

                                saved.v_L = Some(v_l);
                                v_l
                            }
                        };

                        stack.push(v_l);
                    },
                    'N' => stack.push(sum { r: rng.gen_range(0..=255), g: rng.gen_range(0..=255), b: rng.gen_range(0..=255) }),
                    'h' => {
                        let v_h = match saved.v_h {
                            Some(v_h) => v_h,
                            None => {
                                let h = width - x - 1;
                                let pixel = input.get_pixel(h, y).0;

                                let v_h = sum {
                                    r: pixel[0],
                                    g: pixel[1],
                                    b: pixel[2]
                                };

                                saved.v_h = Some(v_h);
                                v_h
                            }
                        };

                        stack.push(v_h);
                    },
                    'v' => {
                        let v_v = match saved.v_v {
                            Some(v_v) => v_v,
                            None => {
                                let v = height - y - 1;
                                let pixel = input.get_pixel(x, v).0;

                                let v_v = sum {
                                    r: pixel[0],
                                    g: pixel[1],
                                    b: pixel[2]
                                };

                                saved.v_v = Some(v_v);
                                v_v
                            }
                        };

                        stack.push(v_v);
                    },
                    'd' => {
                        let v_d = match saved.v_d {
                            Some(v_d) => v_d,
                            None => {
                                let x = width - x - 1;
                                let y = height - y - 1;
                                let pixel = input.get_pixel(x, y).0;

                                let v_d = sum {
                                    r: pixel[0],
                                    g: pixel[1],
                                    b: pixel[2]
                                };

                                saved.v_d = Some(v_d);
                                v_d
                            }
                        };

                        stack.push(v_d);
                    },
                    _ => return Err(format!("Unexpected token: {:?}", c)),
                }
            }

            _ => return Err(format!("Unexpected token: {:?}", tok)),
        }
    }

    let col = stack.last().ok_or("Stack underflow")?;
    Ok(Rgba([col.r, col.g, col.b, a]))
}

fn fetch_boxed(input: &DynamicImage, x: i32, y: i32, r: u8, g: u8, b: u8) -> [sum; 9] {
    let mut k = 0;

    let mut boxed: [sum; 9] = [sum { r: 0, g: 0, b: 0 }; 9];

    for i in x - 1..=x + 1 {
        for j in y - 1..=y + 1 {
            if i == x && j == y {
                boxed[k] = sum { r, g, b };
                k += 1;
                continue;
            }

            if i < 0 || j < 0 {
                boxed[k] = sum { r: 0, g: 0, b: 0 };
                k += 1;
                continue;
            }

            let pixel = input.get_pixel(i as u32, j as u32).0;
            boxed[k] = sum { r: pixel[0], g: pixel[1], b: pixel[2] };
            k += 1;
        }
    }
    boxed
}

fn max(vals: Vec<u8>) -> Option<u8> {
    vals.iter().cloned().max()
}

fn min(vals: Vec<u8>) -> Option<u8> {
    vals.iter().cloned().min()
}

fn wrapping_vec_add(a: Vec<u8>) -> u8 {
    let mut sum: u8 = 0;
    for i in a {
        sum = sum.wrapping_add(i);
    }
    sum
}

fn wrapping_vec_add_u32(a: Vec<u8>) -> u32 {
    let mut sum: u32 = 0;
    for i in a {
        sum = sum.wrapping_add(i as u32);
    }
    sum
}