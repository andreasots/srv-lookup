$(DEPS_DIR)/libsrv.a: src/srv.c
	$(CC) $(CPPFLAGS) $(CFLAGS) src/srv.c -c -o $(DEPS_DIR)/srv.o
	$(AR) rcs $@ $(DEPS_DIR)/srv.o
