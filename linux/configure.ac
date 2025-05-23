#                                               -*- Autoconf -*-
# Process this file with autoconf to produce a configure script.

AC_PREREQ([2.71])
AC_INIT([CCExtractor], [0.94], [carlos@ccextractor.org])
AC_CONFIG_AUX_DIR([build-conf])
AC_CONFIG_SRCDIR([../src/ccextractor.c])
AM_INIT_AUTOMAKE([foreign subdir-objects])
AC_CONFIG_MACRO_DIRS([m4])

# Checks for programs.
AC_PROG_CC
AC_PROG_INSTALL
AC_PROG_MAKE_SET

#Checks for "pkg-config" utility
AC_MSG_CHECKING([pkg-config m4 macros])
if test m4_ifdef([PKG_CHECK_MODULES], [yes], [no]) = yes; then
	AC_MSG_RESULT([yes]);
else
	AC_MSG_RESULT([no]);
	AC_MSG_ERROR([
pkg-config is required.])
fi

# Checks for libraries.
AC_CHECK_LIB([m], [sin], [], [AC_MSG_ERROR(Math library not installed. Install it before proceeding.)])
AC_CHECK_LIB([leptonica], [getLeptonicaVersion], [HAS_LEPT=1 && PKG_CHECK_MODULES([lept], [lept])], [HAS_LEPT=0])
AC_CHECK_LIB([tesseract], [TessVersion], [HAS_TESSERACT=1 && PKG_CHECK_MODULES([tesseract], [tesseract])], [HAS_TESSERACT=0])
AC_CHECK_LIB([avcodec], [avcodec_version], [HAS_AVCODEC=1 && PKG_CHECK_MODULES([libavcodec], [libavcodec])], [HAS_AVCODEC=0])
AC_CHECK_LIB([avformat], [avformat_version], [HAS_AVFORMAT=1 && PKG_CHECK_MODULES([libavformat], [libavformat])], [HAS_AVFORMAT=0])
AC_CHECK_LIB([avutil], [avutil_version], [HAS_AVUTIL=1 && PKG_CHECK_MODULES([libavutil], [libavutil])], [HAS_AVUTIL=0])
AC_CHECK_LIB([swscale], [swscale_version], [HAS_SWSCALE=1 && PKG_CHECK_MODULES([libswscale], [libswscale])], [HAS_SWSCALE=0])

# Checks for header files.
AC_CHECK_HEADERS([arpa/inet.h fcntl.h float.h inttypes.h limits.h locale.h malloc.h netdb.h netinet/in.h stddef.h stdint.h stdlib.h string.h sys/socket.h sys/time.h sys/timeb.h termios.h unistd.h wchar.h])

# Checks for typedefs, structures, and compiler characteristics.
AC_CHECK_HEADER_STDBOOL
AC_C_INLINE
AC_TYPE_INT16_T
AC_TYPE_INT32_T
AC_TYPE_INT64_T
AC_TYPE_INT8_T
AC_TYPE_OFF_T
AC_TYPE_PID_T
AC_TYPE_SIZE_T
AC_TYPE_SSIZE_T
AC_TYPE_UINT16_T
AC_TYPE_UINT32_T
AC_TYPE_UINT64_T
AC_TYPE_UINT8_T
AC_CHECK_TYPES([ptrdiff_t])

# Checks for library functions.
AC_FUNC_ERROR_AT_LINE
AC_FUNC_FSEEKO
AC_FUNC_MALLOC
AC_FUNC_MKTIME
AC_FUNC_REALLOC
AC_FUNC_STRERROR_R
AC_CHECK_FUNCS([floor ftruncate gethostbyname gettimeofday inet_ntoa mblen memchr memmove memset mkdir modf pow realpath rmdir select setlocale socket sqrt strcasecmp strchr strdup strerror strndup strrchr strstr strtol])

# Checks for arguments with configure
AC_ARG_ENABLE([hardsubx],
AS_HELP_STRING([--enable-hardsubx], [Enables extraction of burnt subtitles (hard subtitles)]),
[case "${enableval}" in
  yes) hardsubx=true ;;
  no)  hardsubx=false ;;
  *) AC_MSG_ERROR([bad value ${enableval} for --enable-hardsubx]) ;;
esac],[hardsubx=false])

AC_ARG_ENABLE([ocr],
AS_HELP_STRING([--enable-ocr], [Enables Optical Character Recognition]),
[case "${enableval}" in
  yes) ocr=true ;;
  no)  ocr=false ;;
  *) AC_MSG_ERROR([bad value ${enableval} for --enable-ocr]) ;;
esac],[ocr=false])

AC_ARG_ENABLE([ffmpeg],
AS_HELP_STRING([--enable-ffmpeg], [Enable FFmpeg integration]),
[case "${enableval}" in
  yes) ffmpeg=true ;;
  no)  ffmpeg=false ;;
  *) AC_MSG_ERROR([bad value ${enableval} for --enable-ffmpeg]) ;;
esac],[ffmpeg=false])

#Add argument for rust
AC_ARG_WITH([rust],
AS_HELP_STRING([--with-rust], [Builds CCExtractor with rust library]),
[with_rust=$withval],
[with_rust=yes])

