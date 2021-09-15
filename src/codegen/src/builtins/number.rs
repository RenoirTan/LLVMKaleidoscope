use std::{
    cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
    fmt,
    ops::{Add, Div, Mul, Sub}
};

use inkwell::{
    types::StructType,
    values::{AggregateValue, BasicValue, BasicValueEnum, FloatValue, IntValue, StructValue},
    FloatPredicate,
    IntPredicate
};

use crate::{
    error::{Error, ErrorKind, Result},
    CodeGen
};

pub const NUM_TYPE_NAME: &'static str = "num";

pub fn make_number_type<'ctx>(code_gen: &CodeGen<'ctx>) -> StructType<'ctx> {
    let int_type = code_gen.get_int_type().into();
    let float_type = code_gen.get_float_type().into();
    let bool_type = code_gen.get_context().bool_type().into();
    let struct_type = code_gen.get_context().opaque_struct_type(NUM_TYPE_NAME);
    struct_type.set_body(&[int_type, float_type, bool_type], true);
    code_gen
        .get_module()
        .get_struct_type(NUM_TYPE_NAME)
        .expect(&format!("{} could not be created", NUM_TYPE_NAME))
}


fn make_bit_width_error<'ctx>(_left: IntValue<'ctx>, _right: IntValue<'ctx>) -> Error {
    Error::new(
        format!("Bit widths of left and right do not match."),
        ErrorKind::BitWidthError,
        None
    )
}


fn make_float_format_error<'ctx>(_left: FloatValue<'ctx>, _right: FloatValue<'ctx>) -> Error {
    Error::new(
        format!("Left and right float types do not match."),
        ErrorKind::BitWidthError,
        None
    )
}


pub fn check_int_widths<'ctx>(left: IntValue<'ctx>, right: IntValue<'ctx>) -> bool {
    left.get_type().get_bit_width() == right.get_type().get_bit_width()
}


pub fn check_float_formats<'ctx>(left: FloatValue<'ctx>, right: FloatValue<'ctx>) -> bool {
    left.get_type() == right.get_type()
}


macro_rules! impl_int_math {
    ($fn_name: ident, $method: ident) => {
        pub fn $fn_name<'ctx>(
            left: inkwell::values::IntValue<'ctx>,
            right: inkwell::values::IntValue<'ctx>
        ) -> $crate::error::Result<inkwell::values::IntValue<'ctx>> {
            if check_int_widths(left, right) {
                Ok(left.$method(right))
            } else {
                Err(make_bit_width_error(left, right))
            }
        }
    };
}


impl_int_math!(add_ints, const_add);
impl_int_math!(sub_ints, const_sub);
impl_int_math!(mul_ints, const_mul);
impl_int_math!(div_ints, const_signed_div);


macro_rules! impl_int_cmp {
    ($fn_name: ident, $predicate: expr) => {
        pub fn $fn_name<'ctx>(
            left: inkwell::values::IntValue<'ctx>,
            right: inkwell::values::IntValue<'ctx>
        ) -> $crate::error::Result<inkwell::values::IntValue<'ctx>> {
            if check_int_widths(left, right) {
                Ok(left.const_int_compare($predicate, right))
            } else {
                Err(make_bit_width_error(left, right))
            }
        }
    };
}


impl_int_cmp!(cmp_lt_ints, IntPredicate::SLT);
impl_int_cmp!(cmp_le_ints, IntPredicate::SLE);
impl_int_cmp!(cmp_eq_ints, IntPredicate::EQ);
impl_int_cmp!(cmp_ge_ints, IntPredicate::SGE);
impl_int_cmp!(cmp_gt_ints, IntPredicate::SGT);


macro_rules! impl_float_math {
    ($fn_name: ident, $method: ident) => {
        pub fn $fn_name<'ctx>(
            left: inkwell::values::FloatValue<'ctx>,
            right: inkwell::values::FloatValue<'ctx>
        ) -> $crate::error::Result<inkwell::values::FloatValue<'ctx>> {
            if check_float_formats(left, right) {
                Ok(left.$method(right))
            } else {
                Err(make_float_format_error(left, right))
            }
        }
    };
}


