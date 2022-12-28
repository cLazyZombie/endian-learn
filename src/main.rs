fn main() {}

#[cfg(test)]
mod tests {

    #[test]
    fn test_endian() {
        let v: u32 = 0x01020304;

        let be = v.to_be_bytes();
        assert_eq!(be, [0x01, 0x02, 0x03, 0x04]);

        let le = v.to_le_bytes();
        assert_eq!(le, [0x04, 0x03, 0x02, 0x01]);

        let ne = v.to_ne_bytes();
        println!("ne: {:?}", ne);
    }
}
