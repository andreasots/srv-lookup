target/libsrv.a: src/srv.c
	mkdir -p target
	$(CC) $(CPPFLAGS) $(CFLAGS) src/srv.c -c -o target/srv.o
	$(AR) rcs $@ target/srv.o 
