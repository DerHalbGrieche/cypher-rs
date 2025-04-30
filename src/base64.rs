use crate::matching::Cipher;

pub struct Base64;

impl Base64 {
    // Standard base64 alphabet
    const ALPHABET: &'static [u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    const PADDING: char = '=';

    pub fn encrypt(text: &str) -> String {
        let input = text.as_bytes();
        let len = input.len();

        // Calculate result length: 4 characters for every 3 bytes, plus padding
        let padding_needed = if len % 3 == 0 { 0 } else { 3 - (len % 3) };
        let result_len = ((len + padding_needed) / 3) * 4;
        let mut result = Vec::with_capacity(result_len);

        // Process 3 bytes at a time
        let mut i = 0;
        while i + 3 <= len {
            // Convert 3 bytes to 24 bits
            let b1 = input[i] as u32;
            let b2 = input[i + 1] as u32;
            let b3 = input[i + 2] as u32;
            let bits = (b1 << 16) | (b2 << 8) | b3;

            // Split into 4 groups of 6 bits and convert to base64 characters
            result.push(Self::ALPHABET[(bits >> 18) as usize]);
            result.push(Self::ALPHABET[((bits >> 12) & 0x3F) as usize]);
            result.push(Self::ALPHABET[((bits >> 6) & 0x3F) as usize]);
            result.push(Self::ALPHABET[(bits & 0x3F) as usize]);

            i += 3;
        }

        // Handle the remaining 1 or 2 bytes if any
        if i < len {
            let remaining = len - i;

            if remaining == 1 {
                // 1 byte -> 2 base64 chars + 2 padding
                let bits = (input[i] as u32) << 16;
                result.push(Self::ALPHABET[(bits >> 18) as usize]);
                result.push(Self::ALPHABET[((bits >> 12) & 0x3F) as usize]);
                result.push(Self::PADDING as u8);
                result.push(Self::PADDING as u8);
            } else {
                // remaining == 2
                // 2 bytes -> 3 base64 chars + 1 padding
                let bits = ((input[i] as u32) << 16) | ((input[i + 1] as u32) << 8);
                result.push(Self::ALPHABET[(bits >> 18) as usize]);
                result.push(Self::ALPHABET[((bits >> 12) & 0x3F) as usize]);
                result.push(Self::ALPHABET[((bits >> 6) & 0x3F) as usize]);
                result.push(Self::PADDING as u8);
            }
        }

        String::from_utf8(result).unwrap()
    }

    pub fn decrypt(text: &str) -> String {
        let input = text.as_bytes();
        let len = input.len();

        if len % 4 != 0 {
            panic!("Invalid base64 string length");
        }

        // Create a decoding table: maps ASCII character to 6-bit value
        let mut decode_table = [0u8; 256];
        for (i, &c) in Self::ALPHABET.iter().enumerate() {
            decode_table[c as usize] = i as u8;
        }

        // Count padding characters
        let padding_count = input
            .iter()
            .rev()
            .take_while(|&&c| c == Self::PADDING as u8)
            .count();

        // Calculate output size: each 4 characters represent 3 bytes, minus padding
        let result_len = (len / 4) * 3 - padding_count;
        let mut result = Vec::with_capacity(result_len);

        // Process 4 characters at a time
        let mut i = 0;
        while i + 4 <= len {
            // Convert 4 base64 chars to 24 bits
            let mut bits = 0u32;

            for j in 0..4 {
                let c = input[i + j];
                if c == Self::PADDING as u8 {
                    // Skip padding characters
                    continue;
                }

                if !Self::ALPHABET.contains(&c) {
                    panic!("Invalid base64 character");
                }

                bits = (bits << 6) | (decode_table[c as usize] as u32);
            }

            // Convert 24 bits to 3 bytes
            // Handle different padding scenarios
            if i + 3 < len && input[i + 3] == Self::PADDING as u8 {
                if i + 2 < len && input[i + 2] == Self::PADDING as u8 {
                    // One byte scenario (two padding chars)
                    result.push(((bits >> 4) & 0xFF) as u8);
                } else {
                    // Two bytes scenario (one padding char)
                    result.push(((bits >> 10) & 0xFF) as u8);
                    result.push(((bits >> 2) & 0xFF) as u8);
                }
            } else {
                // Three bytes scenario (no padding)
                result.push(((bits >> 16) & 0xFF) as u8);
                result.push(((bits >> 8) & 0xFF) as u8);
                result.push((bits & 0xFF) as u8);
            }

            i += 4;
        }

        // Truncate the result to the correct length (removing any extra bytes)
        result.truncate(result_len);

        String::from_utf8(result).expect("Invalid UTF-8 sequence in decoded base64")
    }
}

impl Cipher for Base64 {
    fn encrypt(text: &str) -> String {
        Base64::encrypt(text)
    }
    fn decrypt(text: &str) -> String {
        Base64::decrypt(text)
    }
}
