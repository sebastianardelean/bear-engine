CC = cl
LINKER = link
RC = rc

CFLAGS = /W3 /D_UNICODE /DUNICODE

DBGCFLAGS = $(CFLAGS) /DDEBUG /Zi /W3 # temporarly not Wall

RELCFLAGS = $(CFLAGS)


EXE_SRC = demo.cpp
DLL_SRC = engine_dll.cpp engine_dll_error.cpp engine_dll_window.cpp engine_dll_core.cpp engine_dll_draw.cpp engine_dll_texture.cpp
RES_FILE = engine_resources.rc


DBGDIR = debug
RELDIR = release

EXE_NAME = demo.exe
DLL_NAME = bengine_dll.dll

LIBS = user32.lib opengl32.lib Gdi32.lib Glu32.lib

all: clean debug release

debug: prep resource demo.cpp
	$(CC) $(DBGCFLAGS) $(EXE_SRC) $(RES_FILE).res /link $(LIBS) /out:$(DBGDIR)/$(EXE_NAME)
	$(CC) $(DBGCFLAGS) /D_WINDLL $(DLL_SRC) /LD /link $(LIBS) /DEF:engine_dll.def /out:$(DBGDIR)/$(DLL_NAME)

release: prep resource demo.cpp
	$(CC) $(RELCFLAGS) $(EXE_SRC) $(RES_FILE).res /link $(LIBS) /out:$(RELDIR)/$(EXE_NAME)	
	$(CC) $(RELCFLAGS) /D_WINDLL $(DLL_SRC) /LD /link $(LIBS) /DEF:engine_dll.def /out:$(RELDIR)/$(DLL_NAME)

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
