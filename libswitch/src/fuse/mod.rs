//! Tegra210 Fuse implementation.

use crate::timer::usleep;

use register::mmio::ReadWrite;

/// Representation of the fuse registers.
#[repr(C)]
pub struct Fuse {
    pub ctrl: ReadWrite<u32>,
    pub reg_addr: ReadWrite<u32>,
    pub reg_read: ReadWrite<u32>,
    pub reg_write: ReadWrite<u32>,
    pub time_rd1: ReadWrite<u32>,
    pub time_rd2: ReadWrite<u32>,
    pub time_pgm1: ReadWrite<u32>,
    pub time_pgm2: ReadWrite<u32>,
    pub priv2intfc: ReadWrite<u32>,
    pub fusebypass: ReadWrite<u32>,
    pub privatekeydisable: ReadWrite<u32>,
    pub dis_pgm: ReadWrite<u32>,
    pub write_access: ReadWrite<u32>,
    pub pwr_good_sw: ReadWrite<u32>,
    _0x38: [ReadWrite<u32>; 0x32],
}

impl Fuse {
    pub fn get() -> *const Self {
        0x7000_F800 as *const Fuse
    }
}

/// Representation of the fuse chip.
#[repr(C)]
pub struct FuseChip {
    pub production_mode: ReadWrite<u32>,
    _0x4: ReadWrite<u32>,
    _0x8: ReadWrite<u32>,
    _0xc: ReadWrite<u32>,
    pub sku_info: ReadWrite<u32>,
    pub cpu_speedo_0: ReadWrite<u32>,
    pub cpu_iddq: ReadWrite<u32>,
    _0x1c: ReadWrite<u32>,
    _0x20: ReadWrite<u32>,
    _0x24: ReadWrite<u32>,
    pub ft_rev: ReadWrite<u32>,
    pub cpu_speedo_1: ReadWrite<u32>,
    pub cpu_speedo_2: ReadWrite<u32>,
    pub soc_speedo_0: ReadWrite<u32>,
    pub soc_speedo_1: ReadWrite<u32>,
    pub soc_speedo_2: ReadWrite<u32>,
    pub soc_iddq: ReadWrite<u32>,
    _0x44: ReadWrite<u32>,
    pub fa: ReadWrite<u32>,
    _0x4c: ReadWrite<u32>,
    _0x50: ReadWrite<u32>,
    _0x54: ReadWrite<u32>,
    _0x58: ReadWrite<u32>,
    _0x5c: ReadWrite<u32>,
    _0x60: ReadWrite<u32>,
    pub public_key: [ReadWrite<u32>; 0x8],
    pub tsensor_1: ReadWrite<u32>,
    pub tsensor_2: ReadWrite<u32>,
    _0x8c: ReadWrite<u32>,
    pub cp_rev: ReadWrite<u32>,
    _0x84: ReadWrite<u32>,
    pub tsensor_0: ReadWrite<u32>,
    pub first_bootrom_patch_size_reg: ReadWrite<u32>,
    pub security_mode: ReadWrite<u32>,
    pub private_key: [ReadWrite<u32>; 0x4],
    device_key: ReadWrite<u32>,
    _0xb8: ReadWrite<u32>,
    _0xbc: ReadWrite<u32>,
    pub reserved_sw: ReadWrite<u32>,
    pub vp8_enable: ReadWrite<u32>,
    pub reserved_odm: [ReadWrite<u32>; 0x8],
    _0xe8: ReadWrite<u32>,
    _0xec: ReadWrite<u32>,
    pub sku_usb_calib: ReadWrite<u32>,
    pub sku_direct_config: ReadWrite<u32>,
    _0xf8: ReadWrite<u32>,
    _0xfc: ReadWrite<u32>,
    pub vendor_code: ReadWrite<u32>,
    pub fab_code: ReadWrite<u32>,
    pub lot_code_0: ReadWrite<u32>,
    pub lot_code_1: ReadWrite<u32>,
    pub wafer_id: ReadWrite<u32>,
    pub x_coordinate: ReadWrite<u32>,
    pub y_coordinate: ReadWrite<u32>,
    _0x11c: ReadWrite<u32>,
    _0x120: ReadWrite<u32>,
    pub sata_calib: ReadWrite<u32>,
    pub gpu_iddq: ReadWrite<u32>,
    pub tsensor_3: ReadWrite<u32>,
    _0x130: ReadWrite<u32>,
    _0x134: ReadWrite<u32>,
    _0x138: ReadWrite<u32>,
    _0x13c: ReadWrite<u32>,
    _0x140: ReadWrite<u32>,
    _0x144: ReadWrite<u32>,
    pub opt_subrevision: ReadWrite<u32>,
    _0x14c: ReadWrite<u32>,
    _0x150: ReadWrite<u32>,
    pub tsensor_4: ReadWrite<u32>,
    pub tsensor_5: ReadWrite<u32>,
    pub tsensor_6: ReadWrite<u32>,
    pub tsensor_7: ReadWrite<u32>,
    pub opt_priv_sec_dis: ReadWrite<u32>,
    pub pkc_disable: ReadWrite<u32>,
    _0x16c: ReadWrite<u32>,
    _0x170: ReadWrite<u32>,
    _0x174: ReadWrite<u32>,
    _0x178: ReadWrite<u32>,
    _0x17c: ReadWrite<u32>,
    pub tsensor_common: ReadWrite<u32>,
    _0x184: ReadWrite<u32>,
    _0x188: ReadWrite<u32>,
    _0x18c: ReadWrite<u32>,
    _0x190: ReadWrite<u32>,
    _0x194: ReadWrite<u32>,
    _0x198: ReadWrite<u32>,
    pub debug_auth_override: ReadWrite<u32>,
    _0x1a0: ReadWrite<u32>,
    _0x1a4: ReadWrite<u32>,
    _0x1a8: ReadWrite<u32>,
    _0x1ac: ReadWrite<u32>,
    _0x1b0: ReadWrite<u32>,
    _0x1b4: ReadWrite<u32>,
    _0x1b8: ReadWrite<u32>,
    _0x1bc: ReadWrite<u32>,
    _0x1d0: ReadWrite<u32>,
    pub tsensor_8: ReadWrite<u32>,
    _0x1d8: ReadWrite<u32>,
    _0x1dc: ReadWrite<u32>,
    _0x1e0: ReadWrite<u32>,
    _0x1e4: ReadWrite<u32>,
    _0x1e8: ReadWrite<u32>,
    _0x1ec: ReadWrite<u32>,
    _0x1f0: ReadWrite<u32>,
    _0x1f4: ReadWrite<u32>,
    _0x1f8: ReadWrite<u32>,
    _0x1fc: ReadWrite<u32>,
    _0x200: ReadWrite<u32>,
    pub reserved_calib: ReadWrite<u32>,
    _0x208: ReadWrite<u32>,
    _0x20c: ReadWrite<u32>,
    _0x210: ReadWrite<u32>,
    _0x214: ReadWrite<u32>,
    _0x218: ReadWrite<u32>,
    pub tsensor_9: ReadWrite<u32>,
    _0x220: ReadWrite<u32>,
    _0x224: ReadWrite<u32>,
    _0x228: ReadWrite<u32>,
    _0x22c: ReadWrite<u32>,
    _0x230: ReadWrite<u32>,
    _0x234: ReadWrite<u32>,
    _0x238: ReadWrite<u32>,
    _0x23c: ReadWrite<u32>,
    _0x240: ReadWrite<u32>,
    _0x244: ReadWrite<u32>,
    _0x248: ReadWrite<u32>,
    _0x24c: ReadWrite<u32>,
    pub usb_calib_ext: ReadWrite<u32>,
    _0x254: ReadWrite<u32>,
    _0x258: ReadWrite<u32>,
    _0x25c: ReadWrite<u32>,
    _0x260: ReadWrite<u32>,
    _0x264: ReadWrite<u32>,
    _0x268: ReadWrite<u32>,
    _0x26c: ReadWrite<u32>,
    _0x270: ReadWrite<u32>,
    _0x274: ReadWrite<u32>,
    _0x278: ReadWrite<u32>,
    _0x27c: ReadWrite<u32>,
    pub spare_bit: [ReadWrite<u32>; 0x20],
}