impl_float_math!(add_floats, const_add);
impl_float_math!(sub_floats, const_sub);
impl_float_math!(mul_floats, const_mul);
impl_float_math!(div_floats, const_div);


macro_rules! impl_float_cmp {
    ($fn_name: ident, $predicate: expr) => {
        pub fn $fn_name<'ctx>(
            left: inkwell::values::FloatValue<'ctx>,
            right: inkwell::values::FloatValue<'ctx>
        ) -> $crate::error::Result<inkwell::values::IntValue<'ctx>> {
            if check_float_formats(left, right) {
                Ok(left.const_compare($predicate, right))
            } else {
                Err(make_float_format_error(left, right))
            }
        }
    };
}


impl_float_cmp!(cmp_lt_floats, FloatPredicate::OLT);
impl_float_cmp!(cmp_le_floats, FloatPredicate::OLE);
impl_float_cmp!(cmp_eq_floats, FloatPredicate::OEQ);
impl_float_cmp!(cmp_ge_floats, FloatPredicate::OGE);
impl_float_cmp!(cmp_gt_floats, FloatPredicate::OGT);


pub struct NumValue<'ctx: 'cdg, 'cdg> {
    value:    StructValue<'ctx>,
    code_gen: &'cdg CodeGen<'ctx>
}


impl<'ctx: 'cdg, 'cdg> NumValue<'ctx, 'cdg> {
    pub fn new(value: StructValue<'ctx>, code_gen: &'cdg CodeGen<'ctx>) -> Result<Self> {
        if value.get_type() != code_gen.get_num_type() {
            Err(Error::new(
                format!("Invalid type for NumValue"),
                ErrorKind::TypeError,
                None
            ))
        } else {
            Ok(Self { value, code_gen })
        }
    }

    pub fn make_i128(value: i128, code_gen: &'cdg CodeGen<'ctx>) -> Self {
        Self {
            value: code_gen.make_num_from_i128(value),
            code_gen
        }
    }

    pub fn make_f64(value: f64, code_gen: &'cdg CodeGen<'ctx>) -> Self {
        Self {
            value: code_gen.make_num_from_f64(value),
            code_gen
        }
    }

    fn make_true(&self) -> IntValue<'ctx> {
        self.code_gen.make_bool(true)
    }

    #[allow(unused)]
    fn make_false(&self) -> IntValue<'ctx> {
        self.code_gen.make_bool(false)
    }

    pub fn destructure(&self) -> [BasicValueEnum; 3] {
        [
            self.value.const_extract_value(&mut [0]),
            self.value.const_extract_value(&mut [1]),
            self.value.const_extract_value(&mut [2])
        ]
    }

    pub fn get_int_switch(&self) -> IntValue<'ctx> {
        self.value.const_extract_value(&mut [2]).into_int_value()
    }

    pub fn is_int(&self) -> bool {
        self.get_int_switch() == self.make_true()
    }

    pub fn is_float(&self) -> bool {
        !self.is_int()
    }

    pub fn get_raw_int_value(&self) -> IntValue<'ctx> {
        self.value.const_extract_value(&mut [0]).into_int_value()
    }

    pub fn get_raw_float_value(&self) -> FloatValue<'ctx> {
        self.value.const_extract_value(&mut [1]).into_float_value()
    }

    pub fn to_float(&self) -> Result<Self> {
        let float = if self.is_float() {
            self.code_gen.copy_float(self.get_raw_float_value())
        } else {
            self.code_gen.int_to_float(self.get_raw_int_value())
        };
        let raw = self.code_gen.make_num_from_float(float)?;
        Self::new(raw, self.code_gen)
    }

    pub fn to_int(&self) -> Result<Self> {
        let integer = if self.is_int() {
            self.code_gen.copy_int(self.get_raw_int_value())
        } else {
            self.code_gen.float_to_int(self.get_raw_float_value())
        };
        let raw = self.code_gen.make_num_from_int(integer)?;
        Self::new(raw, self.code_gen)
    }

    pub fn cast_to_same_type_as(&self, other: &Self) -> Result<Self> {
        if other.is_int() {
            self.to_int()
        } else {
            self.to_float()
        }
    }

    pub fn simplify_to_basic_value(&self) -> Box<dyn BasicValue<'ctx> + 'ctx> {
        if self.is_int() {
            Box::new(self.get_raw_int_value())
        } else {
            Box::new(self.get_raw_float_value())
        }
    }
}


