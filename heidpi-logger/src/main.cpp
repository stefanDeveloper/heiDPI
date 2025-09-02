/**
 * @file main.cpp
 * @brief Main entry point for the HeidPi Logger application.
 *
 * This file implements the primary logic for connecting to an event source (TCP/Unix socket),
 * parsing command-line arguments, configuring event processing, and dispatching events to
 * appropriate handlers using a thread-safe queue and dispatcher.
 *
 * Key components:
 * - Command-line argument parsing
 * - Thread-safe event queue and dispatcher
 * - Worker threads for event processing
 * - Logging and configuration initialization
 * - Graceful shutdown handling
 */

#include "Config.hpp"
#include "Logger.hpp"
#include "NDPIClient.hpp"
#include "EventProcessor.hpp"

#include <algorithm>
#include <cstdlib>
#include <filesystem>
#include <iostream>
#include <sstream>
#include <string>
#include <thread>
#include <vector>

#include <queue>
#include <mutex>
#include <condition_variable>
#include <atomic>

/**
 * @struct CLIOptions
 * @brief Stores command-line and environment variable configuration.
 *
 * Populates default values from environment variables and command-line arguments.
 */
struct CLIOptions
{
    std::string host{"127.0.0.1"};         ///< Default TCP host
    std::string unix_path{};               ///< Unix socket path (empty by default)
    int port{7000};                        ///< Default TCP port
    std::string write_path{"/var/log"};    ///< Directory for log/event files
    std::string config_path{"config.yml"}; ///< Configuration file path
    std::string filter{};                  ///< Event filter expression
    bool show_daemon{false};               ///< Enable daemon event processing
    bool show_packet{false};               ///< Enable packet event processing
    bool show_error{false};                ///< Enable error event processing
    bool show_flow{false};                 ///< Enable flow event processing
};

/**
 * @brief Gets an environment variable or returns a default value.
 *
 * @param env Environment variable name
 * @param def Default value if `env` is not set
 * @return std::string The value of `env` or `def`
 */
static std::string envOrDefault(const char *env, const std::string &def)
{
    const char *v = std::getenv(env);
    return v ? std::string(v) : def;
}

/**
 * @brief Parses command-line arguments and environment variables.
 *
 * Merges environment variables (HOST, UNIX, etc.) with command-line flags.
 *
 * @param argc Number of command-line arguments
 * @param argv Command-line arguments
 * @return CLIOptions Parsed configuration
 */
CLIOptions parse(int argc, char **argv)
{
    CLIOptions o;
    o.host = envOrDefault("HOST", o.host);
    o.unix_path = envOrDefault("UNIX", o.unix_path);
    o.port = std::stoi(envOrDefault("PORT", std::to_string(o.port)));
    o.write_path = envOrDefault("WRITE", o.write_path);
    o.config_path = envOrDefault("CONFIG", o.config_path);
    o.filter = envOrDefault("FILTER", o.filter);
    o.show_daemon = envOrDefault("SHOW_DAEMON_EVENTS", "0") == "1";
    o.show_packet = envOrDefault("SHOW_PACKET_EVENTS", "0") == "1";
    o.show_error = envOrDefault("SHOW_ERROR_EVENTS", "0") == "1";
    o.show_flow = envOrDefault("SHOW_FLOW_EVENTS", "0") == "1";
    for (int i = 1; i < argc; ++i)
    {
        std::string a = argv[i];
        auto next = [&](int &i)
        { return std::string(argv[++i]); };
        if (a == "--host" && i + 1 < argc)
            o.host = next(i);
        else if (a == "--unix" && i + 1 < argc)
            o.unix_path = next(i);
        else if (a == "--port" && i + 1 < argc)
            o.port = std::stoi(next(i));
        else if (a == "--write" && i + 1 < argc)
            o.write_path = next(i);
        else if (a == "--config" && i + 1 < argc)
            o.config_path = next(i);
        else if (a == "--filter" && i + 1 < argc)
            o.filter = next(i);
        else if (a == "--show-daemon-events")
            o.show_daemon = !o.show_daemon;
        else if (a == "--show-packet-events")
            o.show_packet = !o.show_packet;
        else if (a == "--show-error-events")
            o.show_error = !o.show_error;
        else if (a == "--show-flow-events")
            o.show_flow = !o.show_flow;
        else if (a == "--help" || a == "-h")
        {
            std::cout << "Usage: " << argv[0] << " [options]\n"
                      << "  --host <host>            Set host\n"
                      << "  --unix <path>            Set unix socket path\n"
                      << "  --port <port>            Set port\n"
                      << "  --write <path>           Set write path\n"
                      << "  --config <path>          Set config path\n"
                      << "  --filter <expr>          Filter expression\n"
                      << "  --show-daemon-events     Toggle daemon events\n"
                      << "  --show-packet-events     Toggle packet events\n"
                      << "  --show-error-events      Toggle error events\n"
                      << "  --show-flow-events       Toggle flow events\n"
                      << "  -h, --help               Show this help message\n";
            std::exit(0);
        }
    }
    return o;
}

/**
 * @struct Worker
 * @brief Encapsulates event processing logic for a specific event type.
 *
 * Combines an event configuration and processor to handle events of a single type.
 */
struct Worker
{
    std::string eventKey;     ///< Type of event to process (e.g., "flow_event_name")
    EventConfig config;       ///< Configuration for the event type
    EventProcessor processor; ///< Processor for event data

