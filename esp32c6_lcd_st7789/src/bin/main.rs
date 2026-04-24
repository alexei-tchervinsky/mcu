#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use defmt as _; // Глобальный обработчик defmt
use esp_backtrace as _; // Или panic-probe
use esp_println as _; // Подключает дефолтный вывод

use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_10X20}, // Импорт шрифта и стиля
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Arc, Circle, PrimitiveStyle, PrimitiveStyleBuilder},
    text::Text, // Импорт структуры Text
};

use embedded_graphics::draw_target::DrawTarget;
use esp_hal::Async;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::spi::master::{Config, Spi, SpiDmaBus};
use mipidsi::interface::SpiInterface;

use esp_hal::time::Rate;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use esp_hal::interrupt::software::SoftwareInterruptControl;
use mipidsi::Builder;
use static_cell::StaticCell;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
// 1. ОБЪЯВЛЕНИЕ ДОЛЖНО БЫТЬ ТУТ (ВНЕ MAIN)
use mipidsi::{Display, models::ST7789};
// Обертка, которая делает наш асинхронный SPI "понятным" для mipidsi
pub struct MySpiDevice<SPI>(pub SPI);

impl<SPI: embedded_hal::spi::SpiBus> embedded_hal::spi::ErrorType for MySpiDevice<SPI> {
    // impl<SPI: esp_hal::spi::master::SpiBus> embedded_hal::spi::ErrorType for MySpiDevice<SPI> {
    type Error = SPI::Error;
}

