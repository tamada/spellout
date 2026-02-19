#! /bin/sh

get_crate_info() {
    cargo metadata --no-deps --format-version 1 | jq -r '.packages[0] | "\(.name) \(.version)"'
}

INFO=$(get_crate_info)
CRATE_NAME=${INFO%% *}
VERSION=${INFO#* }

for i in no_features_glibc no_features_musl unicode_normalization_glibc unicode_normalization_musl; do
    echo "Building image for $CRATE_NAME:$VERSION-$i..."
    docker buildx build --platform linux/amd64,linux/arm64 --push -t tamada/$CRATE_NAME:$VERSION-$i -f containers/$i/Containerfile .
done

# docker buildx build -t spellout:no_features_glibc   -f containers/no_features_glibc/Containerfile .
# docker buildx build -t spellout:no_features_musl    -f containers/no_features_musl/Containerfile .
# docker buildx build -t spellout:normalization_glibc -f containers/normalization_glibc/Containerfile .
# docker buildx build -t spellout:normalization_musl  -f containers/normalization_musl/Containerfile .
