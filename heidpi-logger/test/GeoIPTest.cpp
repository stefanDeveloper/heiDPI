#include <gtest/gtest.h>
#include "GeoIP.hpp"

using namespace testing;
using json = nlohmann::json;

// A helper to simulate MMDB_entry_data_s for UTF8 string
MMDB_entry_data_s makeStringEntry(const std::string &s) {
    MMDB_entry_data_s entry{};
    entry.type = MMDB_DATA_TYPE_UTF8_STRING;
    entry.utf8_string = s.c_str();
    entry.data_size = static_cast<uint16_t>(s.size());
    entry.has_data = 1;
    return entry;
}

class GeoIPTest : public ::testing::Test {
protected:
    void SetUp() override {
        // Normally, you'd inject or mock MMDB functions.
        // For simplicity, assume GeoIP is modifiable for testing.
    }

    void TearDown() override {
    }
};

// A mock/fake class to override behavior
class FakeGeoIP : public GeoIP {
public:
    FakeGeoIP(const std::vector<std::string>& keys)
        : GeoIP("/dev/null", keys) {
        loaded = true;
    }

    // Override lookup to simulate IP data
    nlohmann::json lookup(const std::string &ip) const override {
        if (ip == "1.2.3.4") {
            return {
                {"country", "USA"},
                {"city", "San Francisco"},
                {"lat", 37.7749},
                {"lon", -122.4194}
            };
        } else if (ip == "8.8.8.8") {
            return {
                {"country", "USA"},
                {"city", "Mountain View"}
            };
        }
        return json{};
    }
};

TEST_F(GeoIPTest, LookupReturnsExpectedFields) {
    FakeGeoIP geoip({"country", "city", "lat", "lon"});

    json result = geoip.lookup("1.2.3.4");

    EXPECT_EQ(result["country"], "USA");
    EXPECT_EQ(result["city"], "San Francisco");
    EXPECT_DOUBLE_EQ(result["lat"], 37.7749);
    EXPECT_DOUBLE_EQ(result["lon"], -122.4194);
}

TEST_F(GeoIPTest, LookupReturnsEmptyForUnknownIP) {
    FakeGeoIP geoip({"country", "city"});
    json result = geoip.lookup("192.0.2.1");

    EXPECT_TRUE(result.empty());
}

TEST_F(GeoIPTest, EnrichAddsSrcAndDstGeoFields) {
    FakeGeoIP geoip({"country", "city"});

    json out;
    geoip.enrich("1.2.3.4", "8.8.8.8", out);

    ASSERT_TRUE(out.contains("src_geoip2_city"));
    ASSERT_TRUE(out.contains("dst_geoip2_city"));

    EXPECT_EQ(out["src_geoip2_city"]["city"], "San Francisco");
    EXPECT_EQ(out["dst_geoip2_city"]["city"], "Mountain View");
}

TEST_F(GeoIPTest, EnrichHandlesEmptyLookups) {
    FakeGeoIP geoip({"country", "city"});

    json out;
    geoip.enrich("192.0.2.1", "203.0.113.5", out); // Non-existent

    EXPECT_FALSE(out.contains("src_geoip2_city"));
    EXPECT_FALSE(out.contains("dst_geoip2_city"));
}
