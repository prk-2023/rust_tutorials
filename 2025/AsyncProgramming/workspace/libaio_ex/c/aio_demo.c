// Note: O_DIRECT: is a Linux-specific flag that tells the kernel to bypass the Page Cache. 
// Because it's not part of the POSIX standard, headers like fcntl.h hide it unless
// _GNU_SOURCE is defined.
#define _GNU_SOURCE  // Must be the first line
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <fcntl.h>
#include <unistd.h>
#include <libaio.h>
#include <errno.h>

int main() {
    const char *path = "demo_aio.txt";
    int fd;
    
    // 1. Setup the AIO context
    // This is the queue manager. We'll allow up to 10 simultaneous events.
    io_context_t ctx = 0;
    if (io_setup(10, &ctx) != 0) {
        perror("io_setup error");
        return 1;
    }

    // 2. Open the file
    // Note: O_DIRECT is often used with libaio to bypass the OS cache,
    // ensuring true asynchronous behavior to the disk.
    fd = open(path, O_WRONLY | O_CREAT | O_TRUNC | O_DIRECT | O_SYNC, 0644);
    if (fd < 0) {
        perror("open error");
        return 1;
    }

    // 3. Prepare the data buffer
    // O_DIRECT requires memory alignment (usually 4KB/512B pages)
    void *buf;
    size_t size = 4096;
    posix_memalign(&buf, 4096, size);
    memset(buf, 'A', size); // Fill it with 'A's

    // 4. Initialize the I/O Control Block (iocb)
    struct iocb cb;
    struct iocb *cbs[1];
    io_prep_pwrite(&cb, fd, buf, size, 0);
    cbs[0] = &cb;

    // 5. Submit the I/O request
    // This is non-blocking! The kernel takes the request and returns immediately.
    int ret = io_submit(ctx, 1, cbs);
    if (ret != 1) {
        perror("io_submit error");
        return 1;
    }
    printf("Request submitted. The program is now free to do other things...\n");

    // --- DO OTHER WORK HERE ---
    printf("Doing some heavy math calculations while disk writes...\n");
    sleep(1); 

    // 6. Wait for the result
    // This will block until at least 1 event is completed.
    struct io_event events[1];
    ret = io_getevents(ctx, 1, 1, events, NULL);
    
    if (ret > 0) {
        printf("I/O completed! Bytes written: %ld\n", events[0].res);
    }

    // 7. Cleanup
    free(buf);
    close(fd);
    io_destroy(ctx);
    return 0;
}
