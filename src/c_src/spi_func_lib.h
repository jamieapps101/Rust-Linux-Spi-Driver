#ifndef SPI_FUNC_LIB
#define SPI_FUNC_LIB
#include <stdint.h>


uint8_t get_dev_fd(const char *device, int32_t *fd);
uint8_t set_mode_on_fd(int32_t fd, uint8_t encoded_mode);
uint8_t transfer_8_bit_on_fd(int32_t fd, 
                                uint8_t *tx, uint32_t tx_words, 
                                uint8_t *rx, 
                                uint32_t rx_words, 
                                uint16_t delay_us, uint32_t speed_hz, 
                                uint8_t bits
                            );
void close_dev_fd(int32_t fd);


#endif