#include "file.h"

#define _CRT_SECURE_NO_WARNINGS

#include "error.h"

#include <stdio.h>
#include <stdlib.h>

const char *readBinaryFile(const char *path, long int *size) {
#define CHECK(p, m, d) ERROR_IF(!(p), "readBinaryFile()", (m), d, NULL)

    FILE *file;
    {
        file = fopen(path, "rb");
        CHECK(file != NULL, "failed to open a file", {});
    }

    fpos_t pos = 0;
    {
        CHECK(fseek(file, 0L, SEEK_END) == 0, "failed to seek end.", fclose(file));
        CHECK(fgetpos(file, &pos) == 0, "failed to get the file size.", fclose(file));
        CHECK(fseek(file, 0L, SEEK_SET) == 0, "failed to seek set.", fclose(file));
    }

    const char *binary;
    {
        binary = (const char *)malloc(sizeof(const char) * pos);
        CHECK(
            fread((void *)binary, sizeof(const char), pos, file) == (size_t)pos,
            "failed to read a file.",
            { free((void *)binary); fclose(file); }
        );
    }

    fclose(file);

    if (size != NULL) {
        *size = (long int)pos;
    }
    
    return binary;

#undef CHECK
}
