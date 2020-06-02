# CMakeLists.txt 
# tasinput2 - Plugin for creating TAS inputs
#
# This file is part of tasinput2.
#
# tasinput2 is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# tasinput2 is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with tasinput2.  If not, see <https://www.gnu.org/licenses/>.

cmake_minimum_required(VERSION 3.1.0)

project(tasinput2 VERSION 0.2.0 LANGUAGES C CXX)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC ON)

if (CMAKE_VERSION VERSION_LESS "3.7.0")
    set(CMAKE_INCLUDE_CURRENT_DIR ON)
endif ()

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)
set(CMAKE_CXX_FLAGS "-fPIC")

set(SOURCE_FILES "${CMAKE_CURRENT_SOURCE_DIR}/src/controller.cpp"
                 "${CMAKE_CURRENT_SOURCE_DIR}/src/inputs.cpp"
                 "${CMAKE_CURRENT_SOURCE_DIR}/src/plugin.cpp"
                 "${CMAKE_CURRENT_SOURCE_DIR}/src/tasinput2.c")
set(ALL_SOURCE_FILES "${CMAKE_CURRENT_SOURCE_DIR}/include/controller.hpp" 
                     "${CMAKE_CURRENT_SOURCE_DIR}/include/corelib.hpp"
                     "${CMAKE_CURRENT_SOURCE_DIR}/include/plugin.h"
                     "${CMAKE_CURRENT_SOURCE_DIR}/include/tasinput2.h"
                     "${CMAKE_CURRENT_SOURCE_DIR}/include/version.h.in")
set(ALL_SOURCE_FILES ${ALL_SOURCE_FILES} ${SOURCE_FILES})

add_custom_target(format 
    COMMAND clang-format -style=Google -i ${ALL_SOURCE_FILES}
)

find_package(Qt5 COMPONENTS Widgets REQUIRED)

set(TASINPUT2_INCLUDE2 "${CMAKE_CURRENT_BINARY_DIR}/include")
configure_file("${CMAKE_CURRENT_SOURCE_DIR}/include/version.h.in" "${TASINPUT2_INCLUDE2}/version.h")

add_library(tasinput2 SHARED ${SOURCE_FILES})
target_include_directories(tasinput2 PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/include" "${TASINPUT2_INCLUDE2}")
target_link_libraries(tasinput2 Qt5::Widgets)
set_property(TARGET tasinput2 PROPERTY AUTOMOC ON)
