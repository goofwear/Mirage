use mirage_mmio::{Mmio, VolatileStorage};

use super::display_config::*;
use crate::{
    clock::{Car, CLOCK_BASE},
    gpio::{Gpio, GpioDirection, GpioLevel, GpioMode},
    i2c::*,
    pinmux::{Pinmux, TRISTATE},
    pmc::Pmc,
    timer::{get_microseconds, usleep},
};

static mut DISPLAY_VERSION: u32 = 0;

/// Base address for DI registers.
pub(crate) const DI_BASE: u32 = 0x5420_0000;

/// Base address for DSI registers.
pub(crate) const DSI_BASE: u32 = 0x5430_0000;

/// Base address for MIPI CAL registers.
pub(crate) const MIPI_CAL_BASE: u32 = 0x700E_3000;

/// Waits for DSI to be updated.
unsafe fn dsi_wait(timeout: u32, offset: u32, mask: u32) {
    let register = &*((DSI_BASE + offset * 4) as *const Mmio<u32>);
    let end = get_microseconds() + timeout;

    while get_microseconds() < end && register.read() & mask != 0 {
        // Wait.
    }
    usleep(5);
}

/// Initializes the display.
pub fn initialize() {
    let car = unsafe { Car::get() };
    let pinmux = unsafe { Pinmux::get() };
    let pmc = unsafe { Pmc::get() };

    // Power on.
    I2c::C5
        .write_byte(Device::Max77620Pwr, 0x23, 0xD0)
        .unwrap();
    I2c::C5
        .write_byte(Device::Max77620Pwr, 0x3D, 0x9)
        .unwrap();

    // Enable MIPI CAL, DSI, DISP1, HOST1X, UART_FST_MIPI_CAL, DSIA LP clocks.
    car.rst_dev_h_clr.write(0x1010000);
    car.clk_enb_h_set.write(0x1010000);
    car.rst_dev_l_clr.write(0x1800_0000);
    car.clk_enb_l_set.write(0x1800_0000);
    car.clk_enb_x_set.write(0x20000);
    car.clk_source_uart_fst_mipi_cal.write(0xA);
    car.clk_enb_w_set.write(0x80000);
    car.clk_source_dsia_lp.write(0xA);

    // DPD idle.
    pmc.io_dpd_req.write(0x4000_0000);
    pmc.io_dpd2_req.write(0x4000_0000);

    // Configure pins.
    pinmux.nfc_en.write(pinmux.nfc_en.read() & !TRISTATE);
    pinmux.nfc_int.write(pinmux.nfc_int.read() & !TRISTATE);
    pinmux
        .lcd_bl_pwm
        .write(pinmux.lcd_bl_pwm.read() & !TRISTATE);
    pinmux.lcd_bl_en.write(pinmux.lcd_bl_en.read() & !TRISTATE);
    pinmux.lcd_rst.write(pinmux.lcd_rst.read() & !TRISTATE);

    // Configure Backlight +-5V GPIOs.
    Gpio::LCD_BL_P5V.set_mode(GpioMode::GPIO);
    Gpio::LCD_BL_N5V.set_mode(GpioMode::GPIO);
    Gpio::LCD_BL_P5V.set_direction(GpioDirection::Output);
    Gpio::LCD_BL_N5V.set_direction(GpioDirection::Output);

    // Enable Backlight +5V.
    Gpio::LCD_BL_P5V.write(GpioLevel::High);

    usleep(10_000);

    // Enable Backlight -5V.
    Gpio::LCD_BL_N5V.write(GpioLevel::High);

    usleep(10_000);

    // Configure Backlight PWM, EN and RST GPIOs.
    Gpio::LCD_BL_PWM.set_mode(GpioMode::GPIO);
    Gpio::LCD_BL_EN.set_mode(GpioMode::GPIO);
    Gpio::LCD_BL_RST.set_mode(GpioMode::GPIO);
    Gpio::LCD_BL_PWM.set_direction(GpioDirection::Output);
    Gpio::LCD_BL_EN.set_direction(GpioDirection::Output);
    Gpio::LCD_BL_RST.set_direction(GpioDirection::Output);

    // Enable Backlight EN.
    Gpio::LCD_BL_EN.write(GpioLevel::High);

    unsafe {
        // Configure display interface and display.
        (*((MIPI_CAL_BASE + 0x60) as *const Mmio<u32>)).write(0);

        execute(CLOCK_BASE as *mut u32, &DISPLAY_CONFIG_1);
        execute(DI_BASE as *mut u32, &DISPLAY_CONFIG_2);
        execute(DSI_BASE as *mut u32, &DISPLAY_CONFIG_3);
    }

    usleep(10_000);

    // Enable Backlight RST.
    Gpio::LCD_BL_RST.write(GpioLevel::High);

    usleep(60_000);

    unsafe {
        (*((DSI_BASE + 0x3F * 4) as *const Mmio<u32>)).write(0x50204);
        (*((DSI_BASE + 0xA * 4) as *const Mmio<u32>)).write(0x337);
        (*((DSI_BASE + 0x13 * 4) as *const Mmio<u32>)).write(1 << 1);

        dsi_wait(250_000, 0x13, 0x3);

        (*((DSI_BASE + 0xA * 4) as *const Mmio<u32>)).write(0x406);
        (*((DSI_BASE + 0x13 * 4) as *const Mmio<u32>)).write(1 << 1);
        dsi_wait(250_000, 0x13, 0x3);

        (*((DSI_BASE + 0xF * 4) as *const Mmio<u32>)).write(0x200B);
        dsi_wait(150_000, 0xF, 1 << 3);

        usleep(5_000);

        DISPLAY_VERSION = (*((DSI_BASE + 0x9 * 4) as *const Mmio<u32>)).read();

        if DISPLAY_VERSION == 0x10 {
            execute(DSI_BASE as *mut u32, &DISPLAY_CONFIG_4);
        }

        (*((DSI_BASE + 0xA * 4) as *const Mmio<u32>)).write(0x1105);
        (*((DSI_BASE + 0x13 * 4) as *const Mmio<u32>)).write(1 << 1);

        usleep(180_000);

        (*((DSI_BASE + 0xA * 4) as *const Mmio<u32>)).write(0x2905);
        (*((DSI_BASE + 0x13 * 4) as *const Mmio<u32>)).write(1 << 1);

        usleep(20_000);

        execute(DSI_BASE as *mut u32, &DISPLAY_CONFIG_5);
        execute(CLOCK_BASE as *mut u32, &DISPLAY_CONFIG_6);

        (*((DI_BASE + 0x42E * 4) as *const Mmio<u32>)).write(4);
        execute(DSI_BASE as *mut u32, &DISPLAY_CONFIG_7);

        usleep(10_000);

        execute(MIPI_CAL_BASE as *mut u32, &DISPLAY_CONFIG_8);
        execute(DSI_BASE as *mut u32, &DISPLAY_CONFIG_9);
        execute(MIPI_CAL_BASE as *mut u32, &DISPLAY_CONFIG_10);

        usleep(10_000);

        execute(DI_BASE as *mut u32, &DISPLAY_CONFIG_11);
    }
}

