use crate::int::To64LLVMWord;

#[test]
fn from_u8() {
    assert_eq!(To64LLVMWord::from_u8(216), [216]);
}

#[test]
fn from_u16() {
    assert_eq!(To64LLVMWord::from_u16(46800), [46800]);
}

#[test]
fn from_u32() {
    assert_eq!(To64LLVMWord::from_u32(2_286_178_951), [2_286_178_951]);
}

#[test]
fn from_u64() {
    assert_eq!(To64LLVMWord::from_u64(12_686_819_128_171_057_799), [
        12_686_819_128_171_057_799
    ]);
}

#[test]
fn from_u128() {
    assert_eq!(
        To64LLVMWord::from_u128(234_030_505_566_814_440_810_823_979_640_880_910_982),
        [12_686_819_128_171_057_798, 12_686_819_128_171_057_799]
    );
}

#[test]
fn from_i8() {
    assert_eq!(To64LLVMWord::from_i8(-81), [18446744073709551535]);
}

#[test]
fn from_i16() {
    assert_eq!(To64LLVMWord::from_i16(-26420), [18446744073709525196]);
}

#[test]
fn from_i32() {
    assert_eq!(To64LLVMWord::from_i32(-2117483648), [18446744071592067968]);
}

#[test]
fn from_i64() {
    assert_eq!(To64LLVMWord::from_i64(-9023322036354771845), [
        9423422037354779771
    ]);
}

#[test]
fn from_i128() {
    assert_eq!(
        To64LLVMWord::from_i128(-160141183450469231781687303715814105328),
        [9886960196732067600, 9765473123639629108]
    );
}
