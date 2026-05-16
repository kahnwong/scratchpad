#!/bin/sh
if [ ! -e /dev/dri ] || [ -z "$(ls /dev/dri/ 2>/dev/null)" ]; then
	export GSK_RENDERER=cairo
fi
exec /app/bin/scratchpad "$@"
