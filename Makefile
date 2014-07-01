target/libsrv.a: src/srv.c
	$(CC) $(CPPFLAGS) $(CFLAGS) src/srv.c -c -o target/srv.o
	$(AR) rcs $@ target/srv.o 
