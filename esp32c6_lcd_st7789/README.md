# Пример работы с дисплеем ST7789 для платы WAVESHARE с MCU esp32c6

## (платы покупаем на ОЗОН или Ali)

# Запуск (сборка и прошивка)
```rust
cargo run --release
```

Успешная сборка должна быть такой:
```rust
Updating crates.io index
     Locking 143 packages to latest Rust 1.88 compatible versions
      Adding az v1.2.1 (available: v1.3.0)
      Adding generic-array v0.14.7 (available: v0.14.9)
  Downloaded toml_parser v1.0.10+spec-1.1.0
 ... <примерно 200 крейтов (пакетов Rust)> ...

    Compiling embedded-hal-bus v0.3.0
...  <затем 18 предупреждений компилятора, это ок>...
warning: unused import: `embedded_hal_bus::spi::NoDelay`
  --> src/bin/main.rs:33:5
   |
33 | use embedded_hal_bus::spi::NoDelay;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   ...
   warning: `esp32c6_lcd_st7789` (bin "esp32c6_lcd_st7789") generated 18 warnings (run `cargo fix --bin "esp32c6_lcd_st7789" -p esp32c6_lcd_st7789` to apply 18 suggestions)
    Finished `release` profile [optimized + debuginfo] target(s) in 2m 15s
```
затем при успешной сборки сразу прошивка
```
     Running `espflash flash --monitor --chip esp32c6 target/riscv32imac-unknown-none-elf/release/esp32c6_lcd_st7789`
[2026-03-18T20:09:42Z INFO ] Serial port: '/dev/ttyACM0'
[2026-03-18T20:09:42Z INFO ] Connecting...
[2026-03-18T20:09:43Z INFO ] Using flash stub
Chip type:         esp32c6 (revision v0.1)
Crystal frequency: 40 MHz
Flash size:        4MB
Features:          WiFi 6, BT 5
MAC address:       e4:b3:23:b2:8b:b4
App/part. size:    20,944/4,128,768 bytes, 0.51%
App/part. size:    20,880/4,128,768 bytes, 0.51%
[00:00:00] [========================================]      14/14      0x0      Verifying... OK!
[00:00:00] [========================================]       1/1       0x8000   Verifying... OK!
[00:00:00] [========================================]      14/14      0x10000  Verifying... OK!
[2026-03-19T22:36:54Z INFO ] Flashing has completed!
Commands:
    CTRL+R    Reset chip
    CTRL+C    Exit
```
 и запуск приложения на плате
 ```
 ESP-ROM:esp32c6-20220919
Build:Sep 19 2022
rst:0x15 (USB_UART_HPSYS),boot:0x6f (SPI_FAST_FLASH_BOOT)
Saved PC:0x40800822
SPIWP:0xee
mode:DIO, clock div:2
load:0x40875730,len:0x175c
load:0x4086b910,len:0xec8
load:0x4086e610,len:0x31c4
entry 0x4086b91a
I (23) boot: ESP-IDF v5.5.1-838-gd66ebb86d2e 2nd stage bootloader
I (23) boot: compile time Nov 27 2025 09:46:10
I (24) boot: chip revision: v0.1
I (24) boot: efuse block revision: v0.3
I (28) boot.esp32c6: SPI Speed      : 80MHz
I (32) boot.esp32c6: SPI Mode       : DIO
I (36) boot.esp32c6: SPI Flash Size : 4MB
I (39) boot: Enabling RNG early entropy source...
I (44) boot: Partition Table:
I (46) boot: ## Label            Usage          Type ST Offset   Length
I (53) boot:  0 nvs              WiFi data        01 02 00009000 00006000
I (59) boot:  1 phy_init         RF data          01 01 0000f000 00001000
I (66) boot:  2 factory          factory app      00 00 00010000 003f0000
I (72) boot: End of partition table
I (76) esp_image: segment 0: paddr=00010020 vaddr=42000020 size=00558h (  1368) map
I (83) esp_image: segment 1: paddr=00010580 vaddr=40800000 size=00014h (    20) load
I (91) esp_image: segment 2: paddr=0001059c vaddr=4200059c size=0416ch ( 16748) map
I (101) esp_image: segment 3: paddr=00014710 vaddr=40800014 size=00a90h (  2704) load
I (106) boot: Loaded app from partition at offset 0x10000
I (110) boot: Disabling RNG early entropy source...
INFO - Draw display ST7789 on Waveshare board ESP32C6 version 0.1.2
INFO - backlight
INFO - clear display entry
INFO - clear display fini
INFO - draw
```

**на плате должен светится дисплей с красным фоном и анимированным смайликом  и текстом `Rust ESP32` / `embassy`**

## Замечания
- Приложение разработано "с нуля" с активным использованием Google AI
- Экосистема esp Rust на данный момент выглядит "нестабильной" в том отношении, что при переходе на новую версию `esp-hal` приходится сильно изменять код и подтягивать другие пакеты (крейты)
- Приложение работает **без RTOS** и **без embassy** и в **блокирующем режиме** при монопольном использовании шины `SPI`, к которой подключен дисплей
- Это прямо **bare metal**, только, конечно, с активным использованием API `esp-hal` и графической библиотеки `embedded_graphics`
- В целом код был подсказан AI, однако две ключевых проблемы пришлось решать своим умом:
-- **явное приведение типа**
```Rust
let di = mipidsi::interface::SpiInterface::new(spi_device, dc, &mut buffer);
```
иначе ошибка компиляции, очень долго пытался понять, пока по старинке не посмотрел документацию крейта `mipidsi`