/// Turns the display off.
pub fn finish() {
    let car = unsafe { Car::get() };
    let pinmux = unsafe { Pinmux::get() };

    // Disable backlight.
    hide_backlight();

    unsafe {
        (*((DSI_BASE + 0x4E * 4) as *const Mmio<u32>)).write(1);
        (*((DSI_BASE + 0xA * 4) as *const Mmio<u32>)).write(0x2805);

        (*((DI_BASE + 0x40 * 4) as *const Mmio<u32>)).write(0x5);
        (*((DSI_BASE + 0x4E * 4) as *const Mmio<u32>)).write(0);

        execute(DI_BASE as *mut u32, &DISPLAY_CONFIG_12);
        execute(DSI_BASE as *mut u32, &DISPLAY_CONFIG_13);

        usleep(10_000);

        if DISPLAY_VERSION == 0x10 {
            execute(DSI_BASE as *mut u32, &DISPLAY_CONFIG_14);
        }

        (*((DSI_BASE + 0xA * 4) as *const Mmio<u32>)).write(0x1005);
        (*((DSI_BASE + 0x13 * 4) as *const Mmio<u32>)).write(1 << 1);
    }

    usleep(50_000);

    // Disable Backlight RST.
    Gpio::LCD_BL_RST.write(GpioLevel::Low);

    usleep(10_000);

    // Disable Backlight -5V.
    Gpio::LCD_BL_N5V.write(GpioLevel::Low);

    usleep(10_000);

    // Disable Backlight +5V.
    Gpio::LCD_BL_P5V.write(GpioLevel::Low);

    usleep(10_000);

    // Disable clocks.
    car.rst_dev_h_set.write(0x1010000);
    car.clk_enb_h_clr.write(0x1010000);
    car.rst_dev_l_set.write(0x1800_0000);
    car.clk_enb_l_clr.write(0x1800_0000);

    unsafe {
        (*((DSI_BASE + 0x4B * 4) as *const Mmio<u32>)).write(0x10F010F);
        (*((DSI_BASE + 0xB * 4) as *const Mmio<u32>)).write(0);
    }

    // Backlight PWM.
    Gpio::LCD_BL_PWM.set_mode(GpioMode::SFIO);

    pinmux
        .lcd_bl_pwm
        .write((pinmux.lcd_bl_pwm.read() & !TRISTATE) | TRISTATE);
    pinmux
        .lcd_bl_pwm
        .write(((pinmux.lcd_bl_pwm.read() >> 2) << 2) | 1);
}