impl FuseChip {
    pub fn get() -> *const Self {
        0x7000_F900 as *const FuseChip
    }
}

/// Initializes the fuse driver.
pub fn init() {
    make_registers_visible(true);
    disable_secondary_private_key();
    disable_programming();
}

/// Whether or not the fuse registers should be made visible.
pub fn make_registers_visible(make_visible: bool) {
    let misc_clk_enable_reg = unsafe { &(*(0x6000_6048 as *const ReadWrite<u32>)) };

    misc_clk_enable_reg.set((misc_clk_enable_reg.get() & 0xEFFF_FFFF) | ((make_visible & 1) << 28));
}

/// Disables all fuse programming.
pub fn disable_programming() {
    let fuse = Fuse::get();

    let dis_pgm_reg = unsafe { &(*fuse).dis_pgm };
    dis_pgm_reg.set(1);
}

pub fn disable_secondary_private_key() {
    let fuse = Fuse::get();

    let privatekeydisable_reg = unsafe { &(*fuse).privatekeydisable };
    privatekeydisable_reg.set(0x10);
}

/// Wait for the fuse driver to enter an idle state.
pub fn wait_idle() {
    let fuse = Fuse::get();

    let ctrl_reg = unsafe { &(*fuse).ctrl };

    // Wait for STATE_IDLE.
    let mut ctrl_val = 0;
    while (ctrl_val & 0xF0000) != 0x40000 {
        usleep(1);
        ctrl_val = ctrl_reg.get();
    }
}

