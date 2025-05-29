# 🦀 Embedded Rust: ADC + UART + Timer on ATmega328P

This project demonstrates **register-level programming** of the ATmega328P microcontroller using **Embedded Rust**. It integrates three key peripherals: **ADC**, **UART**, and **Timer1**, to periodically read an analog input and transmit the digital value over UART.

## 🎯 Project Objective

To develop a Rust-based embedded system on the ATmega328P that:

- Configures a 16-bit timer (Timer1) to overflow every **500 ms**
- Starts an **ADC** conversion on each overflow
- Transmits the digital result over **UART** at **9600 bps**
- Uses **interrupts** for both Timer and ADC for efficient execution

## 🧰 Tools & Hardware

- **Microcontroller**: ATmega328P (used in Arduino Uno)
- **Language**: Embedded Rust
- **Simulation**: Proteus (for oscilloscope + UART terminal)
- **Build tool**: `avr-gcc`, `cargo-avr`, `ravedude` (or similar)
- **Target**: `avr-atmega328p`

## 🛠️ System Components

### Timer1 (16-bit)
- Configured in **Normal Mode**
- Overflow every **500ms** using:
  - System clock: 8 MHz
  - Prescaler: 64 → tick time = 8 μs
  - Timer counter preload = `65535 - 62500 = 3035`

### ADC (10-bit)
- Channel: ADC0 (e.g., potentiometer)
- Reference: AVCC with capacitor at AREF
- Prescaler = 8
- Interrupt enabled on conversion complete

### UART
- Baud Rate: 9600 bps
- Frame: 8 data bits, 1 stop bit, no parity
- TX only (asynchronous)
- Blocking transmission for simplicity

---

## 🔁 Program Flow

```text
1. Timer1 Overflows every 500ms → Timer1 ISR triggered
2. Timer1 ISR:
   - Toggles PB7 LED
   - Starts ADC Conversion
3. ADC Conversion Complete → ADC ISR triggered
4. ADC ISR:
   - Sets SEM=true
5. Super Loop checks SEM:
   - Sends ADC result over UART
   - Toggles PB6 LED
