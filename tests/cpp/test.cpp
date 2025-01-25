#include <stdio.h>
#include <catch2/catch_test_macros.hpp>
#include <catch2/benchmark/catch_benchmark.hpp>
#include "ramble.hpp"

TEST_CASE("Roundtrip serialization", "[serialize]")
{

    uint8_t alpha[sizeof(Packet_t)];
    uint8_t beta[sizeof(Packet_t)];

    Packet_t send_pkt;
    send_pkt.packet_type = PacketTypes::HELLO;
    send_pkt.hello.seq = 2;

    auto alpha_len = serialize(alpha, &send_pkt);
    REQUIRE(alpha_len > 0);

    auto recv_pkt = deserialize(alpha);
    auto beta_len = serialize(beta, &send_pkt);
    REQUIRE(alpha_len == beta_len);

    for (int i = 0; i < alpha_len; i++)
    {
        CHECK(alpha[i] == beta[i]);
    }
}