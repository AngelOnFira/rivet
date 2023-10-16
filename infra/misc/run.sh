echo "$(pwd)"

# TODO: Update NETCONF_PATH to /opt/cni/net.d
export CNI_PATH="/opt/cni/bin"
export NETCONFPATH="/opt/cni/config"

OCI_IMAGE_PATH="$(pwd)/local/oci-image"
OCI_BUNDLE_PATH="$(pwd)/local/oci-bundle"

# Need to prefix with "rivet-" in order to not interfere with any
# auto-generated resources that Nomad creates for the given alloc ID
CONTAINER_ID="rivet-$NOMAD_ALLOC_ID"
echo "CONTAINER_ID: $CONTAINER_ID"


# MARK: Load container
echo "Converting Docker image -> OCI image"
time skopeo copy "docker-archive:local/docker-image/image.tar" "oci:$OCI_IMAGE_PATH:default"

# TODO: Remov hjke
# Install umoci
curl -Lf -o umoci 'https://github.com/opencontainers/umoci/releases/download/v0.4.7/umoci.amd64'
chmod +x umoci

# This allows us to run the bundle natively with runc
echo "Converting OCI image -> OCI bundle"
time ./umoci unpack --image "$OCI_IMAGE_PATH:default" "$OCI_BUNDLE_PATH"


# MARK: Create network
# Name of the network in /opt/cni/config/$NETWORK_NAME.conflist
NETWORK_NAME="rivet-job"
# Path to the created namespace
NETNS_PATH="/var/run/netns/$CONTAINER_ID"

echo "Creating network $NETWORK_NAME"
ip netns add "$CONTAINER_ID"

echo "Adding network $NETWORK_NAME to namespace $NETNS_PATH"
cnitool add "$NETWORK_NAME" "$NETNS_PATH"


# MARK: Config
# Copy the Docker-specific values from the OCI bundle config.json to the base config
#
# This way, we enforce our own capabilities on the container instead of trusting the
# provided config.json
echo "Templating config.json"
OVERRIDE_CONFIG="local/oci-bundle-config.overrides.json"
mv "$OCI_BUNDLE_PATH/config.json" "$OVERRIDE_CONFIG"
jq "
.process.args = $(jq '.process.args' $OVERRIDE_CONFIG) |
.process.env = $(jq '.process.env' $OVERRIDE_CONFIG) |
.process.user = $(jq '.process.user' $OVERRIDE_CONFIG) |
.process.cwd = $(jq '.process.cwd' $OVERRIDE_CONFIG) |
.linux.namespaces += [{\"type\": \"network\", \"path\": \"$NETNS_PATH\"}]
" local/oci-bundle-config.base.json > "$OCI_BUNDLE_PATH/config.json"


# MARK: Run container
echo "Running container"
runc run $CONTAINER_ID -b $OCI_BUNDLE_PATH


# TODO: Move this to poststop
# MARK: Cleanup
# Clean up: remove network and namespace (you may want to do this later or on failure)
cnitool del $NETWORK_NAME $NETNS_PATH
ip netns del "$CONTAINER_ID"