    /**
     * @brief Constructs a Worker with configuration and directory.
     *
     * @param k Event type identifier
     * @param c Event-specific configuration
     * @param dir Directory for output files
     */
    Worker(const std::string &k, const EventConfig &c, const std::string &dir)
        : eventKey(k), config(c), processor(c, dir) {}
};

/**
 * @brief Main entry point for the HeidPi Logger application.
 *
 * Initializes logging, parses command-line arguments, connects to the event source,
 * and dispatches events to appropriate workers using a thread-safe queue.
 *
 * @param argc Number of command-line arguments
 * @param argv Command-line arguments
 * @return int Exit code (0 for success, 1 for error)
 */
int main(int argc, char **argv)
{
    // Early check for --help to avoid redundant parsing
    for (int i = 1; i < argc; ++i)
    {
        std::string a = argv[i];
        if (a == "-h" || a == "--help")
        {
            std::string name = std::filesystem::path(argv[0]).filename();
            std::cout << "usage: " << name
                      << " [-h] [--host HOST | --unix UNIX] [--port PORT] [--write WRITE]\n"
                         "            [--config CONFIG] [--filter FILTER]\n"
                         "            [--show-daemon-events]\n"
                         "            [--show-packet-events]\n"
                         "            [--show-error-events]\n"
                         "            [--show-flow-events]\n";
            return 0;
        }
    }

    // --- [1] Parse Command-Line Options ---
    CLIOptions opts = parse(argc, argv);

    // --- [2] Initialize Configuration & Logger ---
    Config cfg(opts.config_path);
    Logger::init(cfg.logging());

    // --- [3] Create Workers for Enabled Event Types ---
    std::vector<Worker> workers;
    workers.reserve(4);
    if (opts.show_flow)
        workers.emplace_back("flow_event_name", cfg.flowEvent(), opts.write_path);
    if (opts.show_packet)
        workers.emplace_back("packet_event_name", cfg.packetEvent(), opts.write_path);
    if (opts.show_daemon)
        workers.emplace_back("daemon_event_name", cfg.daemonEvent(), opts.write_path);
    if (opts.show_error)
        workers.emplace_back("error_event_name", cfg.errorEvent(), opts.write_path);

    if (workers.empty())
    {
        Logger::error("No event types enabled. Use --show-*_events flags to enable processing.");
        return 1;
    }

    // --- [4] Connect to Event Source ---
    NDPIClient client;
    try
    {
        if (!opts.unix_path.empty())
            client.connectUnix(opts.unix_path);
        else
            client.connectTcp(opts.host, static_cast<unsigned short>(opts.port));
    }
    catch (const std::exception &ex)
    {
        Logger::error(std::string("Failed to connect: ") + ex.what());
        return 1;
    }

    // --- [5] Thread-Safe Event Queue and Dispatcher ---
    std::queue<nlohmann::json> eventQueue; // FIFO queue for incoming events
    std::mutex mtx;                        // Mutex for thread-safe access
    std::condition_variable cv;            // Notifies worker of new events
    std::atomic<bool> done{false};         // Signals shutdown

    // Dispatcher thread: processes events from the queue
    std::thread dispatcher([&]
                           {
        auto getAllowedNames = [&](const std::string &eventKey) -> const std::vector<std::string>& {
            if (eventKey == "flow_event_name")   return cfg.flowEvent().event_names;
            if (eventKey == "packet_event_name") return cfg.packetEvent().event_names;
            if (eventKey == "daemon_event_name") return cfg.daemonEvent().event_names;
            if (eventKey == "error_event_name")  return cfg.errorEvent().event_names;
            static const std::vector<std::string> empty;
            return empty;
        };

        while (true) {
            nlohmann::json event;
            {
                std::unique_lock<std::mutex> lk(mtx);
                cv.wait(lk, [&]{ return done || !eventQueue.empty(); });
                if (done && eventQueue.empty()) break;
                event = std::move(eventQueue.front());
                eventQueue.pop();
            }

            std::string key;
            std::string name;
            if (event.contains("flow_event_name")) {
                key = "flow_event_name";
                name = event["flow_event_name"].get<std::string>();
            } else if (event.contains("packet_event_name")) {
                key = "packet_event_name";
                name = event["packet_event_name"].get<std::string>();
            } else if (event.contains("daemon_event_name")) {
                key = "daemon_event_name";
                name = event["daemon_event_name"].get<std::string>();
            } else if (event.contains("error_event_name")) {
                key = "error_event_name";
                name = event["error_event_name"].get<std::string>();
            } else {
                Logger::info("Received unknown event: missing event name");
                continue;
            }

            const auto &allowedNames = getAllowedNames(key);
            if (!allowedNames.empty() &&
                std::find(allowedNames.begin(), allowedNames.end(), name) == allowedNames.end()) {
                Logger::info("Skipping event '" + name + "' of type " + key);
                continue;
            }

            bool handled = false;
            for (auto &w : workers) {
                if (w.eventKey != key) continue;
                w.processor.process(event);
                handled = true;
            }
            if (!handled) {
                Logger::info("No handler enabled for event '" + name + "' of type " + key);
            }
        } });

    // --- [6] Event Reader Loop ---
    client.loop([&](const nlohmann::json &j)
                {
        {
            std::lock_guard<std::mutex> lk(mtx);
            eventQueue.push(j);
        }
        cv.notify_one(); }, opts.filter);

    // --- [7] Graceful Shutdown ---
    {
        std::lock_guard<std::mutex> lk(mtx);
        done = true;
    }
    cv.notify_all();
    dispatcher.join();

    return 0;
}
