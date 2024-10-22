#![no_std]
#![no_main]
#![feature(linkage)]
#![feature(generic_const_exprs)]

mod mmio;
mod sys;
mod vectors;

mod periph;

use crate::periph::gpio::GpioModer;
use crate::periph::gpio::GpioOdr;
use periph::gpio::GpioModerCompat;

fn stub_delay() {
    for _ in 0..(1 << 12) {
        unsafe { core::ptr::read_volatile(0x40020000 as *const u32) };
    }
}

fn call_example<R, F>()
where
    R: periph::gpio::Gpio<MODER: GpioModer>,
    F: GpioModerCompat<{ R::MODER::REG_ADDR }>,
{
    R::MODER::set_out::<F>();
}

pub fn main() {
    // Turn on RCC clocks on port D
    {
        type Reg = <periph::rcc::RCC as periph::rcc::Rcc>::AHB1ENR;
        type Fld = periph::rcc::AHB1ENR_GPIODEN;
        Reg::ahb1p_en::<Fld>();
    }

    // Set output of PD12 to PD15 to output
    {
        type Reg = <periph::gpio::GPIOD as periph::gpio::Gpio>::MODER;
        type Mode12 = periph::gpio::GPIOD_MODER12;
        type Mode13 = periph::gpio::GPIOD_MODER13;
        type Mode14 = periph::gpio::GPIOD_MODER14;
        type Mode15 = periph::gpio::GPIOD_MODER15;

        Reg::set_out::<Mode12>();
        Reg::set_out::<Mode13>();
        Reg::set_out::<Mode14>();
        Reg::set_out::<Mode15>();
    }

    // Toggle all PD12 to PD15
    loop {
        type Reg = <periph::gpio::GPIOD as periph::gpio::Gpio>::ODR;
        type Pin12 = periph::gpio::GPIOD_ODR12;
        type Pin13 = periph::gpio::GPIOD_ODR13;
        type Pin14 = periph::gpio::GPIOD_ODR14;
        type Pin15 = periph::gpio::GPIOD_ODR15;

        Reg::set::<Pin12>();
        Reg::set::<Pin13>();
        Reg::set::<Pin14>();
        Reg::set::<Pin15>();

        stub_delay();

        Reg::reset::<Pin12>();
        Reg::reset::<Pin13>();
        Reg::reset::<Pin14>();
        Reg::reset::<Pin15>();

        stub_delay();
    }
}

#[no_mangle]
pub unsafe extern "C" fn custom_fault_handler() {
    loop {}
}
