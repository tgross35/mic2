cmake_minimum_required(VERSION 3.25)
project(test_build LANGUAGES CXX)
add_executable(test_build test_build.cpp)
set_property(TARGET test_build PROPERTY C_STANDARD 17)

target_link_libraries(test_build sfml-audio)
target_link_libraries(test_build sfml-graphics)
target_link_libraries(test_build sfml-window)
target_link_libraries(test_build sfml-system)
target_link_libraries(test_build udev)
target_link_libraries(test_build usb-1.0)
target_link_libraries(test_build ftdi1)
target_link_libraries(test_build /home/drebbe/dev/neovi_mic/target/debug/libmic2.a)

include_directories(/home/drebbe/dev/neovi_mic/target/debug/)