AC_MSG_CHECKING(whether to build with rust library)
if test "x$with_rust" = "xyes" ; then
    AC_MSG_RESULT(yes)

    #Check if cargo and rust is installed
    AC_PATH_PROG([CARGO], [cargo], [notfound])
    AS_IF([test "$CARGO" = "notfound"], [AC_MSG_ERROR([cargo is required])])

    AC_PATH_PROG([RUSTC], [rustc], [notfound])
    AS_IF([test "$RUSTC" = "notfound"], [AC_MSG_ERROR([rustc is required])])

    rustc_version=$(rustc --version)
    MSRV="1.54.0"
    AX_COMPARE_VERSION($rustc_version, [ge], [$MSRV],
      [AC_MSG_RESULT(rustc >= $MSRV)],
      [AC_MSG_ERROR([Minimum supported rust version(MSRV) is $MSRV, please upgrade rust])])
else
    AC_MSG_RESULT(no)
fi
AM_CONDITIONAL([WITH_RUST], [test "x$with_rust" = "xyes"])

AC_ARG_ENABLE(debug,
              AS_HELP_STRING([--enable-debug],
                             [Build Rust code with debugging information [default=no]]),
              [debug_release=$enableval],
              [debug_release=no])

AC_MSG_CHECKING(whether to build Rust code with debugging information)
if test "x$debug_release" = "xyes" ; then
    AC_MSG_RESULT(yes)
    RUST_TARGET_SUBDIR=debug
else
    AC_MSG_RESULT(no)
    RUST_TARGET_SUBDIR=release
fi
AM_CONDITIONAL([DEBUG_RELEASE], [test "x$debug_release" = "xyes"])

AC_SUBST([RUST_TARGET_SUBDIR])

#Checks and prompts if libraries found/not found to avoild failure while building
AS_IF([ test x$hardsubx = xtrue && test $HAS_AVCODEC -gt 0 ], [AC_MSG_NOTICE(avcodec library found)])
AS_IF([ test x$hardsubx = xtrue && test ! $HAS_AVCODEC -gt 0 ], [AC_MSG_ERROR(avcodec library not found. Please install the avcodec library before proceeding)])
AS_IF([ test x$hardsubx = xtrue && test $HAS_AVFORMAT -gt 0 ], [AC_MSG_NOTICE(avformat library found)])
AS_IF([ test x$hardsubx = xtrue && test ! $HAS_AVFORMAT -gt 0 ], [AC_MSG_ERROR(avformat library not found. Please install the avformat library before proceeding)])
AS_IF([ test x$hardsubx = xtrue && test $HAS_AVUTIL -gt 0 ], [AC_MSG_NOTICE(avutil library found)])
AS_IF([ test x$hardsubx = xtrue && test ! $HAS_AVUTIL -gt 0 ], [AC_MSG_ERROR(avutil library not found. Please install the avutil library before proceeding)])
AS_IF([ test x$hardsubx = xtrue && test $HAS_SWSCALE -gt 0 ], [AC_MSG_NOTICE(swscale library found)])
AS_IF([ test x$hardsubx = xtrue && test ! $HAS_SWSCALE -gt 0 ], [AC_MSG_ERROR(swscale library not found. Please install the swscale library before proceeding)])
AS_IF([ (test x$ocr = xtrue || test x$hardsubx = xtrue) && test $HAS_TESSERACT -gt 0 ], [TESS_VERSION=$(tesseract --version 2>&1 | grep tesseract) && AC_MSG_NOTICE(tesseract library found... $TESS_VERSION)])
AS_IF([ (test x$ocr = xtrue || test x$hardsubx = xtrue) && test ! $HAS_TESSERACT -gt 0 ], [AC_MSG_ERROR(tesserect library not found. Please install the tesseract library before proceeding)])
AS_IF([ (test x$ocr = xtrue || test x$hardsubx = xtrue) && test $HAS_LEPT -gt 0 ], [LEPT_VERSION=$(tesseract --version 2>&1 | grep leptonica) && AC_MSG_NOTICE(leptonica library found... $LEPT_VERSION)])
AS_IF([ (test x$ocr = xtrue || test x$hardsubx = xtrue) && test ! $HAS_LEPT -gt 0 ], [AC_MSG_ERROR(leptonica library not found. Please install the leptonica library before proceeding)])

#AM_CONDITIONAL(s) for setting values to enable/disable flags in Makefile.am
AM_CONDITIONAL(HARDSUBX_IS_ENABLED, [ test x$hardsubx = xtrue ])
AM_CONDITIONAL(OCR_IS_ENABLED, [ test x$ocr = xtrue || test x$hardsubx = xtrue ])
AM_CONDITIONAL(FFMPEG_IS_ENABLED, [ test x$ffmpeg = xtrue ])
AM_CONDITIONAL(TESSERACT_PRESENT, [ test ! -z  $(pkg-config --libs-only-l --silence-errors tesseract) ])
AM_CONDITIONAL(TESSERACT_PRESENT_RPI, [ test -d "/usr/include/tesseract" && test $(ls -A /usr/include/tesseract | wc -l) -gt 0 ])
AM_CONDITIONAL(SYS_IS_LINUX, [ test $(uname -s) = "Linux"])
AM_CONDITIONAL(SYS_IS_MAC, [ test $(uname -s) = "Darwin"])
AM_CONDITIONAL(SYS_IS_APPLE_SILICON, [ test $(uname -a | awk '{print $NF}') = "arm64" ])
AM_CONDITIONAL(SYS_IS_64_BIT,[test $(getconf LONG_BIT) = "64"])

AC_CONFIG_FILES([Makefile])
AC_OUTPUT
