#include <fcntl.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include <sys/ioctl.h>
#include <linux/types.h>
#include <linux/spi/spidev.h>

#define ARRAY_SIZE(a) (sizeof(a) / sizeof((a)[0]))

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
    
    if(1<<0 & encoded_mode) // 1
        mode |= SPI_LOOP;

    if(1<<1 & encoded_mode)  // 2
        mode |= SPI_CPHA;

    if(1<<2 & encoded_mode)  // 4
        mode |= SPI_CPOL;

    if(1<<3 & encoded_mode)  // 8
        mode |= SPI_LSB_FIRST;

    if(1<<4 & encoded_mode)  // 16
        mode |= SPI_CS_HIGH;

    if(1<<5 & encoded_mode)  // 32
        mode |= SPI_3WIRE;

    if(1<<6 & encoded_mode)  // 64
        mode |= SPI_NO_CS;

    if(1<<7 & encoded_mode)  // 128
        mode |= SPI_READY;


    int ioctl_ret = 0;
    uint8_t func_ret = 0;
    int fd = open(device, O_RDWR);
    ioctl_ret = ioctl(fd, SPI_IOC_WR_MODE, &mode);
	if (ioctl_ret == -1)
        func_ret |= 1; // cant set mode

	ioctl_ret = ioctl(fd, SPI_IOC_RD_MODE, &mode);
	if (ioctl_ret == -1)
        func_ret |= 1<<1; // cant get mode
    close(fd);
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
    close(fd);
    return func_ret;
}

uint8_t set_bits_per_word(const char *device, uint8_t bits) {
    int ioctl_ret = 0;
    uint8_t func_ret = 0;
    int fd = open(device, O_RDWR);
    ioctl_ret = ioctl(fd, SPI_IOC_WR_BITS_PER_WORD, &bits);
	if (ioctl_ret == -1)
        func_ret |= 1; // cant set bits


	ioctl_ret = ioctl(fd, SPI_IOC_RD_BITS_PER_WORD, &bits);
	if (ioctl_ret == -1)
        func_ret |= 1<<1; // cant get bits
    close(fd);
    return func_ret;
}


uint8_t transfer_8_bit( const char *device,
                            uint8_t *tx, uint32_t tx_words, 
                            uint8_t *rx, 
                            uint32_t rx_words, 
                            uint16_t delay_us, uint32_t speed_hz, 
                            uint8_t bits
                            ) {

    printf("device: %s\n", device);
	int ret;
    uint8_t func_return = 0;
    rx = (uint8_t*)calloc((rx_words+1), sizeof(uint8_t));
    rx[rx_words] = 0;
	struct spi_ioc_transfer tr = {
		.tx_buf = (unsigned long)tx,
		.rx_buf = (unsigned long)rx,
		.len = tx_words,
		.delay_usecs = delay_us,
		.speed_hz = speed_hz,
		.bits_per_word = bits,
	};
    int fd = open(device, O_RDWR);
    if (fd < 0) {
        return 2; // indicating could not get a file handle
    }
	ret = ioctl(fd, SPI_IOC_MESSAGE(1), &tr);
	if (ret < 1)
        func_return = 1;
    close(fd);
    return func_return;
}



