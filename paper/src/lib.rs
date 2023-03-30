
extern "C" {
    fn DEV_Digital_Write(input: i32);
    fn DEV_Digital_Read(input: i32) -> i32;
    
    fn DEV_SPI_WriteByte(input: i32) -> i32;
    fn DEV_SPI_Write_nByte(input: i32) -> i32;
    fn DEV_Delay_ms(xms: u32);
    
    fn DEV_Module_Init() -> u8;
    fn DEV_Module_Exit(input: i32) -> i32;

    fn EPD_7IN5B_V2_Init() -> usize;
    fn EPD_7IN5B_V2_Clear();
    fn EPD_7IN5B_V2_ClearRed();
    fn EPD_7IN5B_V2_ClearBlack();
    fn EPD_7IN5B_V2_Display(blackimage: &u8, ryimage: &u8);
    fn EPD_7IN5B_V2_Sleep();

}

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
    unsafe { EPD_7IN5B_V2_Clear() };
}

