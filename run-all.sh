PACKAGES=$(cargo metadata --format-version 1 | jq -r '.packages[] | .name' | grep "sol_2024_")
for package in $PACKAGES; do
    cargo run --release --quiet -p $package
done

echo ""

PACKAGES=$(cargo metadata --format-version 1 | jq -r '.packages[] | .name' | grep "sol_2023_")
for package in $PACKAGES; do
    cargo run --release --quiet -p $package
done
