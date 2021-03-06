#![no_std]
#![no_main]

extern crate bcm2837_hal as hal;

use crate::hal::bcm2837::dma::{DMA, ENABLE};
use crate::hal::bcm2837::gpio::GPIO;
use crate::hal::bcm2837::mbox::MBOX;
use crate::hal::bcm2837::uart1::UART1;
use crate::hal::clocks::Clocks;
use crate::hal::dma;
use crate::hal::mailbox::*;
use crate::hal::prelude::*;
use crate::hal::serial::Serial;
use crate::hal::time::Bps;
use core::fmt::Write;
use display::embedded_graphics::prelude::*;
use display::embedded_graphics::{fonts::Font12x16, pixelcolor::Rgb888, text_12x16};
use display::{Display, SCRATCHPAD_MEM_MIN_SIZE};

fn kernel_entry() -> ! {
    let mut mbox = Mailbox::new(MBOX::new());
    let clocks = Clocks::freeze(&mut mbox).unwrap();
    let gpio = GPIO::new();
    let gp = gpio.split();

    let tx = gp.p14.into_alternate_af5();
    let rx = gp.p15.into_alternate_af5();

    let mut serial = Serial::uart1(UART1::new(), (tx, rx), Bps(115200), clocks);

    // Construct the DMA peripheral, reset and enable CH0
    let dma = DMA::new();
    let dma_parts = dma.split();
    dma_parts.enable.ENABLE.modify(ENABLE::EN0::SET);
    let mut dma_chan = dma_parts.ch0;
    dma_chan.reset();

    writeln!(serial, "DMA Channel ID: 0x{:X}", dma_chan.dma_id()).ok();

    writeln!(serial, "Embedded graphics example").ok();

    let sn = get_serial_number(&mut mbox).serial_number();
    writeln!(serial, "Serial number: {:#010X}", sn).ok();

    writeln!(serial, "Requesting default framebuffer allocation").ok();

    let fb = alloc_framebuffer(&mut mbox);

    writeln!(
        serial,
        "  width: {} height: {} pitch {} {:?}",
        fb.virt_width,
        fb.virt_height,
        fb.pitch(),
        fb.pixel_order,
    )
    .ok();

    writeln!(
        serial,
        "  address: {:#010X} bus_address: {:#010X} size: 0x{:X}",
        fb.alloc_buffer_address(),
        fb.alloc_buffer_bus_address(),
        fb.alloc_buffer_size()
    )
    .ok();

    let vc_mem_size = fb.alloc_buffer_size();
    let vc_mem_words = vc_mem_size / 4;
    writeln!(serial, "  bytes {} - words {}", vc_mem_size, vc_mem_words,).ok();
    let frontbuffer_mem = unsafe {
        core::slice::from_raw_parts_mut(fb.alloc_buffer_address() as *mut u32, vc_mem_words)
    };

    const STATIC_SIZE: usize = 800 * 600 * 4;
    assert!(vc_mem_size <= STATIC_SIZE);
    let mut backbuffer_mem = [0; STATIC_SIZE / 4];
    let mut scratchpad_mem = [0; SCRATCHPAD_MEM_MIN_SIZE / 4];

    let mut display = Display::new(
        fb,
        dma_chan,
        dma::ControlBlock::new(),
        &mut scratchpad_mem,
        &mut backbuffer_mem[..vc_mem_words],
        &mut frontbuffer_mem[..vc_mem_words],
    )
    .unwrap();

    // Clear back and front buffers
    display.clear_screen().unwrap();

    let background_color = Rgb888::new(0x00, 0xFF, 0xFF);

    // Fill the backbuffer
    display.fill_color(&background_color).unwrap();

    // DMA the backbuffer to the frontbuffer/display
    display.swap_buffers().unwrap();

    let styled_text: Font12x16<Rgb888> = text_12x16!(
        "Hello world!",
        stroke = Some(Rgb888::RED),
        fill = Some(Rgb888::GREEN)
    );

    display.draw(styled_text.translate(Point::new(100, 100)));
    display.swap_buffers().unwrap();

    writeln!(serial, "All done").ok();

    loop {}
}

fn get_serial_number(mbox: &mut Mailbox) -> GetSerialNumRepr {
    let resp = mbox
        .call(Channel::Prop, &GetSerialNumRepr::default())
        .expect("MBox call()");

    if let RespMsg::GetSerialNum(repr) = resp {
        repr
    } else {
        panic!("Invalid response\n{:#?}", resp);
    }
}

fn alloc_framebuffer(mbox: &mut Mailbox) -> AllocFramebufferRepr {
    let resp = mbox
        .call(Channel::Prop, &AllocFramebufferRepr::default())
        .expect("MBox call()");

    if let RespMsg::AllocFramebuffer(repr) = resp {
        repr
    } else {
        panic!("Invalid response\n{:#?}", resp);
    }
}

raspi3_boot::entry!(kernel_entry);
