use crate::dnd::{Args, Component, Sign, DropDirection};

impl Args {
    pub fn evaluate(&self) -> String {
        let mut result = "".to_string();
        let mut sum = 0;
        let mut total_bonus = 0;
        for arg in &self.terms {
            let mul = match arg.sign {
                Sign::Positive => 1,
                Sign::Negative => -1
            };
            let (sum_change, bonus_change) = match &arg.component {
                Component::Dice(dice) => {
                    if let Some(err) = dice.validate() {
                        return format!("Error validating dice {}{}: {}", match arg.sign {
                            Sign::Positive => "",
                            Sign::Negative => "-"
                        }, dice, err);
                    }
                    let mut values = dice.generate(&mut rand::thread_rng());
                    let dropped;
                    if let Some(drop) = &dice.drop {
                        dropped =
                            match drop.direction {
                                DropDirection::Highest => values.drain(values.len() - drop.value as usize..),
                                DropDirection::Lowest => values.drain(0..drop.value as usize)
                            }.as_slice().iter().map(|x| format!("{}", x.to_string())).collect::<Vec<String>>();
                    } else {
                        dropped = [].to_vec();
                    }
                    result = format!("{}\n{}: [~~{}~~, {}]", result, dice, dropped.join(", "), values.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "));
                    (values.iter().sum::<u32>(), 0)
                }
                Component::Bonus(bonus) => {
                    (*bonus, *bonus)
                }
            };
            sum += mul * sum_change as i32;
            total_bonus += mul * bonus_change as i32;
        }
        format!("{}\n{}**Sum: {}**", &result[1..], if total_bonus > 0 { format!("Total bonus: {}\n", total_bonus) } else { "".to_string() }, sum)
    }
}