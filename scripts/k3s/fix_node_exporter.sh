#!/bin/sh
set -euf

docker exec k3d-rivet-dev-server-0 mount --make-rshared /
