pub struct Base58CheckConfig {
    pub alphabet_vec: Vec<char>,
    pub base: u64,
    pub leader: char,
    pub i_factor: f64
}

impl Default for Base58CheckConfig {
    fn default() -> Self {
        let alphabet = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".to_string();
        let alphabet_vec: Vec<char> = alphabet.chars().collect();

        let mut base_map: [u8; 256] = [0; 256];
        let int_bm_length = base_map.len() as i64;
        for x in 0..int_bm_length {
            base_map[x as usize] = 255;
        }
        let alphabet_length = alphabet.len();
        let int_alphabet_length = alphabet_length as u64;
        
        for x in 0..int_alphabet_length {

            let char_at_usize: usize = x as usize;        
            let char_at = alphabet_vec[char_at_usize];
            let mut u8_bytes: [u8;2] = [0;2];
            char_at.encode_utf8(&mut u8_bytes);
            let char_byte = u8_bytes[0];
            let char_code_usize = char_byte as usize;
            let x_as_u8 = x as u8;
            base_map[char_code_usize] = x_as_u8;
        }

        let base = alphabet.len();
        let base_u8 = base as u8;
        let log_256 = (256 as f32).log(std::f32::consts::E);

        let float_base: f32 = base_u8 as f32;
        let base_log = float_base.log(std::f32::consts::E);

        let i_factor_f = log_256 / base_log;

        let leader =  alphabet_vec[0];


        
        Base58CheckConfig {
            alphabet_vec: alphabet_vec,
            base: base as u64,
            leader: leader,
            i_factor: i_factor_f as f64
        }   
    }
}