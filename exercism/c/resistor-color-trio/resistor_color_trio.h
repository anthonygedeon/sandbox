#ifndef RESISTOR_COLOR_TRIO_H
#define RESISTOR_COLOR_TRIO_H

#include <stdint.h>

typedef enum {
	BLACK = 0, BROWN, RED, ORANGE, YELLOW,
    GREEN, BLUE, VIOLET, GREY, WHITE
} resistor_band_t;

typedef enum {
	OHMS = 0,
	KILOOHMS,
	MEGAOHMS,
	GIGAOHMS,
} resistor_unit_t;

typedef struct {
	resistor_unit_t unit;
	uint16_t value;
} resistor_value_t ;

resistor_value_t color_code(resistor_band_t bands[]);

#endif
