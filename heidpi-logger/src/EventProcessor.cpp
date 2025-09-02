/** 
 * @file EventProcessor.cpp
 * Implementation of event processing logic including timestamping, 
 * GeoIP enrichment, and output formatting
 */

#include "EventProcessor.hpp"
#include <fstream>
#include <chrono>
#include <iomanip>
#include <ctime>
#include <filesystem>

/**
 * @class EventProcessor
 * Handles event processing pipeline including:
 * - Timestamp injection
 * - GeoIP enrichment (if enabled)
 * - Field/risk filtering
 * - Output file writing
 */
EventProcessor::EventProcessor(const EventConfig &cfg, const std::string &outDir)
    : config(cfg), directory(outDir)
{
    if (cfg.geoip_enabled && !cfg.geoip_path.empty())
    {
        geo = std::make_unique<GeoIP>(cfg.geoip_path, cfg.geoip_keys);
    }
    else
    {
        // optional, aber hilfreich zur Diagnose:
        Logger::info(std::string("GeoIP disabled for '") + cfg.filename +
                     "' (enabled=" + (cfg.geoip_enabled ? "true" : "false") +
                     ", path=" + (cfg.geoip_path.empty() ? "<empty>" : cfg.geoip_path) + ")");
    }
}

/**
 * @return Current timestamp in ISO 8601 format (YYYY-MM-DDTHH:MM:SS)
 */
static std::string nowTs()
{
    auto now = std::chrono::system_clock::now();
    std::time_t tt = std::chrono::system_clock::to_time_t(now);
    std::tm tm = *std::localtime(&tt); // Thread-safe with local variable
    std::ostringstream oss;
    oss << std::put_time(&tm, "%FT%T"); // ISO 8601 format (2024-03-20T12:34:56)
    return oss.str();
}

/**
 * Processes a raw event JSON object through the full pipeline:
 * 1. Injects current timestamp
 * 2. Adds GeoIP enrichment (if configured)
 * 3. Removes ignored fields
 * 4. Filters out ignored risks
 * 5. Writes to output file in specified directory
 * 
 * @param j Raw event JSON to process
 */
void EventProcessor::process(const nlohmann::json &j)
{
    nlohmann::json out = j;
    out["timestamp"] = nowTs();

    if (geo)
    { // statt config.geoip_enabled
        std::string src = j.value("src_ip", "");
        std::string dst = j.value("dst_ip", "");
        geo->enrich(src, dst, out);
    }

    for (const auto &field : config.ignore_fields)
    {
        out.erase(field);
    }

    if (!config.ignore_risks.empty() && out.contains("ndpi") && out["ndpi"].contains("flow_risk"))
    {
        for (const auto &risk : config.ignore_risks)
        {
            out["ndpi"]["flow_risk"].erase(risk);
        }
    }

    std::filesystem::create_directories(directory);
    auto path = std::filesystem::path(directory) / (config.filename + ".json");
    std::ofstream ofs(path, std::ios::app);
    if (!ofs.is_open())
    {
        Logger::error("Failed to open output file: " + path.string());
        return;
    }
    ofs << out.dump() << std::endl;
}
