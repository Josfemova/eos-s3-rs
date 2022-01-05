#!/bin/sh

(cd pac; cargo clean; cargo fmt; cargo build)
(cd hal; cargo clean; cargo fmt; cargo build)
(cd bsp/quickfeather; cargo clean; cargo fmt; cargo build)
(cd bsp/quicklogic-thing-plus; cargo clean; cargo fmt; cargo build)
(cd bsp/qomu; cargo clean; cargo fmt; cargo build)