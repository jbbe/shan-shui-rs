use shan_shui_paper::*;

// use UBYTE as  u8;
// #define UWORD   uint16_t
// #define UDOUBLE uint32_t

const EPD_7IN5_V2_WIDTH: u16 = 800;
const EPD_7IN5_V2_HEIGHT: u16 = 480;
const WHITE: u8 = 0xFF;
const BLACK: u8 = 0x00;

fn test() -> i32 {
    println!("EPD_7IN5_V2_test Demo\r\n");
    match dev_module_init() != 0 {
        true => return -1,
        false => (),
    }

    println!("e-Paper Init and Clear...\r\n");
    epd_7in5_v2_init();

        // struct timespec start={0,0}, finish={0,0};
        // clock_gettime(CLOCK_REALTIME,&start);
        epd_7in5_v2_clear();
        // clock_gettime(CLOCK_REALTIME,&finish);
        // printf("%ld S\r\n",finish.tv_sec-start.tv_sec);
        println!("clearing");
        dev_delay_ms(500);

        // let image_size: usize = ((EPD_7IN5_V2_WIDTH as usize + 7) / 8) * EPD_7IN5_V2_HEIGHT as usize;
        // let mut black_image = vec![WHITE; image_size];

        // println!("Paint_NewImage");
        // Paint_NewImage(black_image.as_mut_ptr(), EPD_7IN5_V2_WIDTH, EPD_7IN5_V2_HEIGHT, 0, WHITE);

        // Show BMP
        // println!("show window BMP-----------------");
        // Paint_SelectImage(black_image.as_mut_ptr());
        // Paint_Clear(WHITE);
        // let bmp_path1 = CString::new("./pic/800x480.bmp").unwrap();
        // GUI_ReadBmp(bmp_path1.as_ptr(), 0, 0);
        // EPD_7IN5_V2_Display(black_image.as_ptr());
        // DEV_Delay_ms(2000);

        // println!("show bmp------------------------");
        // Paint_SelectImage(black_image.as_mut_ptr());
        // Paint_Clear(WHITE);
        // let bmp_path2 = CString::new("./pic/100x100.bmp").unwrap();
        // GUI_ReadBmp(bmp_path2.as_ptr(), 0, 0);
        // EPD_7IN5_V2_Display(black_image.as_ptr());
        // DEV_Delay_ms(2000);

        // // Show from array
        // println!("show image for array");
        // Paint_SelectImage(black_image.as_mut_ptr());
        // Paint_Clear(WHITE);
        // Paint_DrawBitMap(gImage_7in5_V2.as_ptr());
        // EPD_7IN5_V2_Display(black_image.as_ptr());
        // DEV_Delay_ms(2000);

        // // Drawing
        // println!("SelectImage:BlackImage");
        // Paint_SelectImage(black_image.as_mut_ptr());
        // Paint_Clear(WHITE);

        // println!("Drawing:BlackImage");
        // Paint_DrawPoint(10, 80, BLACK, 1, 0);  // adjust style
        // ... other drawing functions ...

        // let hello = CString::new("hello world").unwrap();
        // let ws = CString::new("waveshare").unwrap();
        // Paint_DrawString_EN(10, 0, ws.as_ptr(), &Font16, BLACK, WHITE);
        // Paint_DrawString_EN(10, 20, hello.as_ptr(), &Font12, WHITE, BLACK);
        // Paint_DrawNum(10, 33, 123456789, &Font12, BLACK, WHITE);
        // Paint_DrawNum(10, 50, 987654321, &Font16, WHITE, BLACK);

        // let cn1 = CString::new(" ���abc").unwrap();
        // let cn2 = CString::new("΢ѩ����").unwrap();
        // Paint_DrawString_CN(130, 0, cn1.as_ptr(), &Font12CN, BLACK, WHITE);
        // Paint_DrawString_CN(130, 20, cn2.as_ptr(), &Font24CN, WHITE, BLACK);

        // println!("EPD_Display");
        // EPD_7IN5_V2_Display(black_image.as_ptr());
        // DEV_Delay_ms(2000);

        // println!("Clear...");
        // EPD_7IN5_V2_Clear();

        // println!("Goto Sleep...");
        // EPD_7IN5_V2_Sleep();
        // DEV_Delay_ms(2000);
        // println!("close 5V, Module enters 0 power consumption ...");
        // DEV_Module_Exit();

        
    /*
        //Create a new image cache
        UBYTE *BlackImage;
        /* you have to edit the startup_stm32fxxx.s file and set a big enough heap size */
        UWORD Imagesize = ((EPD_7IN5_V2_WIDTH % 8 == 0)? (EPD_7IN5_V2_WIDTH / 8 ): (EPD_7IN5_V2_WIDTH / 8 + 1)) * EPD_7IN5_V2_HEIGHT;
        if((BlackImage = (UBYTE *)malloc(Imagesize)) == NULL) {
            printf("Failed to apply for black memory...\r\n");
            return -1;
        }
        printf("Paint_NewImage\r\n");
        Paint_NewImage(BlackImage, EPD_7IN5_V2_WIDTH, EPD_7IN5_V2_HEIGHT, 0, WHITE);

    // show bmp
        printf("show window BMP-----------------\r\n");
        Paint_SelectImage(BlackImage);
        Paint_Clear(WHITE);
        GUI_ReadBmp("./pic/800x480.bmp", 0, 0);
        EPD_7IN5_V2_Display(BlackImage);
        DEV_Delay_ms(2000);

        printf("show bmp------------------------\r\n");
        Paint_SelectImage(BlackImage);
        Paint_Clear(WHITE);
        GUI_ReadBmp("./pic/100x100.bmp", 0, 0);
        EPD_7IN5_V2_Display(BlackImage);
        DEV_Delay_ms(2000);

    #if 1   // show image for array
        printf("show image for array\r\n");
        Paint_SelectImage(BlackImage);
        Paint_Clear(WHITE);
        Paint_DrawBitMap(gImage_7in5_V2);
        EPD_7IN5_V2_Display(BlackImage);
        DEV_Delay_ms(2000);
    #endif

    #if 1   // Drawing on the image
        //1.Select Image
        printf("SelectImage:BlackImage\r\n");
        Paint_SelectImage(BlackImage);
        Paint_Clear(WHITE);

        // 2.Drawing on the image
        printf("Drawing:BlackImage\r\n");
        Paint_DrawPoint(10, 80, BLACK, DOT_PIXEL_1X1, DOT_STYLE_DFT);
        Paint_DrawPoint(10, 90, BLACK, DOT_PIXEL_2X2, DOT_STYLE_DFT);
        Paint_DrawPoint(10, 100, BLACK, DOT_PIXEL_3X3, DOT_STYLE_DFT);
        Paint_DrawLine(20, 70, 70, 120, BLACK, DOT_PIXEL_1X1, LINE_STYLE_SOLID);
        Paint_DrawLine(70, 70, 20, 120, BLACK, DOT_PIXEL_1X1, LINE_STYLE_SOLID);
        Paint_DrawRectangle(20, 70, 70, 120, BLACK, DOT_PIXEL_1X1, DRAW_FILL_EMPTY);
        Paint_DrawRectangle(80, 70, 130, 120, BLACK, DOT_PIXEL_1X1, DRAW_FILL_FULL);
        Paint_DrawCircle(45, 95, 20, BLACK, DOT_PIXEL_1X1, DRAW_FILL_EMPTY);
        Paint_DrawCircle(105, 95, 20, WHITE, DOT_PIXEL_1X1, DRAW_FILL_FULL);
        Paint_DrawLine(85, 95, 125, 95, BLACK, DOT_PIXEL_1X1, LINE_STYLE_DOTTED);
        Paint_DrawLine(105, 75, 105, 115, BLACK, DOT_PIXEL_1X1, LINE_STYLE_DOTTED);
        Paint_DrawString_EN(10, 0, "waveshare", &Font16, BLACK, WHITE);
        Paint_DrawString_EN(10, 20, "hello world", &Font12, WHITE, BLACK);
        Paint_DrawNum(10, 33, 123456789, &Font12, BLACK, WHITE);
        Paint_DrawNum(10, 50, 987654321, &Font16, WHITE, BLACK);
        Paint_DrawString_CN(130, 0, " ���abc", &Font12CN, BLACK, WHITE);
        Paint_DrawString_CN(130, 20, "΢ѩ����", &Font24CN, WHITE, BLACK);

        printf("EPD_Display\r\n");
        EPD_7IN5_V2_Display(BlackImage);
        DEV_Delay_ms(2000);
    #endif

        printf("Clear...\r\n");
        EPD_7IN5_V2_Clear();

        printf("Goto Sleep...\r\n");
        EPD_7IN5_V2_Sleep();
        free(BlackImage);
        BlackImage = NULL;
        DEV_Delay_ms(2000);//important, at least 2s
        // close 5V
        printf("close 5V, Module enters 0 power consumption ...\r\n");
        DEV_Module_Exit();

        */
    return 0;
}

fn main() {
    println!("Begin Test...");
    test();
    println!("Success");
    
}