/// Shows a single color on the display.
pub fn color_screen(color: u32) {
    unsafe {
        execute(DI_BASE as *mut u32, &DISPLAY_ONE_COLOR);
    }

    // Configure display to show a single color.
    unsafe {
        let cmd_state_control_reg = &*((DI_BASE + 0x41 * 4) as *const Mmio<u32>);

        (*((DI_BASE + 0xB80 * 4) as *const Mmio<u32>)).write(0);
        (*((DI_BASE + 0xD80 * 4) as *const Mmio<u32>)).write(0);
        (*((DI_BASE + 0xF80 * 4) as *const Mmio<u32>)).write(0);
        (*((DI_BASE + 0x4E4 * 4) as *const Mmio<u32>)).write(color);
        cmd_state_control_reg.write((cmd_state_control_reg.read() & 0xFFFF_FFFE) | (1 << 0));
    }

    usleep(35_000);

    display_backlight();
}

/// Turns the backlight on/off.
#[inline]
fn set_backlight(enable: bool) {
    let level = if enable {
        GpioLevel::High
    } else {
        GpioLevel::Low
    };

    // Enable backlight PWM.
    Gpio::LCD_BL_PWM.write(level);
}

/// Displays the backlight.
#[inline]
pub fn display_backlight() {
    set_backlight(true);
}

/// Disables the backlight.
#[inline]
pub fn hide_backlight() {
    set_backlight(false);
}

/// Initializes display in full 1280x720 resolution.
/// (B8G8R8A8, line stride 768, framebuffer size = 1280*768*4 bytes).
pub fn initialize_framebuffer(address: u32) {
    let mut config: [ConfigTable; 32] = [ConfigTable::new(); 32];
    config.copy_from_slice(&DISPLAY_FRAMEBUFFER);

    config[19].value = address;

    // This configures the framebuffer @ address with a resolution of 1280x720 (line stride 768).
    unsafe {
        execute(DI_BASE as *mut u32, &config);
    }
}
