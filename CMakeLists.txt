cmake_minimum_required(VERSION 3.16)
project(AuroraVisualizer LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC ON)

# --- Find Packages ---
find_package(Qt6 REQUIRED COMPONENTS Widgets Gui OpenGLWidgets)
find_package(GTest REQUIRED)
find_package(OpenGL REQUIRED)
find_package(Threads REQUIRED)

# --- Find the system-installed projectM library ---
find_package(PkgConfig REQUIRED)
pkg_check_modules(PROJECTM REQUIRED libprojectM)
pkg_check_modules(FREETYPE REQUIRED freetype2)

# --- Enable Testing ---
enable_testing()
add_subdirectory(tests)

# --- Project Sources ---
set(PROJECT_SOURCES
    src/main.cpp
    src/gui/MainWindow.h
    src/gui/MainWindow.cpp
    src/gui/Renderer.h
    src/gui/Renderer.cpp
    src/gui/TextRenderer.h
    src/gui/TextRenderer.cpp
    src/gui/TextAnimator.h
    src/gui/TextAnimator.cpp
    src/gui/SongTitleAnimator.h
    src/gui/SongTitleAnimator.cpp
    src/core/audio/AudioEngine.h
    src/core/audio/AudioEngine.cpp
)

# --- Executable ---
add_executable(AuroraVisualizer ${PROJECT_SOURCES})

# --- Link Libraries ---
target_link_libraries(AuroraVisualizer PRIVATE
    Qt6::Widgets
    Qt6::Gui
    Qt6::OpenGLWidgets
    OpenGL::GL
    Threads::Threads
    ${PROJECTM_LIBRARIES} # Link against the system library found by pkg-config
    ${FREETYPE_LIBRARIES}
)

# --- Include Directories ---
target_include_directories(AuroraVisualizer PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src
    ${CMAKE_CURRENT_SOURCE_DIR}/deps
    ${PROJECTM_INCLUDE_DIRS} # Use the include path from pkg-config
    ${FREETYPE_INCLUDE_DIRS}
)