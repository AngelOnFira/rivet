#!/bin/sh
set -euf

docker cp k3d-rivet-dev-server-0:/output/kubeconfig.yaml .
k9s --kubeconfig kubeconfig.yaml --all-namespaces
