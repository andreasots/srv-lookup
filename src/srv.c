#include <string.h>
#include <netinet/in.h>
#include <arpa/nameser.h>
#include <resolv.h>
#include <stdint.h>

void srv_lookup(const char *name, void(*callback)(void*, uint16_t, uint16_t, uint16_t, const char*), void *cb_data)
{
    unsigned char buf[NS_MAXMSG];
    char domain_name[NS_MAXDNAME];
    const unsigned char *data;
    HEADER *header = (HEADER*)buf;
    ns_msg msg;
    ns_rr rr;
    int i, j, length;

    for (i = 0; i < sizeof(buf); i++)
        buf[i] = 0;

    res_init();
    length = res_query(name, C_IN, T_SRV, buf, sizeof(buf));
    if (length < 0 || length == sizeof(buf)) {
        return;
    }
    if (ns_initparse(buf, length, &msg) == -1) {
        fprintf(stderr, "%s: ns_initparse: %m\n", name);
        return;
    }
    for (i = 0; i < ntohs(header->ancount); i++) {
        if (ns_parserr(&msg, ns_s_an, i, &rr) == -1) {
            fprintf(stderr, "%s: ns_parserr: %m\n", name);
            return;
        }
        if (strcmp(name, ns_rr_name(rr)) != 0)
            continue;
        data = ns_rr_rdata(rr);
        ns_name_unpack(buf, buf+sizeof(buf), data+6, domain_name, sizeof(domain_name));
        ns_name_ntop(domain_name, domain_name, sizeof(domain_name));
        callback(cb_data, ns_get16(data), ns_get16(data+2), ns_get16(data+4), domain_name);
    }
}
