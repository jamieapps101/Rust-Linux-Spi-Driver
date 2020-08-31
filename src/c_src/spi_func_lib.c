#include <fcntl.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdint.h>
#include <sys/ioctl.h>
#include <linux/types.h>
#include <linux/spi/spidev.h>

#include "spi_func_lib.h"


// 1<<0 = SPI_LOOP
// 1<<1 = SPI_CPHA
// 1<<2 = SPI_CPOL
// 1<<3 = SPI_LSB_FIRST
// 1<<4 = SPI_CS_HIGH
// 1<<5 = SPI_3WIRE
// 1<<6 = SPI_NO_CS
// 1<<7 = SPI_READY

// encoded mode options, defined above
// this decouples values imported from header
// file from the values required to select them
uint8_t set_mode(const char *device, uint8_t encoded_mode) {
    uint8_t mode = 0;
    
    if(1<<0 & encoded_mode) 
        mode |= SPI_LOOP;

    if(1<<1 & encoded_mode) 
        mode |= SPI_CPHA;

    if(1<<2 & encoded_mode) 
        mode |= SPI_CPOL;

    if(1<<3 & encoded_mode) 
        mode |= SPI_LSB_FIRST;

    if(1<<4 & encoded_mode) 
        mode |= SPI_CS_HIGH;

    if(1<<5 & encoded_mode) 
        mode |= SPI_3WIRE;

    if(1<<6 & encoded_mode) 
        mode |= SPI_NO_CS;

    if(1<<7 & encoded_mode) 
        mode |= SPI_READY;


    int ioctl_ret = 0;
    uint8_t func_ret = 0;
    int fd = open(device, O_RDWR);
    ioctl_ret = ioctl(fd, SPI_IOC_WR_MODE, &mode);
	if (ioctl_ret == -1)
        func_ret |= 1; // cant et mode

	ioctl_ret = ioctl(fd, SPI_IOC_RD_MODE, &mode);
	if (ioctl_ret == -1)
        func_ret |= 1<<1; // cant get mode

    return func_ret;
}


uint8_t set_speed(const char *device, uint32_t speed) {
    int ioctl_ret = 0;
    uint8_t func_ret = 0;
    int fd = open(device, O_RDWR);
    ioctl_ret = ioctl(fd, SPI_IOC_WR_MAX_SPEED_HZ, &speed);
	if (ioctl_ret == -1)
        func_ret |= 1; // cant set max speed


	ioctl_ret = ioctl(fd, SPI_IOC_RD_MAX_SPEED_HZ, &speed);
	if (ioctl_ret == -1)
        func_ret |= 1<<1; // cant get max speed
    return func_ret;
}


uint8_t transfer_8_bit( const char *device,
                            uint8_t *tx, uint32_t tx_words, 
                            uint8_t *rx, 
                            // uint32_t *rx_words, 
                            uint16_t delay_us, uint32_t speed_hz, 
                            uint8_t bits
                            ) {
	int ret;
    uint8_t func_return = 0;
	// uint8_t *rx;
    rx = (uint8_t*)calloc(tx_words, sizeof(uint8_t));
	struct spi_ioc_transfer tr = {
		.tx_buf = (unsigned long)tx,
		.rx_buf = (unsigned long)rx,
		.len = tx_words,
		.delay_usecs = delay_us,
		.speed_hz = speed_hz,
		.bits_per_word = bits,
	};
    int fd = open(device, O_RDWR);
	ret = ioctl(fd, SPI_IOC_MESSAGE(1), &tr);
	if (ret < 1)
        func_return = 1;
		// pabort("can't send spi message");

	// for (ret = 0; ret < words; ret++) {
	// 	if (!(ret % 6))
	// 		puts("");
	// 	printf("%.2X ", rx[ret]);
	// }
	// puts("");
    return func_return;
}



