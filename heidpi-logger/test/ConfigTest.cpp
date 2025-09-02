// test_config.cpp
#include <gtest/gtest.h>
#include <fstream>
#include "Config.hpp"

namespace
{

    // Helper to write temporary YAML file
    std::string writeTempYAML(const std::string &content)
    {
        std::string filename = "temp_config.yaml";
        std::ofstream ofs(filename);
        ofs << content;
        ofs.close();
        return filename;
    }

} // namespace

TEST(ConfigTest, LoadsLoggingFilename)
{
    std::string yaml = R"(
logging:
  filename: "log.txt"
)";
    std::string path = writeTempYAML(yaml);
    Config cfg(path);
    EXPECT_EQ(cfg.logging().filename, "log.txt");
}

TEST(ConfigTest, LoadsBasicEventConfig)
{
    std::string yaml = R"(
flow_event:
  ignore_fields: ["field1", "field2"]
  ignore_risks: ["risk1"]
  flow_event_name: ["flow1", "flow2"]
  filename: "flow_file.log"
  threads: 4
)";
    std::string path = writeTempYAML(yaml);
    Config cfg(path);

    EXPECT_EQ(cfg.flowEvent().ignore_fields, std::vector<std::string>({"field1", "field2"}));
    EXPECT_EQ(cfg.flowEvent().ignore_risks, std::vector<std::string>({"risk1"}));
    EXPECT_EQ(cfg.flowEvent().event_names, std::vector<std::string>({"flow1", "flow2"}));
    EXPECT_EQ(cfg.flowEvent().filename, "flow_file.log");
    EXPECT_EQ(cfg.flowEvent().threads, 4);
}

TEST(ConfigTest, OverwritesEventNames)
{
    std::string yaml = R"(
packet_event:
  flow_event_name: ["should_be_overwritten"]
  packet_event_name: ["packet1"]
)";
    std::string path = writeTempYAML(yaml);
    Config cfg(path);

    // Due to multiple overwrites, only the last one should be kept
    EXPECT_EQ(cfg.packetEvent().event_names, std::vector<std::string>({"packet1"}));
}

TEST(ConfigTest, LoadsGeoIPSettings)
{
    std::string yaml = R"(
error_event:
  geoip2_city:
    enabled: true
    filepath: "/path/to/geoip"
    keys: ["country", "city"]
)";
    std::string path = writeTempYAML(yaml);
    Config cfg(path);

    EXPECT_TRUE(cfg.errorEvent().geoip_enabled);
    EXPECT_EQ(cfg.errorEvent().geoip_path, "/path/to/geoip");
    EXPECT_EQ(cfg.errorEvent().geoip_keys, std::vector<std::string>({"country", "city"}));
}

TEST(ConfigTest, HandlesMissingOptionalFieldsGracefully)
{
    std::string yaml = R"(
daemon_event: {}
)";
    std::string path = writeTempYAML(yaml);
    Config cfg(path);

    // Defaults
    EXPECT_TRUE(cfg.daemonEvent().ignore_fields.empty());
    EXPECT_TRUE(cfg.daemonEvent().ignore_risks.empty());
    EXPECT_TRUE(cfg.daemonEvent().event_names.empty());
    EXPECT_EQ(cfg.daemonEvent().threads, 1);
    EXPECT_FALSE(cfg.daemonEvent().geoip_enabled);
}
