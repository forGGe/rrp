use crate::{custom_fault_handler, sys::early_main};

macro_rules! define_isr {
    ($n:ident, $d:ident) => {
        #[no_mangle]
        pub static $n: unsafe extern "C" fn() = $d;
    };
}

macro_rules! isr_reserved {
    () => {
        Vector { reserved: 0 }
    };
}

macro_rules! isr_link {
    ($x:ident) => {
        Vector { handler: $x }
    };
}

pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

// The reset vector, a pointer into the reset handler
#[link_section = ".vector_table.reset_vector"]
#[used]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

// ARM-specific exceptions
define_isr!(NMI, custom_fault_handler);
define_isr!(HardFault, DefaultExceptionHandler);
define_isr!(MemManage, DefaultExceptionHandler);
define_isr!(BusFault, DefaultExceptionHandler);
define_isr!(UsageFault, DefaultExceptionHandler);
define_isr!(SVCall, DefaultExceptionHandler);
define_isr!(PendSV, DefaultExceptionHandler);
define_isr!(SysTick, DefaultExceptionHandler);

// Vendor-specific interrupt
define_isr!(WWDG, DefaultExceptionHandler);
define_isr!(PVD, DefaultExceptionHandler);
define_isr!(TAMP_STAMP, DefaultExceptionHandler);
define_isr!(RTC_WKUP, DefaultExceptionHandler);
define_isr!(FLASH, DefaultExceptionHandler);
define_isr!(RCC, DefaultExceptionHandler);
define_isr!(EXTI0, DefaultExceptionHandler);
define_isr!(EXTI1, DefaultExceptionHandler);
define_isr!(EXTI2, DefaultExceptionHandler);
define_isr!(EXTI3, DefaultExceptionHandler);
define_isr!(EXTI4, DefaultExceptionHandler);
define_isr!(DMA1_Stream0, DefaultExceptionHandler);
define_isr!(DMA1_Stream1, DefaultExceptionHandler);
define_isr!(DMA1_Stream2, DefaultExceptionHandler);
define_isr!(DMA1_Stream3, DefaultExceptionHandler);
define_isr!(DMA1_Stream4, DefaultExceptionHandler);
define_isr!(DMA1_Stream5, DefaultExceptionHandler);
define_isr!(DMA1_Stream6, DefaultExceptionHandler);
define_isr!(ADC, DefaultExceptionHandler);
define_isr!(CAN1_TX, DefaultExceptionHandler);
define_isr!(CAN1_RX0, DefaultExceptionHandler);
define_isr!(CAN1_RX1, DefaultExceptionHandler);
define_isr!(CAN1_SCE, DefaultExceptionHandler);
define_isr!(EXTI9_5, DefaultExceptionHandler);
define_isr!(TIM1_BRK_TIM9, DefaultExceptionHandler);
define_isr!(TIM1_UP_TIM10, DefaultExceptionHandler);
define_isr!(TIM1_TRG_COM_TIM11, DefaultExceptionHandler);
define_isr!(TIM1_CC, DefaultExceptionHandler);
define_isr!(TIM2, DefaultExceptionHandler);
define_isr!(TIM3, DefaultExceptionHandler);
define_isr!(TIM4, DefaultExceptionHandler);
define_isr!(I2C1_EV, DefaultExceptionHandler);
define_isr!(I2C1_ER, DefaultExceptionHandler);
define_isr!(I2C2_EV, DefaultExceptionHandler);
define_isr!(I2C2_ER, DefaultExceptionHandler);
define_isr!(SPI1, DefaultExceptionHandler);
define_isr!(SPI2, DefaultExceptionHandler);
define_isr!(USART1, DefaultExceptionHandler);
define_isr!(USART2, DefaultExceptionHandler);
define_isr!(USART3, DefaultExceptionHandler);
define_isr!(EXTI15_10, DefaultExceptionHandler);
define_isr!(RTC_Alarm, DefaultExceptionHandler);
define_isr!(OTG_FS_WKUP, DefaultExceptionHandler);
define_isr!(TIM8_BRK_TIM12, DefaultExceptionHandler);
define_isr!(TIM8_UP_TIM13, DefaultExceptionHandler);
define_isr!(TIM8_TRG_COM_TIM14, DefaultExceptionHandler);
define_isr!(TIM8_CC, DefaultExceptionHandler);
define_isr!(DMA1_Stream7, DefaultExceptionHandler);
define_isr!(FSMC, DefaultExceptionHandler);
define_isr!(SDIO, DefaultExceptionHandler);
define_isr!(SPI3, DefaultExceptionHandler);
define_isr!(UART4, DefaultExceptionHandler);
define_isr!(UART5, DefaultExceptionHandler);
define_isr!(TIM6_DAC, DefaultExceptionHandler);
define_isr!(TIM7, DefaultExceptionHandler);
define_isr!(DMA2_Stream0, DefaultExceptionHandler);
define_isr!(DMA2_Stream1, DefaultExceptionHandler);
define_isr!(DMA2_Stream2, DefaultExceptionHandler);
define_isr!(DMA2_Stream3, DefaultExceptionHandler);
define_isr!(DMA2_Stream4, DefaultExceptionHandler);
define_isr!(ETH, DefaultExceptionHandler);
define_isr!(ETH_WKUP, DefaultExceptionHandler);
define_isr!(CAN2_TX, DefaultExceptionHandler);
define_isr!(CAN2_RX0, DefaultExceptionHandler);
define_isr!(CAN2_RX1, DefaultExceptionHandler);
define_isr!(CAN2_SCE, DefaultExceptionHandler);
define_isr!(OTG_FS, DefaultExceptionHandler);
define_isr!(DMA2_Stream5, DefaultExceptionHandler);
define_isr!(DMA2_Stream6, DefaultExceptionHandler);
define_isr!(DMA2_Stream7, DefaultExceptionHandler);
define_isr!(USART6, DefaultExceptionHandler);
define_isr!(I2C3_EV, DefaultExceptionHandler);
define_isr!(I2C3_ER, DefaultExceptionHandler);
define_isr!(OTG_HS_EP1_OUT, DefaultExceptionHandler);
define_isr!(OTG_HS_EP1_IN, DefaultExceptionHandler);
define_isr!(OTG_HS_WKUP, DefaultExceptionHandler);
define_isr!(OTG_HS, DefaultExceptionHandler);
define_isr!(DCMI, DefaultExceptionHandler);
define_isr!(CRYP, DefaultExceptionHandler);
define_isr!(HASH_RNG, DefaultExceptionHandler);
define_isr!(FPU, DefaultExceptionHandler);

