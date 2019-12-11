#include <mqueue.h>
#include <sys/stat.h>
#include <fcntl.h>
#include "tlpi_hdr.h"

static voidusageError(const char *progName){    fprintf(stderr, "Usage: %s [-cx] [-m maxmsg] [-s msgsize] mq-name "            "[octal-perms]\n", progName);    fprintf(stderr, "    -c          Create queue (O_CREAT)\n");    fprintf(stderr, "    -m maxmsg   Set maximum # of messages\n");    fprintf(stderr, "    -s msgsize  Set maximum message size\n");    fprintf(stderr, "    -x          Create exclusively (O_EXCL)\n");    exit(EXIT_FAILURE);}intmain(int argc, char *argv[]){    int flags, opt;    mode_t perms;    mqd_t mqd;    struct mq_attr attr, *attrp;    attrp = NULL;    attr.mq_maxmsg = 50;    attr.mq_msgsize = 2048;    flags = O_RDWR;
