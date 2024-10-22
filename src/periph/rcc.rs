use crate::bitfld_check;
use crate::mmio::{self, BitFldR, BitFldRW};
// -----------------------------------------------------------------------------
// RCC root object

const RCC_ADDR: u32 = 0x40023800; // RM, page 65

// Need to use the trait, since associated types for impl are unstable
pub trait Rcc {
    type CR;
    type PLLFGR;
    type AHB1ENR;
}

pub struct RCC;

impl Rcc for RCC {
    type CR = RCC_CR;
    type PLLFGR = RCC_PLLFGR;
    type AHB1ENR = RCC_AHB1ENR;
}

// -----------------------------------------------------------------------------
// RCC set of registers

const RCC_CR_ADDR: u32 = RCC_ADDR + 0x0; // RM, page 226
const RCC_PLLCFGR_ADDR: u32 = RCC_ADDR + 0x4; // RM, page 228
const RCC_AHB1ENR_ADDR: u32 = RCC_ADDR + 0x30; // RM, page 244

pub struct RCC_CR;

impl RCC_CR {}

pub struct RCC_PLLFGR;

impl RCC_PLLFGR {}

pub struct RCC_AHB1ENR;

impl RCC_AHB1ENR {
    pub fn ahb1p_en<B>()
    where
        B: Ahb1Compat,
    {
        bitfld_check!(V, 1, 1);
        B::set::<V>();
    }

    pub fn ahb1p_dis<B>()
    where
        B: Ahb1Compat,
    {
        bitfld_check!(V, 1, 0);
        B::set::<V>();
    }
}

// Bit fields compatible with AHB1ENR register
pub trait Ahb1Compat: mmio::BitFldRW {}

// -----------------------------------------------------------------------------
// RCC set of register regions

const AHB1ENR_GPIOAEN_OFFT: u32 = 0;
const AHB1ENR_GPIOBEN_OFFT: u32 = 1;
const AHB1ENR_GPIOCEN_OFFT: u32 = 2;
const AHB1ENR_GPIODEN_OFFT: u32 = 3;

pub struct AHB1ENR_GPIOxEN<const O: u32>;
impl<const O: u32> Ahb1Compat for AHB1ENR_GPIOxEN<O> {}
impl<const O: u32> BitFldRW for AHB1ENR_GPIOxEN<O> {}
impl<const O: u32> BitFldR for AHB1ENR_GPIOxEN<O> {
    const REG: u32 = RCC_AHB1ENR_ADDR;
    const OFFT: u32 = O;
    const BITS: u32 = 1;
}

pub type AHB1ENR_GPIOAEN = AHB1ENR_GPIOxEN<AHB1ENR_GPIOAEN_OFFT>;
pub type AHB1ENR_GPIOBEN = AHB1ENR_GPIOxEN<AHB1ENR_GPIOBEN_OFFT>;
pub type AHB1ENR_GPIOCEN = AHB1ENR_GPIOxEN<AHB1ENR_GPIOCEN_OFFT>;
pub type AHB1ENR_GPIODEN = AHB1ENR_GPIOxEN<AHB1ENR_GPIODEN_OFFT>;
