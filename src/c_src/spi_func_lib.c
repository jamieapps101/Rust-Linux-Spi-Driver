#include <fcntl.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdbool.h>
#include <stdint.h>
#include <sys/ioctl.h>
#include <linux/types.h>
#include <linux/gpio.h>
#include <gpiod.h>
#include <linux/spi/spidev.h>

#define ARRAY_SIZE(a) (sizeof(a) / sizeof((a)[0]))

#include "spi_func_lib.h"

#ifndef	CONSUMER
#define	CONSUMER	"Consumer"
#endif

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
uint8_t get_dev_fd(const char *device, int32_t *fd) {
    *fd = open(device, O_RDWR);
    if (fd == NULL) {
        return 1; // indicating could not get a file handle
    }
    return 0;
}

void close_dev_fd(int32_t fd) {
    close(fd);
}

uint8_t set_mode_on_fd(int32_t fd, uint8_t encoded_mode) {
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
    ioctl_ret = ioctl(fd, SPI_IOC_WR_MODE, &mode);
	if (ioctl_ret == -1) {
        func_ret |= 1; // cant set mode
    }

	ioctl_ret = ioctl(fd, SPI_IOC_RD_MODE, &mode);
	if (ioctl_ret == -1) {
        func_ret |= 1<<1; // cant get mode
    }
    return func_ret;
}

uint8_t transfer_8_bit_on_fd(int32_t fd, 
    uint8_t *tx,
    uint32_t tx_words,
    uint8_t *rx,
    uint32_t rx_words,
    uint16_t delay_us,
    uint32_t speed_hz,
    uint8_t bits
) {
    int ret;
    uint8_t func_return = 0;
    // rx = (uint8_t*)calloc((rx_words+1), sizeof(uint8_t));
    // rx[rx_words] = 0;
	struct spi_ioc_transfer tr = {
		.tx_buf = (unsigned long)tx,
		.rx_buf = (unsigned long)rx,
		.len = tx_words,
		.delay_usecs = delay_us,
		.speed_hz = speed_hz,
		.bits_per_word = bits,
	};
	ret = ioctl(fd, SPI_IOC_MESSAGE(1), &tr);
	if (ret < 1) {
        func_return = 1;
    }
    return func_return;
}

uint8_t transfer_8_bit_DC_on_fd(int32_t fd, 
    const char *gpio_dev,
    uint8_t dc_line_no,
    uint64_t *command_tx,
    uint32_t command_tx_words,
    uint64_t *data_tx,
    uint32_t data_tx_words,
    bool command_mode_active_high,
    // bool cs_active_high,
    // uint64_t *rx,
    // uint32_t rx_words,
    uint16_t delay_us,
    uint32_t speed_hz,
    uint8_t bits
) {
    unsigned int dc_command = 0;
    unsigned int dc_data = 0;
    if (command_mode_active_high) {
        dc_command = !dc_data;
    } else {
        dc_data = !dc_command;
    }
    // // get inst of gpio dev for cs and DC line 
    struct gpiod_chip *chip;
	struct gpiod_line *dc_line;
    chip = gpiod_chip_open_by_name(gpio_dev);
    if (!chip) {
        return 1;
    }
    dc_line = (struct gpiod_line*) gpiod_chip_get_line(chip, dc_line_no);

    if (!dc_line) {
		gpiod_chip_close(chip);
		return 2;
	}
    
    // request ability to control dc_line as an output
    if(gpiod_line_request_output(dc_line, CONSUMER, 0) < 0) {
        gpiod_line_release(dc_line);
        gpiod_chip_close(chip);
		return 15;
	}

    // // set DC line
    gpiod_line_set_value(dc_line, dc_command);

    // // send command byte(s)
    int ret;

    struct spi_ioc_transfer tr = {
        .tx_buf = command_tx,
        .rx_buf = command_tx,
        .len = command_tx_words,
        .delay_usecs = delay_us,
        .speed_hz = speed_hz,
        .bits_per_word = bits,
        .cs_change = 0,
    };
    if (command_tx_words>0) {

        ret = ioctl(fd, SPI_IOC_MESSAGE(1), &tr);
        if (ret < 1) {
            // reset dc line
            gpiod_line_set_value(dc_line, 0);
            return 7;
        }
    }
    // set DC line
    gpiod_line_set_value(dc_line, dc_data);

    // send data byte(s)
    if(data_tx_words>0) {
        tr.tx_buf = data_tx;
        tr.rx_buf = data_tx;
        tr.len = data_tx_words;
        tr.cs_change = 1;

        ret = ioctl(fd, SPI_IOC_MESSAGE(1), &tr);
        if (ret < 1) {
            gpiod_line_set_value(dc_line, 0);
            return 10;
        }
    }

    gpiod_line_set_value(dc_line, 0);
    gpiod_line_release(dc_line);
    gpiod_chip_close(chip);
    return 0;
}