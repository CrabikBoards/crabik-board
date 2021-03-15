#![no_std]

use core::{
    panic::PanicInfo,
    sync::atomic::{self, Ordering},
};
use cortex_m::asm;
pub use embedded_hal::*;
use log::Log;
pub use nrf52810_hal as hal;
use nrf52810_hal::gpio::*;
pub use nrf52810_pac as pac;
pub use nrf52810_pac::{interrupt, Interrupt, Peripherals, NVIC_PRIO_BITS};
use rtt_target::rprintln;

/// Логгер с выводом через RTT
pub struct Logger;

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::STATIC_MAX_LEVEL
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        rprintln!(
            "{}:{} -- {}",
            record.level(),
            record.target(),
            record.args()
        );
    }

    fn flush(&self) {}
}

/// Выход из приложения (завершение вывода логов)
pub fn exit() -> ! {
    log::info!("`Завершение программы ...`");
    // принудительно завершает любую отложенную операцию в памяти перед инструкцией BKPT
    atomic::compiler_fence(Ordering::SeqCst);
    loop {
        asm::bkpt()
    }
}

// Обработчик паники
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("{}", info);

    // Запускает исключение HardFault которое говорит probe-run о завершении программы
    asm::udf()
}

pub struct CrabikPins {
    pub d1: p0::P0_09<Disconnected>,
    pub d2: p0::P0_10<Disconnected>,
    pub d3: p0::P0_12<Disconnected>,
    pub d4: p0::P0_14<Disconnected>,
    pub d5: p0::P0_15<Disconnected>,
    pub d6: p0::P0_16<Disconnected>,
    pub d7: p0::P0_18<Disconnected>,
    pub d8: p0::P0_20<Disconnected>,
    pub d9: p0::P0_25<Disconnected>,
    pub d10: p0::P0_00<Disconnected>,
    pub d11: p0::P0_01<Disconnected>,
    pub d12: p0::P0_06<Disconnected>,
    pub a0: p0::P0_05<Disconnected>,
    pub a1: p0::P0_04<Disconnected>,
    pub a2: p0::P0_30<Disconnected>,
    pub a3: p0::P0_28<Disconnected>,
}

/// Макрос который переименовывает пины микроконтроллера в названия пинов на плате
#[macro_export]
macro_rules! rename_pins {
    (
        $PX:ident
    ) => {
        CrabikPins {
            d1: $PX.p0_09,
            d2: $PX.p0_10,
            d3: $PX.p0_12,
            d4: $PX.p0_14,
            d5: $PX.p0_15,
            d6: $PX.p0_16,
            d7: $PX.p0_18,
            d8: $PX.p0_20,
            d9: $PX.p0_25,
            d10: $PX.p0_00,
            d11: $PX.p0_01,
            d12: $PX.p0_06,
            a0: $PX.p0_05,
            a1: $PX.p0_04,
            a2: $PX.p0_30,
            a3: $PX.p0_28,
        }
    };
}
