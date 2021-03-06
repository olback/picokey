cmake_minimum_required(VERSION 3.13)

# initialize pico-sdk from submodule
# note: this must happen before project()
include(pico-sdk/pico_sdk_init.cmake)

project(picokey)

# initialize the Raspberry Pi Pico SDK
pico_sdk_init()

# Crypto
include_directories(${CMAKE_SOURCE_DIR}/lib/crypto)
link_directories(${CMAKE_SOURCE_DIR}/lib/crypto/target/thumbv6m-none-eabi/release)

# rest of your project
add_executable(picokey
    src/main.c
    src/rust_compat.c
)

# Add pico_stdlib library which aggregates commonly used features
target_link_libraries(picokey pico_stdlib pico_unique_id crypto)

# enable usb output, disable uart output
pico_enable_stdio_usb(picokey 1)
pico_enable_stdio_uart(picokey 0)

# create map/bin/hex/uf2 file in addition to ELF.
pico_add_extra_outputs(picokey)

