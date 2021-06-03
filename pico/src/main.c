/*
 *  Picokey
 *  Rewrite in Rust when rp-hal is ready
 */

#include <stdio.h>
#include <stdlib.h>
#include "pico/stdlib.h"

int main() {

    // Init stdio
    stdio_init_all();

    // Hello
    printf("Hello\n");

    // Config led pin
    const uint LED_PIN = PICO_DEFAULT_LED_PIN;
    gpio_init(LED_PIN);
    gpio_set_dir(LED_PIN, GPIO_OUT);

    for(;;) {
        printf("A: %d\n", rand());
        gpio_put(LED_PIN, 1);
        sleep_ms(100);
        gpio_put(LED_PIN, 0);
        sleep_ms(100);
    }
    return 0;
}
