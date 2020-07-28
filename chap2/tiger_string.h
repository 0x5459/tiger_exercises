#pragma once


typedef struct {
    int len;
    int cap;
    char *val;
} *TIGER_STRING;


void tiger_string_init();
void tiger_string_append(char *str, int len);
char* tiger_string_inner();