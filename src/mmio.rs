trait BitRegionValueChecked<const S: u32, const V: u32> {
    const BITS: u32 = S;
    const VALUE: u32 = V;

    fn check() {
        // See https://users.rust-lang.org/t/cant-use-type-parameters-of-outer-function-but-only-in-constant-expression/96023
        struct AssertProxy<const S_: u32, const V_: u32> {}
        impl<const S_: u32, const V_: u32> AssertProxy<S_, V_> {
            // Check if the value fits into an available space
            const ASSERTION: () = assert!(((1 << S_) - 1) & V_ == V_);
        }

        let _ = AssertProxy::<S, V>::ASSERTION;
    }
}

struct BitRegionCheck;
impl<const S: u32, const V: u32> BitRegionValueChecked<S, V>
    for BitRegionCheck
{
}

pub trait BitRegionW<const R: u32, const O: u32, const S: u32> {
    fn set<const V: u32>() {
        <BitRegionCheck as BitRegionValueChecked<S, V>>::check();

        let reg_val: u32 = V << O;

        let preg = R as *mut u32;
        unsafe {
            *preg = reg_val;
        }
    }
}

pub trait BitRegionR<const R: u32, const O: u32, const S: u32> {
    fn get() -> u32 {
        let preg = R as *mut u32;
        let regval = unsafe { *preg } >> O;

        let mask = (1 << S) - 1;
        regval & mask
    }
}

pub trait BitRegionRW<const R: u32, const O: u32, const S: u32>:
    BitRegionR<R, O, S>
{
    fn set<const V: u32>() {
        <BitRegionCheck as BitRegionValueChecked<S, V>>::check();

        let reg_val: u32 = V << O;
        let mask: u32 = ((1 << S) - 1) << O;

        let preg = R as *mut u32;
        let current = unsafe { *preg };

        let new = (current & !mask) | reg_val;
        unsafe { *preg = new }
    }
}
