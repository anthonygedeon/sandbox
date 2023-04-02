pub struct Luhn(String);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let code: String = self.0.to_string().trim().split_whitespace().collect();

        if code.len() <= 1 || !code.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }

        let code = code
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>();

        let values_1 = (0..code.len()).rev().skip(1).step_by(2).map(|n| {
            let card_number = code
                .chars()
                .nth(n)
                .unwrap()
                .to_string()
                .parse::<u32>()
                .unwrap();
            let sum = card_number * 2;
            if sum > 9 {
                sum - 9
            } else {
                sum
            }
        });

        let values_2 = (0..code.len()).rev().step_by(2).map(|n| {
            code.chars()
                .nth(n)
                .unwrap()
                .to_string()
                .parse::<u32>()
                .unwrap()
        });

        let checksum: u32 = values_1.chain(values_2).sum();

        checksum % 10 == 0
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> From<T> for Luhn
where
    T: ToString,
{
    fn from(input: T) -> Self {
        Luhn(input.to_string())
    }
}
