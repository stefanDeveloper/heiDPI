#include "Logger.hpp"
#include <gtest/gtest.h>
#include <fstream>
#include <sstream>
#include <cstdio>     // For remove()
#include <thread>
#include <regex>

// Helper to read file contents
std::string readFile(const std::string& filename) {
    std::ifstream in(filename);
    std::stringstream buffer;
    buffer << in.rdbuf();
    return buffer.str();
}

// Captures std::cout and std::cerr output
class OutputCapture {
public:
    void startCapture() {
        oldCout = std::cout.rdbuf(coutStream.rdbuf());
        oldCerr = std::cerr.rdbuf(cerrStream.rdbuf());
    }

    void stopCapture() {
        std::cout.rdbuf(oldCout);
        std::cerr.rdbuf(oldCerr);
    }

    std::string getCapturedStdout() const {
        return coutStream.str();
    }

    std::string getCapturedStderr() const {
        return cerrStream.str();
    }

private:
    std::ostringstream coutStream;
    std::ostringstream cerrStream;
    std::streambuf* oldCout = nullptr;
    std::streambuf* oldCerr = nullptr;
};

TEST(LoggerTest, LogsToFileAndConsole) {
    std::string testFile = "test_log.txt";
    std::remove(testFile.c_str());  // Ensure clean start

    LoggingConfig cfg;
    cfg.filename = testFile;
    Logger::init(cfg);

    OutputCapture capture;
    capture.startCapture();

    Logger::info("This is an info message.");
    Logger::error("This is an error message.");

    capture.stopCapture();

    std::string fileContent = readFile(testFile);
    std::string stdoutContent = capture.getCapturedStdout();
    std::string stderrContent = capture.getCapturedStderr();

    std::regex infoPattern(R"(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2} INFO: This is an info message\.\n)");
    std::regex errorPattern(R"(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2} ERROR: This is an error message\.\n)");

    // Console output checks
    EXPECT_TRUE(std::regex_match(stdoutContent, infoPattern));
    EXPECT_TRUE(std::regex_match(stderrContent, errorPattern));

    // File output checks
    EXPECT_NE(fileContent.find("INFO: This is an info message."), std::string::npos);
    EXPECT_NE(fileContent.find("ERROR: This is an error message."), std::string::npos);

    std::remove(testFile.c_str());  // Cleanup
    Logger::destroy();
}

TEST(LoggerTest, LogsWithoutFile) {
    LoggingConfig cfg;
    cfg.filename = ""; // No file
    Logger::init(cfg);

    OutputCapture capture;
    capture.startCapture();

    Logger::info("Console only info");
    Logger::error("Console only error");

    capture.stopCapture();

    EXPECT_NE(capture.getCapturedStdout().find("INFO: Console only info"), std::string::npos);
    EXPECT_NE(capture.getCapturedStderr().find("ERROR: Console only error"), std::string::npos);

    Logger::destroy();
}

TEST(LoggerTest, ThreadSafeLogging) {
    std::string testFile = "threaded_log.txt";
    std::remove(testFile.c_str());

    LoggingConfig cfg;
    cfg.filename = testFile;
    Logger::init(cfg);

    auto logTask = [](int threadId) {
        for (int i = 0; i < 10; ++i) {
            Logger::info("Thread " + std::to_string(threadId) + " message " + std::to_string(i));
        }
    };

    std::thread t1(logTask, 1);
    std::thread t2(logTask, 2);

    t1.join();
    t2.join();

    std::string content = readFile(testFile);

    for (int i = 0; i < 10; ++i) {
        EXPECT_NE(content.find("Thread 1 message " + std::to_string(i)), std::string::npos);
        EXPECT_NE(content.find("Thread 2 message " + std::to_string(i)), std::string::npos);
    }

    std::remove(testFile.c_str());
    Logger::destroy();
}
