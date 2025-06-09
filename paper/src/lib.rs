include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// pub struct TimeSpec

pub fn dev_module_init() -> u8 {
    return unsafe { DEV_Module_Init() };
}

pub fn epd_7in5_v2_init() {

}

pub fn dev_delay_ms(ms: u32) {
    unsafe { DEV_Delay_ms(ms) }
}

pub fn epd_7in5_v2_clear() -> () {
    unsafe { EPD_7IN5_V2_Clear() };
}

