#include "NDPIClient.hpp"
#include <sys/socket.h>
#include <sys/un.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <cstring>
#include <sstream>
#include <iomanip>
#include <stdexcept>
#include <errno.h>

/**
 * @class NDPIClient
 * @brief Client for connecting to the NDPIServer and receiving JSON messages.
 * 
 * This class provides TCP and Unix domain socket connectivity to an NDPIServer,
 * with support for optional message filters and automatic JSON parsing.
 * Implements safety limits to prevent memory exhaustion from large payloads.
 */
NDPIClient::NDPIClient() {}
NDPIClient::~NDPIClient() { if (fd >= 0) ::close(fd); }

/**
 * @brief Establishes a TCP connection to the specified host and port.
 * 
 * @param host IPv4 address string (e.g., "127.0.0.1")
 * @param port TCP port number (0-65535)
 * @throws std::runtime_error if socket creation or connection fails
 * @throws std::invalid_argument for invalid IPv4 address format
 */
void NDPIClient::connectTcp(const std::string &host, unsigned short port) {
    fd = ::socket(AF_INET, SOCK_STREAM, 0);
    if (fd < 0)
        throw std::runtime_error("socket: " + std::string(strerror(errno)));

    sockaddr_in addr{};
    addr.sin_family = AF_INET;
    addr.sin_port = htons(port);
    ::inet_pton(AF_INET, host.c_str(), &addr.sin_addr);

    if (::connect(fd, (sockaddr *)&addr, sizeof(addr)) < 0)
        throw std::runtime_error("connect: " + std::string(strerror(errno)));
}

/**
 * @brief Establishes a Unix domain socket connection to the specified path.
 * 
 * @param path Path to Unix socket file (max 107 bytes)
 * @throws std::runtime_error if socket creation or connection fails
 * @throws std::invalid_argument for invalid path length
 */
void NDPIClient::connectUnix(const std::string &path) {
    fd = ::socket(AF_UNIX, SOCK_STREAM, 0);
    if (fd < 0)
        throw std::runtime_error("socket: " + std::string(strerror(errno)));

    sockaddr_un addr{};
    addr.sun_family = AF_UNIX;
    
    if (path.size() >= sizeof(addr.sun_path) - 1) {
        throw std::invalid_argument("Unix socket path too long (max " 
                                  + std::to_string(sizeof(addr.sun_path) - 1) 
                                  + " bytes)");
    }
    
    std::strncpy(addr.sun_path, path.c_str(), sizeof(addr.sun_path)-1);
    if (::connect(fd, (sockaddr*)&addr, sizeof(addr)) < 0)
        throw std::runtime_error("connect: " + std::string(strerror(errno)));
}

/**
 * @brief Main message loop for receiving and processing JSON messages
 * 
 * @param cb Callback function to process parsed JSON messages
 * @param filter Optional message filter (sent to server in 6-digit length format)
 * @throws std::runtime_error for send/receive errors or malformed messages
 * 
 * Message format: 6-digit length field followed by JSON payload
 * Maximum allowed payload size: 10MB (safety limit)
 */
void NDPIClient::loop(const std::function<void(const nlohmann::json &)> &cb, const std::string &filter) {
    // Send optional filter expression before starting the receive loop
    if (!filter.empty())
    {
        std::ostringstream ss;
        ss << std::setw(6) << std::setfill('0') << filter.size() << filter;
        std::string msg = ss.str();

        ssize_t sent = ::send(fd, msg.c_str(), msg.size(), 0);
        if (sent < 0 || static_cast<size_t>(sent) != msg.size())
            throw std::runtime_error("send: " + std::string(strerror(errno)));
    }

    while (true)
    {
        char lenbuf[6];
        // Read 6 bytes for length (server uses 6-digit format with leading zeros)
        ssize_t n = ::recv(fd, lenbuf, 5, MSG_WAITALL);
        if (n <= 0) break;
        
        lenbuf[5] = '\0';  // Properly null-terminate the buffer
        size_t len;

        try
        {
            len = std::stoul(lenbuf);
            if (len == 0 || len > 10 * 1024 * 1024)
            { // 10MB safety limit
                throw std::runtime_error("invalid message length");
            }
        }
        catch (const std::exception &e)
        {
            throw std::runtime_error("invalid message length format: " + std::string(e.what()));
        }

        // Read JSON payload with safety checks
        std::string payload(len, '\0');
        n = ::recv(fd, payload.data(), len, MSG_WAITALL);
        if (n <= 0)
            break;

        try
        {
            auto j = nlohmann::json::parse(payload);
            cb(j);
        }
        catch (...)
        {
            // JSON-Fehler ignorieren
        }
    }
}
