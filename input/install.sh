# Install dependencies
brew install hdf5;
brew install netcdf;
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh;

# Build from source and install binaries
cargo install --path . --all-features