impl<'ctx: 'cdg, 'cdg> Into<StructValue<'ctx>> for NumValue<'ctx, 'cdg> {
    fn into(self) -> StructValue<'ctx> {
        self.value
    }
}


impl<'ctx: 'cdg, 'cdg> fmt::Display for NumValue<'ctx, 'cdg> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({:?})", NUM_TYPE_NAME, self.simplify_to_basic_value())
    }
}


macro_rules! impl_binop_for_numvalue {
    ($trait_name: ident, $fn_name: ident, $int_op: ident, $float_op: ident) => {
        impl<'ctx: 'cdg, 'cdg> $trait_name for &NumValue<'ctx, 'cdg> {
            type Output = NumValue<'ctx, 'cdg>;

            fn $fn_name(self, rhs: Self) -> Self::Output {
                let raw = if self.is_int() && rhs.is_int() {
                    let result =
                        $int_op(self.get_raw_int_value(), rhs.get_raw_int_value()).unwrap();
                    self.code_gen.make_num_from_int(result).unwrap()
                } else {
                    let left = self.to_float().unwrap();
                    let right = rhs.to_float().unwrap();
                    let result =
                        $float_op(left.get_raw_float_value(), right.get_raw_float_value()).unwrap();
                    self.code_gen.make_num_from_float(result).unwrap()
                };
                NumValue::new(raw, &self.code_gen).unwrap()
            }
        }
    };
}

impl_binop_for_numvalue!(Add, add, add_ints, add_floats);
impl_binop_for_numvalue!(Sub, sub, sub_ints, sub_floats);
impl_binop_for_numvalue!(Mul, mul, mul_ints, mul_floats);
impl_binop_for_numvalue!(Div, div, div_ints, div_floats);


impl<'ctx: 'cdg, 'cdg> PartialEq for NumValue<'ctx, 'cdg> {
    fn eq(&self, other: &Self) -> bool {
        if self.is_int() && other.is_int() {
            cmp_eq_ints(self.get_raw_int_value(), other.get_raw_int_value()).unwrap()
                == self.make_true()
        } else {
            let left = self.to_float().unwrap();
            let right = other.to_float().unwrap();
            cmp_eq_floats(left.get_raw_float_value(), right.get_raw_float_value()).unwrap()
                == self.make_true()
        }
    }
}


impl<'ctx: 'cdg, 'cdg> Eq for NumValue<'ctx, 'cdg> {}


impl<'ctx: 'cdg, 'cdg> PartialOrd for NumValue<'ctx, 'cdg> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_int() && other.is_int() {
            if cmp_lt_ints(self.get_raw_int_value(), other.get_raw_int_value()).ok()?
                == self.make_true()
            {
                Some(Ordering::Less)
            } else if cmp_eq_ints(self.get_raw_int_value(), other.get_raw_int_value()).ok()?
                == self.make_true()
            {
                Some(Ordering::Equal)
            } else if cmp_gt_ints(self.get_raw_int_value(), other.get_raw_int_value()).ok()?
                == self.make_true()
            {
                Some(Ordering::Greater)
            } else {
                None
            }
        } else {
            let left = self.to_float().ok()?;
            let right = other.to_float().ok()?;
            if cmp_lt_floats(left.get_raw_float_value(), right.get_raw_float_value()).ok()?
                == self.make_true()
            {
                Some(Ordering::Less)
            } else if cmp_eq_floats(left.get_raw_float_value(), right.get_raw_float_value()).ok()?
                == self.make_true()
            {
                Some(Ordering::Equal)
            } else if cmp_gt_floats(left.get_raw_float_value(), right.get_raw_float_value()).ok()?
                == self.make_true()
            {
                Some(Ordering::Greater)
            } else {
                None
            }
        }
    }
}


impl<'ctx: 'cdg, 'cdg> Ord for NumValue<'ctx, 'cdg> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}