#include "Logger.hpp"
#include <iostream>
#include <chrono>
#include <iomanip>
#include <ctime>
#include <sstream>

std::mutex Logger::mtx;
std::ofstream Logger::file;

/**
 * @file Logger.cpp
 * @brief Implements thread-safe logging functionality with console and file output.
 *
 * This file provides the implementation for the Logger class, which supports logging
 * messages with timestamps to both the console and an optional file. It ensures thread
 * safety using a mutex and flushes critical error logs immediately to disk.
 */

/**
 * @namespace Logger
 * @brief Namespace for logging-related functionality.
 *
 * Contains the Logger class and associated functions for logging messages.
 */

/**
 * @brief Generates a timestamp in ISO 8601 format.
 *
 * @return std::string A formatted timestamp (e.g., "2024-03-20T12:34:56").
 */
static std::string timestamp()
{
    auto now = std::chrono::system_clock::now();
    std::time_t tt = std::chrono::system_clock::to_time_t(now);
    std::tm tm = *std::localtime(&tt); // Thread-safe with local variable
    std::ostringstream oss;
    oss << std::put_time(&tm, "%FT%T"); // ISO 8601 format (2024-03-20T12:34:56)
    return oss.str();
}

/**
 * @brief Initializes the logger with a configuration.
 *
 * Opens the log file in append mode if a filename is provided in the configuration.
 * If the file cannot be opened, an error is printed to stderr.
 *
 * @param cfg A LoggingConfig object containing log file settings.
 */
void Logger::init(const LoggingConfig &cfg)
{
    if (!cfg.filename.empty())
    {
        file.open(cfg.filename, std::ios::app); // Append mode
        if (!file.is_open())
        {
            std::cerr << "Failed to open log file: " << cfg.filename << std::endl;
        }
    }
}

/**
 * @brief Logs an informational message.
 *
 * Writes the message to stdout and the log file (if open), prefixed with a timestamp
 * and "INFO:".
 *
 * @param msg The message to log.
 */
void Logger::info(const std::string &msg)
{
    std::lock_guard<std::mutex> lock(mtx);
    std::string line = timestamp() + " INFO: " + msg + "\n";
    std::cout << line;
    if (file.is_open())
    {
        file << line;
        file.flush(); // Ensure logs are flushed for critical messages
    }
}

/**
 * @brief Logs an error message.
 *
 * Writes the message to stderr and the log file (if open), prefixed with a timestamp
 * and "ERROR:". Ensures immediate disk writes to avoid data loss in critical scenarios.
 *
 * @param msg The error message to log.
 */
void Logger::error(const std::string &msg)
{
    std::lock_guard<std::mutex> lock(mtx);
    std::string line = timestamp() + " ERROR: " + msg + "\n";
    std::cerr << line;
    if (file.is_open())
    {
        file << line;
        file.flush(); // Critical logs should be immediately written
    }
}

/**
 * @brief Closes the log file when the Logger object is destroyed.
 *
 * Ensures the log file is properly closed to release system resources.
 */
Logger::~Logger()
{
    if (file.is_open())
    {
        file.close();
    }
}
