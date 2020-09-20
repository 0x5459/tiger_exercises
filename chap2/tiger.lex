%{
#include <string.h>
#include "util.h"
#include "tokens.h"
#include "errormsg.h"
#include "tiger_string.h"

int charPos=1;

int yywrap(void)
{
 charPos=1;
 return 1;
}


void adjust(void)
{
 EM_tokPos=charPos;
 charPos+=yyleng;
}

%}

%Start TIGER_INITIAL TIGER_COMMENT TIGER_STRING

%%
<TIGER_INITIAL>[ \t]+                   {adjust(); continue;}
<TIGER_INITIAL>\r?\n                    {adjust(); EM_newline(); continue;}
<TIGER_INITIAL>","                      {adjust(); return COMMA;}
<TIGER_INITIAL>":"                      {adjust(); return COLON;}
<TIGER_INITIAL>";"                      {adjust(); return SEMICOLON;}
<TIGER_INITIAL>"("                      {adjust(); return LPAREN;}
<TIGER_INITIAL>")"                      {adjust(); return RPAREN;}
<TIGER_INITIAL>"["                      {adjust(); return LBRACK;}
<TIGER_INITIAL>"]"                      {adjust(); return RBRACK;}
<TIGER_INITIAL>"{"                      {adjust(); return LBRACE;}
<TIGER_INITIAL>"}"                      {adjust(); return RBRACE;}
<TIGER_INITIAL>"."                      {adjust(); return DOT;}
<TIGER_INITIAL>"+"                      {adjust(); return PLUS;}
<TIGER_INITIAL>"-"                      {adjust(); return MINUS;}
<TIGER_INITIAL>"*"                      {adjust(); return TIMES;}
<TIGER_INITIAL>"/"                      {adjust(); return DIVIDE;}
<TIGER_INITIAL>"="                      {adjust(); return EQ;}
<TIGER_INITIAL>"<>"                     {adjust(); return NEQ;}
<TIGER_INITIAL>"<"                      {adjust(); return LT;}
<TIGER_INITIAL>"<="                     {adjust(); return LE;}
<TIGER_INITIAL>">"                      {adjust(); return GT;}
<TIGER_INITIAL>">="                     {adjust(); return GE;}
<TIGER_INITIAL>"&"                      {adjust(); return AND;}
<TIGER_INITIAL>"|"                      {adjust(); return OR;}
<TIGER_INITIAL>":="                     {adjust(); return ASSIGN;}
<TIGER_INITIAL>while                    {adjust(); return WHILE;}
<TIGER_INITIAL>for                      {adjust(); return FOR;}
<TIGER_INITIAL>to                       {adjust(); return TO;}
<TIGER_INITIAL>break                    {adjust(); return BREAK;}
<TIGER_INITIAL>let                      {adjust(); return LET;}
<TIGER_INITIAL>in                       {adjust(); return IN;}
<TIGER_INITIAL>end                      {adjust(); return END;}
<TIGER_INITIAL>function                 {adjust(); return FUNCTION;}
<TIGER_INITIAL>var                      {adjust(); return VAR;}
<TIGER_INITIAL>type                     {adjust(); return TYPE;}
<TIGER_INITIAL>array                    {adjust(); return ARRAY;}
<TIGER_INITIAL>if                       {adjust(); return IF;}
<TIGER_INITIAL>then                     {adjust(); return THEN;}
<TIGER_INITIAL>else                     {adjust(); return ELSE;}
<TIGER_INITIAL>do                       {adjust(); return DO;}
<TIGER_INITIAL>of                       {adjust(); return OF;}
<TIGER_INITIAL>nil                      {adjust(); return NIL;}
<TIGER_INITIAL>\"                       {adjust(); tiger_string_init(); BEGIN TIGER_STRING;}
<TIGER_STRING>\\n                       {adjust(); tiger_string_append("\n", 1);}
<TIGER_STRING>\"                        {adjust(); BEGIN TIGER_INITIAL; yylval.sval = tiger_string_inner(); return STRING;}
<TIGER_STRING>.                         {adjust(); tiger_string_append(yytext, yyleng);}
<TIGER_INITIAL>[a-zA-Z][_0-9a-zA-Z]*    {adjust(); yylval.sval = yytext; return ID;}
<TIGER_INITIAL>[0-9]+                   {adjust(); yylval.ival=atoi(yytext); return INT;}
<TIGER_INITIAL>"/*"                     {adjust(); BEGIN TIGER_COMMENT;}
<TIGER_INITIAL>.	                    {adjust(); EM_error(EM_tokPos,"illegal tokenillegal token");}
<TIGER_COMMENT>"*/"                     {adjust(); BEGIN TIGER_INITIAL;}
<TIGER_COMMENT>.                        {adjust();}
.                                       {BEGIN TIGER_INITIAL; yyless(0);}
