pub trait BitFldChecked {
    const BITS: u32;
    const VALUE: u32;
    const ASSERTION: () =
        assert!(((1 << Self::BITS) - 1) & Self::VALUE == Self::VALUE);
}

pub trait BitFldR {
    const REG: u32;
    const OFFT: u32;
    const BITS: u32;

    fn get() -> u32 {
        let preg = Self::REG as *mut u32;
        let regval = unsafe { *preg } >> Self::OFFT;

        let mask = (1 << Self::BITS) - 1;
        regval & mask
    }
}

pub trait BitFldW {
    const REG: u32;
    const OFFT: u32;
    const BITS: u32;

    fn set<V>()
    where
        V: BitFldChecked,
    {
        let reg_val: u32 = V::VALUE << Self::OFFT;

        let preg = Self::REG as *mut u32;
        unsafe {
            *preg = reg_val;
        }
    }
}

pub trait BitFldRW: BitFldR {
    fn set<V>()
    where
        V: BitFldChecked,
    {
        let reg_val: u32 = V::VALUE << Self::OFFT;
        let mask: u32 = ((1 << Self::BITS) - 1) << Self::OFFT;

        let preg = Self::REG as *mut u32;
        let current = unsafe { *preg };

        let new = (current & !mask) | reg_val;
        unsafe { *preg = new }
    }
}

// -----------------------------------------------------------------------------
// Useful tools

#[macro_export]
macro_rules! bitfld_check {
    ($n: ident, $b: expr, $v: expr) => {
        struct $n;
        impl mmio::BitFldChecked for $n {
            const BITS: u32 = $b;
            const VALUE: u32 = $v;
        }
    };
}
