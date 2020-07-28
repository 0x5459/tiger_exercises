#include "tiger_string.h"
#include "util.h"

TIGER_STRING _new_tiger_string() {
    TIGER_STRING s = (TIGER_STRING)checked_malloc(sizeof(TIGER_STRING));
    s->len = 0;
    s->cap = 32;
    s->val = (char*)checked_malloc(s->cap);
    return s;
}

void _tiger_string_append(TIGER_STRING tiger_string, char *str, int len) {
    int i, new_cap;
    // Check if the tiger_string needs to be expanded
    if(tiger_string->len + len > tiger_string->cap) {
        new_cap = (tiger_string->len+len) * 1.5;
        tiger_string->val = (char*)checked_malloc(new_cap);
        tiger_string->cap = new_cap;
    }
    for (i=0; i<len; i++) {
        tiger_string->val[tiger_string->len + i] = str[i];
    }
    tiger_string->len += len;
}

TIGER_STRING _cur_tiger_str;

void tiger_string_init() {
    _cur_tiger_str = _new_tiger_string();
}

void tiger_string_append(char *str, int len) {
    _tiger_string_append(_cur_tiger_str, str, len);
}

char* tiger_string_inner() {
    return _cur_tiger_str->val;
}
