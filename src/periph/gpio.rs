use crate::{
    bitfld_check,
    mmio::{self, BitFldR, BitFldRW},
};

// -----------------------------------------------------------------------------
// GPIO ports

const GPIOA_ADDR: u32 = 0x40020000; // RM, page 65
const GPIOB_ADDR: u32 = 0x40020400; // RM, page 65
const GPIOC_ADDR: u32 = 0x40020800; // RM, page 65
const GPIOD_ADDR: u32 = 0x40020C00; // RM, page 65
const GPIOE_ADDR: u32 = 0x40021000; // RM, page 65
const GPIOF_ADDR: u32 = 0x40021400; // RM, page 65
const GPIOG_ADDR: u32 = 0x40021800; // RM, page 65
const GPIOH_ADDR: u32 = 0x40021C00; // RM, page 65
const GPIOI_ADDR: u32 = 0x40022000; // RM, page 65
const GPIOJ_ADDR: u32 = 0x40022400; // RM, page 65
const GPIOK_ADDR: u32 = 0x40022800; // RM, page 65

pub trait Gpio {
    type MODER;
    type ODR;
    const BASE_ADDR: u32;
}

pub struct GPIOx<const A: u32>;
impl<const A: u32> Gpio for GPIOx<A> {
    type MODER = GPIOx_MODER<A>;
    type ODR = GPIOx_ODR<A>;
    const BASE_ADDR: u32 = A;
}

pub type GPIOA = GPIOx<GPIOA_ADDR>;
pub type GPIOB = GPIOx<GPIOB_ADDR>;
pub type GPIOC = GPIOx<GPIOC_ADDR>;
pub type GPIOD = GPIOx<GPIOD_ADDR>;
pub type GPIOE = GPIOx<GPIOE_ADDR>;
pub type GPIOF = GPIOx<GPIOF_ADDR>;
pub type GPIOG = GPIOx<GPIOG_ADDR>;
pub type GPIOH = GPIOx<GPIOH_ADDR>;
pub type GPIOI = GPIOx<GPIOI_ADDR>;
pub type GPIOJ = GPIOx<GPIOJ_ADDR>;

// -----------------------------------------------------------------------------
// GPIO set of registers

const GPIOD_MODER_ADDR: u32 = <GPIOx_MODER<GPIOD_ADDR>>::REG_ADDR;
const GPIOD_ODR_ADDR: u32 = <GPIOx_ODR<GPIOD_ADDR>>::REG_ADDR;
pub trait GpioModer {
    const REG_ADDR: u32;

    fn set_in<C>()
    where
        C: GpioModerCompat<{ Self::REG_ADDR }>,
    {
        bitfld_check!(V, 2, 0b00);
        C::set::<V>();
    }

    fn set_out<C>()
    where
        C: GpioModerCompat<{ Self::REG_ADDR }>,
    {
        bitfld_check!(V, 2, 0b01);
        C::set::<V>();
    }

    fn set_af<C>()
    where
        C: GpioModerCompat<{ Self::REG_ADDR }>,
    {
        bitfld_check!(V, 2, 0b10);
        C::set::<V>();
    }

    fn set_analog<C>()
    where
        C: GpioModerCompat<{ Self::REG_ADDR }>,
    {
        bitfld_check!(V, 2, 0b11);
        C::set::<V>();
    }
}

pub struct GPIOx_MODER<const A: u32>;
impl<const A: u32> GpioModer for GPIOx_MODER<A> {
    const REG_ADDR: u32 = A + 0x0; // RM page 284
}

// Bit fields compatible with GPIOx_MODER register
pub trait GpioModerCompat<const R: u32>: mmio::BitFldRW {}

pub trait GpioOdr {
    const REG_ADDR: u32;

    fn set<C>()
    where
        C: GpioOdrCompat<{ Self::REG_ADDR }>,
    {
        bitfld_check!(V, 1, 0b1);
        C::set::<V>();
    }

