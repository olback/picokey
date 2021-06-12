/*
 *  Picokey
 *  Rewrite in Rust when rp-hal is ready
 */

#include <crypto.h>
#include <pico/stdio.h>
#include <pico/stdlib.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#define INPUT_BUFFER_LENGTH 256

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
        size_t data_in_len = read_line(data_in, INPUT_BUFFER_LENGTH, '\n');
        printf("(%ld) %s\n", data_in_len, data_in);
        gpio_put(LED_PIN, 1);

        const uint8_t *key = (uint8_t *)"an example very very secret key.";
        const uint32_t key_len = strlen((char *)key);
        printf("(%d) %s\n", key_len, key);

        const uint8_t *iv = (uint8_t *)"unique nonce";
        const uint32_t iv_len = strlen((char *)iv);
        printf("(%d) %s\n", iv_len, iv);

        uint8_t data_out[INPUT_BUFFER_LENGTH] = {0};
        uint32_t data_out_len = INPUT_BUFFER_LENGTH;

        int32_t res = aes_gcm_siv_encrypt(key, key_len, iv, iv_len, data_in,
                                          data_in_len, data_out, &data_out_len);
        printf("Res: %d, (%d) %s\n", res, data_out_len, data_out);

        uint32_t base64_encode_len = INPUT_BUFFER_LENGTH;
        res =
            base64_encode(data_out, data_out_len, data_out, &base64_encode_len);
        printf("Base64 enocde: %d, (%d) %s\n", res, base64_encode_len,
               data_out);

        gpio_put(LED_PIN, 0);
    }
    return 0;
}

void led_off() { gpio_put(PICO_DEFAULT_LED_PIN, 0); }

void panic_handler() {
    while (1) {
        gpio_put(PICO_DEFAULT_LED_PIN, 1);
        sleep_ms(100);
        gpio_put(PICO_DEFAULT_LED_PIN, 0);
        sleep_ms(100);
    }
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
