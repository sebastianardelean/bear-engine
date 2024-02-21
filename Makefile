CC = cl
LINKER = link
RC = rc

CFLAGS = /W3

DBGCFLAGS = $(CFLAGS) /DDEBUG /Zi /W3 # temporarly not Wall

RELCFLAGS = $(CFLAGS) /0x


EXE_SRC = demo.c
DLL_SRC = engine_dll.c engine_dll_error.c engine_dll_window.c engine_dll_core.c engine_dll_draw.c
RES_FILE = engine_resources.rc


DBGDIR = debug
RELDIR = release

EXE_NAME = demo.exe
DLL_NAME = bengine_dll.dll

all: clean debug release

debug: prep resource demo.c
	$(CC) $(DBGCFLAGS) $(EXE_SRC) /link user32.lib opengl32.lib Gdi32.lib Glu32.lib /out:$(DBGDIR)/$(EXE_NAME)
	$(CC) $(DBGCFLAGS) $(DLL_SRC) /LD /link user32.lib opengl32.lib Gdi32.lib Glu32.lib /DEF:engine_dll.def /out:$(DBGDIR)/$(DLL_NAME)

release: prep resource demo.c
	$(CC) $(RELCFLAGS) $(EXE_SRC) /link user32.lib opengl32.lib Gdi32.lib Glu32.lib /out:$(RELDIR)/$(EXE_NAME)	
	$(CC) $(RELCFLAGS) $(DLL_SRC) /LD /link user32.lib opengl32.lib Gdi32.lib Glu32.lib /DEF:engine_dll.def /out:$(RELDIR)/$(DLL_NAME)

resource: $(RES_FILE)
	$(RC) /fo $(RES_FILE).res $(RES_FILE)

prep:
	@echo off > temp.bat && \
	echo IF NOT EXIST $(DBGDIR) mkdir $(DBGDIR) >> temp.bat && \
	echo IF NOT EXIST $(RELDIR) mkdir $(RELDIR) >> temp.bat && \
	temp.bat && \
	del temp.bat


clean:
	del *.obj *.exe *.dll *.lib *.exp *.ilk *.pdb *.res

	@echo off > temp.bat && \
	echo IF EXIST $(DBGDIR) del /S /Q $(DBGDIR) >> temp.bat && \
	echo IF EXIST $(RELDIR) del /S /Q $(RELDIR) >> temp.bat && \
	echo IF EXIST $(DBGDIR) rd /S /Q $(DBGDIR) >> temp.bat && \
	echo IF EXIST $(RELDIR) rd /S /Q $(RELDIR) >> temp.bat && \
	temp.bat && \
	del temp.bat