#[link_section = ".vector_table.exceptions"]
#[used]
#[no_mangle]
pub static EXCEPTIONS: [Vector; 14] = [
    isr_link!(NMI),
    isr_link!(HardFault),
    isr_link!(MemManage),
    isr_link!(BusFault),
    isr_link!(UsageFault),
    isr_reserved!(),
    isr_reserved!(),
    isr_reserved!(),
    isr_reserved!(),
    isr_link!(SVCall),
    isr_reserved!(),
    isr_reserved!(),
    isr_link!(PendSV),
    isr_link!(SysTick),
];

#[link_section = ".vector_table.interrupts"]
#[used]
#[no_mangle]
pub static IRQS: [Vector; 81] = [
    // Window Watchdog interrupt
    isr_link!(WWDG),
    // PVD through EXTI line detection interrupt
    isr_link!(PVD),
    // Tamper and TimeStamp interrupts through the EXTI line
    isr_link!(TAMP_STAMP),
    // RTC Wake-up interrupt through the EXTI line
    isr_link!(RTC_WKUP),
    // Flash global interrupt
    isr_link!(FLASH),
    // RCC global interrupt
    isr_link!(RCC),
    // EXTI Line0 interrupt
    isr_link!(EXTI0),
    // EXTI Line1 interrupt
    isr_link!(EXTI1),
    // EXTI Line2 interrupt
    isr_link!(EXTI2),
    // EXTI Line3 interrupt
    isr_link!(EXTI3),
    // EXTI Line4 interrupt
    isr_link!(EXTI4),
    // DMA1 Stream0 global interrupt
    isr_link!(DMA1_Stream0),
    // DMA1 Stream1 global interrupt
    isr_link!(DMA1_Stream1),
    // DMA1 Stream2 global interrupt
    isr_link!(DMA1_Stream2),
    // DMA1 Stream3 global interrupt
    isr_link!(DMA1_Stream3),
    // DMA1 Stream4 global interrupt
    isr_link!(DMA1_Stream4),
    // DMA1 Stream5 global interrupt
    isr_link!(DMA1_Stream5),
    // DMA1 Stream6 global interrupt
    isr_link!(DMA1_Stream6),
    // ADC1, ADC2 and ADC3 global interrupts
    isr_link!(ADC),
    // CAN1 TX interrupts
    isr_link!(CAN1_TX),
    // CAN1 RX0 interrupts
    isr_link!(CAN1_RX0),
    // CAN1 RX1 interrupt
    isr_link!(CAN1_RX1),
    // CAN1 SCE interrupt
    isr_link!(CAN1_SCE),
    // EXTI Line[9:5] interrupts
    isr_link!(EXTI9_5),
    // TIM1 Break interrupt and TIM9 global interrupt
    isr_link!(TIM1_BRK_TIM9),
    // TIM1 Update interrupt and TIM10 global interrupt
    isr_link!(TIM1_UP_TIM10),
    // TIM1 Trigger and Commutation interrupts and TIM11 global interrupt
    isr_link!(TIM1_TRG_COM_TIM11),
    // TIM1 Capture Compare interrupt
    isr_link!(TIM1_CC),
    // TIM2 global interrupt
    isr_link!(TIM2),
    // TIM3 global interrupt
    isr_link!(TIM3),
    // TIM4 global interrupt
    isr_link!(TIM4),
    // I2C1 event interrupt
    isr_link!(I2C1_EV),
    // I2C1 error interrupt
    isr_link!(I2C1_ER),
    // I2C2 event interrupt
    isr_link!(I2C2_EV),
    // I2C2 error interrupt
    isr_link!(I2C2_ER),
    // SPI1 global interrupt
    isr_link!(SPI1),
    // SPI2 global interrupt
    isr_link!(SPI2),
    // USART1 global interrupt
    isr_link!(USART1),
    // USART2 global interrupt
    isr_link!(USART2),
    // USART3 global interrupt
    isr_link!(USART3),
    // EXTI Line[15:10] interrupts
    isr_link!(EXTI15_10),
    // RTC Alarms (A and B) through EXTI line interrupt
    isr_link!(RTC_Alarm),
    // USB On-The-Go FS Wake-up through EXTI line interrupt
    isr_link!(OTG_FS_WKUP),
    // TIM8 Break interrupt and TIM12 global interrupt
    isr_link!(TIM8_BRK_TIM12),
    // TIM8 Update interrupt and TIM13 global interrupt
    isr_link!(TIM8_UP_TIM13),
    // TIM8 Trigger and Commutation interrupts and TIM14 global interrupt
    isr_link!(TIM8_TRG_COM_TIM14),
    // TIM8 Capture Compare interrupt
    isr_link!(TIM8_CC),
    // DMA1 Stream7 global interrupt
    isr_link!(DMA1_Stream7),
    // FSMC global interrupt
    isr_link!(FSMC),
    // SDIO global interrupt
    isr_link!(SDIO),
    // SPI3 global interrupt
    isr_link!(SPI3),
    // UART4 global interrupt
    isr_link!(UART4),
    // UART5 global interrupt
    isr_link!(UART5),
    // TIM6 global interrupt, DAC1 and DAC2 underrun error interrupts
    isr_link!(TIM6_DAC),
    // TIM7 global interrupt
    isr_link!(TIM7),
    // DMA2 Stream0 global interrupt
    isr_link!(DMA2_Stream0),
    // DMA2 Stream1 global interrupt
    isr_link!(DMA2_Stream1),
    // DMA2 Stream2 global interrupt
    isr_link!(DMA2_Stream2),
    // DMA2 Stream3 global interrupt
    isr_link!(DMA2_Stream3),
    // DMA2 Stream4 global interrupt
    isr_link!(DMA2_Stream4),
    // Ethernet global interrupt
    isr_link!(ETH),
    // Ethernet Wake-up through EXTI line interrupt
    isr_link!(ETH_WKUP),
    // CAN2 TX interrupts
    isr_link!(CAN2_TX),
    // CAN2 RX0 interrupts
    isr_link!(CAN2_RX0),
    // CAN2 RX1 interrupt
    isr_link!(CAN2_RX1),
    // CAN2 SCE interrupt
    isr_link!(CAN2_SCE),
    // USB On The Go FS global interrupt
    isr_link!(OTG_FS),
    // DMA2 Stream5 global interrupt
    isr_link!(DMA2_Stream5),
    // DMA2 Stream6 global interrupt
    isr_link!(DMA2_Stream6),
    // DMA2 Stream7 global interrupt
    isr_link!(DMA2_Stream7),
    // USART6 global interrupt
    isr_link!(USART6),
    // I2C3 event interrupt
    isr_link!(I2C3_EV),
    // I2C3 error interrupt
    isr_link!(I2C3_ER),
    // USB On The Go HS End Point 1 Out global interrupt
    isr_link!(OTG_HS_EP1_OUT),
    // USB On The Go HS End Point 1 In global interrupt
    isr_link!(OTG_HS_EP1_IN),
    // USB On The Go HS Wake-up through EXTI interrupt
    isr_link!(OTG_HS_WKUP),
    // USB On The Go HS global interrupt
    isr_link!(OTG_HS),
    // DCMI global interrupt
    isr_link!(DCMI),
    // CRYP crypto global interrupt
    isr_link!(CRYP),
    // Hash and Rng global interrupt
    isr_link!(HASH_RNG),
    // FPU global interrupt
    isr_link!(FPU),
];

// The reset handler
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    early_main();
    loop {}
}

#[no_mangle]
pub extern "C" fn DefaultExceptionHandler() {
    loop {}
}
