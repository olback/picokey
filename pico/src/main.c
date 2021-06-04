/*
 *  Picokey
 *  Rewrite in Rust when rp-hal is ready
 */

#include <keys.h>
#include <pico/stdio.h>
#include <pico/stdlib.h>
#include <stdint.h>
#include <tomcrypt.h>

#define INPUT_BUFFER_LENGTH 128

size_t read_line(uint8_t *buf, size_t max_len, char eol);

int main() {
    // Init stdio
    stdio_init_all();

    // Config led pin
    const size_t LED_PIN = PICO_DEFAULT_LED_PIN;
    gpio_init(LED_PIN);
    gpio_set_dir(LED_PIN, GPIO_OUT);

    for (;;) {
        uint8_t data_in[INPUT_BUFFER_LENGTH] = {0};
        size_t len = read_line(data_in, INPUT_BUFFER_LENGTH, '\n');
        gpio_put(LED_PIN, 1);

        uint8_t based[128] = {0};
        unsigned long int based_len = 128;
        base64_encode(data_in, len, based, &based_len);

        // printf("Data in: (%li) %s, Base64: (%li) %s\n", len, data_in,
        // based_len, based);
        struct Key k = get_public_key();
        printf("%li, %s\n", k.len, k.data);
        gpio_put(LED_PIN, 0);
    }
    return 0;
}

size_t read_line(uint8_t buf[], size_t max_len, char eol) {
    size_t read_chars = 0;
    for (size_t i = 0; i < max_len; i++) {
        char c = getc(stdin);
        if (c == eol) {
            break;
        } else {
            buf[i] = c;
            read_chars++;
        }
    }
    return read_chars;
}
