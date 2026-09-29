call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
cd /d E:\nux\Nux_Lang\nux\nux_oleg\nux_dist
cargo run --bin nux -- build-native E:\nux\Nux_Lang\test_gpu.nux --target cuda -o E:\nux\Nux_Lang\test_gpu2
