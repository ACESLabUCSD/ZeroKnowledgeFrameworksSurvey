cmake_minimum_required(VERSION 3.10)

project(ACESLib VERSION 1.0)

set(CMAKE_CXX_STANDARD 20)

include_directories(/usr/local/include/emp-ot/)
include_directories(/usr/local/include/emp-tool/)
include_directories(/usr/local/include/emp-zk/)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -march=native -O2")
#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -msse4.1 -maes -pthread")
set(EXECUTABLE_OUTPUT_PATH ${CMAKE_BINARY_DIR}/bin)

FIND_PACKAGE(EMP-TOOL REQUIRED)
FIND_PACKAGE(EMP-OT REQUIRED)
FIND_PACKAGE(EMP-ZK REQUIRED)
FIND_PACKAGE(Threads REQUIRED)

set(ZK_LIBRARIES "/usr/local/lib/libemp-tool.so"
                 "/usr/local/lib/libemp-zk.so"
                                 "-lpthread"
                                 "-lssl"
                                 "-lcrypto")
if (FALSE)
endif()

add_executable(zktest src/hash.cpp)
target_link_libraries(zktest ${ZK_LIBRARIES})
