lextest: driver.o lex.yy.o errormsg.o util.o tiger_string.o
	cc -g -o lextest driver.o lex.yy.o errormsg.o util.o tiger_string.o

driver.o: driver.c tokens.h errormsg.h util.h
	cc -g -c driver.c

errormsg.o: errormsg.c errormsg.h util.h
	cc -g -c errormsg.c

lex.yy.o: lex.yy.c tokens.h errormsg.h util.h
	cc -g -c lex.yy.c

lex.yy.c: tiger.lex
	lex tiger.lex

util.o: util.c util.h
	cc -g -c util.c

tiger_string.o: tiger_string.c tiger_string.h
	cc -g -c tiger_string.c

clean: 
	rm -f lextest util.o driver.o lex.yy.o lex.yy.c errormsg.o tiger_string.o
