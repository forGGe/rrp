#![no_std]
#![no_main]
#![feature(linkage)]

mod mmio;
mod sys;
mod vectors;

use mmio::BitRegionR;
use mmio::BitRegionRW;

struct RCC_AHB1ENR_GPIODEN;

const RCC: u32 = 0x40023800; // RM, page 65
const RCC_AHB1ENR: u32 = RCC + 0x30; // RM, page 244
const AHB1ENR_GPIODEN_OFFT: u32 = 3;

impl mmio::BitRegionR<RCC_AHB1ENR, AHB1ENR_GPIODEN_OFFT, 1>
    for RCC_AHB1ENR_GPIODEN
{
}
impl mmio::BitRegionRW<RCC_AHB1ENR, AHB1ENR_GPIODEN_OFFT, 1>
    for RCC_AHB1ENR_GPIODEN
{
}

struct GPIOD_MODER15;

const GPIOD: u32 = 0x40020C00; // RM page 65
const GPIOD_MODER: u32 = GPIOD + 0; // RM page 284
const MODER_PIN15_OFFT: u32 = 30;

impl mmio::BitRegionR<GPIOD_MODER, MODER_PIN15_OFFT, 2> for GPIOD_MODER15 {}
impl mmio::BitRegionRW<GPIOD_MODER, MODER_PIN15_OFFT, 2> for GPIOD_MODER15 {}

struct GPIOD_ODR15;

const GPIOD_ODR: u32 = GPIOD + 0x14; // RM page 286
const ODR_PIN15_OFFT: u32 = 15;

impl mmio::BitRegionR<GPIOD_ODR, ODR_PIN15_OFFT, 1> for GPIOD_ODR15 {}
impl mmio::BitRegionRW<GPIOD_ODR, ODR_PIN15_OFFT, 1> for GPIOD_ODR15 {}

fn stub_delay() {
    for _ in 0..(1 << 12) {
        unsafe { core::ptr::read_volatile(GPIOD_MODER as *const u32) };
    }
}

pub fn main() {
    // let b = mmio::BitRegionValue::<1, 1>::new();

    RCC_AHB1ENR_GPIODEN::set::<1>();
    GPIOD_MODER15::set::<1>();

    loop {
        stub_delay();
        GPIOD_ODR15::set::<1>();
        stub_delay();
        GPIOD_ODR15::set::<0>();
    }
}

#[no_mangle]
pub unsafe extern "C" fn custom_fault_handler() {
    loop {}
}
