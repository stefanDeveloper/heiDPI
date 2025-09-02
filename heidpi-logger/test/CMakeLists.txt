find_package(GTest REQUIRED)

# Add test executables
add_executable(LoggerTest LoggerTest.cpp ${PROJECT_SOURCE_DIR}/src/Logger.cpp)
target_include_directories(LoggerTest PRIVATE
    ${PROJECT_SOURCE_DIR}/include
)
target_link_libraries(LoggerTest PRIVATE ${GTEST_LIBRARIES} nlohmann_json yaml-cpp maxminddb nlohmann_json_schema_validator gtest_main pthread)

add_executable(ConfigTest ConfigTest.cpp  ${PROJECT_SOURCE_DIR}/src/Logger.cpp ${PROJECT_SOURCE_DIR}/src/Config.cpp)
target_include_directories(ConfigTest PRIVATE
    ${PROJECT_SOURCE_DIR}/include
)
target_link_libraries(ConfigTest PRIVATE ${GTEST_LIBRARIES} nlohmann_json yaml-cpp maxminddb nlohmann_json_schema_validator gtest_main pthread)

add_executable(GeoIPTest GeoIPTest.cpp ${PROJECT_SOURCE_DIR}/src/Logger.cpp ${PROJECT_SOURCE_DIR}/src/GeoIP.cpp)
target_include_directories(GeoIPTest PRIVATE
    ${PROJECT_SOURCE_DIR}/include
)
target_link_libraries(GeoIPTest PRIVATE ${GTEST_LIBRARIES} nlohmann_json yaml-cpp maxminddb nlohmann_json_schema_validator gtest_main pthread)

# Register tests with CTest
add_test(NAME LoggerTest COMMAND LoggerTest)
add_test(NAME GeoIPTest COMMAND GeoIPTest)
add_test(NAME ConfigTest COMMAND ConfigTest)