    fn reset<C>()
    where
        C: GpioOdrCompat<{ Self::REG_ADDR }>,
    {
        bitfld_check!(V, 1, 0b0);
        C::set::<V>();
    }
}

pub struct GPIOx_ODR<const A: u32>;
impl<const A: u32> GpioOdr for GPIOx_ODR<A> {
    const REG_ADDR: u32 = A + 0x14; // RM page 286
}

// Bit fields compatible with GPIOx_ODR register
pub trait GpioOdrCompat<const R: u32>: mmio::BitFldRW {}

// -----------------------------------------------------------------------------
// GPIO set of register regions

const MODER_PIN12_OFFT: u32 = 24;
const MODER_PIN13_OFFT: u32 = 26;
const MODER_PIN14_OFFT: u32 = 28;
const MODER_PIN15_OFFT: u32 = 30;

pub struct GPIOx_MODERy<const R: u32, const O: u32>;

impl<const R: u32, const O: u32> GpioModerCompat<R> for GPIOx_MODERy<R, O> {}
impl<const R: u32, const O: u32> BitFldRW for GPIOx_MODERy<R, O> {}
impl<const R: u32, const O: u32> BitFldR for GPIOx_MODERy<R, O> {
    const REG: u32 = R;
    const OFFT: u32 = O;
    const BITS: u32 = 2;
}

const ODR_PIN12_OFFT: u32 = 12;
const ODR_PIN13_OFFT: u32 = 13;
const ODR_PIN14_OFFT: u32 = 14;
const ODR_PIN15_OFFT: u32 = 15;

pub struct GPIOx_ODRy<const R: u32, const O: u32>;

impl<const R: u32, const O: u32> GpioOdrCompat<R> for GPIOx_ODRy<R, O> {}
impl<const R: u32, const O: u32> BitFldRW for GPIOx_ODRy<R, O> {}
impl<const R: u32, const O: u32> BitFldR for GPIOx_ODRy<R, O> {
    const REG: u32 = R;
    const OFFT: u32 = O;
    const BITS: u32 = 1;
}

// ----------------------------------------------------------------------------
// GPIOA

// ----------------------------------------------------------------------------
// GPIOB

// ----------------------------------------------------------------------------
// GPIOC

// ----------------------------------------------------------------------------
// GPIOD

pub type GPIOD_MODER12 = GPIOx_MODERy<GPIOD_MODER_ADDR, MODER_PIN12_OFFT>;
pub type GPIOD_MODER13 = GPIOx_MODERy<GPIOD_MODER_ADDR, MODER_PIN13_OFFT>;
pub type GPIOD_MODER14 = GPIOx_MODERy<GPIOD_MODER_ADDR, MODER_PIN14_OFFT>;
pub type GPIOD_MODER15 = GPIOx_MODERy<GPIOD_MODER_ADDR, MODER_PIN15_OFFT>;

pub type GPIOD_ODR12 = GPIOx_ODRy<GPIOD_ODR_ADDR, ODR_PIN12_OFFT>;
pub type GPIOD_ODR13 = GPIOx_ODRy<GPIOD_ODR_ADDR, ODR_PIN13_OFFT>;
pub type GPIOD_ODR14 = GPIOx_ODRy<GPIOD_ODR_ADDR, ODR_PIN14_OFFT>;
pub type GPIOD_ODR15 = GPIOx_ODRy<GPIOD_ODR_ADDR, ODR_PIN15_OFFT>;

// ----------------------------------------------------------------------------
// GPIOE

// ----------------------------------------------------------------------------
// GPIOF

// ----------------------------------------------------------------------------
// GPIOG

// ----------------------------------------------------------------------------
// GPIOH

// ----------------------------------------------------------------------------
// GPIOI

// ----------------------------------------------------------------------------
// GPIOJ

// ----------------------------------------------------------------------------
// GPIOK
