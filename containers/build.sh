#! /bin/sh

get_crate_info() {
    cargo metadata --no-deps --format-version 1 | jq -r '.packages[0] | "\(.name) \(.version)"'
}

INFO=$(get_crate_info)
CRATE_NAME=${INFO%% *}
VERSION=${INFO#* }

echo "Building image for $CRATE_NAME:$VERSION-no_features_glibc..."
docker buildx build --platform linux/amd64,linux/arm64 --push \
    -t ghcr.io/tamada/$CRATE_NAME:$VERSION-no_features_glibc \
    -t ghcr.io/tamada/$CRATE_NAME:$VERSION-glibc \
    -t ghcr.io/tamada/$CRATE_NAME:$VERSION \
    -f containers/no_features_glibc/Containerfile .

echo "Building image for $CRATE_NAME:$VERSION-no_features_musl..."
docker buildx build --platform linux/amd64,linux/arm64 --push \
    -t ghcr.io/tamada/$CRATE_NAME:$VERSION-no_features_musl \
    -t ghcr.io/tamada/$CRATE_NAME:$VERSION-musl \
    -f containers/no_features_musl/Containerfile .

echo "Building image for $CRATE_NAME:$VERSION-unicode_normalization_glibc..."
docker buildx build --platform linux/amd64,linux/arm64 --push \
    -t ghcr.io/tamada/$CRATE_NAME:$VERSION-unicode_normalization_glibc \
    -f containers/unicode_normalization_glibc/Containerfile .

echo "Building image for $CRATE_NAME:$VERSION-unicode_normalization_musl..."
docker buildx build --platform linux/amd64,linux/arm64 --push \
    -t ghcr.io/tamada/$CRATE_NAME:$VERSION-unicode_normalization_musl \
    -f containers/unicode_normalization_musl/Containerfile .

# docker buildx build -t spellout:no_features_glibc   -f containers/no_features_glibc/Containerfile .
# docker buildx build -t spellout:no_features_musl    -f containers/no_features_musl/Containerfile .
# docker buildx build -t spellout:normalization_glibc -f containers/normalization_glibc/Containerfile .
# docker buildx build -t spellout:normalization_musl  -f containers/normalization_musl/Containerfile .
