//! funny code 



const fn is_divisible_by_5(value: usize) -> bool {
    value % 5 == 0
}

macro_rules! num_divisible_by_5 {
    ($value:expr) => {
        {
            const VALUE: usize = $value;
            if !is_divisible_by_5(VALUE) {
                panic!("Number is not divisible by 5");
            }
            VALUE
        }
    };
}


struct MultipleOf5(usize);

impl MultipleOf5 {
    #[track_caller]
    pub const fn new(num: usize) -> Self {
        if num % 5 == 0 {
            Self(num)
        } else {
            panic!("nooo")
        }
    }
}


// this must be divisible by 5
// const PIXEL_WIDTH: MultipleOf5 = MultipleOf5::new(500);