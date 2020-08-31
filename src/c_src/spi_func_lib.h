#ifndef SPI_FUNC_LIB
#define SPI_FUNC_LIB
#include <stdint.h>


// setup

// use
uint8_t set_mode(const char *device, uint8_t encoded_mode);
uint8_t set_speed(const char *device, uint32_t speed);
uint8_t transfer_8_bit( const char *device,
                            uint8_t *tx, uint32_t tx_words, 
                            uint8_t *rx, 
                            uint32_t rx_words, 
                            uint16_t delay_us, uint32_t speed_hz, 
                            uint8_t bits
                            );
uint8_t set_bits_per_word(const char *device, uint8_t bits);



#endif