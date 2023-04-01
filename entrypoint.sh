#!/bin/bash

set -m

/usr/local/bin/anime-ost &

nginx -g "daemon off;" &

wait -n

exit $?
