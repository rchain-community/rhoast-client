pub fn encode(buffer: &[u8]) -> String {
    const ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

    if buffer.len() == 0 {
        return "".to_string();
    } else {
        let mut digits = vec![0];
        let mut i = 0;

        while i < buffer.len() {
            let mut j = 0;
            while j < digits.len() {
                #![allow(arithmetic_overflow)]
                digits[j] <<= 8;
                j += 1;
            }
            digits[0] += buffer[i];
            let mut carry = 0;
            j = 0;
            while j < digits.len() {
                digits[j] += carry;
                carry = (digits[j] / 58) | 0;
                digits[j] %= 58;
                j = j + 1;
            }
            while carry > 0 {
                digits.push(carry % 58);
                carry = (carry / 58) | 0;
            }
            i += 1;
        }
        i = 0;
        while buffer[i] == 0 && i < buffer.len() - 1 {
            digits.push(0);
            i += 1;
        }
        let y: Vec<_> = digits.into_iter().rev().collect();
        let value: String = y
            .iter()
            .map(|&x| ALPHABET.to_string().chars().nth(x as usize).unwrap())
            .collect();
        return value;
    }
}