/// Reads a fuse from the hardware array.
pub fn hardware_read(address: u32) -> u32 {
    let fuse = Fuse::get();
    wait_idle();

    // Program the target address.
    let reg_addr_reg = unsafe { &(*fuse).reg_addr };
    reg_addr_reg.set(address);

    // Enable read operation in control register.
    let ctrl_reg = unsafe { &(*fuse).ctrl };
    let mut ctrl_val = ctrl_reg.get();
    ctrl_val &= !0x3;
    ctrl_val |= 0x1; // Set FUSE_READ command.
    ctrl_reg.set(ctrl_val);

    wait_idle();

    let reg_read_reg = unsafe { &(*fuse).reg_read };
    reg_read_reg.get()
}

/// Writes a fuse to the hardware array.
pub fn hardware_write(address: u32, value: u32) {
    let fuse = Fuse::get();
    wait_idle();

    // Program the target address and value.
    let reg_addr_reg = unsafe { &(*fuse).reg_addr };
    let reg_write_reg = unsafe { &(*fuse).reg_write };
    reg_addr_reg.set(address);
    reg_write_reg.set(value);

    // Enable write operation in control register.
    let ctrl_reg = unsafe { &(*fuse).ctrl };
    let mut ctrl_val = ctrl_reg.get();
    ctrl_val &= !0x3;
    ctrl_val |= 0x2; // Set FUSE_WRITE command.
    ctrl_reg.set(ctrl_val);

    wait_idle();
}

/// Senses the fuse hardware array into the shadow cache.
pub fn hardware_sense() {
    let fuse = Fuse::get();
    wait_idle();

    // Enable sense operation in control register.
    let ctrl_reg = unsafe { &(*fuse).ctrl };
    let mut ctrl_val = ctrl_reg.get();
    ctrl_val &= !0x3;
    ctrl_val |= 0x3; // Set FUSE_SENSE command.
    ctrl_reg.set(ctrl_val);

    wait_idle();
}

/// Retrieves the Device ID from the shadow cache.
pub fn get_device_id() -> u64 {
    let fuse_chip = FuseChip::get();
    let mut device_id = 0;

    let y_coord = unsafe { &(*fuse_chip).y_coordinate.get() & 0x1FF };
    let x_coord = unsafe { &(*fuse_chip).x_coordinate.get() & 0x1FF };
    let wafer_id = unsafe { &(*fuse_chip).wafer_id.get() & 0x3F };
    let lot_code = unsafe { &(*fuse_chip).lot_code_0.get() };
    let fab_code = unsafe { &(*fuse_chip).fab_code.get() & 0x3F };

    let mut derived_lot_code = 0;
    for i in 0..5 {
        derived_lot_code = (derived_lot_code * 0x24) + ((lot_code >> (24 - 6 * i)) & 0x3F);
    }
    derived_lot_code &= 0x03FF_FFFF;

    device_id |= y_coord << 0;
    device_id |= x_coord << 9;
    device_id |= wafer_id << 18;
    device_id |= derived_lot_code << 24;
    device_id |= fab_code << 50;

    device_id
}