# Get all packages from cargo metadata that match the solution pattern
PACKAGES=$(cargo metadata --format-version 1 | jq -r '.packages[] | .name' | grep "sol_2024_")
for package in $PACKAGES; do
    cargo run --release --quiet -p $package
done
