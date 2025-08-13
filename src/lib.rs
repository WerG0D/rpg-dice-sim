use rand::Rng;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdvMode {
    None,
    Advantage,
    Disadvantage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiceTerm {
    pub sign: i32,
    pub count: u32,
    pub sides: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlatMod {
    pub sign: i32,
    pub value: i32,
}

// expr completa: alguns dados + mods
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expression {
    pub dice: Vec<DiceTerm>,
    pub flats: Vec<FlatMod>,
}

#[derive(Debug, Clone)]
pub struct RollDetail {
    pub term: DiceTerm,
    pub rolls: Vec<u32>,
    pub subtotal: i32,
}

#[derive(Debug, Clone)]
pub struct RollResult {
    pub details: Vec<RollDetail>,
    pub flat_total: i32,
    pub total: i32,
}

impl fmt::Display for RollDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = if self.term.sign >= 0 { "+" } else { "-" };
        write!(f, "{}{}d{}: {:?} = {}", sign, self.term.count, self.term.sides, self.rolls, self.subtotal)
    }
}

impl Expression {
    pub fn parse(input: &str) -> Result<Self, String> {
        let normalized = input.replace(' ', "").replace('-', "+-");
        if normalized.is_empty() {
            return Err("expressão vazia, confere aí".into());
        }

        let mut dice = Vec::new();
        let mut flats = Vec::new();

        for token in normalized.split('+') {
            let t = token.trim();
            if t.is_empty() { continue; }

            if !t.contains('d') && (t.parse::<i32>().is_ok() || t.starts_with('-')) {
                let value: i32 = t.parse().map_err(|_| format!("modificador inválido: {t}"))?;
                let sign = if value >= 0 { 1 } else { -1 };
                flats.push(FlatMod { sign, value: value.abs() });
                continue;
            }

            let (sign, core) = if let Some(stripped) = t.strip_prefix('-') {
                (-1, stripped)
            } else {
                (1, t)
            };

            let parts: Vec<&str> = core.split('d').collect();
            if parts.len() != 2 {
                return Err(format!("termo esquisito: {t}"));
            }

            let count_str = parts[0];
            let sides_str = parts[1];

            let count = if count_str.is_empty() { 1 } else {
                count_str.parse::<u32>().map_err(|_| format!("quantidade inválida em: {t}"))?
            };
            let sides = sides_str.parse::<u32>().map_err(|_| format!("lados inválidos em: {t}"))?;

            if count == 0 || sides == 0 {
                return Err(format!("tem q ser > 0 em: {t}"));
            }

            dice.push(DiceTerm { sign, count, sides });
        }

        if dice.is_empty() && flats.is_empty() {
            return Err("não achei nada pra rolar, sério".into());
        }

        Ok(Self { dice, flats })
    }

    pub fn roll(&self, adv: AdvMode) -> RollResult {
        let mut rng = rand::thread_rng();
        let mut details = Vec::new();
        let mut total: i32 = 0;

        for term in &self.dice {
            let mut rolls = Vec::with_capacity(term.count as usize);
            let mut subtotal: i32 = 0;

            for _ in 0..term.count {
                let roll = if adv != AdvMode::None && term.sides == 20 && term.count == 1 {
                    // vantagem/desvantagem só no d20 solitárip
                    let a = rng.gen_range(1..=20);
                    let b = rng.gen_range(1..=20);
                    match adv {
                        AdvMode::Advantage => a.max(b),
                        AdvMode::Disadvantage => a.min(b),
                        AdvMode::None => unreachable!(),
                    }
                } else {
                    rng.gen_range(1..=term.sides)
                };
                rolls.push(roll);
                subtotal += roll as i32;
            }

            subtotal *= term.sign;
            total += subtotal;

            details.push(RollDetail { term: *term, rolls, subtotal });
        }

        let flat_total: i32 = self.flats.iter().map(|m| m.sign * m.value).sum();
        total += flat_total;

        RollResult { details, flat_total, total }
    }
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub count: usize,
    pub min: i32,
    pub max: i32,
    pub mean: f64,
}

pub fn compute_stats(results: &[i32]) -> Option<Stats> {
    if results.is_empty() { return None; }
    let count = results.len();
    let mut min = results[0];
    let mut max = results[0];
    let mut sum: i64 = 0;

    for &v in results {
        if v < min { min = v; }
        if v > max { max = v; }
        sum += v as i64;
    }

    let mean = sum as f64 / count as f64;
    Some(Stats { count, min, max, mean })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_basic() {
        let e = Expression::parse("2d6+3").unwrap();
        assert_eq!(e.dice.len(), 1);
        assert_eq!(e.flats.len(), 1);
        assert_eq!(e.dice[0].count, 2);
        assert_eq!(e.dice[0].sides, 6);
        assert_eq!(e.flats[0].value, 3);
    }

    #[test]
    fn parse_multiple_terms() {
        let e = Expression::parse("3d6+2d8-1").unwrap();
        assert_eq!(e.dice.len(), 2);
        assert_eq!(e.flats.len(), 1);
        assert_eq!(e.flats[0].sign, -1);
        assert_eq!(e.flats[0].value, 1);
    }

    #[test]
    fn parse_implicit_one() {
        let e = Expression::parse("d20+5").unwrap();
        assert_eq!(e.dice[0].count, 1);
        assert_eq!(e.dice[0].sides, 20);
        assert_eq!(e.flats[0].value, 5);
    }
}