// Реализуем синхронный трейт, который внутри использует эффективный DMA
impl<SPI: embedded_hal::spi::SpiBus> embedded_hal::spi::SpiDevice for MySpiDevice<SPI> {
    // impl<SPI: esp_hal::spi::master::SpiBus> embedded_hal::spi::SpiDevice for MySpiDevice<SPI> {
    fn transaction(
        &mut self,
        operations: &mut [embedded_hal::spi::Operation<'_, u8>],
    ) -> Result<(), Self::Error> {
        for op in operations {
            match op {
                embedded_hal::spi::Operation::Write(buf) => self.0.write(buf)?,
                embedded_hal::spi::Operation::Transfer(read, write) => {
                    self.0.transfer(read, write)?
                }
                _ => {}
            }
        }
        Ok(())
    }
}

// Удобный алиас для типа дисплея
type MyDisplay = Display<
    SpiInterface<'static, MySpiDevice<SpiDmaBus<'static, Async>>, Output<'static>>,
    ST7789,
    Output<'static>, // RST пин
>;

#[derive(Clone, Copy)] // Важно: Signal копирует данные
struct Rgb565Arr {
    color1: Rgb565,
    color2: Rgb565,
}
// Сигнал, хранящий цвет
static COLOR_SIGNAL: Signal<CriticalSectionRawMutex, Rgb565Arr> = Signal::new();

// Async task producer
#[embassy_executor::task]
async fn my_async_task() {
    defmt::info!("Start async producer task");

    let mut colors_arr = Rgb565Arr {
        color1: Rgb565::BLACK,
        color2: Rgb565::YELLOW,
    };

    loop {
        // Отправляем новый цвет в сигнал
        COLOR_SIGNAL.signal(colors_arr);

        let colors_temp = colors_arr.color1;
        colors_arr.color1 = colors_arr.color2;
        colors_arr.color2 = colors_temp;

        defmt::info!("producer task sends signal");
        // Меняем цвет каждые 500 мс
        embassy_time::Timer::after_millis(500).await;
    }
}

#[embassy_executor::task]
async fn render_task(display: &'static mut MyDisplay) {
    defmt::info!("Start async render task");
    // Очищаем экран (заливка черным через DMA)
    display.clear(Rgb565::RED).unwrap();

    // face :-)
    Circle::new(Point::new(0, 100), 172)
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::YELLOW)
                .build(),
        )
        .draw(&mut *display)
        .unwrap_or_else(|_| loop {}) // do not use  strings
        ;

    // smile
    // Создаем дугу: центр (50, 50), диаметр 100, от 0° до 270°
    Arc::new(Point::new(40, 150), 100, 0.0.deg(), 180.0.deg())
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::BLACK, 10))
        .draw(&mut *display)
        .unwrap_or_else(|_| loop {}) // do not use  strings
        ;

    // left eye
    let mut left_eye = Circle::new(Point::new(40, 155), 20).into_styled(
        PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::BLACK)
            .build(),
    );

    left_eye.draw(&mut *display).unwrap_or_else(|_| loop {}); // do not use  strings

    // right eye
    let mut right_eye = Circle::new(Point::new(120, 155), 20).into_styled(
        PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::BLACK)
            .build(),
    );

    right_eye.draw(&mut *display)
        .unwrap_or_else(|_| loop {}) // do not use  strings
        ;

    // Text 1 (animated)
    let text_colors = [Rgb565::WHITE, Rgb565::BLUE, Rgb565::BLACK, Rgb565::YELLOW];
    let mut i = 0;
    // 1. Создаем стиль текста: выбираем шрифт и цвет
    let text_style = MonoTextStyle::new(&FONT_10X20, text_colors[i]);
    // 2. Создаем объект текста и отрисовываем его
    let mut text = Text::new(
        "Rust ESP32",       // Сам текст
        Point::new(40, 50), // Координаты (X, Y)
        text_style,         // Применяем созданный стиль
    );
    text.draw(&mut *display).unwrap_or_else(|_| loop {}); // do not use  strings

    // Text 2 (static)
    // 1. Создаем стиль текста: выбираем шрифт и цвет
    let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
    // 2. Создаем объект текста и отрисовываем его
    let text_framework = Text::new(
        "embassy",          // Сам текст
        Point::new(55, 75), // Координаты (X, Y)
        text_style,         // Применяем созданный стиль
    );
    text_framework
        .draw(&mut *display)
        .unwrap_or_else(|_| loop {}); // do not use  strings

    loop {
        // Ждем нового цвета. Если цвет прислали 5 раз, пока мы рисовали,
        // wait() вернет только самый последний.
        let color_arr = COLOR_SIGNAL.wait().await;
        defmt::info!("render task receives signal!");
        // animate "eyes"
        // 2. Чтобы "изменить" цвет, создаем новый объект с тем же кругом, но другим стилем
        let new_style_right_eye = PrimitiveStyle::with_fill(color_arr.color1);
        right_eye = right_eye.primitive.into_styled(new_style_right_eye); // Переопределяем п
        right_eye.draw(&mut *display).ok();

        let new_style_left_eye = PrimitiveStyle::with_fill(color_arr.color2);
        left_eye = left_eye.primitive.into_styled(new_style_left_eye); // Переопределяем п
        left_eye.draw(&mut *display).ok();
        // animate text
        text.character_style = MonoTextStyle::new(&FONT_10X20, text_colors[i]);
        text.draw(&mut *display).unwrap_or_else(|_| loop {}); // do not use  strings
        i = (i + 1) % text_colors.len();
    }
}

