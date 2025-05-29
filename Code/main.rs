#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;
use avr_device::interrupt;
use core::sync::atomic::{AtomicBool, Ordering};
mod atmega_328p_ports;
use crate::atmega_328p_ports::*;
extern crate itoa; // Importer la crate itoa

use itoa::Buffer; // Importer le buffer de la crate itoa
const PB7: u8 = 7;
const PB6: u8 = 6;
const U2X0: u8 = 1;
const ADSC:u8 = 6 ;


static SEM: AtomicBool = AtomicBool::new(false);


#[avr_device::entry]
fn main() -> ! {
    config_timer();
    config_uart0();
    config_adc () ;
    unsafe {
        DDRB.write(0xFF); // Configurer PORTB comme sortie
          }

    loop {
        unsafe {
		
            // Attendre que l'UART soit prêt à envoyer la donnée et que SEM soit vrai
              if SEM.load(Ordering::SeqCst){
				          
                   let mut adc0: u16 = ((ADCH.read() as u16) << 8) | (ADCL.read() as u16);
                    PORTB.write(PORTB.read() ^ (1 << PB6));
                   //adc0 = (adc0 *5 )/1024;  // Conversion en v   //adc7 = (adc7 * 5000) >> 10; en millivolts(mV)
					trans_adc0(adc0) ;

                  SEM.store(false, Ordering::SeqCst); // Réinitialiser SEM après envoi
            }
        }

}}

fn config_timer() {
    unsafe {
		// Configurer le mode du timer
        TCCR1A.write(0); // Mode normal
        TCCR1B.write(0b00000011); 
        TIMSK1.write(1); // Activer l'interruption de débordement
        TCNT1.write(3035); // Définir la valeur initiale
        interrupt::enable(); // Activer les interruptions globales // SREG.write(SREG.read() | (1 << 7)); 
    }
}

// Configuration de l'UART
fn config_uart0() {
    unsafe {
        UCSR0B.write(0x18); // Activer TX et RX
        UCSR0C.write(0b00000110); // Format de trame : 8 bits de données, 1 bit d'arrêt
        UCSR0A.write(UCSR0A.read() | (1 << U2X0)); // Mode haute vitesse
        UBRR0.write(103); // Baudrate de 9600 bps
    }
}

fn config_adc() {
	
	unsafe {
        ADMUX.write(0b01000000); // Canal 0 et Vref = 5V
        ADCSRA.write(0b10001011);
		
		}	
}



fn trans_data(data_to_send: u8) {
    unsafe {
        // Attendre que le registre UDR0 soit prêt à recevoir de nouvelles données
        while (UCSR0A.read() & (1 << 5)) == 0 {}
        UDR0.write(data_to_send); // Envoyer les données
    }
}

// Fonction pour envoyer une chaîne de caractères par UART
fn trans_string(data: &str) {
    for &byte in data.as_bytes() {
    trans_data(byte); // Envoyer chaque caractère individuellement
    for _k in 0..10_000 {avr_device::asm::nop()};// short Delay nécessaire pour transmission efficace via uart 

    }
}
 fn trans_adc0(adc0: u16) {
                let mut buffer = Buffer::new(); // Créer un buffer pour convertir l'entier
                let adc0_str = buffer.format(adc0); // Convertir l'entier en chaîne
                trans_string("La valeur numerique est "); // Envoie la chaîne via UART
                //trans_string("La valeur anlogique en v est "); // Envoie la chaîne via UART
                trans_string(adc0_str); // Envoie la chaîne via UART
                trans_string("\r\n");

                }



// Gestionnaire d'interruption du timer
#[interrupt(atmega328p)]
fn TIMER1_OVF() {
	
    unsafe {
        PORTB.write(PORTB.read() ^ (1 << PB7));
        TCNT1.write(3035);           // Recharger la valeur initiale
        ADCSRA.write(ADCSRA.read() | (1 << ADSC)); // Démarrer la conversion     
          }
} 
#[interrupt(atmega328p)]
fn ADC() {	
  SEM.store(true, Ordering::SeqCst); // Activer l'envoi de données
  	     }
