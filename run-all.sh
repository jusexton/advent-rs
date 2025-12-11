PACKAGES=$(cargo metadata --format-version 1 | jq -r '.packages[] | .name' | grep "sol_")
for package in $PACKAGES; do
    cargo run --release --quiet -p $package
done