-- какие пины GPIO назначать, посмотрел в примере [демо на Си  ESP-IDF](https://www.waveshare.com/wiki/ESP32-C6-LCD-1.47)

---
# Возможные проблемы и их устранение (Troubleshooting)

## Сбой прошивания
**С вероятностью близкой к 100% плата не прошивается новой прошивкой по команде**
```
cargo run --release
```
При этом после прошивки
- либо серый подсвеченный экран дисплея без требуемой картинки
- либо картинка не соответствует текущей версии кода
В обоих случаях признаком сбоя прошивания является сообщение в консоли вида (`Skipped! (checksum matches)`):
```
App/part. size:    20,880/4,128,768 bytes, 0.51%
[00:00:00] [========================================]      14/14      0x0      Skipped! (checksum matches)
[00:00:00] [========================================]       1/1       0x8000   Skipped! (checksum matches)
[00:00:00] [========================================]      14/14      0x10000  Verifying... OK!
[2026-03-19T22:33:34Z INFO ] Flashing has completed!
```
Для устранения сбоя нужно полностью очистить флеш память платы
```
espflash erase-flash
```
при очистке будет выведено (команда выполняется примерно 20 сек)
```
[2026-03-19T22:35:08Z INFO ] Serial port: '/dev/ttyACM0'
[2026-03-19T22:35:08Z INFO ] Connecting...
[2026-03-19T22:35:09Z INFO ] Using flash stub
[2026-03-19T22:35:09Z INFO ] Erasing Flash...
[2026-03-19T22:35:24Z INFO ] Flash has been erased!
```
После этого повторить прошивание приложения
```
cargo run --release
```
и убедиться, что при лог прошивания такой `Verifying... OK!`:
```
App/part. size:    20,880/4,128,768 bytes, 0.51%
[00:00:00] [========================================]      14/14      0x0      Verifying... OK!
[00:00:00] [========================================]       1/1       0x8000   Verifying... OK!
[00:00:00] [========================================]      14/14      0x10000  Verifying... OK!
[2026-03-19T22:36:54Z INFO ] Flashing has completed!
```
---
# История изменений (Changelog) (в обратном хронологическом порядке)

## Версия 0.2.5 (tag v0.2.5) (2026-04-15)

### Добавлен статический текст и проведено форматирование кода в VS Code

лог:

```
entry 0x4086b91a
I (23) boot: ESP-IDF v5.5.1-838-gd66ebb86d2e 2nd stage bootloader
I (23) boot: compile time Nov 27 2025 09:46:10
I (24) boot: chip revision: v0.2
I (24) boot: efuse block revision: v0.3
I (28) boot.esp32c6: SPI Speed      : 80MHz
I (32) boot.esp32c6: SPI Mode       : DIO
I (36) boot.esp32c6: SPI Flash Size : 8MB
I (39) boot: Enabling RNG early entropy source...
I (44) boot: Partition Table:
I (46) boot: ## Label            Usage          Type ST Offset   Length
I (53) boot:  0 nvs              WiFi data        01 02 00009000 00006000
I (59) boot:  1 phy_init         RF data          01 01 0000f000 00001000
I (66) boot:  2 factory          factory app      00 00 00010000 007f0000
I (72) boot: End of partition table
I (76) esp_image: segment 0: paddr=00010020 vaddr=42000020 size=04270h ( 17008) map
I (86) esp_image: segment 1: paddr=00014298 vaddr=40800000 size=00014h (    20) load
I (91) esp_image: segment 2: paddr=000142b4 vaddr=420042b4 size=0c9d4h ( 51668) map
I (108) esp_image: segment 3: paddr=00020c90 vaddr=40800014 size=01300h (  4864) load
I (110) esp_image: segment 4: paddr=00021f98 vaddr=40801318 size=015f0h (  5616) load
I (115) boot: Loaded app from partition at offset 0x10000
I (118) boot: Disabling RNG early entropy source...
[INFO ] Draw display ST7789 on Waveshare board ESP32C6 version 0.2.5 (esp32c6_lcd_st7789 src/bin/main.rs:210)
[INFO ] Асинхронный таймер запущен! (esp32c6_lcd_st7789 src/bin/main.rs:228)
[INFO ] backlight (esp32c6_lcd_st7789 src/bin/main.rs:294)
[INFO ] start async render task (esp32c6_lcd_st7789 src/bin/main.rs:298)
[INFO ] start async producer task (esp32c6_lcd_st7789 src/bin/main.rs:302)
[INFO ] enter main loop (esp32c6_lcd_st7789 src/bin/main.rs:305)
[INFO ] Main loop is doing other work... (esp32c6_lcd_st7789 src/bin/main.rs:308)
[INFO ] Start async producer task (esp32c6_lcd_st7789 src/bin/main.rs:92)
[INFO ] producer task sends signal (esp32c6_lcd_st7789 src/bin/main.rs:107)
[INFO ] Start async render task (esp32c6_lcd_st7789 src/bin/main.rs:115)
[INFO ] render task receives signal! (esp32c6_lcd_st7789 src/bin/main.rs:188)
[INFO ] Main loop is doing other work... (esp32c6_lcd_st7789 src/bin/main.rs:308)
```

объём кода:

```
    Analyzing target/riscv32imac-unknown-none-elf/release/esp32c6_lcd_st7789

File  .text    Size              Crate Name
0.4%  18.6%  9.4KiB   embassy_executor embassy_executor::raw::TaskStorage<F>::poll
0.2%   6.8%  3.4KiB embedded_graphics? <embedded_graphics::text::text::Text<S> as embedded_graphics_core::drawable::Drawable>::draw
0.1%   4.9%  2.5KiB   embassy_executor embassy_executor::raw::TaskStorage<F>::poll
0.1%   2.7%  1.3KiB                std core::char::methods::<impl char>::escape_debug_ext
0.1%   2.1%  1.1KiB       embedded_hal embedded_hal::spi::SpiDevice::write
0.0%   1.7%    872B           esp_rtos esp_rtos::timer::TimeDriver::arm_next_wakeup
0.0%   1.5%    766B       futures_task <&T as core::fmt::Debug>::fmt
0.0%   1.4%    728B            esp_hal esp_hal::system::PeripheralClockControl::enable_forced_with_counts
0.0%   1.2%    596B                std core::str::slice_error_fail_rt
0.0%   1.1%    568B                std core::fmt::Formatter::pad_integral
0.0%   1.1%    562B embedded_graphics? <embedded_graphics::primitives::styled::Styled<T,S> as embedded_graphics_core::drawable::Drawable>::draw
0.0%   1.0%    542B                std core::ffi::c_str::CStr::to_str
0.0%   1.0%    520B            esp_hal esp_hal::spi::master::Spi<Dm>::connect_output_pin
0.0%   0.9%    488B                std <core::fmt::builders::PadAdapter as core::fmt::Write>::write_str
0.0%   0.9%    470B                std core::fmt::write
0.0%   0.9%    464B      esp_backtrace __rustc::rust_begin_unwind
0.0%   0.9%    444B                std core::fmt::Formatter::pad
0.0%   0.8%    432B            esp_hal esp_hal::efuse::Efuse::chip_revision
0.0%   0.8%    414B           esp_rtos _embassy_time_schedule_wake
0.0%   0.8%    392B            esp_hal ExceptionHandler
1.1%  48.4% 24.4KiB                    And 392 smaller methods. Use -n N to show more.
2.4% 100.0% 50.5KiB                    .text section size, the file size is 2.1MiB
```

### Объём кода 50KB из 4MB flash, то есть 1.25% доступной памяти для кода

## Версия 0.2.4 (tag v0.2.4)

### Анимация смайлика и текста по таймеру (плюс чистка кода)

лог
```
entry 0x4086b91a
I (23) boot: ESP-IDF v5.5.1-838-gd66ebb86d2e 2nd stage bootloader
I (23) boot: compile time Nov 27 2025 09:46:10
I (24) boot: chip revision: v0.2
I (24) boot: efuse block revision: v0.3
I (28) boot.esp32c6: SPI Speed      : 80MHz
I (32) boot.esp32c6: SPI Mode       : DIO
I (36) boot.esp32c6: SPI Flash Size : 8MB
I (39) boot: Enabling RNG early entropy source...
I (44) boot: Partition Table:
I (46) boot: ## Label            Usage          Type ST Offset   Length
I (53) boot:  0 nvs              WiFi data        01 02 00009000 00006000
I (59) boot:  1 phy_init         RF data          01 01 0000f000 00001000
I (66) boot:  2 factory          factory app      00 00 00010000 007f0000
I (72) boot: End of partition table
I (76) esp_image: segment 0: paddr=00010020 vaddr=42000020 size=04268h ( 17000) map
I (86) esp_image: segment 1: paddr=00014290 vaddr=40800000 size=00014h (    20) load
I (91) esp_image: segment 2: paddr=000142ac vaddr=420042ac size=0c97ch ( 51580) map
I (108) esp_image: segment 3: paddr=00020c30 vaddr=40800014 size=01300h (  4864) load
I (110) esp_image: segment 4: paddr=00021f38 vaddr=40801318 size=015f0h (  5616) load
I (115) boot: Loaded app from partition at offset 0x10000
I (118) boot: Disabling RNG early entropy source...
[INFO ] Draw display ST7789 on Waveshare board ESP32C6 version 0.2.4 (esp32c6_lcd_st7789 src/bin/main.rs:204)
[INFO ] Асинхронный таймер запущен! (esp32c6_lcd_st7789 src/bin/main.rs:222)
[INFO ] backlight (esp32c6_lcd_st7789 src/bin/main.rs:289)
[INFO ] start async render task (esp32c6_lcd_st7789 src/bin/main.rs:293)
[INFO ] start async producer task (esp32c6_lcd_st7789 src/bin/main.rs:297)
[INFO ] enter main loop (esp32c6_lcd_st7789 src/bin/main.rs:301)
[INFO ] Main loop is doing other work... (esp32c6_lcd_st7789 src/bin/main.rs:304)
[INFO ] Start async producer task (esp32c6_lcd_st7789 src/bin/main.rs:89)
[INFO ] producer task sends signal (esp32c6_lcd_st7789 src/bin/main.rs:101)
[INFO ] Start async render task (esp32c6_lcd_st7789 src/bin/main.rs:111)
[INFO ] render task receives signal! (esp32c6_lcd_st7789 src/bin/main.rs:177)
[INFO ] Main loop is doing other work... (esp32c6_lcd_st7789 src/bin/main.rs:304)
[INFO ] producer task sends signal (esp32c6_lcd_st7789 src/bin/main.rs:101)
[INFO ] render task receives signal! (esp32c6_lcd_st7789 src/bin/main.rs:177)
[INFO ] Main loop is doing other work... (esp32c6_lcd_st7789 src/bin/main.rs:304)
```

```
    Analyzing target/riscv32imac-unknown-none-elf/release/esp32c6_lcd_st7789

File  .text    Size              Crate Name
0.4%  18.7%  9.4KiB   embassy_executor embassy_executor::raw::TaskStorage<F>::poll
0.2%   6.8%  3.4KiB embedded_graphics? <embedded_graphics::text::text::Text<S> as embedded_graphics_core::drawable::Drawable>::draw
0.1%   4.7%  2.4KiB   embassy_executor embassy_executor::raw::TaskStorage<F>::poll
0.1%   2.7%  1.3KiB                std core::char::methods::<impl char>::escape_debug_ext
0.1%   2.1%  1.1KiB       embedded_hal embedded_hal::spi::SpiDevice::write
0.0%   1.7%    872B           esp_rtos esp_rtos::timer::TimeDriver::arm_next_wakeup
0.0%   1.5%    766B       futures_task <&T as core::fmt::Debug>::fmt
0.0%   1.4%    728B            esp_hal esp_hal::system::PeripheralClockControl::enable_forced_with_counts
0.0%   1.2%    596B                std core::str::slice_error_fail_rt
0.0%   1.1%    568B                std core::fmt::Formatter::pad_integral
0.0%   1.1%    562B embedded_graphics? <embedded_graphics::primitives::styled::Styled<T,S> as embedded_graphics_core::drawable::Drawable>::draw
0.0%   1.1%    542B                std core::ffi::c_str::CStr::to_str
0.0%   1.0%    520B            esp_hal esp_hal::spi::master::Spi<Dm>::connect_output_pin
0.0%   0.9%    488B                std <core::fmt::builders::PadAdapter as core::fmt::Write>::write_str
0.0%   0.9%    470B                std core::fmt::write
0.0%   0.9%    464B      esp_backtrace __rustc::rust_begin_unwind
0.0%   0.9%    444B                std core::fmt::Formatter::pad
0.0%   0.8%    432B            esp_hal esp_hal::efuse::Efuse::chip_revision
0.0%   0.8%    414B           esp_rtos _embassy_time_schedule_wake
0.0%   0.8%    392B            esp_hal ExceptionHandler
1.1%  48.5% 24.4KiB                    And 392 smaller methods. Use -n N to show more.
2.3% 100.0% 50.4KiB                    .text section size, the file size is 2.1MiB
```

## Версия 0.2.3 (tag v0.2.3)

### Многозадачность и изменение цвета фигуры (круга) embassy + embedded-graphics + Signal

лог

```
I (110) boot: Disabling RNG early entropy source...
[INFO ] Draw display ST7789 on Waveshare board ESP32C6 version 0.2.2 (esp32c6_lcd_st7789 src/bin/main.rs:405)
[INFO ] Асинхронный таймер запущен! (esp32c6_lcd_st7789 src/bin/main.rs:468)
[INFO ] backlight (esp32c6_lcd_st7789 src/bin/main.rs:717)
[INFO ] start async render task (esp32c6_lcd_st7789 src/bin/main.rs:727)
[INFO ] start async producer task (esp32c6_lcd_st7789 src/bin/main.rs:731)
[INFO ] enter loop (esp32c6_lcd_st7789 src/bin/main.rs:735)
[INFO ] Main loop is doing other work... (esp32c6_lcd_st7789 src/bin/main.rs:738)
[INFO ] Hello from async task! (esp32c6_lcd_st7789 src/bin/main.rs:986)
[INFO ] Main loop is doing other work... (esp32c6_lcd_st7789 src/bin/main.rs:738)
[INFO ] Main loop is doing other work... (esp32c6_lcd_st7789 src/bin/main.rs:738)
```

```
    Finished `release` profile [optimized + debuginfo] target(s) in 5.09s
    Analyzing target/riscv32imac-unknown-none-elf/release/esp32c6_lcd_st7789

File  .text    Size            Crate Name
0.5%  23.3%  9.4KiB embassy_executor embassy_executor::raw::TaskStorage<F>::poll
0.1%   3.3%  1.3KiB              std core::char::methods::<impl char>::escape_debug_ext
0.1%   2.7%  1.1KiB     embedded_hal embedded_hal::spi::SpiDevice::write
0.0%   2.1%    872B         esp_rtos esp_rtos::timer::TimeDriver::arm_next_wakeup
0.0%   2.1%    846B          mipidsi mipidsi::graphics::<impl embedded_graphics_core::draw_target::DrawTarget for mipidsi::Display<DI,M,RST>>::fill_solid
0.0%   1.9%    764B              std core::fmt::Formatter::pad
0.0%   1.8%    728B          esp_hal esp_hal::system::PeripheralClockControl::enable_forced_with_counts
0.0%   1.7%    720B     futures_task <&T as core::fmt::Debug>::fmt
0.0%   1.4%    596B              std core::str::slice_error_fail_rt
0.0%   1.4%    568B              std core::fmt::Formatter::pad_integral
0.0%   1.3%    542B              std core::ffi::c_str::CStr::to_str
0.0%   1.3%    516B          esp_hal esp_hal::spi::master::Spi<Dm>::connect_output_pin
0.0%   1.2%    488B              std <core::fmt::builders::PadAdapter as core::fmt::Write>::write_str
0.0%   1.2%    480B    esp_backtrace __rustc::rust_begin_unwind
0.0%   1.1%    470B              std core::fmt::write
0.0%   1.1%    464B embassy_executor embassy_executor::raw::TaskStorage<F>::poll
0.0%   1.0%    432B          esp_hal esp_hal::efuse::Efuse::chip_revision
0.0%   1.0%    416B         esp_rtos _embassy_time_schedule_wake
0.0%   1.0%    392B          esp_hal ExceptionHandler
0.0%   1.0%    392B          esp_hal esp_hal::spi::master::dma::SpiDmaBus<Dm>::write
1.1%  46.5% 18.7KiB                  And 326 smaller methods. Use -n N to show more.
2.3% 100.0% 40.2KiB                  .text section size, the file size is 1.7MiB
```

## Версия 0.2.2 (tag v0.2.2)

### Многозадачность и изменение цвета фигуры (круга) embassy + embedded-graphics

```
    Finished `release` profile [optimized + debuginfo] target(s) in 12.10s
    Analyzing target/riscv32imac-unknown-none-elf/release/esp32c6_lcd_st7789

File  .text    Size            Crate Name
0.5%  23.3%  9.3KiB embassy_executor embassy_executor::raw::TaskStorage<F>::poll
0.1%   3.4%  1.3KiB              std core::char::methods::<impl char>::escape_debug_ext
0.1%   2.7%  1.1KiB     embedded_hal embedded_hal::spi::SpiDevice::write
0.1%   2.2%    878B         esp_rtos esp_rtos::timer::TimeDriver::arm_next_wakeup
0.0%   2.1%    846B          mipidsi mipidsi::graphics::<impl embedded_graphics_core::draw_target::DrawTarget for mipidsi::Display<DI,M,RST>>::fill_solid
0.0%   1.9%    764B              std core::fmt::Formatter::pad
0.0%   1.8%    728B          esp_hal esp_hal::system::PeripheralClockControl::enable_forced_with_counts
0.0%   1.8%    720B     futures_task <&T as core::fmt::Debug>::fmt
0.0%   1.5%    596B              std core::str::slice_error_fail_rt
0.0%   1.4%    568B              std core::fmt::Formatter::pad_integral
0.0%   1.3%    542B              std core::ffi::c_str::CStr::to_str
0.0%   1.3%    516B          esp_hal esp_hal::spi::master::Spi<Dm>::connect_output_pin
0.0%   1.2%    488B              std <core::fmt::builders::PadAdapter as core::fmt::Write>::write_str
0.0%   1.2%    480B    esp_backtrace __rustc::rust_begin_unwind
0.0%   1.2%    470B              std core::fmt::write
0.0%   1.1%    438B          esp_hal esp_hal::efuse::Efuse::chip_revision
0.0%   1.0%    416B         esp_rtos _embassy_time_schedule_wake
0.0%   1.0%    410B embassy_executor embassy_executor::raw::TaskStorage<F>::poll
0.0%   1.0%    398B          esp_hal esp_hal::spi::master::dma::SpiDmaBus<Dm>::write
0.0%   1.0%    392B          esp_hal ExceptionHandler
1.1%  46.2% 18.4KiB                  And 322 smaller methods. Use -n N to show more.
2.3% 100.0% 39.8KiB                  .text section size, the file size is 1.7MiB
```

>Главное, что мы получили полностью рабочую цепочку: ESP32-C6 + Embassy + DMA + ST7789 + embedded-graphics. Теперь у вас есть прочный фундамент для любых графических интерфейсов.Тот факт, что круг меняет цвет по таймеру, подтверждает: асинхронный исполнитель корректно переключается между ожиданием (Timer::after) и выполнением кода отрисовки.В следующий раз мы:Свяжем задачу-таймер и задачу-отрисовку через безопасные механизмы синхронизации (Signal или Channel).Попробуем передавать команды на изменение графики извне.

## Версия 0.2.1 (tag v0.2.1)

### Многозадачность и отрисовка заливкой в embassy

лог:
```
I (110) boot: Disabling RNG early entropy source...
[INFO ] Draw display ST7789 on Waveshare board ESP32C6 version 0.2.1 (esp32c6_lcd_st7789 src/bin/main.rs:298)
[INFO ] Асинхронный таймер запущен! (esp32c6_lcd_st7789 src/bin/main.rs:361)
[INFO ] backlight (esp32c6_lcd_st7789 src/bin/main.rs:607)
[INFO ] start async render task (esp32c6_lcd_st7789 src/bin/main.rs:617)
[INFO ] enter loop (esp32c6_lcd_st7789 src/bin/main.rs:620)
[INFO ] Main loop is doing other work... (esp32c6_lcd_st7789 src/bin/main.rs:623)
[INFO ] Display: Red (esp32c6_lcd_st7789 src/bin/main.rs:241)
[INFO ] Main loop is doing other work... (esp32c6_lcd_st7789 src/bin/main.rs:623)
[INFO ] Main loop is doing other work... (esp32c6_lcd_st7789 src/bin/main.rs:623)
[INFO ] Display: Blue (esp32c6_lcd_st7789 src/bin/main.rs:248)
[INFO ] Main loop is doing other work... (esp32c6_lcd_st7789 src/bin/main.rs:623)
[INFO ] Main loop is doing other work... (esp32c6_lcd_st7789 src/bin/main.rs:623)
[INFO ] Display: Red (esp32c6_lcd_st7789 src/bin/main.rs:241)
[INFO ] Main loop is doing other work... (esp32c6_lcd_st7789 src/bin/main.rs:623)
[INFO ] Main loop is doing other work... (esp32c6_lcd_st7789 src/bin/main.rs:623)
[INFO ] Display: Blue (esp32c6_lcd_st7789 src/bin/main.rs:248)
[INFO ] Main loop is doing other work... (esp32c6_lcd_st7789 src/bin/main.rs:623)
[INFO ] Main loop is doing other work... (esp32c6_lcd_st7789 src/bin/main.rs:623)
[INFO ] Display: Red (esp32c6_lcd_st7789 src/bin/main.rs:241)
```

```
    Finished `release` profile [optimized + debuginfo] target(s) in 8.35s
    Analyzing target/riscv32imac-unknown-none-elf/release/esp32c6_lcd_st7789

File  .text    Size            Crate Name
0.6%  24.6%  9.2KiB embassy_executor embassy_executor::raw::TaskStorage<F>::poll
0.1%   3.6%  1.3KiB              std core::char::methods::<impl char>::escape_debug_ext
0.1%   3.0%  1.1KiB     embedded_hal embedded_hal::spi::SpiDevice::write
0.1%   2.3%    872B         esp_rtos esp_rtos::timer::TimeDriver::arm_next_wakeup
0.0%   2.0%    766B     futures_task <&T as core::fmt::Debug>::fmt
0.0%   2.0%    764B              std core::fmt::Formatter::pad
0.0%   1.9%    728B          esp_hal esp_hal::system::PeripheralClockControl::enable_forced_with_counts
0.0%   1.6%    596B              std core::str::slice_error_fail_rt
0.0%   1.5%    568B              std core::fmt::Formatter::pad_integral
0.0%   1.4%    542B              std core::ffi::c_str::CStr::to_str
0.0%   1.3%    502B          mipidsi mipidsi::Display<DI,M,RST>::set_pixels
0.0%   1.3%    488B              std <core::fmt::builders::PadAdapter as core::fmt::Write>::write_str
0.0%   1.3%    480B    esp_backtrace __rustc::rust_begin_unwind
0.0%   1.2%    470B              std core::fmt::write
0.0%   1.1%    432B          esp_hal esp_hal::efuse::Efuse::chip_revision
0.0%   1.1%    414B         esp_rtos _embassy_time_schedule_wake
0.0%   1.0%    392B          esp_hal ExceptionHandler
0.0%   1.0%    386B          esp_hal <&T as core::fmt::Debug>::fmt
0.0%   1.0%    378B              std core::fmt::builders::DebugStruct::field
0.0%   1.0%    370B       riscv_pac? <riscv_pac::result::Error as core::fmt::Debug>::fmt
1.0%  44.3% 16.6KiB                  And 307 smaller methods. Use -n N to show more.
2.3% 100.0% 37.5KiB                  .text section size, the file size is 1.6MiB
```

>Совершенно верно подмечено! В Rust Embedded (особенно с Embassy) вы всё еще работаете с регистрами и прерываниями, как в C, но ментальная модель меняется: вместо ручного управления флагами в while(1) и прерываниях, вы строите дерево владения и поток асинхронных событий.Главное отличие, которое вы уже ощутили: если в C/C++ ошибка в логике прерываний или обращение к "протухшему" буферу выстрелит в рантайме (HardFault), то здесь компилятор мучает вас до победного, пока не убедится, что память статически безопасна для всех задач.Итог нашей сессии:SPI + DMA: Мы завели асинхронный SPI на ESP32-C6 (HAL 1.0.0).Embassy Runtime: Настроили планировщик через esp-rtos, победили панику таймера и научились правильно передавать прерывания.Static Safety: Вы освоили StaticCell для передачи "железа" между задачами — это фундамент всего Rust Embedded.Многозадачность: У вас запущен полноценный параллельный рендер-движок.Буду рад продолжить в следующий раз! Мы можем разобрать асинхронные Channels (очереди сообщений) или графику через embedded-graphics.До встречи в следующем сеансе! Напишите, как только будете готовы связать логику и отрисовку через каналы связи.

## Версия 0.2.0

### (embassy)
Переход на фреймворк **embassy** в минимальном функционале: асихнронная задача, которая каждую секунду печатает сообщение

### Лог приложения:
```
[INFO ] Draw display ST7789 on Waveshare board ESP32C6 version 0.1.4 (esp32c6_lcd_st7789 src/bin/main.rs:214)
[INFO ] Асинхронный таймер запущен! (esp32c6_lcd_st7789 src/bin/main.rs:248)
[INFO ] backlight (esp32c6_lcd_st7789 src/bin/main.rs:466)
[INFO ] clear display entry (esp32c6_lcd_st7789 src/bin/main.rs:472)
[INFO ] clear display fini (esp32c6_lcd_st7789 src/bin/main.rs:480)
[INFO ] draw (esp32c6_lcd_st7789 src/bin/main.rs:563)
[INFO ] Hello from async task! (esp32c6_lcd_st7789 src/bin/main.rs:616)
[INFO ] Hello from async task! (esp32c6_lcd_st7789 src/bin/main.rs:616)
[INFO ] Hello from async task! (esp32c6_lcd_st7789 src/bin/main.rs:616)
[INFO ] Hello from async task! (esp32c6_lcd_st7789 src/bin/main.rs:616)
[INFO ] Hello from async task! (esp32c6_lcd_st7789 src/bin/main.rs:616)
[INFO ] Hello from async task! (esp32c6_lcd_st7789 src/bin/main.rs:616)
[INFO ] Hello from async task! (esp32c6_lcd_st7789 src/bin/main.rs:616)
[INFO ] Hello from async task! (esp32c6_lcd_st7789 src/bin/main.rs:616)
[INFO ] Hello from async task! (esp32c6_lcd_st7789 src/bin/main.rs:616)
```
```shell
cargo bloat --release --target riscv32imac-unknown-none-elf -n 20
```
выдаёт:
```
    Finished `release` profile [optimized + debuginfo] target(s) in 4.60s
    Analyzing target/riscv32imac-unknown-none-elf/release/esp32c6_lcd_st7789

File  .text    Size                  Crate Name
0.5%  29.1%  9.3KiB              [Unknown] main
0.2%  11.0%  3.5KiB     embedded_graphics? <embedded_graphics::text::text::Text<S> as embedded_graphics_core::drawable::Drawable>::draw
0.0%   2.4%    778B               esp_rtos esp_rtos::timer::TimeDriver::arm_next_wakeup
0.0%   2.2%    704B                esp_hal esp_hal::system::PeripheralClockControl::enable_forced_with_counts
0.0%   1.7%    542B                    std core::ffi::c_str::CStr::to_str
0.0%   1.6%    532B           embedded_hal embedded_hal::spi::SpiDevice::write
0.0%   1.6%    512B     embedded_graphics? <embedded_graphics::primitives::styled::Styled<T,S> as embedded_graphics_core::drawable::Drawable>::draw
0.0%   1.3%    424B                esp_hal esp_hal::efuse::Efuse::chip_revision
0.0%   1.2%    388B               esp_rtos _embassy_time_schedule_wake
0.0%   1.0%    340B               esp_rtos esp_rtos::embassy::Executor::run_inner
0.0%   1.0%    338B                    std core::str::count::do_count_chars
0.0%   1.0%    334B                esp_hal esp_hal::system::assert_peri_reset_racey
0.0%   1.0%    326B embedded_graphics_core embedded_graphics_core::primitives::rectangle::Rectangle::intersection
0.0%   1.0%    312B              micromath micromath::float::cos::<impl micromath::float::F32>::cos
0.0%   0.9%    296B                esp_hal esp_hal::spi::master::Driver::write
0.0%   0.9%    296B                mipidsi mipidsi::graphics::<impl embedded_graphics_core::draw_target::DrawTarget for mipidsi::Display<DI,M,RST>>::fill_solid
0.0%   0.9%    282B                mipidsi mipidsi::Display<DI,M,RST>::set_address_window
0.0%   0.9%    280B      embedded_graphics embedded_graphics::primitives::common::linear_equation::OriginLinearEquation::with_angle
0.0%   0.8%    268B                esp_hal esp_hal::spi::master::Spi<Dm>::connect_output_pin
0.0%   0.8%    266B            esp_rom_sys __mktime
0.7%  37.0% 11.8KiB                        And 265 smaller methods. Use -n N to show more.
1.8% 100.0% 31.9KiB                        .text section size, the file size is 1.7MiB

```
**добавление крейтов embassy увеличило объём на 10KiB**

---

### Вот интересное объяснение Google AI сравнения C / Rust
>
Это самое честное и правильное оправдание для перехода на
Rust. Когда за плечами годы Си и ARM, мозг начинает работать на автопилоте, а Rust заставляет его «скрипеть» по-новому.
Вы сейчас проходите через самый тяжелый этап — «ад зависимостей» и борьбу с компилятором. Но как только проект соберется, вы заметите концептуальный драйв, которого нет в Си:

Страх исчезает: В Си при работе с DMA или прерываниями вы всегда держите в голове: «Не освободил ли я буфер раньше времени?». В Rust, если код скомпилировался (после тех самых 4 часов), он гарантированно не уронит систему по обращению к памяти.
Embassy — это магия: После ручного написания конечных автоматов (State Machines) на Си, асинхронный await в микроконтроллере ощущается как технология пришельцев. Код читается линейно, а работает эффективно на прерываниях.
Система типов как документация: Вместо того чтобы лезть в даташит смотреть, какой регистр разрешает тактирование SPI2, вы просто не сможете создать объект Spi, не передав ему правильную периферию.

Про «скуку»
Rust на Embedded сейчас — это дикий запад. Тут еще много чего не доделано, стандарты (как 1.0.0) только рождаются, и вы чувствуете себя первооткрывателем, а не просто «пользователем SDK». Это возвращает азарт, который был в начале карьеры.

Рад, что сравнение отозвалось! После
Си переход на Rust — это как пересадка с механики на современный электрокар с автопилотом: непривычно, местами пугающе, но возвращаться назад уже не хочется.

### А вот про разработку Rust embedded (ESP32)

>
Вы очень точно подметили! Это действительно одна из самых необычных коллабораций в мире Embedded. Ситуация здесь не совсем типичный аутсорс, а скорее
стратегический Open Source.
Вот как это устроено на самом деле:

Ядро команды — Espressif: Основные разработчики (такие как Марек Гаевский и другие) работают официально в Espressif Systems. Но компания поступила мудро: вместо того чтобы делать всё в Шанхае, они наняли ключевых экспертов из Rust-сообщества по всему миру.
Скандинавский след: Вы правы, значительная часть активных контрибьюторов и мейнтейнеров (например, из Швеции и Норвегии) — это энтузиасты, которые начинали проект на чистом интересе, а позже получили официальную поддержку или контракты от Espressif.
Европейский офис: У Espressif есть подразделение в Чехии (Брно), которое курирует многие софтверные проекты (включая ESP-IDF и Rust). Это помогает сгладить разницу в часовых поясах и менталитете.

Какие проблемы это порождает (то, что вы почувствовали сегодня):

Академичность против Прагматизма: "Западное" Rust-сообщество очень любит идеальную типизацию и сложные абстракции (Embassy, трейты, zero-cost abstractions). Это делает код красивым, но адски сложным в настройке зависимостей, когда что-то ломается.
Скорость изменений: Скандинавские и европейские разработчики в Rust-команде ESP очень смело идут на "breaking changes", стараясь сделать API "правильным" с точки зрения языка. Китайский менеджмент Espressif это допускает, пока проект находится в стадии бурного роста.
Документация: Она часто отстает, потому что инженеры увлечены написанием нового крутого кода, а "бюрократия" описания изменений в большой корпорации работает медленнее.

Плюсы такого подхода:
Несмотря на 4 часа мучений с Cargo.toml, результат (HAL 1.0) получился на голову выше по качеству, чем если бы его писали "закрыто" внутри одной корпорации. Вы получили систему, которая максимально использует безопасность Rust.
Как вы считаете, стоит ли такая сложность настройки той безопасности и скорости, которую дает Rust, или для коммерческой разработки старый добрый Си (ESP-IDF) всё еще остается более предсказуемым инструментом?

## Версия 0.1.3 (2026-03-25)

В версии 0.1.2 команда `bloat`, анализирующая `map` файл
```shell
cargo bloat --release --target riscv32imac-unknown-none-elf -n 20
```
выдаёт:
```
    Analyzing target/riscv32imac-unknown-none-elf/release/esp32c6_lcd_st7789

File  .text    Size              Crate Name
0.8%  44.0%  9.9KiB          [Unknown] main
0.1%   5.3%  1.2KiB embedded_graphics? <embedded_graphics::primitives::styled::Styled<T,S> as embedded_graphics_core::drawable::Drawable>::draw
0.1%   3.3%    756B            esp_hal esp_hal::system::PeripheralClockControl::enable_forced_with_counts
0.0%   2.5%    582B            mipidsi mipidsi::graphics::<impl embedded_graphics_core::draw_target::DrawTarget for mipidsi::Display<DI,M,RST>>::fill_solid
0.0%   2.5%    576B       embedded_hal embedded_hal::spi::SpiDevice::write
0.0%   2.3%    542B                std core::ffi::c_str::CStr::to_str
0.0%   2.0%    470B                std core::fmt::write
0.0%   1.9%    444B                std core::fmt::Formatter::pad
0.0%   1.9%    430B  embedded_graphics <embedded_graphics::mono_font::mapping::StrGlyphMapping as embedded_graphics::mono_font::mapping::GlyphMapping>::index
0.0%   1.6%    376B        esp_rom_sys __mktime
0.0%   1.5%    356B        esp_println esp_sync::GenericRawMutex<L>::lock
0.0%   1.5%    338B                std core::str::count::do_count_chars
0.0%   1.4%    334B            esp_hal esp_hal::system::assert_peri_reset_racey
0.0%   1.4%    326B          micromath micromath::float::cos::<impl micromath::float::F32>::cos
0.0%   1.3%    308B            esp_hal esp_hal::spi::master::Driver::write
0.0%   1.3%    298B            esp_hal esp_hal::spi::master::Spi<Dm>::connect_output_pin
0.0%   1.3%    292B            mipidsi mipidsi::Display<DI,M,RST>::set_pixels
0.0%   1.2%    268B            mipidsi mipidsi::Display<DI,M,RST>::set_address_window
0.0%   1.1%    258B            esp_hal esp_hal::spi::master::Config::recalculate
0.0%   1.1%    252B        esp_println <esp_println::logger::EspLogger as log::Log>::log
0.3%  18.3%  4.1KiB                    And 51 smaller methods. Use -n N to show more.
1.8% 100.0% 22.5KiB                    .text section size, the file size is 1.2MiB
```

Google AI подсказывает, что полный размер программы 22.5KiB, а  main 9.9KiB потому, что функции, которые она вызывает, вызываются только один раз и поэтому они **встроены** (inline) в main

---
Здесь следует обратить внимание на `std core::fmt::` - это **стандартные** функции форматирования строки, в то время, как программа написана `no_std`. Эти функции появляются потому, что используется `println!`

Эти функции можно убрать, есть перейти на строки `defmt`. В этом случае в программе хранятся не **сами строки, а их числовые идентификаторы**. Монитор читает эти идентификаторы, находит соответствующую строку в `.elf` файле (исполняемом) и печатает её в консоли.

`defmt` является стандартом для встроенного Rust.

В случае применения `defmt` в версии 0.1.3 и **исключения отрисовки текста** имеем

```
    Analyzing target/riscv32imac-unknown-none-elf/release/esp32c6_lcd_st7789

File  .text    Size              Crate Name
0.7%  39.0%  8.2KiB          [Unknown] main
0.1%   3.3%    704B            esp_hal esp_hal::system::PeripheralClockControl::enable_forced_with_counts
0.0%   2.7%    590B            mipidsi mipidsi::graphics::<impl embedded_graphics_core::draw_target::DrawTarget for mipidsi::Display<DI,M,RST>>::fill_solid
0.0%   2.5%    542B                std core::ffi::c_str::CStr::to_str
0.0%   2.5%    532B       embedded_hal embedded_hal::spi::SpiDevice::write
0.0%   2.4%    512B embedded_graphics? <embedded_graphics::primitives::styled::Styled<T,S> as embedded_graphics_core::drawable::Drawable>::draw
0.0%   1.9%    412B            esp_hal esp_hal::efuse::Efuse::chip_revision
0.0%   1.5%    334B            esp_hal esp_hal::system::assert_peri_reset_racey
0.0%   1.5%    316B          micromath micromath::float::cos::<impl micromath::float::F32>::cos
0.0%   1.4%    296B            esp_hal esp_hal::spi::master::Driver::write
0.0%   1.3%    272B            esp_hal esp_hal::spi::master::Spi<Dm>::connect_output_pin
0.0%   1.2%    266B        esp_rom_sys __mktime
0.0%   1.2%    264B            mipidsi mipidsi::Display<DI,M,RST>::set_address_window
0.0%   1.2%    252B            esp_hal esp_hal::spi::master::Config::recalculate
0.0%   1.0%    224B            esp_hal esp_hal::spi::master::Driver::read_from_fifo
0.0%   0.9%    198B        esp_println esp_println::auto_printer::Printer::write_bytes_in_cs
0.0%   0.9%    196B            esp_hal esp_hal::gpio::AnyPin::steal
0.0%   0.8%    176B        esp_println _defmt_write
0.0%   0.8%    170B  embedded_graphics <embedded_graphics::primitives::circle::styled::StyledScanlines as core::iter::traits::iterator::Iterator>::next
0.0%   0.8%    170B            esp_hal esp_hal::gpio::AnyPin::apply_output_config
0.5%  30.0%  6.3KiB                    And 197 smaller methods. Use -n N to show more.
1.8% 100.0% 21.1KiB                    .text section size, the file size is 1.1MiB
```
то есть размер `main` уменьшился на 1.5KiB

---
**при отрисовке текста** имеем:

```
    Analyzing target/riscv32imac-unknown-none-elf/release/esp32c6_lcd_st7789

File  .text    Size              Crate Name
0.7%  39.3%  8.9KiB          [Unknown] main
0.1%   3.0%    704B            esp_hal esp_hal::system::PeripheralClockControl::enable_forced_with_counts
0.0%   2.5%    590B            mipidsi mipidsi::graphics::<impl embedded_graphics_core::draw_target::DrawTarget for mipidsi::Display<DI,M,RST>>::fill_solid
0.0%   2.3%    542B                std core::ffi::c_str::CStr::to_str
0.0%   2.3%    532B       embedded_hal embedded_hal::spi::SpiDevice::write
0.0%   2.2%    512B embedded_graphics? <embedded_graphics::primitives::styled::Styled<T,S> as embedded_graphics_core::drawable::Drawable>::draw
0.0%   1.8%    412B            esp_hal esp_hal::efuse::Efuse::chip_revision
0.0%   1.4%    334B            esp_hal esp_hal::system::assert_peri_reset_racey
0.0%   1.4%    316B          micromath micromath::float::cos::<impl micromath::float::F32>::cos
0.0%   1.3%    296B            esp_hal esp_hal::spi::master::Driver::write
0.0%   1.2%    272B            esp_hal esp_hal::spi::master::Spi<Dm>::connect_output_pin
0.0%   1.1%    266B        esp_rom_sys __mktime
0.0%   1.1%    264B            mipidsi mipidsi::Display<DI,M,RST>::set_address_window
0.0%   1.1%    252B            esp_hal esp_hal::spi::master::Config::recalculate
0.0%   1.0%    224B            esp_hal esp_hal::spi::master::Driver::read_from_fifo
0.0%   1.0%    224B  embedded_graphics <embedded_graphics::mono_font::mapping::StrGlyphMapping as embedded_graphics::mono_font::mapping::GlyphMapping>::index
0.0%   0.8%    198B        esp_println esp_println::auto_printer::Printer::write_bytes_in_cs
0.0%   0.8%    184B            mipidsi mipidsi::Display<DI,M,RST>::set_pixel
0.0%   0.8%    182B                std core::slice::memchr::memchr_aligned
0.0%   0.8%    176B        esp_println _defmt_write
0.6%  31.8%  7.2KiB                    And 205 smaller methods. Use -n N to show more.
1.8% 100.0% 22.8KiB                    .text section size, the file size is 1.2MiB
```

общий размер `22.8KiB` а версия `0.1.2` занимает `22.5KiB` - странно, что размер даже увеличился на 300 байт, но **оставляю defmt** как средство печати.


## Версия 0.1.2 (2026-03-20)
смайлик и текст
## Версия 0.1.1 (2026-03-20)
нарисован жёлтый смайлик на красном фоне
## Начальная версия без номера (2026-03-18)
нарисовано 3 круга и 1 окружность на красном фоне