#[esp_rtos::main]
async fn main(spawner: embassy_executor::Spawner) {
    // Теперь всё инициализируется одной функцией
    let peripherals = esp_hal::init(esp_hal::Config::default());

    defmt::info!("Draw display ST7789 on Waveshare board ESP32C6 version 0.2.5");

    let timg0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);

    // Получаем контроль над программными прерываниями
    let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);

    // 2. Делаем конкретное прерывание статическим (нужно для esp-rtos)
    static SW_INT: StaticCell<esp_hal::interrupt::software::SoftwareInterrupt<0>> =
        StaticCell::new();
    let sw_int0_ref = SW_INT.init(sw_int.software_interrupt0);
    // 3. Читаем значение из ссылки, чтобы передать его "по значению" (move)
    // Это безопасно, так как мы делаем это один раз при старте системы
    let sw_int_val = unsafe { core::ptr::read(sw_int0_ref as *const _) };

    // Инициализация рантайма (заменяет старый init_embassy)
    // Передаем таймер и одно из программных прерываний
    esp_rtos::start(timg0.timer0, sw_int_val);
    defmt::info!("Асинхронный таймер запущен!");

    // --- Настройка Пинов (измените под вашу схему) ---
    let sclk = peripherals.GPIO7;
    let mosi = peripherals.GPIO6;

    // Вместо вызова как функции, используем метод инициализации
    let dc_config = OutputConfig::default();
    let dc = Output::new(peripherals.GPIO15, Level::Low, dc_config);
    let cs = Output::new(peripherals.GPIO14, Level::Low, OutputConfig::default());

    // 1. Настройка шины SPI (на примере esp-hal)
    use esp_hal::dma_buffers;

    // 1. Создаем буферы и дескрипторы (имена переменных должны совпадать)
    let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = dma_buffers!(3200, 3200);

    let spi_bus_driver = Spi::new(
        peripherals.SPI2,
        Config::default()
            .with_frequency(Rate::from_khz(40000)) // ST7789 обычно тянет до 40-80MHz
            .with_mode(esp_hal::spi::Mode::_3), // Важно для ST7789
    )
    .unwrap_or_else(|_| loop {}) // do not use  strings
    .with_sck(sclk)
    .with_mosi(mosi)
    // Добавляем аппаратный CS. Теперь контроллер сам им управляет.
    .with_cs(cs)
    .with_dma(peripherals.DMA_CH0) // Назначаем свободный канал DMA
    .with_buffers(
        esp_hal::dma::DmaRxBuf::new(rx_descriptors, rx_buffer).unwrap(),
        esp_hal::dma::DmaTxBuf::new(tx_descriptors, tx_buffer).unwrap(),
    );

    // Перевод в асинхронный режим
    let spi_async = spi_bus_driver.into_async();

    // Оборачиваем в нашу структуру вместо ExclusiveDevice
    let spi_device = MySpiDevice(spi_async);

    // 4. Теперь создаем интерфейс. Он НЕ владеет всей шиной SPI2.

    // Настройка пина Reset для ESP32-C6 (например, GPIO 10)
    let rst = Output::new(peripherals.GPIO21, Level::Low, OutputConfig::default());

    // Create a buffer
    static BUFFER: StaticCell<[u8; 128]> = StaticCell::new();
    let buffer = BUFFER.init([0u8; 128]);

    let di = mipidsi::interface::SpiInterface::new(spi_device, dc, buffer);

    // 2. Инициализация с явным указанием модели и отсутствием пина сброса
    let display_local = Builder::new(mipidsi::models::ST7789, di)
        .display_size(172, 320) // Физический размер матрицы
        .display_offset(34, 0) // Используйте этот метод
        .reset_pin(rst)
        .color_order(mipidsi::options::ColorOrder::Rgb)
        .invert_colors(mipidsi::options::ColorInversion::Inverted)
        .init(&mut embassy_time::Delay)
        .unwrap_or_else(|_| loop {}) // do not use  strings
        ;

    // 2. Помещаем его в статическую память, чтобы получить 'static ссылку/объект
    static DISPLAY: StaticCell<MyDisplay> = StaticCell::new();
    let display = DISPLAY.init(display_local);

    defmt::info!("backlight");
    Output::new(peripherals.GPIO22, Level::High, OutputConfig::default());

    // Запускаем задачу отрисовки
    defmt::info!("start async render task");
    spawner.spawn(render_task(display)).unwrap();

    // Запускаем задачу отрисовки
    defmt::info!("start async producer task");
    spawner.spawn(my_async_task()).unwrap();

    defmt::info!("enter main loop");

    loop {
        defmt::info!("Main loop is doing other work...");
        embassy_time::Timer::after_millis(500).await;
    }
}
