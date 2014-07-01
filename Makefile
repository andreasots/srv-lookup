target/deps/libsrv.a: src/srv.c
	mkdir -p target/deps
	$(CC) $(CPPFLAGS) $(CFLAGS) src/srv.c -c -o target/deps/srv.o
	$(AR) rcs $@ target/deps/srv.o 
