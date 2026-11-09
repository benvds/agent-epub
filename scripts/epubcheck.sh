#!/bin/sh
set -eu
: "${EPUBCHECK_JAR:?Set EPUBCHECK_JAR to the official epubcheck.jar path}"
exec "${EPUBCHECK_JAVA:-java}" -jar "$EPUBCHECK_JAR" "$@"
