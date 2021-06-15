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

#define DEFAULT_BUFFER_SIZE 256
#define GARBAGE_SIZE 32
#define GARBAGE_SIZE_HALF (GARBAGE_SIZE / 2)
#define ERROR_SIZE 32
#define LED_PIN PICO_DEFAULT_LED_PIN
#define CHECK_ERROR(input) \
    if ((input) != 0) {    \
        send_error(input); \
        continue;          \
    }

#define ASSERT(input, err) \
    if (!(input)) {        \
        CHECK_ERROR(err)   \
    }

// Forward decl
size_t read_line(uint8_t *buf, size_t max_len, char eol);
void send_error(int32_t code);

int main() {
    // Init stdio
    stdio_init_all();

    // Config led pin
    gpio_init(LED_PIN);
    gpio_set_dir(LED_PIN, GPIO_OUT);

    // Get uuid
    pico_unique_board_id_t unique_id;
    pico_get_unique_board_id(&unique_id);

    for (;;) {
        uint32_t res;

        /*
         *  Read input
         */
        uint8_t data_in[DEFAULT_BUFFER_SIZE] = {0};
        size_t data_in_len = read_line(data_in, DEFAULT_BUFFER_SIZE, '\n');
        gpio_put(LED_PIN, 1);

        // Decode incoming data
        uint32_t decode_buf_len = DEFAULT_BUFFER_SIZE;
        res = base64_decode(data_in, data_in_len, data_in, &decode_buf_len);
        CHECK_ERROR(res);

        // Decrypt decoded data
        uint32_t decrypt_buf_len = DEFAULT_BUFFER_SIZE;
        res = aes_gcm_siv_decrypt(get_key_ptr(), KEY_LENGTH, get_iv_ptr(),
                                  IV_LENGTH, data_in, decode_buf_len, data_in,
                                  &decrypt_buf_len);
        CHECK_ERROR(res);
        ASSERT(decrypt_buf_len == GARBAGE_SIZE, -5);

        /*
         *  Construct new data
         */
        uint8_t new_data_buf[GARBAGE_SIZE + PICO_UNIQUE_BOARD_ID_SIZE_BYTES] = {
            0};
        memcpy(&new_data_buf[0], &data_in[0], GARBAGE_SIZE_HALF);
        memcpy(
            &new_data_buf[GARBAGE_SIZE_HALF + PICO_UNIQUE_BOARD_ID_SIZE_BYTES],
            &data_in[GARBAGE_SIZE_HALF], GARBAGE_SIZE_HALF);
        memcpy(&new_data_buf[GARBAGE_SIZE_HALF], (uint8_t *)&unique_id,
               PICO_UNIQUE_BOARD_ID_SIZE_BYTES);

        /*
         *  Prepare response
         */
        uint8_t out_buf[DEFAULT_BUFFER_SIZE] = {0};

        // Encrypt
        uint32_t encrypt_buf_len = DEFAULT_BUFFER_SIZE;
        res = aes_gcm_siv_encrypt(get_key_ptr(), KEY_LENGTH, get_iv_ptr(),
                                  IV_LENGTH, new_data_buf, GARBAGE_SIZE + 8,
                                  out_buf, &encrypt_buf_len);
        CHECK_ERROR(res);

        // Encode
        uint32_t encode_buf_len = DEFAULT_BUFFER_SIZE;
        res = base64_encode(out_buf, encrypt_buf_len, out_buf, &encode_buf_len);
        CHECK_ERROR(res);

        // Add null-termination
        out_buf[encode_buf_len] = 0;

        /*
         *  Send the response
         */
        printf("%s\n", out_buf);

        gpio_put(LED_PIN, 0);
    }
    return 0;
}

void send_error(int32_t code) {
    uint8_t buf[ERROR_SIZE] = {0};
    *((int32_t *)buf) = code;
    uint8_t base64_buf[ERROR_SIZE * 2] = {0};
    uint32_t base64_buf_len = ERROR_SIZE * 2;
    int32_t res = base64_encode(buf, ERROR_SIZE, base64_buf, &base64_buf_len);
    if (res == 0) {
        printf("%s\n", base64_buf);
    } else {
        printf("Invalid\n");
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
