
extern "C" {
    fn DEV_Digital_Write(input: i32) -> void;
    fn DEV_Digital_Read(input: i32) -> i32;
    
    fn DEV_SPI_WriteByte(input: i32) -> i32;
    fn DEV_SPI_Write_nByte(input: i32) -> i32;
    fn DEV_Delay_ms(input: i32) -> i32;
    
    fn DEV_Module_Init() -> u8;
    fn DEV_Module_Exit(input: i32) -> i32;
}

// pub struct TimeSpec

pub fn dev_module_init() {

}

pub fn epd_7in5_v2_init() {

}