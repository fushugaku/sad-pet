cmake_minimum_required(VERSION 3.22)
project(SadPetPlugins VERSION 0.6.1 LANGUAGES C CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)
if(APPLE)
    enable_language(OBJC OBJCXX)
    set(CMAKE_OSX_DEPLOYMENT_TARGET "11.0" CACHE STRING "Minimum macOS version")
endif()

include(FetchContent)
set(SAD_PET_JUCE_PATH "" CACHE PATH "Optional existing JUCE checkout")
if(SAD_PET_JUCE_PATH)
    add_subdirectory("${SAD_PET_JUCE_PATH}" "${CMAKE_BINARY_DIR}/JUCE")
else()
    FetchContent_Declare(JUCE
        URL https://codeload.github.com/juce-framework/JUCE/tar.gz/a74b024c188339fdfe07b73088e6f6961ff66709
        URL_HASH SHA256=30bb5854fe207da084d9146eef30677aadb8a29617e2451e6894bc30a1c5199d
        DOWNLOAD_EXTRACT_TIMESTAMP FALSE)
    FetchContent_MakeAvailable(JUCE)
endif()

find_program(CARGO_EXECUTABLE cargo REQUIRED)
get_filename_component(SAD_PET_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/.." ABSOLUTE)
set(SAD_PET_CARGO_TARGET "" CACHE STRING "Optional Rust target triple")
set(SAD_PET_RUST_ARCHIVE "" CACHE FILEPATH "Prebuilt engine archive (for universal macOS builds)")
if(NOT SAD_PET_RUST_ARCHIVE)
    set(rust_target_args "")
    set(rust_output "${CMAKE_BINARY_DIR}/cargo")
    if(SAD_PET_CARGO_TARGET)
        list(APPEND rust_target_args --target "${SAD_PET_CARGO_TARGET}")
        set(rust_output "${rust_output}/${SAD_PET_CARGO_TARGET}")
    endif()
    if(WIN32)
        set(SAD_PET_RUST_ARCHIVE "${rust_output}/release/sad_pet_plugin.lib")
    else()
        set(SAD_PET_RUST_ARCHIVE "${rust_output}/release/libsad_pet_plugin.a")
    endif()
    # Always invoke Cargo: its own dependency graph catches DSP/assets/manifest edits.
    add_custom_target(sad_pet_rust_build
        COMMAND "${CMAKE_COMMAND}" -E env "CARGO_TARGET_DIR=${CMAKE_BINARY_DIR}/cargo"
            "${CARGO_EXECUTABLE}" build --manifest-path "${SAD_PET_ROOT}/Cargo.toml"
            -p sad-pet-plugin --release --locked ${rust_target_args}
        BYPRODUCTS "${SAD_PET_RUST_ARCHIVE}" WORKING_DIRECTORY "${SAD_PET_ROOT}" VERBATIM)
endif()
add_library(sad_pet_engine STATIC IMPORTED GLOBAL)
set_target_properties(sad_pet_engine PROPERTIES IMPORTED_LOCATION "${SAD_PET_RUST_ARCHIVE}")
if(TARGET sad_pet_rust_build)
    add_dependencies(sad_pet_engine sad_pet_rust_build)
endif()

# Reuse the existing editor, loading its scripts after the native bridge is ready.
set(web_dir "${CMAKE_BINARY_DIR}/web")
file(MAKE_DIRECTORY "${web_dir}")
set_property(DIRECTORY APPEND PROPERTY CMAKE_CONFIGURE_DEPENDS "${SAD_PET_ROOT}/rust/web/index.html")
file(READ "${SAD_PET_ROOT}/rust/web/index.html" editor_html)
foreach(script library selection presets app)
    string(REPLACE "    <script src=\"./${script}.js\" defer></script>\n" "" editor_html "${editor_html}")
endforeach()
string(REPLACE "</head>" "  <link rel=\"stylesheet\" href=\"./plugin.css\" />\n    <script type=\"module\" src=\"./plugin-backend.js\"></script>\n  </head>" editor_html "${editor_html}")
file(WRITE "${web_dir}/index.html" "${editor_html}")
juce_add_binary_data(SadPetResources SOURCES
    "${web_dir}/index.html"
    "${SAD_PET_ROOT}/rust/web/style.css" "${SAD_PET_ROOT}/rust/web/app.js"
    "${SAD_PET_ROOT}/rust/web/library.js" "${SAD_PET_ROOT}/rust/web/selection.js"
    "${SAD_PET_ROOT}/rust/web/presets.js"
    web/plugin-backend.js web/plugin.css
    "${JUCE_MODULES_DIR}/juce_gui_extra/native/javascript/index.js"
    "${JUCE_MODULES_DIR}/juce_gui_extra/native/javascript/check_native_interop.js")

set(formats VST3)
if(APPLE)
    list(APPEND formats AU)
endif()
option(SAD_PET_BUILD_HOST_TESTS "Build the native plugin integration test" ON)

function(add_sad_pet_plugin target product code is_synth is_effect)
    set(extra "")
    if(WIN32)
        list(APPEND extra NEEDS_WEBVIEW2 TRUE)
    endif()
    if(is_synth)
        set(category Instrument Synth)
        set(au_type kAudioUnitType_MusicDevice)
    else()
        set(category Fx Modulation)
        set(au_type kAudioUnitType_MusicEffect)
    endif()
    juce_add_plugin(${target} COMPANY_NAME "SAD PET" BUNDLE_ID "audio.sadpet.${target}"
        PLUGIN_MANUFACTURER_CODE SdPt PLUGIN_CODE ${code}
        PRODUCT_NAME "${product}" IS_SYNTH ${is_synth} NEEDS_MIDI_INPUT TRUE
        NEEDS_MIDI_OUTPUT FALSE IS_MIDI_EFFECT FALSE EDITOR_WANTS_KEYBOARD_FOCUS TRUE
        VST3_CATEGORIES ${category} AU_MAIN_TYPE ${au_type}
        FORMATS ${formats} COPY_PLUGIN_AFTER_BUILD FALSE ${extra})
    target_sources(${target} PRIVATE native/Processor.cpp native/Processor.h native/Engine.h)
    target_compile_definitions(${target} PUBLIC SAD_PET_EFFECT=${is_effect}
        JUCE_WEB_BROWSER=1 JUCE_USE_CURL=0 JUCE_VST3_CAN_REPLACE_VST2=0
        JUCE_USE_WIN_WEBVIEW2_WITH_STATIC_LINKING=1)
    target_link_libraries(${target} PRIVATE sad_pet_engine SadPetResources
        juce::juce_audio_utils juce::juce_gui_extra
        PUBLIC juce::juce_recommended_config_flags juce::juce_recommended_warning_flags)
    if(WIN32)
        target_link_libraries(${target} PRIVATE ws2_32 userenv bcrypt ntdll)
    elseif(APPLE)
        target_link_libraries(${target} PRIVATE "-framework Security" "-framework CoreFoundation" "-framework SystemConfiguration" iconv)
    endif()
endfunction()

add_sad_pet_plugin(SadPet "SAD PET" SaPi TRUE 0)
add_sad_pet_plugin(SadPetFX "SAD PET FX" SaPf FALSE 1)

if(SAD_PET_BUILD_HOST_TESTS)
    enable_testing()
    foreach(target SadPet SadPetFX)
        add_executable(${target}HostTest native/HostTest.cpp)
        target_link_libraries(${target}HostTest PRIVATE ${target})
        target_include_directories(${target}HostTest PRIVATE "${JUCE_MODULES_DIR}")
        target_compile_definitions(${target}HostTest PRIVATE JUCE_GLOBAL_MODULE_SETTINGS_INCLUDED=1
            JucePlugin_Name="$<TARGET_PROPERTY:${target},JUCE_PRODUCT_NAME>")
        add_test(NAME ${target}HostTest COMMAND ${target}HostTest)
    endforeach()
endif()
