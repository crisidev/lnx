# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src
    local stage
    local lnx="lnx"
    src=$(pwd)

    case ${TRAVIS_OS_NAME} in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    # generate cargo lockfile
    test -f Cargo.lock || cargo generate-lockfile

    # build the executables we want to release
    cross rustc --bin "${lnx}" --target "${TARGET}" --release

    # change binary name in case we are on windows
    if [ "${TARGET}" = "x86_64-pc-windows-gnu" ]; then
      lnx="${lnx}.exe"
    fi
    cp "target/${TARGET}/release/${lnx}" "${stage}"/

    cd "${stage}"
    tar czf "${src}/${CRATE_NAME}-${TRAVIS_TAG}-${TARGET}.tar.gz" *
    cd "${src}"

    rm -rf "${stage}"
}

main
