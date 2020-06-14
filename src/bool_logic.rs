use std::convert::{TryFrom};

#[allow(dead_code)]
pub fn entry_point() {
    println!("in bool_logic mod entry_point");
}

pub fn nand(x: u8, y: u8) -> u8 {
    // 全ての基本
    if x == 1 && y == 1 {
        0
    } else {
        1
    }
}

pub fn not(x: u8) -> u8 {
    // Nandに同じ値をいれるとNotになるよ
    nand(x, x)
}

pub fn and(x: u8, y: u8) -> u8 {
    // Nandの結果をNotすればOk
    not(nand(x, y))
    // nand(nand(x, y), nand(x, y))
}

pub fn or(x: u8, y: u8) -> u8 {
    // 入力した値をNotで入替えてNandに通せばOk
    nand(not(x), not(y))
    // nand(nand(x, x), nand(y, y))
}

pub fn nor(x: u8, y: u8) -> u8 {
    // OrをNotすればOk
    not(or(x, y))
    // nand(nand(nand(x, x), nand(y, y)), nand(nand(x, x), nand(y, y)))
}

pub fn xor(x: u8, y: u8) -> u8 {
    // むっず。。。これは調べながら。
    nand(
        nand(x, nand(x, y)),
        nand(nand(x, y), y)
    )
}

pub fn mux(x: u8, y: u8, sel: u8) -> u8 {
    // これもググってくれ。。。
    let selector = not(sel);
    let a = and(x, selector);
    let b = and(y, sel);
    or(a, b)
    // let selector = nand(sel, sel);
    // let a = nand(nand(x, selector), nand(x, selector));
    // let b = nand(nand(y, sel), nand(y, sel));
    // nand(nand(a, a), nand(b, b))
}

pub fn dmux(input: u8, sel: u8) -> [u8; 2] {
    let mut result: [u8; 2] = [0; 2];
    result[0] = and(input, not(sel));
    result[1] = and(input, sel);
    // result[0] = nand(
    //     nand(input, nand(sel, sel)),
    //     nand(input, nand(sel, sel))
    // );
    // result[1] = nand(
    //     nand(input, sel),
    //     nand(input, sel)
    // );
    result
}

pub fn and_16bit (x_arr: &[u8; 16], y_arr: &[u8; 16]) -> [u8; 16] {
    let mut result: [u8; 16] = [0; 16];
    for i in 0..16 {
        result[i] = and(x_arr[i], y_arr[i]);
        // result[i] = nand(nand(x_arr[i], y_arr[i]), nand(x_arr[i], y_arr[i]));
    }
    result
}


#[cfg(test)]
mod test {
    use super::*;

    fn converter_16bit_to_array(input: &str) -> [u8; 16] {
        let mut output: [u8; 16] = [0; 16];
        for i in 0..input.len() {
            output[i] = u8::try_from(
                input.chars().nth(i).unwrap().to_digit(2).unwrap()
            ).unwrap();
        }
        output
    }

    #[test]
    fn nand_test() {
        assert_eq!(0, nand(1,  1));
        assert_eq!(1, nand(1,  0));
        assert_eq!(1, nand(0, 1));
        assert_eq!(1, nand(0, 0));
    }

    #[test]
    fn not_test() {
        assert_eq!(0, not(1));
        assert_eq!(1, not(0));
    }

    #[test]
    fn and_test() {
        assert_eq!(1, and(1,  1));
        assert_eq!(0, and(1,  0));
        assert_eq!(0, and(0, 1));
        assert_eq!(0, and(0, 0));
    }

    #[test]
    fn or_test() {
        assert_eq!(1, or(1, 1));
        assert_eq!(1, or(1, 0));
        assert_eq!(1, or(0, 1));
        assert_eq!(0, or(0, 0));
    }

    #[test]
    fn nor_test() {
        assert_eq!(0, nor(1, 1));
        assert_eq!(0, nor(1, 0));
        assert_eq!(0, nor(0, 1));
        assert_eq!(1, nor(0, 0));
    }

    #[test]
    fn xor_test() {
        assert_eq!(0, xor(1, 1));
        assert_eq!(1, xor(1, 0));
        assert_eq!(1, xor(0, 1));
        assert_eq!(0, xor(0, 0));
    }

    #[test]
    fn mux_test() {
        assert_eq!(0, mux(0, 0, 0));
        assert_eq!(0, mux(0, 1, 0));
        assert_eq!(1, mux(1, 0, 0));
        assert_eq!(1, mux(1, 1, 0));
        assert_eq!(0, mux(0, 0, 1));
        assert_eq!(1, mux(0, 1, 1));
        assert_eq!(0, mux(1, 0, 1));
        assert_eq!(1, mux(1, 1, 1));
    }

    #[test]
    fn dmux_test() {
        assert_eq!([0, 0], dmux(0, 0));
        assert_eq!([0, 0], dmux(0, 1));
        assert_eq!([1, 0], dmux(1, 0));
        assert_eq!([0, 1], dmux(1, 1));
    }

    #[test]
    fn and_16bit_test() {
        assert_eq!(
            converter_16bit_to_array("0000000000000000"),
            and_16bit(
            &converter_16bit_to_array("0000000000000000"),
            &converter_16bit_to_array("0000000000000000")
            )
        );
        assert_eq!(
            converter_16bit_to_array("0000000000000000"),
            and_16bit(
                &converter_16bit_to_array("0000000000000000"),
                &converter_16bit_to_array("1111111111111111")
            )
        );
        assert_eq!(
            converter_16bit_to_array("1111111111111111"),
            and_16bit(
                &converter_16bit_to_array("1111111111111111"),
                &converter_16bit_to_array("1111111111111111")
            )
        );
        assert_eq!(
            converter_16bit_to_array("0000000000000000"),
            and_16bit(
                &converter_16bit_to_array("1010101010101010"),
                &converter_16bit_to_array("0101010101010101")
            )
        );
        assert_eq!(
            converter_16bit_to_array("0000110011000000"),
            and_16bit(
                &converter_16bit_to_array("0011110011000011"),
                &converter_16bit_to_array("0000111111110000")
            )
        );
        assert_eq!(
            converter_16bit_to_array("0001000000110100"),
            and_16bit(
                &converter_16bit_to_array("0001001000110100"),
                &converter_16bit_to_array("1001100001110110")
            )
        );
    }
}
