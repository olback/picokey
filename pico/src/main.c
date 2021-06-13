/*
 *  Picokey
 *  Rewrite in Rust when rp-hal is ready
 */

#include <crypto.h>
#include <pico/stdio.h>
#include <pico/stdlib.h>
#include <pico/unique_id.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#define INPUT_BUFFER_SIZE 256

size_t read_line(uint8_t *buf, size_t max_len, char eol);
void send_error(int32_t code);

#define CHECK_ERROR(input) \
    if ((input) != 0) {    \
        send_error(input); \
        continue;          \
    }

int main() {
    // Init stdio
    stdio_init_all();

    // Config led pin
    const size_t LED_PIN = PICO_DEFAULT_LED_PIN;
    gpio_init(LED_PIN);
    gpio_set_dir(LED_PIN, GPIO_OUT);

    // Get uuid
    // This is fine as long as PICO_UNIQUE_BOARD_ID_SIZE_BYTES is 8 bytes
    uint64_t unique_id;
    pico_get_unique_board_id((pico_unique_board_id_t *)&unique_id);

    for (;;) {
        uint32_t res;
        uint8_t data_in[INPUT_BUFFER_SIZE] = {0};
        size_t data_in_len = read_line(data_in, INPUT_BUFFER_SIZE, '\n');
        // printf("(%ld) %s\n", data_in_len, data_in);
        gpio_put(LED_PIN, 1);

        // printf("%s\n", data_in);

        // Decode incoming data
        uint8_t buf[INPUT_BUFFER_SIZE] = {0};
        uint32_t decode_buf_len = INPUT_BUFFER_SIZE;
        res = base64_decode(data_in, data_in_len, buf, &decode_buf_len);
        CHECK_ERROR(res);

        // Decrypt decoded data
        uint32_t decrypt_buf_len = INPUT_BUFFER_SIZE;
        res = aes_gcm_siv_decrypt(get_key_ptr(), KEY_LENGTH, get_iv_ptr(),
                                  IV_LENGTH, buf, decode_buf_len, buf,
                                  &decrypt_buf_len);
        CHECK_ERROR(res);

        // TODO:
        // [ garbage[0..16] + uid[..] + garbage[16..32] ] = 40
        // or some other pattern
        // encrypt, encode, send

        gpio_put(LED_PIN, 0);
    }
    return 0;
}

void send_error(int32_t code) {
    uint8_t buf[32] = {0};
    *((int32_t *)buf) = code;
    uint8_t base64_buf[64] = {0};
    uint32_t base64_buf_len = 64;
    int32_t res = base64_encode(buf, 32, base64_buf, &base64_buf_len);
    if (res == 0) {
        printf("%s\n", base64_buf);
    } else {
        printf("Invalid\n");
    }
}

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
